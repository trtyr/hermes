use super::*;

pub(crate) const OPENAPI_YAML: &str = include_str!("../../../docs/server-web-client/openapi.yaml");

pub(crate) async fn openapi_spec() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/yaml; charset=utf-8")],
        OPENAPI_YAML,
    )
}

pub(crate) async fn api_docs() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        r#"<!doctype html>
<html lang="zh-CN">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Hermes Server 文档入口</title>
  <style>
    :root {
      color-scheme: light;
      --bg: #f5f1e8;
      --panel: #fffdf8;
      --ink: #1f2937;
      --muted: #5b6470;
      --line: #d8cdb7;
      --accent: #1f6f5f;
      --accent-2: #d97706;
    }
    * { box-sizing: border-box; }
    body {
      margin: 0;
      font-family: "Iowan Old Style", "Palatino Linotype", "Book Antiqua", Georgia, serif;
      background:
        radial-gradient(circle at top left, rgba(217,119,6,0.08), transparent 30%),
        radial-gradient(circle at top right, rgba(31,111,95,0.10), transparent 28%),
        var(--bg);
      color: var(--ink);
    }
    main {
      max-width: 920px;
      margin: 0 auto;
      padding: 48px 20px 64px;
    }
    .card {
      background: rgba(255,253,248,0.92);
      border: 1px solid var(--line);
      border-radius: 20px;
      padding: 28px;
      box-shadow: 0 14px 40px rgba(80, 61, 20, 0.08);
      backdrop-filter: blur(4px);
    }
    h1, h2 { margin: 0 0 12px; }
    h1 { font-size: clamp(2rem, 4vw, 3rem); }
    h2 { margin-top: 28px; font-size: 1.2rem; }
    p, li { line-height: 1.65; color: var(--muted); }
    .pill {
      display: inline-block;
      padding: 6px 10px;
      border-radius: 999px;
      background: rgba(31,111,95,0.10);
      color: var(--accent);
      font-size: 0.95rem;
      margin-bottom: 16px;
    }
    a {
      color: var(--accent);
      text-decoration: none;
      border-bottom: 1px solid rgba(31,111,95,0.28);
    }
    a:hover { color: var(--accent-2); border-bottom-color: rgba(217,119,6,0.38); }
    code {
      font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
      background: rgba(31,41,55,0.06);
      padding: 2px 6px;
      border-radius: 6px;
      color: var(--ink);
    }
    .grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
      gap: 14px;
      margin-top: 20px;
    }
    .tile {
      border: 1px solid var(--line);
      border-radius: 16px;
      padding: 16px;
      background: rgba(255,255,255,0.55);
    }
    .tile strong { display: block; margin-bottom: 6px; color: var(--ink); }
    ul { padding-left: 20px; margin: 10px 0 0; }
  </style>
</head>
<body>
  <main>
    <section class="card">
      <div class="pill">Hermes Server</div>
      <h1>文档与接口入口</h1>
      <p><code>server</code> 是当前体系里的控制平面，负责对接 <code>web_client</code> 与 <code>agent client</code>。这里给出当前接口契约与主要文档入口。</p>

      <div class="grid">
        <div class="tile">
          <strong>OpenAPI 规范</strong>
          <p><a href="/openapi.yaml">/openapi.yaml</a></p>
        </div>
        <div class="tile">
          <strong>健康检查</strong>
          <p><a href="/health">/health</a></p>
        </div>
        <div class="tile">
          <strong>在线 Agent</strong>
          <p><a href="/agents">/agents</a></p>
        </div>
      </div>

      <h2>认证说明</h2>
      <p>当配置了 <code>auth.web_username</code> 与 <code>auth.web_password</code> 后，前端应先调用 <code>POST /auth/login</code> 获取后端会话，再访问受保护的 HTTP API 与 WebSocket 端点。</p>
      <p><code>auth.api_token</code> 仍保留给兼容脚本调用，但它已经不是主登录路径。</p>

      <h2>实时事件</h2>
      <p>Web 端可以订阅 <code>/events/ws</code>，接收快照、Agent 状态与任务状态变更。</p>

      <h2>关键文档</h2>
      <ul>
        <li><code>docs/README.md</code>：文档总入口</li>
        <li><code>docs/server-web-client/http-api.md</code>：HTTP API 说明</li>
        <li><code>docs/server-agent/agent-protocol.md</code>：Agent 协议说明</li>
        <li><code>docs/server-web-client/openapi.yaml</code>：OpenAPI 正式规范</li>
      </ul>
    </section>
  </main>
</body>
</html>"#,
    )
}
