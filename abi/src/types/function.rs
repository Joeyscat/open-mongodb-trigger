use std::str::FromStr;

use crate::{function::Lang, Error, Func, Function, FunctionType};

impl Function {
    pub fn new_wasm(
        user_id: impl Into<String>,
        name: impl Into<String>,
        binary: impl Into<Vec<u8>>,
        lang: Lang,
    ) -> Self {
        Self {
            id: String::new(),
            user_id: user_id.into(),
            name: name.into(),
            function_type: FunctionType::Wasm as i32,
            lang: lang as i32,
            func: Some(Func::Wasm(binary.into())),
        }
    }
}

impl FromStr for Lang {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l = match s {
            "RUST" | "Rust" | "rust" => Lang::Rust,
            "GO" | "Go" | "go" => Lang::Go,
            _ => return Err(Error::UnsupportedFunctionLangStr(s.to_string())),
        };

        Ok(l)
    }
}
