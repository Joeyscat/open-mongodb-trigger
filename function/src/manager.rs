use async_trait::async_trait;
use bson::{doc, oid::ObjectId, spec::BinarySubtype};
use futures::TryStreamExt;
use mongodb::{
    options::{
        Acknowledgment, FindOneAndUpdateOptions, FindOneOptions, IndexOptions, InsertOneOptions,
        WriteConcern,
    },
    Collection, IndexModel,
};
use tokio::sync::mpsc;
use tracing::warn;

use crate::{DefaultFunctionManager, Function, FunctionManager};

#[async_trait]
impl FunctionManager for DefaultFunctionManager {
    async fn create(&self, mut function: abi::Function) -> Result<abi::Function, abi::Error> {
        let f = Function::convert_from_abi(function.clone())?;

        let r = self
            .coll
            .insert_one(
                f,
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
        function.id = id;

        Ok(function)
    }

    async fn update_func(
        &self,
        id: abi::FunctionId,
        func: abi::Func,
    ) -> Result<abi::Function, abi::Error> {
        let ff = match func {
            abi::Func::Wasm(x) => {
                doc! {"wasm": bson::Binary{subtype:BinarySubtype::Generic, bytes:x}}
            }
        };
        // TODO
        let r = self
            .coll
            .find_one_and_update(
                doc! {"_id": ObjectId::parse_str(id)?},
                doc! {
                    "$set": {"func": ff}
                },
                FindOneAndUpdateOptions::builder()
                    .return_document(Some(mongodb::options::ReturnDocument::After))
                    .build(),
            )
            .await?;

        match r {
            Some(tr) => Ok(tr.convert_to_abi()?),
            None => Err(abi::Error::FunctionNotFound),
        }
    }

    async fn delete(&self, id: abi::FunctionId) -> Result<abi::Function, abi::Error> {
        let r = self
            .coll
            .find_one_and_delete(doc! {"_id": ObjectId::parse_str(id)?}, None)
            .await?;

        match r {
            Some(tr) => Ok(tr.convert_to_abi()?),
            None => Err(abi::Error::FunctionNotFound),
        }
    }

    async fn get(&self, id: abi::FunctionId) -> Result<abi::Function, abi::Error> {
        let r = self
            .coll
            .find_one(
                doc! {"_id": ObjectId::parse_str(id)?},
                FindOneOptions::default(),
            )
            .await?;

        match r {
            Some(tr) => Ok(tr.convert_to_abi()?),
            None => Err(abi::Error::FunctionNotFound),
        }
    }

    async fn query(
        &self,
        query: abi::FunctionQuery,
    ) -> mpsc::Receiver<Result<abi::Function, abi::Error>> {
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
                    Some(item) => {
                        let r = item.convert_to_abi();
                        if r.is_err() {
                            break;
                        }
                        if tx.send(Ok(r.unwrap())).await.is_err() {
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

impl DefaultFunctionManager {
    pub fn new(coll: Collection<Function>) -> Self {
        Self { coll }
    }

    pub async fn from_config(config: &abi::DbConfig) -> Result<Self, abi::Error> {
        let db = mongodb::Client::with_uri_str(config.mongodb_uri.clone())
            .await?
            .database(&config.db);

        let coll = db.collection::<Function>(&config.function_coll);

        create_indexes(coll.clone()).await?;

        Ok(Self::new(coll))
    }
}

async fn create_indexes(coll: Collection<Function>) -> Result<(), abi::Error> {
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
    use chrono::Local;
    use mongodb::{Collection, Database};

    use crate::{DefaultFunctionManager, Function, FunctionManager};

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

    async fn get_coll(database: Database, coll_name: &str) -> Collection<Function> {
        let x = Local::now().timestamp_nanos();
        let coll =
            database.collection::<Function>(format!("function_{}_{}", coll_name, x).as_str());
        create_indexes(coll.clone()).await.unwrap();
        coll
    }

    #[tokio::test]
    async fn create_should_work_for_valid_function() {
        let context = setup().await;
        let coll = get_coll(context.db.clone(), "create1").await;
        let (rsvp, _manager) = make_a_function(coll).await;
        assert!(!rsvp.id.is_empty());
        println!("id: {:?}", rsvp.id);
    }

    #[tokio::test]
    async fn get_function_should_work() {
        let context = setup().await;
        let coll = get_coll(context.db.clone(), "get").await;
        let (rsvp, manager) = make_a_function(coll).await;
        let rsvp1 = manager.get(rsvp.id.clone()).await.unwrap();
        assert_eq!(rsvp, rsvp1);
    }

    #[tokio::test]
    async fn query_function_should_work() {
        let context = setup().await;
        let coll = get_coll(context.db.clone(), "query").await;
        let (_rsvp, manager) = make_a_function(coll).await;
        let q = abi::FunctionQuery {
            user_id: "aliceid".to_string(),
            ..Default::default()
        };
        let mut rx = manager.query(q).await;

        assert!(rx.recv().await.is_some());
        assert!(rx.recv().await.is_none());
    }

    #[tokio::test]
    async fn delete_function_should_work() {
        let context = setup().await;
        let coll = get_coll(context.db.clone(), "delete").await;
        let (rsvp, manager) = make_a_function(coll).await;
        let r = manager.delete(rsvp.id.clone()).await;
        assert!(r.is_ok());
        let r = manager.get(rsvp.id.clone()).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn change_function_func_should_work() {
        let context = setup().await;
        let coll = get_coll(context.db.clone(), "change").await;
        let (rsvp, manager) = make_a_function(coll).await;
        let r = manager
            .update_func(rsvp.id.clone(), abi::Func::Wasm("haha".as_bytes().to_vec()))
            .await;

        assert_eq!(
            r.unwrap().func.unwrap(),
            abi::Func::Wasm("haha".as_bytes().to_vec())
        );
    }

    async fn make_a_function(
        coll: Collection<Function>,
    ) -> (abi::Function, DefaultFunctionManager) {
        let manager = DefaultFunctionManager::new(coll);
        let wasm_func_binary = "Hello, World!".as_bytes();
        let function = abi::Function::new_wasm("aliceid", "insert_and_delete", wasm_func_binary);

        (manager.create(function).await.unwrap(), manager)
    }
}
