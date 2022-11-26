
Build your own function to handle ChangeStream event.

# Requirements

- tinygo - [why?](https://github.com/wasmerio/wasmer-go#how-to-run-go-programs-compiled-to-webassembly-modules-with-wasmer-go)

# Coding

write your code in the `handlerEvent` function in `lib.go`, and do not edit the `entry.go`

```bash
// implemented by the user
func handlerEvent(event model.ChangeStreamEvent) error {
	// fmt.Printf("event: %#+v\n", event)
	data, err := json.MarshalIndent(event, "", "\t")
	if err != nil {
		return err
	}
	fmt.Printf("event:\n%s\n", string(data))

	if event.OperationType == "delete" {
		return errors.New("unsuppored op_type: Delete")
	}

	return nil
}

```


# Build

```bash
❯ make build
go generate model/model.go
tinygo build -o target/example_wasm_go_event_handler_lib.wasm -target wasi
❯ tree target/
target/
└── example_wasm_go_event_handler_lib.wasm

0 directories, 1 file
```

Once the .wasm file is built, you can deploy it to the trigger service
