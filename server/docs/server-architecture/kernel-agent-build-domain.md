# Agent 构建能力

这篇文档描述 `server` 中与 Agent 构建相关的能力域。

这里讨论的是：

- 构建请求
- 监听器绑定
- 目标平台选择
- 产物记录

这里不讨论：

- 具体前端页面长什么样
- Agent 二进制分发策略

## 1. 对应代码

- `src/kernel/service/agent_builds/mod.rs`
- `src/kernel/service/agent_builds/build.rs`
- `src/kernel/service/agent_builds/queries.rs`
- `src/kernel/service/agent_builds/toolchain.rs`
- `src/kernel/storage/agent_builds.rs`
- `src/api/agent_builds/*`

## 2. 这个能力域负责什么

这个能力域解决的问题是：

“后端如何根据 listener 和目标平台，生成可追踪的 Agent 构建记录和产物。”

它当前至少覆盖：

- 构建请求参数接收
- listener 绑定
- 编译期嵌入服务端地址
- target triple / profile 选择
- 构建状态记录
- 产物路径记录
- manifest 输出

## 3. 为什么它也算内核能力

因为它不是一个纯前端页面动作，也不是简单文件输出。

它涉及：

- 控制面的受管资源
- 和 listener 配置的关联
- 构建历史持久化
- 后端的统一审计与记录

所以把它收在 `kernel/service/agent_builds` 下是合理的。

## 4. 当前结构

### 4.1 build

负责：

- 发起构建流程
- 组织构建参数
- 驱动工具链执行
- 写出产物 manifest

### 4.2 toolchain

负责：

- 处理构建工具链相关逻辑
- 隔离不同平台或构建环境的细节

### 4.3 queries

负责：

- 查询构建记录
- 返回前端所需的构建状态和产物信息

### 4.4 storage/agent_builds

负责：

- 把构建记录持久化到 SQLite

## 5. 设计规则

关于 Agent 构建能力，建议固定遵守下面几条：

1. 构建请求必须先进入受控记录，再进入执行流程
2. 构建结果必须落盘，不能只靠日志输出
3. 与 listener 的关系必须显式建模
4. `server` 生成的 Agent 必须优先使用构建时写入地址
5. `listener_id` 和嵌入地址不能出现语义冲突
6. 工具链细节尽量隔离在专门模块中

## 6. 当前绑定语义

当前 `POST /agent-builds` 生成的 Agent 产物，服务端地址语义固定为：

- 优先由 `listener_id` 决定绑定哪个 listener
- 没有显式传 `server_addr` 时，默认使用 listener 的绑定地址
- 显式传了 `server_addr` 时，必须和 listener 绑定地址完全一致
- 构建阶段会先写入 `agent/src/server.rs`
- 构建产物运行时不依赖环境变量改接入点或身份

这样做的目的很直接：

- 避免“前端以为绑定了 listener，Agent 实际又连到别的地址”
- 避免运行时再靠外部环境改接入点，导致部署语义漂移
- 让产物可审计、可复现、可解释

为了让前端对接更简单，当前还额外提供：

- `POST /listeners/:listener_id/agent-builds`

这个接口本质上只是 `agent_builds` 内核能力的一个 Web 适配壳，不新增内核状态，也不改变构建规则。

## 7. 当前评价

它目前更接近“服务型能力域”，没有像任务域那样深度进入 runtime。

但它仍然属于 `server` 的核心业务能力之一，所以单独成文是合理的。
