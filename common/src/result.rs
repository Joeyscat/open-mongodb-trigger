use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventResult {
    ok: bool,
    msg: String,
}

impl EventResult {
    pub fn ok() -> Self {
        Self {
            ok: true,
            msg: "".to_string(),
        }
    }

    pub fn error(msg: String) -> Self {
        Self { ok: false, msg }
    }
}
