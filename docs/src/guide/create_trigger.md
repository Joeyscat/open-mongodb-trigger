# 创建触发器

看看如何通过 CLI 创建一个触发器：

```bash
 ❯ tri trigger create -h
create a trigger

Usage: tri trigger create --name <NAME> --user-id <USER_ID> --type <TYPE> --source <SOURCE> --database <DATABASE> --collection <COLLECTION> --operation-types <OPERATION_TYPES> --function-id <FUNCTION_ID>

Options:
  -n, --name <NAME>         # 触发器名称，对于同一个用户不能重复
  -u, --user-id <USER_ID>   # 用户ID，代表触发器所有者
  -t, --type <TYPE>         # 触发器类型，当前仅支持 database
  -s, --source <SOURCE>     # 监听的数据源，就是一个MongoDB连接串
  -d, --database <DATABASE>     # 监听的数据库
  -c, --collection <COLLECTION> # 监听的集合
  -o, --operation-types <OPERATION_TYPES>  # 监听的操作类型
  -f, --function-id <FUNCTION_ID>          # 函数ID
  -h, --help                               Print help information
```

函数 ID 就是前面我们创建函数返回的结果。
