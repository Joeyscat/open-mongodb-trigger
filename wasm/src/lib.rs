use anyhow::Result;
use common::{
    mem::{get_raw_bytes, wrap_bytes},
    ALLOCATE_FUNC_NAME, EVENT_HANDLER_ENTRY_FUNC_NAME,
};
use tracing::{debug, info};
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

#[derive(Default)]
pub struct WasmEngine {
    inner: Engine,
}

impl WasmEngine {
    pub fn new(engine: Engine) -> Self {
        WasmEngine { inner: engine }
    }

    pub fn call_wasm_func(
        &self,
        name: &str,
        func: impl AsRef<[u8]>,
        params: impl AsRef<[u8]>,
    ) -> Result<Vec<u8>> {
        let engine = self.inner.clone();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            // .inherit_args()?
            .build();
        let mut store = Store::new(&engine, wasi);

        debug!("Module::new");
        let module = Module::new(&engine, func)?;
        debug!("linker.instantiate");
        let instance = linker.instantiate(&mut store, &module)?;
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| anyhow::format_err!("failed to find `memory` export"))?;

        let allocate = instance.get_typed_func::<u32, u32, _>(&mut store, ALLOCATE_FUNC_NAME)?;
        let func = instance.get_typed_func::<u32, u32, _>(&mut store, name)?;

        let params = params.as_ref();
        let params_data_all = wrap_bytes(params);
        let params_data_size = params_data_all.len();
        // 为参数分配内存并获取指针
        info!("allocate");
        let allocate_for_params = allocate.call(&mut store, params_data_size as u32)?;

        // 将参数写入内存
        info!("write params data");
        memory.write(&mut store, allocate_for_params as usize, &params_data_all)?;
        // CALL
        info!("call wasm func");
        let result_pointer_in_store = func.call(&mut store, allocate_for_params)?;

        let data_ptr = memory.data_ptr(&store);
        let result_pointer = unsafe { data_ptr.add(result_pointer_in_store as usize) };
        let result_bytes = get_raw_bytes(result_pointer);

        Ok(result_bytes)
    }
}

pub fn call_event_handler(func: impl AsRef<[u8]>, params: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let engine = WasmEngine::default();
    engine.call_wasm_func(EVENT_HANDLER_ENTRY_FUNC_NAME, func, params)
}

#[cfg(test)]
mod tests {
    use common::{event::*, result::EventResult, EVENT_HANDLER_ENTRY_FUNC_NAME};
    use std::{fs::File, io::Read};

    use bson::Document;

    use super::*;

    #[test]
    fn call_wasm_func_should_work() {
        let mut f =
            File::open("../target/wasm32-wasi/release/example_wasm_hello_world_lib.wasm").unwrap();
        let mut func_bytes = Vec::new();
        // read the whole file
        f.read_to_end(&mut func_bytes).unwrap();

        let params_bytes = "World".as_bytes();
        let engine = WasmEngine::default();
        let result = engine.call_wasm_func("greet", func_bytes, params_bytes);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!".as_bytes());
    }

    #[test]
    fn call_event_handler_should_work() {
        tracing_subscriber::fmt::init();

        let engine = WasmEngine::default();

        let mut f =
            File::open("../target/wasm32-wasi/release/example_wasm_rust_event_handler_lib.wasm")
                .unwrap();
        let mut func_bytes = Vec::new();
        // read the whole file
        f.read_to_end(&mut func_bytes).unwrap();

        let mut event: ChangeStreamEvent<Document> = ChangeStreamEvent::default();
        let params_bytes = bson::to_vec(&event).unwrap();

        println!("call_wasm_func");
        let result = engine.call_wasm_func(
            EVENT_HANDLER_ENTRY_FUNC_NAME,
            func_bytes.clone(),
            params_bytes,
        );
        println!("result: {:?}", result);
        let expected_result = EventResult::ok();
        assert_eq!(result.unwrap(), bson::to_vec(&expected_result).unwrap());

        event.operation_type = OperationType::Delete;
        let params_bytes = bson::to_vec(&event).unwrap();

        println!("call_wasm_func");
        let result = engine.call_wasm_func(
            EVENT_HANDLER_ENTRY_FUNC_NAME,
            func_bytes.clone(),
            params_bytes,
        );
        println!("result: {:?}", result);
        let expected_result = EventResult::error("unsuppored op_type: Delete".to_string());
        assert_eq!(result.unwrap(), bson::to_vec(&expected_result).unwrap());
    }
}
