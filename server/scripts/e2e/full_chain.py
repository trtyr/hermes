#!/usr/bin/env python3
"""Full-chain E2E test for Hermes C2.

Validates the complete request-response cycle across all three components:
  API → Server → Agent → Server → API   (command execution chain)
  Agent → Server → API                   (registration chain)

Sections:
  1.  Agent Registration        (Agent → Server → API)
  2.  Task Dispatch              (API → Server → Agent → Server → API)
  3.  Command Session            (open → pwd → cd → pwd → close)
  4.  File Upload                (API → Agent filesystem)
  5.  File Download              (Agent filesystem → API)
  6.  Process List               (ps command)
  7.  Screenshot                 (platform-dependent)
  8.  Agent Tags                 (PATCH → verify)
  9.  Beacon Config              (update interval/jitter)
  10. Agent Build + Download     (compile → download binary)
  11. Audit Trail                (verify all expected actions)
  12. Agent Disconnect           (Agent → Server → API)
"""

import base64
import json
import os
import sys
import urllib.request
from pathlib import Path

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import (
        Harness,
        ensure_binaries,
        request_json,
        wait_agent_online,
        wait_agent_offline,
        wait_task_terminal,
        wait_until,
    )
else:
    from .common import (
        Harness,
        ensure_binaries,
        request_json,
        wait_agent_online,
        wait_agent_offline,
        wait_task_terminal,
        wait_until,
    )


def run(harness):
    base = harness.base
    agent_id = "chain-pc-01"

    # ── 1. Agent Registration Chain ──────────────────────────────────
    # Agent → Server: TCP connect, hello, register
    # Server → API: agent appears in GET /agents
    harness.start_agent(agent_id)
    agent = wait_agent_online(base, agent_id)
    assert agent["agent_id"] == agent_id, agent
    assert agent["hostname"], agent
    # After native.rs fix, hostname should be a real value, not "WORKSTATION"
    assert agent["hostname"] != "WORKSTATION", agent

    # Verify via history endpoint that agent is marked online
    history = request_json(base, "/agents/history?limit=20&offset=0")
    record = next((a for a in history["agents"] if a["agent_id"] == agent_id), None)
    assert record is not None, "agent not in history"
    assert record["is_online"] is True, record

    # ── 2. Task Dispatch Chain ───────────────────────────────────────
    # API POST /agents/{id}/tasks → Server dispatches → Agent executes
    # → Agent returns TaskResult → Server stores → API GET /tasks/{id}
    task = request_json(
        base,
        f"/agents/{agent_id}/tasks",
        method="POST",
        body={"command": "whoami", "payload": None},
    )
    result = wait_task_terminal(base, task["task_id"])
    assert result["status"] == "succeeded", result
    assert result["output"].strip(), result

    # ── 3. Command Session Chain ─────────────────────────────────────
    # Full lifecycle: open → execute (pwd) → cd → execute (pwd) → close
    session_resp = request_json(
        base, f"/agents/{agent_id}/command-sessions", method="POST"
    )
    session = session_resp["session"]
    session_id = session["command_session_id"]
    assert session["status"] == "open", session
    assert session["cwd"], session

    pwd1 = request_json(
        base,
        f"/command-sessions/{session_id}/execute",
        method="POST",
        body={"line": "pwd"},
    )["result"]
    assert pwd1["success"] is True, pwd1
    assert pwd1["cwd_before"] == pwd1["cwd_after"], pwd1

    cd_target = "cd .." if os.name == "nt" else "cd /tmp"
    cd_result = request_json(
        base,
        f"/command-sessions/{session_id}/execute",
        method="POST",
        body={"line": cd_target},
    )["result"]
    assert cd_result["success"] is True, cd_result

    pwd2 = request_json(
        base,
        f"/command-sessions/{session_id}/execute",
        method="POST",
        body={"line": "pwd"},
    )["result"]
    assert pwd2["success"] is True, pwd2
    assert pwd2["stdout"].strip() == cd_result["cwd_after"], pwd2

    closed = request_json(base, f"/command-sessions/{session_id}/close", method="POST")
    assert closed["session"]["status"] == "closed", closed

    # ── 4. File Upload Chain ─────────────────────────────────────────
    # API POST /agents/{id}/upload → Server dispatches → Agent writes file
    test_content = b"Hello Hermes E2E! Full chain file transfer test."
    test_b64 = base64.b64encode(test_content).decode()
    test_file_path = str(harness.temp_root / "agents" / "e2e_test_file.txt")

    upload_task = request_json(
        base,
        f"/agents/{agent_id}/upload",
        method="POST",
        body={"remote_path": test_file_path, "content_base64": test_b64},
    )
    upload_result = wait_task_terminal(base, upload_task["task_id"])
    assert upload_result["status"] == "succeeded", upload_result

    # Verify file exists on agent's filesystem
    assert Path(test_file_path).exists(), f"uploaded file not found: {test_file_path}"
    assert Path(test_file_path).read_bytes() == test_content

    # ── 5. File Download Chain ───────────────────────────────────────
    # API POST /agents/{id}/download → Agent reads file → base64 encode
    # → TaskResult.output → Server stores → API returns
    download_task = request_json(
        base,
        f"/agents/{agent_id}/download",
        method="POST",
        body={"remote_path": test_file_path},
    )
    download_result = wait_task_terminal(base, download_task["task_id"])
    assert download_result["status"] == "succeeded", download_result
    decoded = base64.b64decode(download_result["output"])
    assert decoded == test_content, {
        "expected": test_content,
        "got": decoded,
    }

    # ── 6. Process List Chain ────────────────────────────────────────
    # API POST /agents/{id}/tasks command=ps → Agent runs ps/tasklist
    ps_task = request_json(
        base,
        f"/agents/{agent_id}/tasks",
        method="POST",
        body={"command": "ps", "payload": None},
    )
    ps_result = wait_task_terminal(base, ps_task["task_id"])
    assert ps_result["status"] == "succeeded", ps_result
    assert len(ps_result["output"]) > 0, ps_result

    # ── 7. Screenshot Chain ──────────────────────────────────────────
    # Platform-dependent: succeeds on Windows, fails gracefully elsewhere
    ss_task = request_json(
        base,
        f"/agents/{agent_id}/tasks",
        method="POST",
        body={"command": "screenshot", "payload": None},
    )
    ss_result = wait_task_terminal(base, ss_task["task_id"])
    if os.name == "nt":
        assert ss_result["status"] == "succeeded", ss_result
        assert len(ss_result["output"]) > 100, ss_result
    else:
        # Non-Windows: agent should report failure, not crash
        assert ss_result["status"] == "failed", ss_result

    # ── 8. Agent Tags Chain ──────────────────────────────────────────
    # API PATCH /agents/{id} → Server updates tags → API GET reflects change
    tags_update = request_json(
        base,
        f"/agents/{agent_id}",
        method="PATCH",
        body={"tags": ["e2e-test", "full-chain"]},
    )
    assert tags_update["success"] is True, tags_update

    # Verify tags via the online agents list
    agent_after = wait_agent_online(base, agent_id)
    assert set(agent_after["tags"]) == {"e2e-test", "full-chain"}, {
        "expected": ["e2e-test", "full-chain"],
        "got": agent_after["tags"],
    }

    # ── 9. Beacon Config Chain ───────────────────────────────────────
    # API POST /agents/{id}/beacon → Server forwards → Agent acknowledges
    beacon = request_json(
        base,
        f"/agents/{agent_id}/beacon-config",
        method="POST",
        body={"sleep_interval": 30, "jitter": 20},
    )
    assert beacon["success"] is True, beacon

    # ── 10. Agent Build + Download Chain ─────────────────────────────
    # API POST /listeners/{id}/agent-builds → Server compiles agent binary
    # → API GET /agent-builds/{id}/download → binary octet-stream
    listeners = request_json(base, "/listeners")
    assert listeners["listeners"], "no listeners available"
    listener_id = listeners["listeners"][0]["listener_id"]

    build = request_json(
        base,
        f"/listeners/{listener_id}/agent-builds",
        method="POST",
        body={"profile": "release"},
        timeout=120,
    )
    assert build["build"]["status"] == "succeeded", build
    build_id = build["build"]["build_id"]

    # Download the compiled binary
    download_url = f"{base}/agent-builds/{build_id}/download"
    dl_req = urllib.request.Request(
        download_url, headers={"x-operator": "e2e-regression"}
    )
    with urllib.request.urlopen(dl_req, timeout=30) as resp:
        assert resp.status == 200
        binary_data = resp.read()
        assert len(binary_data) > 1000, f"binary too small: {len(binary_data)} bytes"
        content_disp = resp.headers.get("content-disposition", "")
        assert "attachment" in content_disp, (
            f"missing content-disposition: {content_disp}"
        )

    # ── 11. Audit Trail Verification ─────────────────────────────────
    # Every operation above should have generated audit records
    audits = request_json(base, "/audits?limit=100&offset=0")
    actions = {item["action"] for item in audits["audits"]}
    expected_actions = {
        "dispatch_task",
        "open_command_session",
        "execute_command_session",
        "close_command_session",
        "upload_file",
        "download_file",
        "update_agent_beacon_config",
        "create_listener_agent_build",
    }
    missing = sorted(expected_actions - actions)
    assert not missing, {
        "missing_audit_actions": missing,
        "found_actions": sorted(actions),
    }

    # ── 12. Agent Disconnect Chain ───────────────────────────────────
    # Kill agent → Server detects disconnect → API reflects offline
    harness.stop_agent(agent_id)
    offline_record = wait_agent_offline(base, agent_id)
    assert offline_record["is_online"] is False, offline_record

    return {
        "registration": {
            "agent_id": agent["agent_id"],
            "hostname": agent["hostname"],
        },
        "task_dispatch": result["status"],
        "command_session": {
            "session_id": session_id,
            "initial_cwd": session["cwd"],
            "cwd_after_cd": cd_result["cwd_after"],
            "closed": True,
        },
        "file_upload": upload_result["status"],
        "file_download": download_result["status"],
        "process_list": ps_result["status"],
        "screenshot_platform": os.name,
        "screenshot_status": ss_result["status"],
        "tags": sorted(agent_after["tags"]),
        "beacon_updated": True,
        "build_status": build["build"]["status"],
        "build_download_bytes": len(binary_data),
        "audit_actions": sorted(actions),
        "disconnect": {
            "agent_id": agent_id,
            "is_online": False,
        },
    }


def main() -> int:
    ensure_binaries()
    harness = Harness("hermes-full-chain-e2e-")
    try:
        harness.start_server()
        print(
            json.dumps(
                {"full_chain_suite": run(harness), "base_url": harness.base},
                ensure_ascii=False,
                indent=2,
            )
        )
        return 0
    finally:
        harness.close()


if __name__ == "__main__":
    raise SystemExit(main())
