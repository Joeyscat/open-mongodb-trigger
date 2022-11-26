# Open MongoDB Trigger

Provides triggers that run outside the database server.

![basic arch](rfcs/images/arch01.png)

# Requirements

* rust
* protobuf compiler
* mongodb cluster


# Build

```bash
❯ make build
```


# Test

```bash
❯ make build-example
❯ make test
```

# Usage

## Server

```bash
❯ cp service/fixtrues/config.toml ~/.config/mongodb-trigger.toml
❯ cargo run --release -p service
    Finished release [optimized] target(s) in 0.15s
     Running `target/release/service`
2022-11-22T06:36:31.484279Z  INFO service: Listening on 0.0.0.0:6788

```


## CLI

```
❯ cp target/release/tri ~/.cargo/bin/
# create a function
❯ tri function create -h
# create a trigger
❯ tri trigger create -h
# enable a trigger
❯ tri trigger enable -h
```

server log

>2022-11-22T06:49:13.437160Z  INFO service: add watcher: Watcher { trigger: Trigger { id: "a", user_id: "1", name: "insert-delete", trigger_type: Database, function_id: "a", enabled: true, trigger_config: Some(Database(DatabaseConfig { data_source: "mongodb://test:test@127.0.0.1:27017/db1", database: "db1", collection: "c1", operation_types: [Insert, Delete] })) } }


## Fire the trigger

make a change of the watched collection and we will see the function is called.

```bash
[direct: mongos] db1> db.c1.insertOne({name:1})

[direct: mongos] db1> db.c1.deleteOne({})
```

server log

> 2022-11-22T07:00:12.649416Z  INFO service::func: calling function(id=a)
2022-11-22T07:00:13.128059Z  INFO wasm: allocate
2022-11-22T07:00:13.128101Z  INFO wasm: write params data
2022-11-22T07:00:13.128107Z  INFO wasm: call wasm func
handle event: ChangeStreamEvent { ... }
r ==== EventResult { ok: true, msg: "" }
wrapped_output: [...]
2022-11-22T07:00:13.128750Z  INFO service::func: called function ok: EventResult { ok: true, msg: "" }
2022-11-22T07:00:31.520705Z  INFO service::func: calling function(id=a)
2022-11-22T07:00:31.871531Z  INFO wasm: allocate
2022-11-22T07:00:31.871564Z  INFO wasm: write params data
2022-11-22T07:00:31.871571Z  INFO wasm: call wasm func
handle event: ChangeStreamEvent { ... }
r ==== EventResult { ok: false, msg: "unsuppored op_type: Delete" }
wrapped_output: [...]
2022-11-22T07:00:31.872091Z  INFO service::func: called function ok: EventResult { ok: false, msg: "unsuppored op_type: Delete" }
