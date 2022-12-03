# 创建函数

你可以使用任何编程语言来编写你的函数，前提是它能够将你的代码编译成 wasm 文件。

这里我们用仓库里的一个 Go 示例项目（[go-event-handler](https://github.com/Joeyscat/open-mongodb-trigger/tree/main/examples/wasm/go-event-handler)），来演示怎么编写代码并构建出一个 wasm 文件。

## 安装环境

[TinyGo - Quick install guide](https://tinygo.org/getting-started/install/)

## Clone 示例代码

```bash
❯ git clone https://github.com/Joeyscat/open-mongodb-trigger.git
❯ cd open-mongodb-trigger/examples/wasm/go-event-handler
❯ tree
.
├── entry.go
├── Event_json.go
├── EventResult_json.go
├── go.mod
├── go.sum
├── lib.go # 在 handlerEvent 函数中添加你的代码
├── Makefile
├── model
│   ├── ChangeNamespace_json.go
│   ├── ChangeStreamEvent_json.go
│   ├── EventResult_json.go
│   ├── model.go
│   └── ResumeToken_json.go
├── model.go
└── README.md
```

## 编写函数

修改 lib.go 文件中的 handlerEvent 函数。

## 编译代码

```bash
❯ make build
go generate model/model.go
tinygo build -o target/example_wasm_go_event_handler_lib.wasm -target wasi
❯ tree target/
target/
└── example_wasm_go_event_handler_lib.wasm
```

## 部署 wasm 函数

编译得到 wasm 文件之后，就可以用它来创建一个函数。

```bash
❯ tri function create -h
create a function

Usage: tri function create --name <NAME> --path <PATH> --type <TYPE> --user-id <USER_ID> --lang <LANG>

Options:
  -n, --name <NAME>
  -p, --path <PATH>        path of the function file (*.wasm)
  -t, --type <TYPE>        now supported: wasm
  -u, --user-id <USER_ID>
  -l, --lang <LANG>
  -h, --help               Print help information
```

创建函数成功我们会得到一个函数 ID ，后面创建触发器的时候会用到。
