<div align="right">

  [中文](https://joeyscat.github.io/open-mongodb-trigger/)

</div>


# Open MongoDB Trigger

Provides triggers that run outside the database server.

![basic arch](rfcs/images/arch01.png)

# Requirements

* rust
* protobuf compiler
* mongodb cluster


# Getting start

## build

```bash
❯ make build
❯ make build-example
```

## start the service

```bash
❯ cp service/fixtrues/config.toml ~/.config/mongodb-trigger.toml
❯ cargo run --release -p service
    Finished release [optimized] target(s) in 0.15s
     Running `target/release/service`
2022-11-22T06:36:31.484279Z  INFO service: Listening on 0.0.0.0:6788
```


## access service using CLI

deploy function and create trigger

```
❯ cp target/release/tri ~/.cargo/bin/
# create a function
❯ tri function create -n function1 -p target/wasm32-wasi/release/example_wasm_rust_event_handler_lib.wasm -t wasm -u 1 -l rust
# create a trigger
❯ tri trigger create -n trigger1 -u 1 -t database -s mongodb://test:test@192.168.50.5:26017/admin -d db1 -c c1 -o insert,delete,update,replace -f <function-id>
# enable a trigger
❯ tri trigger enable <trigger-id>
```

## fire the trigger

make a change of the watched collection and we will see the function is called.

```bash
[direct: mongos] db1> db.c1.insertOne({name:1})

[direct: mongos] db1> db.c1.deleteOne({})
```

# Examples

- [wasm functions](examples/wasm/README.md) - functions that run on wasm runtime


# Built On

- [MongoDB Change Streams](https://www.mongodb.com/docs/manual/changeStreams/#change-streams)
- [Wasmtime](https://github.com/bytecodealliance/wasmtime)

# Documentation

- [RFC](rfcs)
- [Guide](https://joeyscat.github.io/open-mongodb-trigger/)
