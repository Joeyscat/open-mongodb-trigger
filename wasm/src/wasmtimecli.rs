use crate::engine::{Config, Engine, WasmBin};
use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{anyhow, Result};
use serde::de::Deserialize;
use serde::ser::Serialize;

pub struct WasmtimeCli {
    _config: Config,
}

impl WasmtimeCli {
    pub fn from_config(config: &Config) -> Self {
        Self { _config: *config }
    }
}

impl Engine for WasmtimeCli {
    fn call<T>(&self, bin: WasmBin, param: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let wasm_bin_path = match bin {
            WasmBin::Path(p) => p,
            WasmBin::Bytes(_) => return Err(anyhow!("expected a path for wasm binary")),
        };
        let mut child = Command::new("wasmtime")
            .arg(wasm_bin_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let child_stdin = child.stdin.as_mut().unwrap();

        let input = serde_json::to_vec(param)?;
        child_stdin.write_all(&input)?;
        // drop(child_stdin);

        let output = child.wait_with_output()?;
        if !output.status.success() {
            if !output.stderr.is_empty() {
                return Err(anyhow!(
                    "wasmtime error: {:?}",
                    String::from_utf8(output.stderr)?
                ));
            }
            if !output.stdout.is_empty() {
                return Err(anyhow!(
                    "wasmtime output: {:?}",
                    String::from_utf8(output.stdout)?
                ));
            }
        }
        println!("output = {:?}", output);

        Ok(())
    }

    fn callrv<'a, P, R>(&self, _bin: WasmBin, _value: &'a P) -> Result<R>
    where
        P: ?Sized + Serialize,
        R: Deserialize<'a>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use bson::Document;
    use common::event::ChangeStreamEvent;

    use crate::engine::{Config, Engine, WasmBin};

    use super::WasmtimeCli;

    #[test]
    fn test_call() {
        let cli = WasmtimeCli::from_config(&Config {});
        let wa = "../target/wasm32-wasi/release/example-wasm-rust-event-handler.wasm";
        let bin = WasmBin::Path(PathBuf::from_str(wa).unwrap());
        let event: ChangeStreamEvent<Document> = ChangeStreamEvent::default();

        let r = cli.call(bin, &event);
        println!("result: {:?}", r);
        assert!(r.is_ok());
    }
}
