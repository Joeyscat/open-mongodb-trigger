use crate::{Func, Function, FunctionType};

impl Function {
    pub fn new_wasm(
        user_id: impl Into<String>,
        name: impl Into<String>,
        binary: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            id: String::new(),
            user_id: user_id.into(),
            name: name.into(),
            function_type: FunctionType::Wasm as i32,
            func: Some(Func::Wasm(binary.into())),
        }
    }
}
