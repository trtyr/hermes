#!/usr/bin/env python3
"""E2E test for HTTPS listener: Python mock TLS agent exercises full protocol cycle.

Covers: TLS handshake, register, heartbeat, task receive, task result report,
heartbeat-triggered task dispatch, upload/download/screenshot APIs, disconnect cleanup.
"""

import json
import socket
import ssl
import sys
import time
from pathlib import Path

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import (
        Harness,
        SERVER_BIN,
        request_json,
        wait_until,
        wait_http,
        wait_task_terminal,
        wait_agent_offline,
        pick_port,
    )
else:
    from .common import (
        Harness,
        SERVER_BIN,
        request_json,
        wait_until,
        wait_http,
        wait_task_terminal,
        wait_agent_offline,
        pick_port,
    )


def connect_tls_agent(host: str, port: int) -> socket.socket:
    """Connect to HTTPS JSON listener, skip cert verification (self-signed)."""
    ctx = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE
    raw = socket.create_connection((host, port), timeout=10)
    return ctx.wrap_socket(raw, server_hostname=host)


def send_frame(sock: socket.socket, obj: dict) -> None:
    """Send a JSON line (protocol uses serde tag=\"type\" internal tagging)."""
    sock.sendall((json.dumps(obj) + "\n").encode())


def recv_frame(sock: socket.socket, timeout: float = 10.0) -> dict | None:
    """Receive one JSON line. Returns None on timeout or disconnect."""
    sock.settimeout(timeout)
    buf = b""
    while True:
        try:
            chunk = sock.recv(4096)
            if not chunk:
                return None
            buf += chunk
            if b"\n" in buf:
                line, _ = buf.split(b"\n", 1)
                return json.loads(line.strip())
        except socket.timeout:
            return None


def dispatch_and_wait(sock: socket.socket, task_resp: dict) -> dict:
    """Send heartbeat to trigger dispatch, then receive and return the DispatchTask frame."""
    send_frame(sock, {"type": "heartbeat"})
    cmd = recv_frame(sock, timeout=10.0)
    assert cmd is not None, f"no dispatch_task received for task {task_resp['task_id']}"
    assert cmd.get("type") == "dispatch_task", f"expected dispatch_task, got: {cmd}"
    return cmd


def run(harness: Harness) -> dict:
    base = harness.base

    # 1. Create an HTTPS JSON listener
    https_port = pick_port()
    created = request_json(
        base,
        "/listeners",
        method="POST",
        body={
            "name": "https-test",
            "kind": "https_json",
            "bind_host": "127.0.0.1",
            "bind_port": https_port,
            "enabled": True,
            "config": {},
        },
    )["listener"]
    listener_id = created["listener_id"]
    assert created["kind"] == "https_json", created

    # Wait for running
    running = wait_until(
        lambda: (
            request_json(base, f"/listeners/{listener_id}")
            if request_json(base, f"/listeners/{listener_id}")["runtime_status"]
            == "running"
            else None
        ),
        timeout=15.0,
        label="https listener running",
    )
    assert running["runtime_status"] == "running", running

    # 2. Connect mock TLS agent
    sock = connect_tls_agent("127.0.0.1", https_port)

    # 3. Register — protocol uses {"type": "register", ...} (serde internal tag)
    agent_id = "tls-agent-01"
    send_frame(
        sock,
        {
            "type": "register",
            "agent_id": agent_id,
            "hostname": "test-host",
            "username": "test-user",
            "os": "Linux",
            "arch": "x86_64",
            "pid": 1234,
            "internal_ip": "10.0.0.5",
            "tags": [],
            "sleep_interval": 5,
            "jitter": 10,
        },
    )

    # 4. Receive Ack (and possibly DispatchTask if task was dispatched during register)
    first = recv_frame(sock, timeout=10.0)
    assert first is not None, "no response after register"

    # Collect any additional frames that arrive quickly
    second = recv_frame(sock, timeout=3.0)

    frames = [first]
    if second is not None:
        frames.append(second)

    ack = next((f for f in frames if f.get("type") == "ack"), None)
    assert ack is not None, f"no ack in frames: {frames}"

    # 5. Verify agent online via API
    agent_online = wait_until(
        lambda: next(
            (
                a
                for a in request_json(base, "/agents")["agents"]
                if a["agent_id"] == agent_id
            ),
            None,
        ),
        timeout=10.0,
        label=f"{agent_id} online",
    )
    assert agent_online is not None

    # 6. Dispatch task via API
    task_resp = request_json(
        base,
        f"/agents/{agent_id}/tasks",
        method="POST",
        body={"command": "sysinfo"},
    )
    assert task_resp["success"] is True, task_resp
    task_id = task_resp["task_id"]

    # 7. Send heartbeat to trigger pending task dispatch
    send_frame(sock, {"type": "heartbeat"})

    # 8. Agent receives DispatchTask
    task_cmd = recv_frame(sock, timeout=10.0)
    assert task_cmd is not None, "no task command received"
    assert task_cmd.get("type") == "dispatch_task", (
        f"expected dispatch_task, got: {task_cmd}"
    )
    assert task_cmd["task_id"] == task_id

    # 8. Agent sends TaskResult
    send_frame(
        sock,
        {
            "type": "task_result",
            "task_id": task_id,
            "success": True,
            "output": '{"os":"Linux","hostname":"test-host"}',
        },
    )

    # 9. Verify task succeeded
    final_task = wait_task_terminal(base, task_id, timeout=10.0)
    assert final_task["status"] == "succeeded", final_task

    # 10. Test heartbeat-triggered task dispatch
    task2_resp = request_json(
        base,
        f"/agents/{agent_id}/tasks",
        method="POST",
        body={"command": "whoami"},
    )
    task2_id = task2_resp["task_id"]

    # Heartbeat triggers pending task dispatch
    send_frame(sock, {"type": "heartbeat"})
    task2_cmd = recv_frame(sock, timeout=10.0)
    assert task2_cmd is not None, "no task2 via heartbeat"
    assert task2_cmd.get("type") == "dispatch_task"
    assert task2_cmd["task_id"] == task2_id

    send_frame(
        sock,
        {
            "type": "task_result",
            "task_id": task2_id,
            "success": True,
            "output": "test-user",
        },
    )
    final_task2 = wait_task_terminal(base, task2_id, timeout=10.0)
    assert final_task2["status"] == "succeeded", final_task2

    # 11. Test upload API
    upload_resp = request_json(
        base,
        f"/agents/{agent_id}/upload",
        method="POST",
        body={"remote_path": "C:\\test.txt", "content_base64": "SGVsbG8gV29ybGQ="},
    )
    assert upload_resp["success"] is True, upload_resp
    upload_task_id = upload_resp["task_id"]

    upload_cmd = dispatch_and_wait(sock, upload_resp)
    assert upload_cmd["command"] == "upload"
    payload = json.loads(upload_cmd["payload"])
    assert payload["remote_path"] == "C:\\test.txt"
    assert payload["content_base64"] == "SGVsbG8gV29ybGQ="

    send_frame(
        sock,
        {
            "type": "task_result",
            "task_id": upload_task_id,
            "success": True,
            "output": "uploaded",
        },
    )
    wait_task_terminal(base, upload_task_id, timeout=10.0)

    # 12. Test download API
    download_resp = request_json(
        base,
        f"/agents/{agent_id}/download",
        method="POST",
        body={"remote_path": "C:\\secret.txt"},
    )
    assert download_resp["success"] is True, download_resp
    download_task_id = download_resp["task_id"]

    download_cmd = dispatch_and_wait(sock, download_resp)
    assert download_cmd["command"] == "download"
    assert download_cmd["payload"] == "C:\\secret.txt"

    send_frame(
        sock,
        {
            "type": "task_result",
            "task_id": download_task_id,
            "success": True,
            "output": "base64content",
        },
    )
    wait_task_terminal(base, download_task_id, timeout=10.0)

    # 13. Test screenshot API
    screenshot_resp = request_json(
        base, f"/agents/{agent_id}/screenshot", method="POST"
    )
    assert screenshot_resp["success"] is True, screenshot_resp
    screenshot_task_id = screenshot_resp["task_id"]

    screenshot_cmd = dispatch_and_wait(sock, screenshot_resp)
    assert screenshot_cmd["command"] == "screenshot"

    send_frame(
        sock,
        {
            "type": "task_result",
            "task_id": screenshot_task_id,
            "success": True,
            "output": "base64img",
        },
    )
    wait_task_terminal(base, screenshot_task_id, timeout=10.0)

    # 14. Disconnect
    sock.close()

    # 15. Verify agent offline
    wait_agent_offline(base, agent_id, timeout=15.0)

    # 16. Cleanup: disable then delete the https listener
    request_json(base, f"/listeners/{listener_id}/disable", method="POST")
    wait_until(
        lambda: (
            request_json(base, f"/listeners/{listener_id}")
            if request_json(base, f"/listeners/{listener_id}")["runtime_status"]
            == "stopped"
            else None
        ),
        timeout=10.0,
        label="https listener stopped",
    )
    deleted = request_json(base, f"/listeners/{listener_id}", method="DELETE")
    assert deleted["success"] is True, deleted

    return {
        "https_listener_id": listener_id,
        "tls_handshake": "ok",
        "register_ack": "ok",
        "task_direct_dispatch": final_task["status"],
        "task_via_heartbeat": final_task2["status"],
        "upload_api": upload_resp["success"],
        "download_api": download_resp["success"],
        "screenshot_api": screenshot_resp["success"],
        "disconnect_cleanup": "ok",
    }


def main() -> int:
    import subprocess

    subprocess.run(["cargo", "build"], cwd=SERVER_BIN.parent.parent.parent, check=True)
    if not SERVER_BIN.exists():
        raise FileNotFoundError("server binary not found")

    harness = Harness("hermes-https-e2e-")
    try:
        harness.start_server()
        result = run(harness)
        print(
            json.dumps(
                {"https_listener_suite": result, "base_url": harness.base},
                ensure_ascii=False,
                indent=2,
            )
        )
        return 0
    finally:
        harness.close()


if __name__ == "__main__":
    raise SystemExit(main())
