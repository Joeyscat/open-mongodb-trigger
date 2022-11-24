mod manager;

use std::str::FromStr;

use async_trait::async_trait;
use bson::{oid::ObjectId, Binary};
use mongodb::Collection;
use tokio::sync::mpsc;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum Func {
    #[serde(rename = "wasm")]
    Wasm(Binary),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct Function {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub name: String,
    pub function_type: i32,
    pub func: Func,
    pub lang: String,
}

impl Function {
    pub fn convert_from_abi(f: abi::Function) -> Result<Self, abi::Error> {
        let func = f.func.ok_or(abi::Error::InvalidFunction)?;
        let ff = match (func, abi::FunctionType::from_i32(f.function_type)) {
            (abi::Func::Wasm(x), Some(abi::FunctionType::Wasm)) => Func::Wasm(Binary {
                subtype: bson::spec::BinarySubtype::Generic,
                bytes: x,
            }),
            _ => return Err(abi::Error::InvalidFunction),
        };
        let lang = abi::function::Lang::from_i32(f.lang)
            .unwrap_or(abi::function::Lang::Unknown)
            .as_str_name()
            .to_string();

        Ok(Self {
            id: None,
            user_id: f.user_id,
            name: f.name,
            function_type: f.function_type,
            func: ff,
            lang,
        })
    }

    pub fn convert_to_abi(self) -> Result<abi::Function, abi::Error> {
        let ff = match (self.func, abi::FunctionType::from_i32(self.function_type)) {
            (Func::Wasm(x), Some(abi::FunctionType::Wasm)) => abi::Func::Wasm(x.bytes),
            _ => return Err(abi::Error::InvalidFunction),
        };
        let lang = abi::function::Lang::from_str(self.lang.as_str())? as i32;
        Ok(abi::Function {
            id: self.id.unwrap().to_hex(),
            user_id: self.user_id,
            name: self.name,
            function_type: self.function_type,
            func: Some(ff),
            lang,
        })
    }
}

#[derive(Clone, Debug)]
pub struct DefaultFunctionManager {
    coll: Collection<Function>,
}

#[async_trait]
pub trait FunctionManager {
    async fn create(&self, function: abi::Function) -> Result<abi::Function, abi::Error>;
    async fn update_func(
        &self,
        id: abi::FunctionId,
        func: abi::Func,
    ) -> Result<abi::Function, abi::Error>;
    async fn delete(&self, id: abi::FunctionId) -> Result<abi::Function, abi::Error>;
    async fn get(&self, id: abi::FunctionId) -> Result<abi::Function, abi::Error>;
    async fn query(
        &self,
        query: abi::FunctionQuery,
    ) -> mpsc::Receiver<Result<abi::Function, abi::Error>>;
}
