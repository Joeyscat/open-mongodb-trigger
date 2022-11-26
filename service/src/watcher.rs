use std::sync::Arc;

use abi::{trigger::DatabaseOperationType, TriggerConfig, TriggerType};
use anyhow::anyhow;
use bson::{doc, Document};
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use function::FunctionManager;
use futures::StreamExt;
use tokio::task::JoinHandle;
use tonic::async_trait;

use crate::func;

#[derive(Clone, Debug)]
pub struct Watcher {
    pub trigger: abi::Trigger,
    pub j: Option<Arc<JoinHandle<()>>>,
}

#[async_trait]
pub trait WatcherManager {
    /// Add the watcher, if exists, return Ok() directly
    async fn add(&mut self, key: String, w: Watcher) -> Result<(), anyhow::Error>;
    /// Get Watcher by the key
    async fn get(&self, key: String) -> Option<Ref<String, Watcher>>;
    /// Stop the Watcher corresponding to the key, and then remove it from the manager
    async fn remove(&mut self, key: String) -> Result<Option<Watcher>, anyhow::Error>;
}

#[derive(Clone, Debug)]
pub struct DefaultWatcherManager<FM: FunctionManager + Clone + Send + Sync> {
    watchers: Arc<DashMap<String, Watcher>>,
    function_manager: FM,
}

impl<FM: FunctionManager + Clone + Send + Sync> DefaultWatcherManager<FM> {
    pub async fn new(fm: FM) -> Result<Self, anyhow::Error> {
        let watchers = DashMap::new();
        Ok(Self {
            watchers: Arc::new(watchers),
            function_manager: fm,
        })
    }
}

#[async_trait]
impl<FM: FunctionManager + Clone + Send + Sync + 'static> WatcherManager
    for DefaultWatcherManager<FM>
{
    async fn add(&mut self, key: String, mut w: Watcher) -> Result<(), anyhow::Error> {
        if self.watchers.contains_key(&key) {
            return Ok(());
        }
        if w.j.is_some() {
            return Err(anyhow!("new watcher should not has a JoinHandle"));
        }

        let tt = TriggerType::from_i32(w.trigger.trigger_type).unwrap();
        match (tt, w.trigger.trigger_config.clone()) {
            (TriggerType::Database, Some(TriggerConfig::Database(config))) => {
                let db = mongodb::Client::with_uri_str(config.data_source)
                    .await?
                    .database(&config.database);
                let coll = db.collection::<Document>(&config.collection);

                let op_types: Vec<&str> = config
                    .operation_types
                    .iter()
                    .map(|i| match DatabaseOperationType::from_i32(*i) {
                        Some(DatabaseOperationType::Unknown) => "",
                        Some(DatabaseOperationType::Insert) => "insert",
                        Some(DatabaseOperationType::Update) => "update",
                        Some(DatabaseOperationType::Replace) => "replace",
                        Some(DatabaseOperationType::Delete) => "delete",
                        None => "",
                    })
                    .filter(|x| !x.is_empty())
                    .collect();

                let mut pipeline = Vec::new();
                match op_types.len() {
                    0 => {}
                    1 => pipeline.push(doc! {"$match": {"operationType": op_types[0]}}),
                    _ => pipeline.push(doc! {"$match": {"operationType": {"$in": op_types}}}),
                };

                let mut trigger_cs = coll.watch(pipeline, None).await.unwrap();

                let fm_cloned = self.function_manager.clone();
                let function_id = w.trigger.function_id.clone();
                let j = tokio::spawn(async move {
                    while let Some(event) = trigger_cs.next().await.transpose().unwrap() {
                        // load the function and call it
                        func::call_func(function_id.clone(), event, fm_cloned.clone()).await;
                    }
                });
                w.j = Some(Arc::new(j));

                self.watchers.insert(key, w);

                Ok(())
            }
            (_, _) => Err(abi::Error::UnsupportedTriggerType(tt as i32).into()),
        }
    }

    async fn get(&self, key: String) -> Option<Ref<String, Watcher>> {
        self.watchers.get(&key)
    }

    async fn remove(&mut self, key: String) -> Result<Option<Watcher>, anyhow::Error> {
        if let Some((_, w)) = self.watchers.remove(&key) {
            match &w.j {
                Some(j) => {
                    j.abort();
                }
                None => {
                    return Err(anyhow!(
                        "the watcher({}) that are being removed has no JoinHandle",
                        key
                    ));
                }
            }
            return Ok(Some(w));
        }

        Ok(None)
    }
}
