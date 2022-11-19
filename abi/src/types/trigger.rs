use crate::{trigger::DatabaseConfig, Trigger, TriggerConfig, TriggerType};

impl Trigger {
    pub fn new_database(
        user_id: impl Into<String>,
        name: impl Into<String>,
        function_id: impl Into<String>,
        enabled: impl Into<bool>,
        database_config: impl Into<DatabaseConfig>,
    ) -> Self {
        Self {
            id: String::new(),
            user_id: user_id.into(),
            name: name.into(),
            trigger_type: TriggerType::Database as i32,
            function_id: function_id.into(),
            enabled: enabled.into(),
            trigger_config: Some(TriggerConfig::Database(database_config.into())),
        }
    }
}
