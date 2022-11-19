mod manager;

use async_trait::async_trait;
use bson::oid::ObjectId;
use mongodb::Collection;
use tokio::sync::mpsc;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub enum Config {
    #[serde(rename = "database")]
    Database(abi::trigger::DatabaseConfig),
    #[serde(rename = "authentication")]
    Authentication(abi::trigger::AuthenticationConfig),
    #[serde(rename = "scheduled")]
    Scheduled(abi::trigger::ScheduledConfig),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct Trigger {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub name: String,
    pub trigger_type: i32,
    pub function_id: String,
    pub enabled: bool,
    pub config: Config,
}

impl Trigger {
    pub fn convert_from_abi_trigger(t: abi::Trigger) -> Result<Self, abi::Error> {
        let c = t.trigger_config.ok_or(abi::Error::InvalidTrigger)?;

        let config = match (c, abi::TriggerType::from_i32(t.trigger_type)) {
            (abi::TriggerConfig::Database(x), Some(abi::TriggerType::Database)) => {
                Config::Database(x)
            }
            (abi::TriggerConfig::Authentication(x), Some(abi::TriggerType::Authentication)) => {
                Config::Authentication(x)
            }
            (abi::TriggerConfig::Scheduled(x), Some(abi::TriggerType::Scheduled)) => {
                Config::Scheduled(x)
            }
            _ => return Err(abi::Error::InvalidTrigger),
        };

        Ok(Self {
            id: None,
            user_id: t.user_id,
            name: t.name,
            trigger_type: t.trigger_type,
            function_id: t.function_id,
            enabled: t.enabled,
            config,
        })
    }

    pub fn convert_to_abi_trigger(self) -> Result<abi::Trigger, abi::Error> {
        let trigger_config = match (self.config, abi::TriggerType::from_i32(self.trigger_type)) {
            (Config::Database(x), Some(abi::TriggerType::Database)) => {
                abi::TriggerConfig::Database(x)
            }
            (Config::Authentication(x), Some(abi::TriggerType::Authentication)) => {
                abi::TriggerConfig::Authentication(x)
            }
            (Config::Scheduled(x), Some(abi::TriggerType::Scheduled)) => {
                abi::TriggerConfig::Scheduled(x)
            }
            _ => return Err(abi::Error::InvalidTrigger),
        };
        Ok(abi::Trigger {
            id: self.id.unwrap().to_hex(),
            user_id: self.user_id,
            name: self.name,
            trigger_type: self.trigger_type,
            function_id: self.function_id,
            enabled: self.enabled,
            trigger_config: Some(trigger_config),
        })
    }
}

#[derive(Debug)]
pub struct DefaultTriggerManager {
    coll: Collection<Trigger>,
}

#[async_trait]
pub trait TriggerManager {
    async fn create(&self, trigger: abi::Trigger) -> Result<abi::Trigger, abi::Error>;
    async fn change_function(
        &self,
        id: abi::TriggerId,
        function_id: abi::FunctionId,
    ) -> Result<abi::Trigger, abi::Error>;
    async fn enable(&self, id: abi::TriggerId) -> Result<abi::Trigger, abi::Error>;
    async fn disable(&self, id: abi::TriggerId) -> Result<abi::Trigger, abi::Error>;
    async fn delete(&self, id: abi::TriggerId) -> Result<abi::Trigger, abi::Error>;
    async fn get(&self, id: abi::TriggerId) -> Result<abi::Trigger, abi::Error>;
    async fn query(
        &self,
        query: abi::TriggerQuery,
    ) -> mpsc::Receiver<Result<abi::Trigger, abi::Error>>;
}
