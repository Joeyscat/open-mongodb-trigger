use bson::{doc, Document};

use crate::FunctionQuery;

impl FunctionQuery {
    pub fn to_filter(&self) -> Document {
        let mut filter = doc! {};

        if !self.user_id.is_empty() {
            filter.insert("user_id", self.user_id.to_string());
        }
        if !self.name.is_empty() {
            filter.insert("name", self.name.to_string());
        }

        filter
    }
}
