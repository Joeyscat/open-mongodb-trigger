use abi::function::function::Func;
use anyhow::anyhow;
use bson::Document;
use common::result::EventResult;
use tracing::{info, warn};
use wasm::WasmEngine;

pub async fn call_func(
    func_key: String,
    event: mongodb::change_stream::event::ChangeStreamEvent<Document>,
) {
    info!("calling function(id={}), with param({:?})", func_key, event);

    match _call_func(func_key, event).await {
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
) -> anyhow::Result<EventResult> {
    let f = abi::Function {
        id: func_key,
        ..Default::default()
    };

    f.func.clone().ok_or_else(|| anyhow!("invalid func"))?;

    let Func::Wasm(func_bytes) = f.func.unwrap();
    let param_bytes = bson::to_vec(&event)?;

    let engine = WasmEngine::default();

    let result_bytes = engine.call_wasm_func("event_handler_helper", func_bytes, param_bytes)?;
    let result = bson::from_slice::<EventResult>(&result_bytes)?;

    Ok(result)
}
