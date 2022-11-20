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
use function::DefaultFunctionManager;
use futures::{Stream, StreamExt};
use mongodb::{change_stream::event::OperationType, options::ChangeStreamOptions};
use tokio::sync::mpsc;
use tonic::{transport::Server, Status};
use tracing::{debug, info, warn};
use trigger::DefaultTriggerManager;
use watcher::WatcherManager;

use crate::watcher::{DefaultWatcherManager, Watcher};

pub struct TrigrService {
    manager: DefaultTriggerManager,
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
    tokio::spawn(async move {
        // change listener
        let wm = DefaultWatcherManager::new();
        watch(&config_cloned, wm).await.unwrap();
    });

    let addr = format!("{}:{}", config.server.host, config.server.port).parse()?;

    let trigr_svc = TriggerServiceServer::new(TrigrService::from_config(config).await?);
    let func_svc = FunctionServiceServer::new(FuncService::from_config(config).await?);

    info!("Listening on {}", addr);
    // api services
    Server::builder()
        .add_service(trigr_svc)
        .add_service(func_svc)
        .serve(addr)
        .await?;

    Ok(())
}

pub async fn watch<T>(config: &Config, mut wm: T) -> Result<(), abi::Error>
where
    T: WatcherManager,
{
    let db = mongodb::Client::with_uri_str(config.db.mongodb_uri.clone())
        .await
        .unwrap()
        .database(&config.db.db);

    let trigger_coll = db.collection::<trigger::Trigger>(&config.db.trigger_coll);
    // let function_coll = db.collection::<function::Function>(&config.db.function_coll);

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
                };
                match wm.add_watcher(key, w).await {
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
                    match wm.remove_watcher(key.clone()).await {
                        Ok(()) => {}
                        Err(e) => warn!("remove watcher error: {}", e),
                    };
                    if update_description
                        .updated_fields
                        .get_bool("enabled")
                        .unwrap()
                    {
                        let w = Watcher {
                            trigger: trigger.convert_to_abi_trigger()?,
                        };
                        info!("add watcher: {:?}", w);
                        match wm.add_watcher(key, w).await {
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
                match wm.remove_watcher(key.clone()).await {
                    Ok(()) => {}
                    Err(e) => warn!("remove watcher error: {}", e),
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

    Ok(())
}
