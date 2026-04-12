#!/usr/bin/env python3
import json
import os
import shutil
import socket
import subprocess
import tempfile
import time
import urllib.error
import urllib.request
from pathlib import Path

ROOT = Path("/Users/trtyr/Documents/Code/Rust/hermes")
SERVER_DIR = ROOT / "server"
AGENT_DIR = ROOT / "agent"
SERVER_BIN = SERVER_DIR / "target" / "debug" / "server"
AGENT_BIN = AGENT_DIR / "target" / "debug" / "agent"
AGENT_SERVER_RS = AGENT_DIR / "src" / "server.rs"


def pick_port() -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]


def wait_http(url: str, timeout: float = 20.0) -> None:
    deadline = time.time() + timeout
    last_error = None
    while time.time() < deadline:
        try:
            with urllib.request.urlopen(url, timeout=2) as resp:
                if resp.status == 200:
                    return
        except Exception as exc:
            last_error = exc
            time.sleep(0.2)
    raise RuntimeError(f"timed out waiting for {url}: {last_error}")


def request_json(
    base: str,
    path: str,
    method: str = "GET",
    body=None,
    headers=None,
    timeout: float = 5,
):
    data = None
    merged_headers = {"x-operator": "e2e-regression"}
    if headers:
        merged_headers.update(headers)
    if body is not None:
        data = json.dumps(body).encode()
        merged_headers["Content-Type"] = "application/json"
    req = urllib.request.Request(
        base + path, data=data, method=method, headers=merged_headers
    )
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        payload = resp.read().decode()
        return json.loads(payload) if payload else None


def request_json_with_status(
    base: str,
    path: str,
    method: str = "GET",
    body=None,
    headers=None,
    timeout: float = 5,
):
    data = None
    merged_headers = {"x-operator": "e2e-regression"}
    if headers:
        merged_headers.update(headers)
    if body is not None:
        data = json.dumps(body).encode()
        merged_headers["Content-Type"] = "application/json"
    req = urllib.request.Request(
        base + path, data=data, method=method, headers=merged_headers
    )
    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            payload = resp.read().decode()
            return resp.status, json.loads(payload) if payload else None
    except urllib.error.HTTPError as exc:
        payload = exc.read().decode()
        return exc.code, json.loads(payload) if payload else None


def wait_until(
    predicate, timeout: float = 20.0, interval: float = 0.2, label: str = "condition"
):
    deadline = time.time() + timeout
    last_value = None
    last_error = None
    while time.time() < deadline:
        try:
            last_value = predicate()
            last_error = None
        except Exception as exc:
            last_value = None
            last_error = exc
        if last_value:
            return last_value
        time.sleep(interval)
    details = f"last value={last_value!r}"
    if last_error is not None:
        details += f", last error={last_error!r}"
    raise RuntimeError(f"timed out waiting for {label}: {details}")


def wait_task_terminal(base: str, task_id: str, timeout: float = 60.0):
    terminal = {"succeeded", "failed", "cancelled"}
    deadline = time.time() + timeout
    last = None
    last_error = None
    while time.time() < deadline:
        try:
            last = request_json(base, f"/tasks/{task_id}")
            last_error = None
        except Exception as exc:
            last_error = exc
            last = None
            time.sleep(0.25)
            continue
        if last["status"] in terminal:
            return last
        time.sleep(0.25)
    raise RuntimeError(
        f"task {task_id} did not reach terminal state: last={last!r}, error={last_error!r}"
    )


def wait_broadcast_settled(base: str, task_id: str, timeout: float = 60.0):
    deadline = time.time() + timeout
    last_parent = None
    last_error = None
    while time.time() < deadline:
        try:
            last_parent = request_json(base, f"/tasks/{task_id}")
            child_states = [
                request_json(base, f"/tasks/{child_id}")["status"]
                for child_id in last_parent["children"]
            ]
            last_error = None
        except Exception as exc:
            last_error = exc
            time.sleep(0.25)
            continue
        if all(
            state not in {"pending", "dispatched", "running"} for state in child_states
        ):
            return request_json(base, f"/tasks/{task_id}")
        time.sleep(0.25)
    raise RuntimeError(
        f"broadcast task {task_id} did not settle: last={last_parent!r}, error={last_error!r}"
    )


def list_online_agents(base: str):
    return request_json(base, "/agents")["agents"]


def online_agent_ids(base: str) -> set[str]:
    return {agent["agent_id"] for agent in list_online_agents(base)}


def agent_history(base: str, limit: int = 50):
    return request_json(base, f"/agents/history?limit={limit}&offset=0")


def agent_history_map(base: str, limit: int = 50) -> dict[str, dict]:
    return {item["agent_id"]: item for item in agent_history(base, limit)["agents"]}


def wait_agent_online(base: str, agent_id: str, timeout: float = 20.0):
    return wait_until(
        lambda: next(
            (
                agent
                for agent in list_online_agents(base)
                if agent["agent_id"] == agent_id
            ),
            None,
        ),
        timeout=timeout,
        label=f"{agent_id} online",
    )


def wait_agent_offline(
    base: str, agent_id: str, timeout: float = 20.0, history_limit: int = 50
):
    def predicate():
        if agent_id in online_agent_ids(base):
            return None
        record = agent_history_map(base, history_limit).get(agent_id)
        if record and record.get("is_online") is False:
            return record
        return None

    return wait_until(predicate, timeout=timeout, label=f"{agent_id} offline")


def terminate_process(proc: subprocess.Popen) -> None:
    if proc.poll() is not None:
        return
    proc.kill()
    try:
        proc.wait(timeout=5)
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.wait(timeout=5)


def spawn(cmd, cwd: Path, env=None) -> subprocess.Popen:
    return subprocess.Popen(
        cmd,
        cwd=cwd,
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        text=True,
    )


def ensure_binaries() -> None:
    subprocess.run(["cargo", "build"], cwd=SERVER_DIR, check=True)
    subprocess.run(["cargo", "build"], cwd=AGENT_DIR, check=True)
    if not SERVER_BIN.exists():
        raise FileNotFoundError(SERVER_BIN)
    if not AGENT_BIN.exists():
        raise FileNotFoundError(AGENT_BIN)


def create_config(
    tcp_port: int, api_port: int, sqlite_path: Path, api_token: str, agent_token: str
) -> str:
    return f'''[server]\nhost = "127.0.0.1"\nport = {tcp_port}\n\n[api]\nhost = "127.0.0.1"\nport = {api_port}\n\n[storage]\nsqlite_path = "{sqlite_path.as_posix()}"\n\n[auth]\napi_token = "{api_token}"\nagent_token = "{agent_token}"\n'''


class Harness:
    def __init__(
        self,
        prefix: str,
        api_token: str = "",
        agent_token: str = "",
        temp_root: Path | None = None,
    ):
        self.tcp_port = pick_port()
        self.api_port = pick_port()
        self._owns_temp_root = temp_root is None
        self.temp_root = temp_root or Path(tempfile.mkdtemp(prefix=prefix))
        self.base = f"http://127.0.0.1:{self.api_port}"
        self.processes = []
        self.agent_processes = {}
        self.server_proc = None
        self._agent_binary_cache = {}
        (self.temp_root / "data").mkdir(parents=True, exist_ok=True)
        (self.temp_root / "config.toml").write_text(
            create_config(
                self.tcp_port,
                self.api_port,
                self.temp_root / "data" / "server.db",
                api_token,
                agent_token,
            )
        )

    def start_server(self) -> None:
        server_proc = spawn([str(SERVER_BIN)], self.temp_root)
        self.server_proc = server_proc
        self.processes.append(server_proc)
        wait_http(self.base + "/health")

    def stop_server(self) -> None:
        if self.server_proc is None:
            return
        terminate_process(self.server_proc)
        self.server_proc = None

    def _render_agent_server_module(
        self, server_addr: str, token: str | None, protocol: str = "tcp"
    ) -> str:
        token_literal = "None" if token is None else json.dumps(token)
        if token is not None:
            token_literal = f"Some({token_literal})"
        return (
            "//! Embedded server connection profile.\n"
            "// Generated by e2e harness. Do not commit.\n\n"
            f"const EMBEDDED_SERVER_ADDR: &str = {json.dumps(server_addr)};\n"
            f"const EMBEDDED_AGENT_TOKEN: Option<&str> = {token_literal};\n"
            f"const EMBEDDED_PROTOCOL: &str = {json.dumps(protocol)};\n\n"
            "pub fn get_server_addr() -> String {\n"
            "    EMBEDDED_SERVER_ADDR.to_string()\n"
            "}\n\n"
            "pub fn get_agent_token() -> Option<String> {\n"
            "    EMBEDDED_AGENT_TOKEN.map(str::to_string)\n"
            "}\n\n"
            "pub fn get_protocol() -> &'static str {\n"
            "    EMBEDDED_PROTOCOL\n"
            "}\n"
        )

    def _ensure_agent_binary(self, server_addr: str, token: str | None) -> Path:
        cache_key = (server_addr, token)
        cached = self._agent_binary_cache.get(cache_key)
        if cached is not None and cached.exists():
            return cached

        previous = AGENT_SERVER_RS.read_text()
        try:
            AGENT_SERVER_RS.write_text(
                self._render_agent_server_module(server_addr, token)
            )
            subprocess.run(["cargo", "build"], cwd=AGENT_DIR, check=True)
        finally:
            AGENT_SERVER_RS.write_text(previous)

        artifact_dir = self.temp_root / "agent-cache"
        artifact_dir.mkdir(parents=True, exist_ok=True)
        suffix = "with-token" if token else "no-token"
        artifact_path = artifact_dir / f"agent-{server_addr.replace(':', '_')}-{suffix}"
        shutil.copy2(AGENT_BIN, artifact_path)
        artifact_path.chmod(0o755)
        self._agent_binary_cache[cache_key] = artifact_path
        return artifact_path

    def start_agent(
        self, agent_id: str, token: str | None = None, server_addr: str | None = None
    ) -> subprocess.Popen:
        # Stop existing agent with the same id to avoid binary-in-use issues on macOS
        self.stop_agent(agent_id)

        source_binary = self._ensure_agent_binary(
            server_addr or f"127.0.0.1:{self.tcp_port}", token
        )
        launch_dir = self.temp_root / "agents"
        launch_dir.mkdir(parents=True, exist_ok=True)
        launch_path = launch_dir / agent_id
        shutil.copy2(source_binary, launch_path)
        launch_path.chmod(0o755)
        proc = spawn([str(launch_path)], launch_dir)
        self.processes.append(proc)
        self.agent_processes[agent_id] = proc
        return proc

    def stop_agent(self, agent_id: str) -> None:
        proc = self.agent_processes.pop(agent_id, None)
        if proc is not None:
            terminate_process(proc)

    def close(self) -> None:
        for proc in reversed(self.processes):
            terminate_process(proc)
        if self._owns_temp_root:
            shutil.rmtree(self.temp_root, ignore_errors=True)
