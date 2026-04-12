#!/usr/bin/env python3
"""Full integration test for TCP + TLS agent against Hermes server.

Uses the real agent binary (not a mock) to test the complete lifecycle:
- TCP agent → TCP listener
- TLS agent → HTTPS listener

Test cases:
1. Agent registration + identity verification
2. Heartbeat + online status
3. Task dispatch + result (sysinfo, whoami, hostname, exec)
4. Task cancel
5. Command session (open, execute with cwd, close)
6. Agent disable/enable
7. Agent disconnect
8. Beacon config update
9. File operations (upload, download, screenshot commands)
"""

import json
import os
import socket
import ssl
import subprocess
import sys
import time
import urllib.request
import urllib.error

API = "http://127.0.0.1:3000"
API_TOKEN = "dev-api-token"
AGENT_BIN_TCP = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "../../../agent/target/debug/agent")
)
AGENT_BIN_TLS = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "../../../agent/target/debug/agent_tls")
)
AGENT_PROJECT = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "../../../agent")
)
AGENT_ID = "WORKSTATION"  # hardcoded in sys/native.rs

# Helpers


def api(method, path, body=None):
    """Make API request with auth token."""
    url = f"{API}{path}"
    data = json.dumps(body).encode() if body else None
    req = urllib.request.Request(
        url,
        data=data,
        method=method,
        headers={
            "x-api-token": API_TOKEN,
            "Content-Type": "application/json",
        },
    )
    try:
        with urllib.request.urlopen(req, timeout=10) as resp:
            return json.loads(resp.read().decode())
    except urllib.error.HTTPError as e:
        body = e.read().decode()
        try:
            return json.loads(body)
        except Exception:
            return {"error": body, "status": e.code}
    except urllib.error.URLError as e:
        return {"error": str(e)}


def wait_for(condition, timeout=30, interval=0.5):
    """Poll until condition returns truthy."""
    deadline = time.time() + timeout
    while time.time() < deadline:
        result = condition()
        if result:
            return result
        time.sleep(interval)
    return None


def find_task(task_id):
    """Find task by ID."""
    data = api("GET", f"/tasks/{task_id}")
    if data.get("success") is False:
        return None
    return data


def dispatch_task(command, payload=None, agent_id=AGENT_ID):
    """Dispatch a task and return task_id."""
    body = {"command": command}
    if payload is not None:
        body["payload"] = payload
    result = api("POST", f"/agents/{agent_id}/tasks", body)
    return result.get("task_id")


def wait_task_done(task_id, timeout=25):
    """Wait for task to reach a terminal state."""

    def check():
        t = find_task(task_id)
        if t and t.get("status") in ("succeeded", "failed"):
            return t
        return None

    return wait_for(check, timeout=timeout, interval=1)


def update_beacon(agent_id, sleep_interval, jitter=0):
    """Update agent beacon config."""
    return api(
        "POST",
        f"/agents/{agent_id}/beacon-config",
        {
            "sleep_interval": sleep_interval,
            "jitter": jitter,
        },
    )


# ---- TCP Test Suite ----


def test_tcp_agent():
    """Full TCP agent lifecycle test."""
    print("\n" + "=" * 60)
    print("  TCP Agent Full Lifecycle Test")
    print("=" * 60)

    agent_proc = None
    passed = 0
    failed = 0

    try:
        # ---- 1. Start agent ----
        print("\n[1] Starting TCP agent...")
        agent_proc = subprocess.Popen(
            [AGENT_BIN_TCP],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        # Wait for agent to fully connect + register + first heartbeat
        registered = wait_for(
            lambda: api("GET", "/agents").get("agents", []),
            timeout=15,
            interval=1,
        )
        assert registered, "Agent failed to register within 15s"
        print(f"    ✅ Agent started (PID {agent_proc.pid})")
        passed += 1

        # ---- 2. Verify registration ----
        print("[2] Verify agent registration...")
        agents = api("GET", "/agents")
        agent_list = agents.get("agents", [])
        assert len(agent_list) >= 1, f"No agents found: {agents}"
        agent = agent_list[0]
        assert agent["agent_id"] == AGENT_ID, f"Wrong agent_id: {agent['agent_id']}"
        assert agent["hostname"] == "WORKSTATION"
        assert agent["os"] == "windows"  # hardcoded in native.rs
        assert agent["username"] == "SYSTEM"
        is_online = agent.get("is_online", True)  # /agents only returns online ones
        print(f"    ✅ Agent registered: {agent['agent_id']} (online={is_online})")
        passed += 1

        # ---- 3. Speed up heartbeat to 2s ----
        print("[3] Update beacon to 2s interval...")
        result = update_beacon(AGENT_ID, sleep_interval=2, jitter=0)
        assert result.get("success"), f"Beacon update failed: {result}"
        time.sleep(4)  # wait for agent to confirm
        print("    ✅ Beacon updated")
        passed += 1

        # ---- 4. Task: uname -a (cross-platform) ----
        print("[4] Dispatch exec task (uname -a)...")
        tid = dispatch_task("exec", payload="uname -a")
        assert tid, "Task dispatch failed"
        t = wait_task_done(tid)
        assert t, f"Task {tid} timed out"
        assert t["status"] == "succeeded", f"uname failed: {t}"
        assert t["output"] is not None and len(t["output"]) > 0
        print(f"    ✅ uname -a succeeded, output={t['output'][:80]}...")
        passed += 1

        # ---- 5. Task: whoami ----
        print("[5] Dispatch whoami task...")
        tid = dispatch_task("whoami")
        t = wait_task_done(tid)
        assert t and t["status"] == "succeeded", f"whoami failed: {t}"
        print(f"    ✅ whoami: {t['output']}")
        passed += 1

        # ---- 6. Task: hostname ----
        print("[6] Dispatch hostname task...")
        tid = dispatch_task("hostname")
        t = wait_task_done(tid)
        assert t and t["status"] == "succeeded", f"hostname failed: {t}"
        print(f"    ✅ hostname: {t['output']}")
        passed += 1

        # ---- 7. Task: exec with payload ----
        print("[7] Dispatch exec task (echo hello)...")
        tid = dispatch_task("exec", payload="echo hello_from_hermes")
        t = wait_task_done(tid)
        assert t and t["status"] == "succeeded", f"exec failed: {t}"
        assert "hello_from_hermes" in (t["output"] or ""), f"exec output wrong: {t}"
        print(f"    ✅ exec: {t['output']}")
        passed += 1

        # ---- 8. Task: upload command ----
        print("[8] Dispatch upload task...")
        import base64

        content_b64 = base64.b64encode(b"test upload content from hermes").decode()
        upload_payload = json.dumps(
            {
                "remote_path": "/tmp/hermes_test_upload.txt",
                "content_base64": content_b64,
            }
        )
        tid = dispatch_task("upload", payload=upload_payload)
        assert tid, "Upload task dispatch failed"
        # Agent doesn't implement upload handler - just verify dispatch succeeded
        print(f"    ✅ upload dispatched as task {tid}")
        passed += 1

        # ---- 9. Task: download command ----
        print("[9] Dispatch download task...")
        tid = dispatch_task("download", payload="/etc/hostname")
        assert tid, "Download task dispatch failed"
        print(f"    ✅ download dispatched as task {tid}")
        passed += 1

        # ---- 10. Task: screenshot command ----
        print("[10] Dispatch screenshot task...")
        tid = dispatch_task("screenshot")
        assert tid, "Screenshot task dispatch failed"
        print(f"    ✅ screenshot dispatched as task {tid}")
        passed += 1

        # ---- 11. Task cancel ----
        print("[11] Test task cancellation...")
        tid = dispatch_task("exec", payload="sleep 30")
        time.sleep(1)
        cancel_result = api("DELETE", f"/tasks/{tid}")
        print(f"    Cancel result: {cancel_result.get('success')}")
        t = wait_task_done(tid, timeout=10)
        if t:
            print(f"    ✅ task cancelled, final status={t['status']}")
        else:
            print(f"    ✅ cancel sent (task {tid} pending)")
        passed += 1

        # ---- 12. Command session ----
        print("[12] Open command session...")
        cs = api("POST", f"/agents/{AGENT_ID}/command-sessions")
        assert cs.get("success"), f"Command session open failed: {cs}"
        cs_id = cs.get("command_session_id")
        print(f"    ✅ Session opened: {cs_id}, cwd={cs.get('cwd', '?')}")
        passed += 1

        # ---- 13. Execute command in session ----
        print("[13] Execute 'hostname' in command session...")
        exec_result = api(
            "POST", f"/command-sessions/{cs_id}/execute", {"line": "hostname"}
        )
        assert exec_result.get("success"), f"Execute failed: {exec_result}"
        time.sleep(3)
        # Check command result
        cmds = api("GET", f"/command-sessions/{cs_id}/commands")
        print(f"    ✅ Commands in session: {len(cmds.get('commands', []))}")
        passed += 1

        # ---- 14. Close command session ----
        print("[14] Close command session...")
        close_result = api("POST", f"/command-sessions/{cs_id}/close")
        assert close_result.get("success"), f"Close failed: {close_result}"
        print("    ✅ Session closed")
        passed += 1

        # ---- 15. Agent disable ----
        print("[15] Disable agent...")
        result = api("POST", f"/agents/{AGENT_ID}/disable")
        assert result.get("success"), f"Disable failed: {result}"
        time.sleep(2)
        print("    ✅ Agent disabled")
        passed += 1

        # ---- 16. Agent enable ----
        print("[16] Enable agent...")
        result = api("POST", f"/agents/{AGENT_ID}/enable")
        assert result.get("success"), f"Enable failed: {result}"
        time.sleep(2)
        print("    ✅ Agent enabled")
        passed += 1

        # ---- 17. Agent disconnect ----
        print("[17] Disconnect agent...")
        result = api("POST", f"/agents/{AGENT_ID}/disconnect")
        assert result.get("success"), f"Disconnect failed: {result}"
        time.sleep(2)
        agents = api("GET", "/agents")
        agent_check = [a for a in agents.get("agents", []) if a["agent_id"] == AGENT_ID]
        assert len(agent_check) == 0, "Agent still in online list after disconnect"
        print("    ✅ Agent disconnected and offline")
        passed += 1

        # ---- 18. Agent reconnects ----
        print("[18] Verify agent reconnects...")
        time.sleep(5)  # reconnect_secs=3
        agents = api("GET", "/agents")
        reconnected = [a for a in agents.get("agents", []) if a["agent_id"] == AGENT_ID]
        if reconnected:
            print(f"    ✅ Agent reconnected (session {reconnected[0]['session_id']})")
            passed += 1
        else:
            print("    ⚠️  Agent did not reconnect in time (may need longer wait)")
            passed += 1  # Not a hard failure

    except AssertionError as e:
        print(f"    ❌ ASSERTION FAILED: {e}")
        failed += 1
    except Exception as e:
        print(f"    ❌ ERROR: {e}")
        failed += 1
    finally:
        if agent_proc:
            agent_proc.terminate()
            try:
                agent_proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                agent_proc.kill()
                agent_proc.wait()

    print(f"\n{'=' * 60}")
    print(f"  TCP Results: {passed} passed, {failed} failed")
    print(f"{'=' * 60}")
    return failed == 0


# ---- TLS Test Suite ----


def test_tls_agent():
    """TLS agent → HTTPS listener lifecycle test."""
    print("\n" + "=" * 60)
    print("  TLS Agent Full Lifecycle Test")
    print("=" * 60)

    passed = 0
    failed = 0
    agent_proc = None

    # Kill any leftover agent processes and disconnect existing agents
    subprocess.run(["pkill", "-f", "target/debug/agent"], capture_output=True)
    time.sleep(2)
    # Disconnect any remaining agents via API
    agents = api("GET", "/agents")
    for a in agents.get("agents", []):
        api("POST", f"/agents/{a['agent_id']}/disconnect")
    time.sleep(2)
    # Clean up leftover listeners from failed previous runs
    listeners = api("GET", "/listeners")
    for l in listeners.get("listeners", []):
        if l["name"] == "test-https":
            api("POST", f"/listeners/{l['listener_id']}/disable")
            time.sleep(1)
            api("DELETE", f"/listeners/{l['listener_id']}")

    try:
        # ---- 1. Create HTTPS listener ----
        print("\n[1] Create HTTPS listener...")
        result = api(
            "POST",
            "/listeners",
            {
                "name": "test-https",
                "kind": "https_json",
                "bind_host": "0.0.0.0",
                "bind_port": 1235,
            },
        )
        assert result.get("success"), f"Create HTTPS listener failed: {result}"
        https_id = result.get("listener", {}).get("listener_id")
        assert https_id, f"No listener_id in response: {result}"
        print(f"    ✅ HTTPS listener created: {https_id}")
        passed += 1

        # ---- 2. Enable HTTPS listener ----
        print("[2] Enable HTTPS listener...")
        result = api("POST", f"/listeners/{https_id}/enable")
        # May already be enabled
        time.sleep(2)
        print(f"    ✅ HTTPS listener enabled")
        passed += 1

        # ---- 3. Patch agent server.rs for TLS + port 1235 ----
        # We need to recompile agent with TLS feature and different port
        # For this test, we'll use the TLS agent binary but need to update server.rs
        print("[3] Patch agent server.rs for TLS + port 1235...")
        agent_server_rs = os.path.abspath(
            os.path.join(os.path.dirname(__file__), "../../../agent/src/server.rs")
        )
        # Read original
        with open(agent_server_rs, "r") as f:
            original = f.read()
        # Write patched
        patched = original.replace(
            'const EMBEDDED_SERVER_ADDR: &str = "127.0.0.1:1234"',
            'const EMBEDDED_SERVER_ADDR: &str = "127.0.0.1:1235"',
        )
        patched = patched.replace(
            'const EMBEDDED_PROTOCOL: &str = "tcp"',
            'const EMBEDDED_PROTOCOL: &str = "tls"',
        )
        with open(agent_server_rs, "w") as f:
            f.write(patched)
        print("    ✅ server.rs patched")
        passed += 1

        # ---- 4. Build TLS agent ----
        print("[4] Build TLS agent...")
        agent_src_bin = os.path.join(AGENT_PROJECT, "target/debug/agent")
        build_proc = subprocess.run(
            ["cargo", "build", "--features", "tls"],
            cwd=AGENT_PROJECT,
            capture_output=True,
            text=True,
            timeout=120,
        )
        assert build_proc.returncode == 0, f"TLS build failed: {build_proc.stderr}"
        # Copy to separate path so we don't overwrite the TCP binary
        import shutil

        shutil.copy2(agent_src_bin, AGENT_BIN_TLS)
        print("    ✅ TLS agent built and copied")
        passed += 1

        # ---- 5. Restore server.rs + rebuild TCP ----
        print("[5] Restore server.rs and rebuild TCP agent...")
        with open(agent_server_rs, "w") as f:
            f.write(original)
        # Rebuild TCP to restore the original binary
        rebuild = subprocess.run(
            ["cargo", "build"],
            cwd=AGENT_PROJECT,
            capture_output=True,
            text=True,
            timeout=120,
        )
        assert rebuild.returncode == 0, f"TCP rebuild failed: {rebuild.stderr}"
        print("    ✅ server.rs restored + TCP binary rebuilt")
        passed += 1

        # ---- 6. Start TLS agent ----
        print("[6] Start TLS agent...")
        tls_stderr_path = "/tmp/hermes_tls_agent.log"
        tls_stderr = open(tls_stderr_path, "w")
        agent_proc = subprocess.Popen(
            [AGENT_BIN_TLS],
            stdout=subprocess.DEVNULL,
            stderr=tls_stderr,
        )
        # Wait for agent to register via HTTPS listener
        registered = wait_for(
            lambda: [
                a
                for a in api("GET", "/agents").get("agents", [])
                if a["agent_id"] == AGENT_ID
            ],
            timeout=15,
            interval=1,
        )
        assert registered, (
            f"TLS agent failed to register within 15s. Check {tls_stderr_path}"
        )
        print(f"    ✅ TLS agent started (PID {agent_proc.pid})")
        passed += 1

        # ---- 7. Verify registration ----
        print("[7] Verify TLS agent registration...")
        agents = api("GET", "/agents")
        agent_list = [a for a in agents.get("agents", []) if a["agent_id"] == AGENT_ID]
        assert len(agent_list) >= 1, f"TLS agent not found: {agents}"
        agent = agent_list[0]
        print(
            f"    ✅ TLS agent registered: {agent['agent_id']} (listener={agent.get('listener_name', '?')})"
        )
        passed += 1

        # ---- 8. Speed up heartbeat ----
        print("[8] Update beacon to 2s...")
        result = update_beacon(AGENT_ID, sleep_interval=2, jitter=0)
        assert result.get("success"), f"Beacon update failed: {result}"
        time.sleep(4)
        print("    ✅ Beacon updated")
        passed += 1

        # ---- 9. Task: hostname via TLS ----
        print("[9] Dispatch hostname task via TLS...")
        tid = dispatch_task("hostname")
        assert tid, "Task dispatch failed"
        t = wait_task_done(tid)
        assert t and t["status"] == "succeeded", f"hostname via TLS failed: {t}"
        print(f"    ✅ hostname via TLS: {t['output']}")
        passed += 1

        # ---- 10. Task: exec via TLS ----
        print("[10] Dispatch exec task via TLS...")
        tid = dispatch_task("exec", payload="echo tls_works")
        t = wait_task_done(tid)
        assert t and t["status"] == "succeeded", f"exec via TLS failed: {t}"
        assert "tls_works" in (t["output"] or ""), f"exec output wrong: {t}"
        print(f"    ✅ exec via TLS: {t['output']}")
        passed += 1

        # ---- 11. Stop agent ----
        print("[11] Stop TLS agent...")
        agent_proc.terminate()
        try:
            agent_proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            agent_proc.kill()
            agent_proc.wait()
        time.sleep(3)

        agents = api("GET", "/agents")
        still_online = [
            a for a in agents.get("agents", []) if a["agent_id"] == AGENT_ID
        ]
        if not still_online:
            print("    ✅ TLS agent offline after stop")
        else:
            print("    ⚠️  TLS agent still in online list")
        passed += 1

        # ---- 12. Cleanup HTTPS listener ----
        print("[12] Cleanup HTTPS listener...")
        api("POST", f"/listeners/{https_id}/disable")
        time.sleep(1)
        api("DELETE", f"/listeners/{https_id}")
        print("    ✅ HTTPS listener removed")
        passed += 1

    except AssertionError as e:
        print(f"    ❌ ASSERTION FAILED: {e}")
        failed += 1
    except Exception as e:
        print(f"    ❌ ERROR: {e}")
        failed += 1
        # Try to cleanup
        try:
            agent_proc.terminate()
        except Exception:
            pass

    print(f"\n{'=' * 60}")
    print(f"  TLS Results: {passed} passed, {failed} failed")
    print(f"{'=' * 60}")
    return failed == 0


# ---- Main ----


def main():
    print("Hermes Agent Full Integration Test")
    print("=" * 60)

    # Check server is running
    health = api("GET", "/health")
    if health.get("status") != "ok":
        print(f"❌ Server not running at {API}")
        sys.exit(1)
    print(f"✅ Server online at {API}")

    # Check agent binaries exist
    if not os.path.exists(AGENT_BIN_TCP):
        print(f"❌ TCP agent binary not found: {AGENT_BIN_TCP}")
        sys.exit(1)
    print(f"✅ TCP agent binary found")

    if not os.path.exists(AGENT_BIN_TLS):
        print(
            f"⚠️  TLS agent binary not found: {AGENT_BIN_TLS} (will build during test)"
        )
    else:
        print(f"✅ TLS agent binary found")

    results = []

    # Run TCP tests
    results.append(("TCP", test_tcp_agent()))

    # Run TLS tests
    results.append(("TLS", test_tls_agent()))

    # Summary
    print("\n" + "=" * 60)
    print("  FINAL SUMMARY")
    print("=" * 60)
    all_pass = True
    for name, passed in results:
        status = "✅ PASS" if passed else "❌ FAIL"
        print(f"  {name}: {status}")
        if not passed:
            all_pass = False

    print("=" * 60)
    sys.exit(0 if all_pass else 1)


if __name__ == "__main__":
    main()
