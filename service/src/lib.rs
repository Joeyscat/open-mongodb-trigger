mod func;
mod function_service;
mod trigger_service;
mod watcher;

#[cfg(test)]
pub mod test_utils;

use std::pin::Pin;

use abi::{
    trigger::trigger_service_server::TriggerServiceServer, trigger::Trigger, Config, Function,
    FunctionServiceServer,
};
use bson::doc;
use chrono::Duration;
use function::DefaultFunctionManager;
use futures::{Stream, StreamExt, TryStreamExt};
use mongodb::{change_stream::event::OperationType, options::ChangeStreamOptions, Collection};
use tokio::{runtime::Runtime, sync::mpsc};
use tonic::{transport::Server, Status};
use tracing::{debug, error, info, warn};
use trigger::DefaultTriggerManager;
use watcher::WatcherManager;

use crate::watcher::{DefaultWatcherManager, Watcher};

pub struct TrigrService {
    manager: DefaultTriggerManager,
    function_manager: DefaultFunctionManager,
}

pub struct FuncService {
    manager: DefaultFunctionManager,
}

pub struct TonicReceiverStream<T> {
    inner: mpsc::Receiver<Result<T, abi::Error>>,
}

impl<T> TonicReceiverStream<T> {
    pub fn new(inner: mpsc::Receiver<Result<T, abi::Error>>) -> Self {
        Self { inner }
    }
}

pub type TriggerStream = Pin<Box<dyn Stream<Item = Result<Trigger, Status>> + Send>>;

pub type FunctionStream = Pin<Box<dyn Stream<Item = Result<Function, Status>> + Send>>;

pub async fn start_server(config: &Config) -> Result<(), anyhow::Error> {
    let config_cloned = config.clone();
    let db = mongodb::Client::with_uri_str(config.db.mongodb_uri.clone())
        .await
        .unwrap()
        .database(&config.db.db);
    let trigger_coll = db.collection::<trigger::Trigger>(&config.db.trigger_coll);
    let function_coll = db.collection::<function::Function>(&config.db.function_coll);
    let trigr_manager = DefaultTriggerManager::new(trigger_coll.clone());
    let func_manager = DefaultFunctionManager::new(function_coll.clone());

    tokio::spawn(async move {
        // change listener
        watch(&config_cloned, trigger_coll, function_coll)
            .await
            .unwrap();
    });

    let trigr_svc =
        TriggerServiceServer::new(TrigrService::new(trigr_manager, func_manager.clone()));
    let func_svc = FunctionServiceServer::new(FuncService::new(func_manager));

    let addr = format!("{}:{}", config.server.host, config.server.port).parse()?;
    info!("Listening on {}", addr);
    // api services
    Server::builder()
        .add_service(trigr_svc)
        .add_service(func_svc)
        .serve(addr)
        .await?;

    Ok(())
}

pub async fn watch(
    config: &Config,
    trigger_coll: Collection<trigger::Trigger>,
    function_coll: Collection<function::Function>,
) -> Result<(), anyhow::Error> {
    let mut wm =
        DefaultWatcherManager::new(DefaultFunctionManager::new(function_coll.clone())).await?;

    start_enabled_watchers(trigger_coll.clone(), wm.clone()).await?;
    let timer = timer::Timer::new();
    let repeat_seconds = config.watcher.scan_delay_seconds as i64;
    let guard = {
        let rt = Runtime::new().unwrap();
        let x = trigger_coll.clone();
        let y = wm.clone();
        timer.schedule_repeating(Duration::seconds(repeat_seconds), move || {
            info!("start_enabled_watchers timer tick");
            let x = x.clone();
            let y = y.clone();
            match rt.block_on(async move { start_enabled_watchers(x, y).await }) {
                Ok(_) => {}
                Err(e) => error!("start_enabled_watchers error: {:?}", e),
            }
        })
    };
    info!("start_enabled_watchers timer setup");

    info!("start watching trigger change");
    let mut trigger_cs = trigger_coll
        .watch(
            None,
            ChangeStreamOptions::builder()
                .full_document(Some(mongodb::options::FullDocumentType::UpdateLookup))
                .build(),
        )
        .await
        .unwrap();

    while let Some(event) = trigger_cs.next().await.transpose()? {
        debug!(
            "operation performed: {:?}, event: {:?}",
            event.operation_type, event
        );
        // operation performed: Insert, document: Some(Document({"x": Int32(1)}))

        match (event.operation_type, event.full_document) {
            (OperationType::Insert, Some(trigger)) => {
                // create ChangeStream for the new trigger
                info!("new trigger: {:?}", trigger);
                if !trigger.enabled {
                    continue;
                }
                if trigger.id.is_none() {
                    warn!("full_document missing '_id'");
                    continue;
                }
                let key = trigger.id.unwrap().to_hex();
                let w = Watcher {
                    trigger: trigger.convert_to_abi_trigger()?,
                    j: None,
                };
                match wm.add(key, w).await {
                    Ok(()) => {}
                    Err(e) => warn!("add watcher error: {}", e),
                };
            }
            (OperationType::Update, Some(trigger)) => {
                info!("update trigger: {:?}", event.update_description);
                if event.update_description.is_none() {
                    warn!("update event missing 'update_description'");
                    continue;
                }

                let update_description = event.update_description.unwrap();
                // check if enabled/fucntion_id updated
                if update_description.updated_fields.contains_key("enabled") {
                    let key = trigger.id.unwrap().to_hex();
                    // remove old then add new
                    info!("remove watcher: {}", key);
                    match wm.remove(key.clone()).await {
                        Ok(_) => {}
                        Err(e) => error!("remove watcher error: {}", e),
                    };
                    if update_description
                        .updated_fields
                        .get_bool("enabled")
                        .unwrap()
                    {
                        let w = Watcher {
                            trigger: trigger.convert_to_abi_trigger()?,
                            j: None,
                        };
                        info!("add watcher: {:?}", w);
                        match wm.add(key, w).await {
                            Ok(()) => {}
                            Err(e) => warn!("add watcher error: {}", e),
                        };
                    }
                }
            }
            (OperationType::Delete, Some(trigger)) => {
                // remove ChangeStream for that trigger
                info!("delete trigger: {:?}", trigger);
                if trigger.id.is_none() {
                    warn!("full_document missing '_id'");
                    continue;
                }
                let key = trigger.id.unwrap().to_hex();
                match wm.remove(key.clone()).await {
                    Ok(_) => {}
                    Err(e) => error!("remove watcher error: {}", e),
                };
            }
            (OperationType::Insert | OperationType::Update | OperationType::Delete, _) => {
                warn!("ChangeStream Event missing 'full_document'!")
            }
            _ => {
                // don't care
            }
        };
    }

    drop(guard);
    error!("watch end");
    Ok(())
}

/// check if there are any enabled triggers that have not add to WatcherManager,
/// then add them.
pub async fn start_enabled_watchers<T>(
    trigger_coll: Collection<trigger::Trigger>,
    mut wm: T,
) -> Result<(), anyhow::Error>
where
    T: WatcherManager,
{
    let filter = doc! {"enabled":true};
    let mut cursor = trigger_coll.find(filter, None).await?;

    loop {
        let ret = cursor.try_next().await?;

        match ret {
            Some(trigger) => {
                let key = trigger.id.unwrap().to_hex();
                if wm.get(key.clone()).await.is_some() {
                    continue;
                }
                info!("watcher({}) has not started", &key);
                let w = Watcher {
                    trigger: trigger.convert_to_abi_trigger()?,
                    j: None,
                };
                info!("add watcher: {:?}", w);
                match wm.add(key, w).await {
                    Ok(()) => {}
                    Err(e) => warn!("add watcher error: {}", e),
                };
            }
            None => {
                break;
            }
        }
    }
    Ok(())
}
