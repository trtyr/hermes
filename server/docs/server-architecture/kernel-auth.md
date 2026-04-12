# 认证内核

这篇文档描述 `server` 自身的认证能力。

这里讨论的是：

- Web 登录会话
- 后端受保护接口认证
- 兼容性 `api_token`

这里不讨论：

- Agent 注册认证协议
- 前端页面怎么渲染登录表单

## 1. 对应代码

- `src/kernel/auth/mod.rs`
- `src/kernel/auth/service.rs`
- `src/kernel/auth/state.rs`
- `src/kernel/auth/types.rs`
- `src/kernel/service/auth.rs`
- `src/api/auth.rs`
- `src/api/common/auth.rs`

## 2. 认证内核的定位

认证能力之所以放在 `kernel` 内，而不是只放在 `api` 层，是因为它不仅服务于 HTTP handler。

它承担的是后端统一认证源：

- API handler 要用它鉴权
- WebSocket 握手要用它鉴权
- session 生命周期也要由它管理

所以它更接近“内核公共能力”，而不是单纯的接口工具函数。

## 3. 配置来源

认证配置来自：

- `src/kernel/config.rs`

当前相关配置包括：

- `auth.web_username`
- `auth.web_password`
- `auth.session_ttl_secs`
- `auth.api_token`

含义如下：

- `web_username` 和 `web_password`
  启用后端统一登录
- `session_ttl_secs`
  控制 Web 会话过期时间
- `api_token`
  兼容脚本和旧集成方式的静态 Token

## 4. 当前认证模型

当前后端支持两套认证来源：

### 4.1 Web 会话

流程是：

1. 用户通过 `POST /auth/login` 提交账号密码
2. `AuthService` 验证配置中的账号密码
3. 后端生成 `session_token`
4. 前端后续请求复用这个 `session_token`

这是当前推荐模式。

### 4.2 兼容性 API Token

如果配置了 `auth.api_token`，后端也允许通过静态 Token 访问受保护接口。

这个能力保留的目的主要是：

- 脚本集成
- 自动化调用
- 旧调用方式兼容

## 5. AuthService 在做什么

对应代码：

- `src/kernel/auth/service.rs`

`AuthService` 当前负责：

1. 判断是否启用认证
2. 判断是否配置了 Web 登录
3. 校验账号密码
4. 创建 session
5. 删除 session
6. 查询 session
7. 解析传入 token 的身份来源

也就是说，它不是一个“只校验密码”的工具，而是后端的认证状态服务。

## 6. 状态模型

认证状态当前主要保存在内存中。

核心内容包括：

- 配置中的用户名和密码
- 兼容性静态 Token
- 已创建的 session 映射
- session 过期时间

当前设计说明：

- session 不落 SQLite
- 服务重启后 session 会丢失
- 这是当前阶段可以接受的管理面语义

## 7. facade 和 API 的分工

`AuthFacade` 位于：

- `src/kernel/service/auth.rs`

它的作用是：

- 给外层提供统一认证入口
- 把 `AuthService` 收束到 `KernelHandle` 之下

`src/api/auth.rs` 的职责则是：

- 解析登录请求
- 调用 `AuthFacade`
- 组装 HTTP 响应

也就是说：

- 认证规则在内核
- HTTP 表达在 API 层

## 8. 设计规则

关于认证能力，建议固定遵守下面几条：

1. 会话创建和销毁只能走 `AuthService`
2. API 层不能自己维护第二套 session 状态
3. WebSocket 鉴权必须复用同一套认证源
4. 兼容性 `api_token` 只作为后备入口，不要和主登录流程混成两套页面逻辑

## 9. 当前边界是否清晰

从代码结构看，这个能力已经比较清晰：

- 配置在 `kernel/config`
- 认证状态在 `kernel/auth`
- 入口包装在 `kernel/service/auth`
- HTTP 适配在 `api/auth`

这就是比较典型的微内核式公共能力拆法。
