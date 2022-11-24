target:
	mkdir target

build: target generate
	tinygo build -o target/example_wasm_go_event_handler_lib.wasm -target wasi

generate:
	go generate model/model.go
