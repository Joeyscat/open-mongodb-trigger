use std::collections::HashMap;

use abi::{trigger::DatabaseOperationType, TriggerConfig, TriggerType};
use bson::{doc, Document};
use futures::StreamExt;
use tokio::task::JoinHandle;
use tonic::async_trait;

use crate::func::call_func;

#[derive(Debug)]
pub struct Watcher {
    pub trigger: abi::Trigger,
}

#[async_trait]
pub trait WatcherManager {
    fn new() -> Self;
    async fn add_watcher(&mut self, key: String, w: Watcher) -> Result<(), anyhow::Error>;
    async fn remove_watcher(&mut self, key: String) -> Result<(), anyhow::Error>;
}

pub struct DefaultWatcherManager {
    watchers: HashMap<String, JoinHandle<()>>,
}

#[async_trait]
impl WatcherManager for DefaultWatcherManager {
    fn new() -> Self {
        let watchers = HashMap::new();
        Self { watchers }
    }

    async fn add_watcher(&mut self, key: String, w: Watcher) -> Result<(), anyhow::Error> {
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

                let j = tokio::spawn(async move {
                    while let Some(event) = trigger_cs.next().await.transpose().unwrap() {
                        // load the function and call it
                        call_func(w.trigger.function_id.clone(), event).await;
                    }
                });

                self.watchers.insert(key, j);

                Ok(())
            }
            (_, _) => Err(abi::Error::UnsupportedTriggerType(tt as i32).into()),
        }
    }

    async fn remove_watcher(&mut self, key: String) -> Result<(), anyhow::Error> {
        if let Some(j) = self.watchers.remove(&key) {
            j.abort()
        }

        Ok(())
    }
}
