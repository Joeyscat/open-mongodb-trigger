use core::time;
use std::collections::HashMap;

use abi::{trigger::DatabaseOperationType, TriggerConfig, TriggerType};
use bson::{doc, Document};
use function::FunctionManager;
use futures::StreamExt;
use tokio::task::JoinHandle;
use tonic::async_trait;

use crate::func;

#[derive(Debug)]
pub struct Watcher {
    pub trigger: abi::Trigger,
}

#[async_trait]
pub trait WatcherManager {
    async fn add(&mut self, key: String, w: Watcher) -> Result<(), anyhow::Error>;
    async fn get(&mut self, key: String) -> Option<&JoinHandle<()>>;
    async fn remove(&mut self, key: String) -> Result<(), anyhow::Error>;
}

pub struct DefaultWatcherManager<FM: FunctionManager + Clone + Send + Sync> {
    watchers: HashMap<String, JoinHandle<()>>,
    function_manager: FM,
}

impl<FM: FunctionManager + Clone + Send + Sync> DefaultWatcherManager<FM> {
    pub async fn new(fm: FM) -> Result<Self, anyhow::Error> {
        let watchers = HashMap::new();
        Ok(Self {
            watchers,
            function_manager: fm,
        })
    }
}

#[async_trait]
impl<FM: FunctionManager + Clone + Send + Sync + 'static> WatcherManager
    for DefaultWatcherManager<FM>
{
    async fn add(&mut self, key: String, w: Watcher) -> Result<(), anyhow::Error> {
        if self.watchers.contains_key(&key) {
            return Ok(());
        }

        let tt = TriggerType::from_i32(w.trigger.trigger_type).unwrap();
        match (tt, w.trigger.trigger_config) {
            (TriggerType::Database, Some(TriggerConfig::Database(config))) => {
                let db = mongodb::Client::with_uri_str(config.data_source)
                    .await
                    .unwrap()
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

                let fm = self.function_manager.clone();
                let j = tokio::spawn(async move {
                    while let Some(event) = trigger_cs.next().await.transpose().unwrap() {
                        // load the function and call it
                        func::call_func(w.trigger.function_id.clone(), event, fm.clone()).await;
                    }
                });

                self.watchers.insert(key, j);

                Ok(())
            }
            (_, _) => Err(abi::Error::UnsupportedTriggerType(tt as i32).into()),
        }
    }

    async fn get(&mut self, key: String) -> Option<&JoinHandle<()>> {
        self.watchers.get(&key)
    }

    async fn remove(&mut self, key: String) -> Result<(), anyhow::Error> {
        if let Some(j) = self.watchers.get(&key) {
            j.abort();

            while !j.is_finished() {
                tokio::time::sleep(time::Duration::from_millis(100)).await;
            }
        }
        self.watchers.remove(&key);

        Ok(())
    }
}
