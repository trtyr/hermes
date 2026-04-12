# 发布流程

## 分支策略

- 默认分支是 `main`
- 功能开发使用短生命周期分支，从 `main` 拉出
- 合并前至少本地通过一次 `make ci`

## 版本策略

- `server` 遵循语义化版本
- crate 版本号定义在 `Cargo.toml`
- Git tag 使用 `server-vMAJOR.MINOR.PATCH`

示例：

- `server-v0.1.0`
- `server-v0.2.3`

## 发布前检查清单

1. 如果本次发布影响对外行为，更新 `Cargo.toml` 版本号
2. 执行 `make ci`
3. 如果包含 listener 或 agent build 相关变更，执行 `make e2e-all`
4. 检查 `docs/server-web-client/http-api.md` 和 `docs/server-web-client/openapi.yaml`，确认接口文档没有漂移
5. 检查 `docs/server-architecture/server-architecture.md`，确认架构说明仍与现状一致
6. 检查 `README.md`，确认启动方式、能力说明、工作流描述仍然准确

## 打标签

```bash
git checkout main
git pull --ff-only
make ci
git tag -a server-v0.1.0 -m "server v0.1.0"
git push origin main --tags
```

## GitHub Release Notes

建议至少包含：

- listener 或 runtime 变更
- API 新增项或破坏性变更
- agent 兼容性说明
- 运维迁移说明
- 已知限制

## CI 范围

- GitHub Actions 会跑格式化、编译检查、测试以及可移植 E2E 套件
- agent 交叉编译验证仍然属于发布时或运维时检查，因为它依赖同级 `agent` 项目和本地工具链
