# Feature Requests

## 审计日志清空功能

**页面:** `/log` (http://localhost:5173/log)

**现状:** 审计日志只有筛选和分页，没有清空功能。

**需求:**
- 后端：新增 `DELETE /api/audit` 或 `POST /api/audit/clear` 端点
- 前端：在 log 页面加"清空日志"按钮，调用该端点

**状态:** 待实现
