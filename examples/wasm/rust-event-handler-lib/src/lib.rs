use anyhow::anyhow;

use std::mem::{self};

use bson::Document;
use common::{
    event::{self, ChangeStreamEvent, OperationType},
    mem::{get_raw_bytes, wrap_bytes},
    result::EventResult,
};

#[no_mangle]
pub extern "C" fn allocate(size: u32) -> u32 {
    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as u32
}
#[no_mangle]
pub extern "C" fn deallocate(pointer: u32, capacity: u32) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer as *mut u8, 0, capacity as usize);
    }
}

#[no_mangle]
pub extern "C" fn event_handler_entry(param_ptr: u32) -> u32 {
    let event_bytes: Vec<u8> = get_raw_bytes(param_ptr as *const u8);
    // deserialize bytes to event struct
    let r = match serde_json::from_slice::<event::ChangeStreamEvent<Document>>(&event_bytes) {
        Ok(event) => match handle_event(event) {
            Ok(_) => EventResult::ok(),
            Err(e) => EventResult::error(e.to_string()),
        },
        Err(e) => EventResult::error(e.to_string()),
    };

    println!("r ==== {:?}", r);
    let result_bytes = serde_json::to_vec(&r).unwrap();

    let wrapped_output = wrap_bytes(&result_bytes);
    println!("wrapped_output: {:?}", wrapped_output);

    let x = wrapped_output.as_ptr();
    x as u32
}

/// implement by user
fn handle_event(event: ChangeStreamEvent<Document>) -> anyhow::Result<()> {
    println!("handle event: {:?}", event);
    if event.operation_type == OperationType::Delete {
        return Err(anyhow!("unsuppored op_type: Delete"));
    }
    Ok(())
}
