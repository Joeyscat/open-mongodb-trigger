# 简介

这是一个基于ChangeStream和WebAssemblly实现的外部MongoDB触发器

参考MongoDB官方的Trigger：

[https://www.mongodb.com/docs/atlas/app-services/triggers/database-triggers/](https://www.mongodb.com/docs/atlas/app-services/triggers/database-triggers/)

## 为什么要创建这个项目？

MongoDB ChangeStream 有许多应用场景：

- 数据迁移/数据同步
- （微服务）变化监听
- 实时分析/实时通知
- 事件驱动架构组件
- 交互系统

要实现这些功能，我们需要编写以下代码，将代码部署到服务器并进行维护：
```go
coll := get_coll(mongodb_uri)
cs := watch(coll)
loop {
    event := cs.next()
    handle(event) # 业务逻辑
}
```

我们需要一个服务，让用户能订阅自己感兴趣的 ChangeStream 事件，并编写一个函数来进行业务处理，即可实现上述的功能。这样，我们将节省很多时间（编写重复代码）和服务器成本（部署代码）。

- [如何编写代码并部署](guide/create_function.md)
- [如何创建触发器](guide/create_trigger.md)
