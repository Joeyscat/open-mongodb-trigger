use std::path::PathBuf;

use anyhow::Result;
use serde::de::Deserialize;
use serde::ser::Serialize;

pub trait Engine {
    /// call the wasm function and do not care about return value
    fn call<T>(&self, bin: WasmBin, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize;

    /// call the wasm function and return the result
    fn callrv<'a, P, R>(&self, bin: WasmBin, value: &'a P) -> Result<R>
    where
        P: ?Sized + Serialize,
        R: Deserialize<'a>;
}

#[derive(Debug, Clone, Copy)]
pub struct Config {}

pub enum WasmBin {
    Path(PathBuf),
    Bytes(Vec<u8>),
}
