use async_trait::async_trait;
use bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use mongodb::{
    options::{
        Acknowledgment, FindOneAndUpdateOptions, FindOneOptions, IndexOptions, InsertOneOptions,
        WriteConcern,
    },
    Collection, IndexModel,
};
use tokio::sync::mpsc;
use tracing::warn;

use crate::{DefaultTriggerManager, Trigger, TriggerManager};

#[async_trait]
impl TriggerManager for DefaultTriggerManager {
    async fn create(&self, mut trigger: abi::Trigger) -> Result<abi::Trigger, abi::Error> {
        let t = Trigger::convert_from_abi_trigger(trigger.clone())?;

        let r = self
            .coll
            .insert_one(
                t,
                InsertOneOptions::builder()
                    .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
                    .build(),
            )
            .await?;
        let id = r
            .inserted_id
            .as_object_id()
            .ok_or(abi::Error::Unknown)?
            .to_hex();
        trigger.id = id;

        Ok(trigger)
    }

    async fn change_function(
        &self,
        id: abi::TriggerId,
        function_id: abi::FunctionId,
    ) -> Result<abi::Trigger, abi::Error> {
        let r = self
            .coll
            .find_one_and_update(
                doc! {"_id": ObjectId::parse_str(id)?},
                doc! {
                    "$set": {"function_id": function_id}
                },
                FindOneAndUpdateOptions::builder()
                    .return_document(Some(mongodb::options::ReturnDocument::After))
                    .build(),
            )
            .await?;

        match r {
            Some(tr) => Ok(tr.convert_to_abi_trigger()?),
            None => Err(abi::Error::TriggerNotFound),
        }
    }

    async fn enable(&self, id: abi::TriggerId) -> Result<abi::Trigger, abi::Error> {
        let r = self
            .coll
            .find_one_and_update(
                doc! {"_id": ObjectId::parse_str(id)?},
                doc! {
                    "$set": {"enabled": true}
                },
                FindOneAndUpdateOptions::builder()
                    .return_document(Some(mongodb::options::ReturnDocument::After))
                    .build(),
            )
            .await?;

        match r {
            Some(tr) => Ok(tr.convert_to_abi_trigger()?),
            None => Err(abi::Error::TriggerNotFound),
        }
    }

    async fn disable(&self, id: abi::TriggerId) -> Result<abi::Trigger, abi::Error> {
        let r = self
            .coll
            .find_one_and_update(
                doc! {"_id": ObjectId::parse_str(id)?},
                doc! {
                    "$set": {"enabled": false}
                },
                FindOneAndUpdateOptions::builder()
                    .return_document(Some(mongodb::options::ReturnDocument::After))
                    .build(),
            )
            .await?;

        match r {
            Some(tr) => Ok(tr.convert_to_abi_trigger()?),
            None => Err(abi::Error::TriggerNotFound),
        }
    }

    async fn delete(&self, id: abi::TriggerId) -> Result<abi::Trigger, abi::Error> {
        let r = self
            .coll
            .find_one_and_delete(doc! {"_id": ObjectId::parse_str(id)?}, None)
            .await?;

        match r {
            Some(tr) => Ok(tr.convert_to_abi_trigger()?),
            None => Err(abi::Error::TriggerNotFound),
        }
    }

    async fn get(&self, id: abi::TriggerId) -> Result<abi::Trigger, abi::Error> {
        let r = self
            .coll
            .find_one(
                doc! {"_id": ObjectId::parse_str(id)?},
                FindOneOptions::default(),
            )
            .await?;

        match r {
            Some(tr) => Ok(tr.convert_to_abi_trigger()?),
            None => Err(abi::Error::TriggerNotFound),
        }
    }

    async fn query(
        &self,
        query: abi::TriggerQuery,
    ) -> mpsc::Receiver<Result<abi::Trigger, abi::Error>> {
        let filter = query.to_filter();
        let mut cursor = self.coll.find(filter, None).await.unwrap();

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            loop {
                let ret = cursor.try_next().await;
                if ret.is_err() {
                    warn!("Query error {:?}", ret);
                    break;
                }
                match ret.unwrap() {
                    Some(trigger) => {
                        let tr = trigger.convert_to_abi_trigger();
                        if tr.is_err() {
                            break;
                        }
                        if tx.send(Ok(tr.unwrap())).await.is_err() {
                            // rx is dropped, so client disconnected
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        });

        rx
    }
}

impl DefaultTriggerManager {
    pub fn new(coll: Collection<Trigger>) -> Self {
        Self { coll }
    }

    pub async fn from_config(config: &abi::DbConfig) -> Result<Self, abi::Error> {
        let db = mongodb::Client::with_uri_str(config.mongodb_uri.clone())
            .await?
            .database(&config.db);

        let coll = db.collection::<Trigger>(&config.trigger_coll);

        create_indexes(coll.clone()).await?;

        Ok(Self::new(coll))
    }
}

async fn create_indexes(coll: Collection<Trigger>) -> Result<(), abi::Error> {
    let index = IndexModel::builder()
        .keys(doc! {
            "user_id":1,
            "name":1
        })
        .options(IndexOptions::builder().unique(true).build())
        .build();
    let _ = coll.create_index(index, None).await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use abi::trigger::DatabaseConfig;
    use chrono::Local;
    use mongodb::{Collection, Database};

    use crate::{DefaultTriggerManager, Trigger, TriggerManager};

    use super::create_indexes;

    struct TestContext {
        db: Database,
    }

    async fn setup() -> TestContext {
        let db = mongodb::Client::with_uri_str("mongodb://test:test@127.0.0.1:27017")
            .await
            .unwrap()
            .database("trigger_unittest");
        TestContext { db }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            // println!("[drop test database: {}]", self.db.name());
            // let _ = self.db.drop(None);
            // std::thread::sleep(std::time::Duration::from_micros(1000));
            // println!("[drop end]");
        }
    }

    async fn get_coll(database: Database, coll_name: &str) -> Collection<Trigger> {
        let x = Local::now().timestamp_nanos();
        let coll = database.collection::<Trigger>(format!("trigger_{}_{}", coll_name, x).as_str());
        create_indexes(coll.clone()).await.unwrap();
        coll
    }

    #[tokio::test]
    async fn create_should_work_for_valid_trigger() {
        let context = setup().await;
        let coll = get_coll(context.db.clone(), "create1").await;
        let (rsvp, _manager) = make_a_trigger(coll).await;
        assert!(!rsvp.id.is_empty());
        println!("id: {:?}", rsvp.id);
    }

    #[tokio::test]
    async fn get_trigger_should_work() {
        let context = setup().await;
        let coll = get_coll(context.db.clone(), "get").await;
        let (rsvp, manager) = make_a_trigger(coll).await;
        let rsvp1 = manager.get(rsvp.id.clone()).await.unwrap();
        assert_eq!(rsvp, rsvp1);
    }

    #[tokio::test]
    async fn query_trigger_should_work() {
        let context = setup().await;
        let coll = get_coll(context.db.clone(), "query").await;
        let (_rsvp, manager) = make_a_trigger(coll).await;
        let q = abi::TriggerQuery {
            user_id: "aliceid".to_string(),
            ..Default::default()
        };
        let mut rx = manager.query(q).await;

        assert!(rx.recv().await.is_some());
        assert!(rx.recv().await.is_none());
    }

    #[tokio::test]
    async fn delete_trigger_should_work() {
        let context = setup().await;
        let coll = get_coll(context.db.clone(), "delete").await;
        let (rsvp, manager) = make_a_trigger(coll).await;
        let r = manager.delete(rsvp.id.clone()).await;
        assert!(r.is_ok());
        let r = manager.get(rsvp.id.clone()).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn disable_and_enable_trigger_should_work() {
        let context = setup().await;
        let coll = get_coll(context.db.clone(), "disable_and_enable").await;
        let (rsvp, manager) = make_a_trigger(coll).await;
        let r = manager.disable(rsvp.id.clone()).await;
        assert!(r.is_ok());
        assert!(!r.unwrap().enabled);
        let r = manager.enable(rsvp.id.clone()).await;
        assert!(r.is_ok());
        assert!(r.unwrap().enabled);
    }

    #[tokio::test]
    async fn change_trigger_function_should_work() {
        let context = setup().await;
        let coll = get_coll(context.db.clone(), "change").await;
        let (rsvp, manager) = make_a_trigger(coll).await;
        let r = manager
            .change_function(rsvp.id.clone(), "xxxxx".to_string())
            .await;

        assert_eq!(r.unwrap().function_id, "xxxxx".to_string());
    }

    async fn make_a_trigger(coll: Collection<Trigger>) -> (abi::Trigger, DefaultTriggerManager) {
        let manager = DefaultTriggerManager::new(coll);
        let trigger = abi::Trigger::new_database(
            "aliceid",
            "insert_and_delete",
            "xxx",
            true,
            DatabaseConfig {
                data_source: "mongodb://root:123456@127.0.0.1:27017/admin".to_string(),
                database: "test".to_string(),
                collection: "c1".to_string(),
                operation_types: vec![
                    abi::trigger::DatabaseOperationType::Insert as i32,
                    abi::trigger::DatabaseOperationType::Delete as i32,
                ],
            },
        );

        (manager.create(trigger).await.unwrap(), manager)
    }
}
