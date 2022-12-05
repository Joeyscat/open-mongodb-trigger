use anyhow::{anyhow, Result};
use common::{
    mem::{get_raw_bytes, wrap_bytes},
    ALLOCATE_FUNC_RUST, DEALLOCATE_FUNC_RUST,
};
use wasi_experimental_http_wasmtime::{HttpCtx, HttpState};
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

#[derive(Default)]
pub struct WasmEngine {
    inner: Engine,
}

#[derive(Clone)]
pub struct CallOptions {
    pub allocate_func: String,
    pub deallocate_func: String,
}

struct WasmtimeHttpCtx {
    pub wasi: WasiCtx,
    pub http: HttpCtx,
}

impl WasmEngine {
    pub fn new(engine: Engine) -> Self {
        WasmEngine { inner: engine }
    }

    pub fn call(
        &self,
        name: &str,
        func: impl AsRef<[u8]>,
        params: impl AsRef<[u8]>,
    ) -> Result<Vec<u8>> {
        self.call_with_options(name, func, params, None)
    }

    pub fn call_with_options(
        &self,
        name: &str,
        func: impl AsRef<[u8]>,
        params: impl AsRef<[u8]>,
        options: Option<CallOptions>,
    ) -> Result<Vec<u8>> {
        let engine = self.inner.clone();
        let mut linker = Linker::new(&engine);

        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            // .inherit_args()?
            .build();

        // link the experimental HTTP support
        let allowed_hosts = Some(vec!["insecure:allow-all".to_string()]);
        let max_concurrent_requests = Some(42);
        let http = HttpCtx {
            allowed_hosts,
            max_concurrent_requests,
        };

        let ctx = WasmtimeHttpCtx { wasi, http };

        let mut store = Store::new(&engine, ctx);
        wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut WasmtimeHttpCtx| -> &mut WasiCtx {
            &mut cx.wasi
        })?;

        // Link `wasi_experimental_http`
        let http = HttpState::new()?;
        http.add_to_linker(&mut linker, |cx: &WasmtimeHttpCtx| -> HttpCtx {
            cx.http.clone()
        })?;

        let module = Module::new(&engine, func)?;
        let instance = linker.instantiate(&mut store, &module)?;
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| anyhow::format_err!("failed to find `memory` export"))?;

        let (allocate_func_name, _) = match options {
            Some(CallOptions {
                allocate_func,
                deallocate_func,
            }) => (allocate_func, deallocate_func),
            None => (
                ALLOCATE_FUNC_RUST.to_string(),
                DEALLOCATE_FUNC_RUST.to_string(),
            ),
        };
        let allocate = instance.get_typed_func::<u32, u32, _>(&mut store, &allocate_func_name)?;
        let func = instance.get_typed_func::<u32, u32, _>(&mut store, name)?;

        let params = params.as_ref();
        let params_data_all = wrap_bytes(params);
        let params_data_size = params_data_all.len();

        let allocate_for_params = allocate.call(&mut store, params_data_size as u32)?;

        memory.write(&mut store, allocate_for_params as usize, &params_data_all)?;

        let result_pointer_in_store = func.call(&mut store, allocate_for_params)?;

        if result_pointer_in_store == 0 {
            return Err(anyhow!("call wasm func error, the result_pointer is 0"));
        }

        let data_ptr = memory.data_ptr(&store);
        let result_pointer = unsafe { data_ptr.add(result_pointer_in_store as usize) };
        let result_bytes = get_raw_bytes(result_pointer);

        Ok(result_bytes)
    }
}

#[cfg(test)]
mod tests {
    use common::{
        event::*, result::EventResult, ALLOCATE_FUNC_GO, DEALLOCATE_FUNC_GO,
        EVENT_HANDLER_ENTRY_FUNC_NAME,
    };
    use std::{fs::File, io::Read};

    use bson::Document;

    use super::*;

    #[test]
    fn call_should_work() {
        let mut f =
            File::open("../target/wasm32-wasi/release/example_wasm_hello_world_lib.wasm").unwrap();
        let mut func_bytes = Vec::new();
        // read the whole file
        f.read_to_end(&mut func_bytes).unwrap();

        let params_bytes = "World".as_bytes();
        let engine = WasmEngine::default();
        let result = engine.call("greet", func_bytes, params_bytes);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!".as_bytes());
    }

    #[test]
    fn call_event_handler_should_work() {
        call_event_handler(
            "../target/wasm32-wasi/release/example_wasm_rust_event_handler_lib.wasm",
            Some(CallOptions {
                allocate_func: ALLOCATE_FUNC_RUST.to_string(),
                deallocate_func: DEALLOCATE_FUNC_RUST.to_string(),
            }),
        );
    }

    #[test]
    fn call_go_event_handler_should_work() {
        call_event_handler(
            "../examples/wasm/go-event-handler/target/example_wasm_go_event_handler_lib.wasm",
            Some(CallOptions {
                allocate_func: ALLOCATE_FUNC_GO.to_string(),
                deallocate_func: DEALLOCATE_FUNC_GO.to_string(),
            }),
        );
    }

    fn call_event_handler(p: &str, options: Option<CallOptions>) {
        tracing_subscriber::fmt::init();

        let engine = WasmEngine::default();

        let mut f = File::open(p).unwrap();
        let mut func_bytes = Vec::new();
        // read the whole file
        f.read_to_end(&mut func_bytes).unwrap();

        let mut event: ChangeStreamEvent<Document> = ChangeStreamEvent::default();

        let params_bytes = serde_json::to_vec(&event).unwrap();

        println!("call_wasm_func");
        let result = engine.call_with_options(
            EVENT_HANDLER_ENTRY_FUNC_NAME,
            func_bytes.clone(),
            params_bytes,
            options.clone(),
        );
        println!("result: {:?}", result);
        let expected_result = EventResult::ok();
        assert_eq!(
            result.unwrap(),
            serde_json::to_vec(&expected_result).unwrap()
        );

        event.operation_type = OperationType::Delete;
        let params_bytes = serde_json::to_vec(&event).unwrap();

        println!("call_wasm_func");
        let result = engine.call_with_options(
            EVENT_HANDLER_ENTRY_FUNC_NAME,
            func_bytes.clone(),
            params_bytes,
            options,
        );
        println!("result: {:?}", result);
        let expected_result = EventResult::error("unsuppored op_type: Delete".to_string());
        assert_eq!(
            result.unwrap(),
            serde_json::to_vec(&expected_result).unwrap()
        );
    }
}
