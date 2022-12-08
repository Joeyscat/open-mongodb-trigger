# CLI

CLI 是用于访服务接口的客户端工具，用户可以使用该工具[上传自己编译的处理函数](guide/create_function.md)，[创建触发器](guide/create_trigger.md)。

```bash
❯ tri -h
Usage: tri [OPTIONS] [NAME] [COMMAND]

Commands:
  test      does testing things
  trigger   trigger manager
  function  function manager
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [NAME]  Optional name to operate on

Options:
  -c, --config <FILE>  Sets a custom config file
  -d, --debug...       Turn debugging information on
  -h, --help           Print help information
  -V, --version        Print version information
```
