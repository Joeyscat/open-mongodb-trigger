
Build your own function to handle ChangeStream event.

# Requirements

- rust
- wasm32-wasi target

# Coding

write your code in the `event_handler` function in `lib.rs`.

```rsut
/// implement by user
fn handle_event(event: ChangeStreamEvent<Document>) -> anyhow::Result<()> {
    println!("handle event: {:?}", event);
    if event.operation_type == OperationType::Delete {
        return Err(anyhow!("unsuppored op_type: Delete"));
    }
    Ok(())
}
```


# Build

```bash
❯ cargo build --release --target wasm32-wasi -p example-wasm-rust-event-handler-lib
    Finished release [optimized] target(s) in 0.12s
❯ file ../../../target/wasm32-wasi/release/example_wasm_rust_event_handler_lib.wasm
../../../target/wasm32-wasi/release/example_wasm_rust_event_handler_lib.wasm: WebAssembly (wasm) binary module version 0x1 (MVP)
```

Once the .wasm file is built, you can deploy it to the trigger service
