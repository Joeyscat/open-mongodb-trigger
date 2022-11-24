use abi::function::function::Func;
use anyhow::anyhow;
use bson::Document;
use common::{
    result::EventResult, ALLOCATE_FUNC_GO, ALLOCATE_FUNC_RUST, DEALLOCATE_FUNC_GO,
    DEALLOCATE_FUNC_RUST, EVENT_HANDLER_ENTRY_FUNC_NAME,
};
use function::FunctionManager;
use tracing::{info, warn};
use wasm::{CallOptions, WasmEngine};

pub async fn call_func(
    func_key: String,
    event: mongodb::change_stream::event::ChangeStreamEvent<Document>,
    function_manager: impl FunctionManager,
) {
    info!("calling function(id={})", func_key);

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
    info!("event: {}", serde_json::to_string_pretty(&event)?);

    let f = function_manager.get(func_key).await?;

    f.func.clone().ok_or_else(|| anyhow!("invalid func"))?;

    let Func::Wasm(func_bytes) = f.func.unwrap();
    let param_bytes = serde_json::to_vec(&event)?;

    let engine = WasmEngine::default();

    let options = match abi::function::Lang::from_i32(f.lang) {
        Some(abi::function::Lang::Rust) => CallOptions {
            allocate_func: ALLOCATE_FUNC_RUST.to_string(),
            deallocate_func: DEALLOCATE_FUNC_RUST.to_string(),
        },
        Some(abi::function::Lang::Go) => CallOptions {
            allocate_func: ALLOCATE_FUNC_GO.to_string(),
            deallocate_func: DEALLOCATE_FUNC_GO.to_string(),
        },
        _ => return Err(anyhow!("unsupported lang: {}", f.name)),
    };

    let result_bytes = engine.call_with_options(
        EVENT_HANDLER_ENTRY_FUNC_NAME,
        func_bytes,
        param_bytes,
        Some(options),
    )?;
    let result = serde_json::from_slice::<EventResult>(&result_bytes)?;

    Ok(result)
}
