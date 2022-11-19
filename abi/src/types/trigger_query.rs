use bson::{doc, Document};

use crate::TriggerQuery;

impl TriggerQuery {
    pub fn to_filter(&self) -> Document {
        let mut filter = doc! {};

        if !self.user_id.is_empty() {
            filter.insert("user_id", self.user_id.to_string());
        }
        if !self.function_id.is_empty() {
            filter.insert("function_id", self.function_id.to_string());
        }
        if !self.trigger_type.is_empty() {
            filter.insert("trigger_type", self.trigger_type.to_string());
        }
        if !self.data_source.is_empty() {
            filter.insert("data_source", self.data_source.to_string());
        }
        if !self.database.is_empty() {
            filter.insert("database", self.database.to_string());
        }
        if !self.collection.is_empty() {
            filter.insert("collection", self.collection.to_string());
        }

        filter
    }
}
