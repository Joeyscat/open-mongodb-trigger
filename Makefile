build:
	cargo build --release

build-example:
	cargo build --release --target wasm32-wasi -p example-wasm-hello-world-lib -p example-wasm-rust-event-handler-lib -p example-wasm-rust-event-handler

# build-go-example:
# 	echo 'x'

test:
	cargo test --release
