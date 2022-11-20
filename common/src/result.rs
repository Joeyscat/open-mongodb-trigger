use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct EventResult {
    ok: bool,
    msg: Vec<u8>,
}

impl EventResult {
    pub fn ok() -> Self {
        Self {
            ok: true,
            msg: vec![],
        }
    }

    pub fn error(msg: String) -> Self {
        Self {
            ok: false,
            msg: msg.as_bytes().to_vec(),
        }
    }
}

impl fmt::Debug for EventResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventResult")
            .field("ok", &self.ok)
            .field("msg", &String::from_utf8(self.msg.clone()).unwrap())
            .finish()
    }
}
