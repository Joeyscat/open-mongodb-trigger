use abi::function::function::Func;
use anyhow::anyhow;
use bson::Document;
use common::{result::EventResult, EXPORT_EVENT_HANDLE_FUNC_NAME};
use function::FunctionManager;
use tracing::{info, warn};
use wasm::WasmEngine;

pub async fn call_func(
    func_key: String,
    event: mongodb::change_stream::event::ChangeStreamEvent<Document>,
    function_manager: impl FunctionManager,
) {
    info!("calling function(id={}), with param({:?})", func_key, event);

    match _call_func(func_key, event, function_manager).await {
        Ok(result) => {
            info!("called function ok: {:?}", result)
        }
        Err(e) => {
            warn!("called function error: {:?}", e)
        }
    }
}

async fn _call_func(
    func_key: String,
    event: mongodb::change_stream::event::ChangeStreamEvent<Document>,
    function_manager: impl FunctionManager,
) -> anyhow::Result<EventResult> {
    let f = function_manager.get(func_key).await?;

    f.func.clone().ok_or_else(|| anyhow!("invalid func"))?;

    let Func::Wasm(func_bytes) = f.func.unwrap();
    let param_bytes = bson::to_vec(&event)?;

    let engine = WasmEngine::default();

    let result_bytes =
        engine.call_wasm_func(EXPORT_EVENT_HANDLE_FUNC_NAME, func_bytes, param_bytes)?;
    let result = bson::from_slice::<EventResult>(&result_bytes)?;

    Ok(result)
}
