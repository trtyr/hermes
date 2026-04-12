#!/usr/bin/env python3
import json
import sys
from pathlib import Path

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import (
        Harness,
        ensure_binaries,
        request_json,
        request_json_with_status,
        wait_task_terminal,
        wait_until,
    )
else:
    from .common import (
        Harness,
        ensure_binaries,
        request_json,
        request_json_with_status,
        wait_task_terminal,
        wait_until,
    )


def run(harness):
    base = harness.base
    # Clean up any leftover agent processes from previous suites in full E2E mode
    for agent_id in list(harness.agent_processes.keys()):
        harness.stop_agent(agent_id)
    for agent_id in ["life-pc-01", "life-pc-02"]:
        if agent_id not in harness.agent_processes:
            harness.start_agent(agent_id)
    wait_until(
        lambda: {"life-pc-01", "life-pc-02"}.issubset(
            {agent["agent_id"] for agent in request_json(base, "/agents")["agents"]}
        ),
        label="lifecycle agents online",
    )
    wait_until(
        lambda: {"life-pc-01", "life-pc-02"}.issubset(
            {
                agent["agent_id"]
                for agent in request_json(base, "/agents/history?limit=50&offset=0")[
                    "agents"
                ]
            }
        ),
        label="lifecycle agents persisted",
    )
    seed_task = request_json(
        base,
        "/agents/life-pc-01/tasks",
        method="POST",
        body={"command": "whoami", "payload": None},
    )
    seed_task_final = wait_task_terminal(base, seed_task["task_id"])
    assert seed_task_final["status"] == "succeeded", seed_task_final

    disabled = request_json(base, "/agents/life-pc-01/disable", method="POST")
    assert disabled["success"] is True, disabled
    wait_until(
        lambda: (
            "life-pc-01"
            not in {
                agent["agent_id"] for agent in request_json(base, "/agents")["agents"]
            }
        ),
        label="life-pc-01 removed from online list after disable",
    )
    disabled_record = wait_until(
        lambda: next(
            (
                item
                for item in request_json(base, "/agents/history?limit=20&offset=0")[
                    "agents"
                ]
                if item["agent_id"] == "life-pc-01"
                and item["is_disabled"] is True
                and item["is_online"] is False
            ),
            None,
        ),
        label="life-pc-01 disabled in persisted history",
    )
    blocked_dispatch_status, blocked_dispatch = request_json_with_status(
        base,
        "/agents/life-pc-01/tasks",
        method="POST",
        body={"command": "whoami", "payload": None},
    )
    assert blocked_dispatch_status == 409, {
        "status": blocked_dispatch_status,
        "body": blocked_dispatch,
    }
    assert "disabled" in blocked_dispatch["detail"], blocked_dispatch

    harness.start_agent("life-pc-01")
    wait_until(
        lambda: (
            "life-pc-01"
            not in {
                agent["agent_id"] for agent in request_json(base, "/agents")["agents"]
            }
        ),
        label="disabled agent cannot re-register while blocked",
    )

    enabled = request_json(base, "/agents/life-pc-01/enable", method="POST")
    assert enabled["success"] is True, enabled
    enabled_record = wait_until(
        lambda: next(
            (
                item
                for item in request_json(base, "/agents/history?limit=20&offset=0")[
                    "agents"
                ]
                if item["agent_id"] == "life-pc-01" and item["is_disabled"] is False
            ),
            None,
        ),
        label="life-pc-01 enabled in persisted history",
    )

    # The blocked agent (started at line 99) is still retrying registration.
    # Now that the agent is enabled, its next retry should succeed.
    # Just wait for it — no need to kill and restart.
    wait_until(
        lambda: {"life-pc-01", "life-pc-02"}.issubset(
            {agent["agent_id"] for agent in request_json(base, "/agents")["agents"]}
        ),
        label="life-pc-01 reconnected after enable",
        timeout=60.0,
    )

    status, body = request_json_with_status(base, "/agents/life-pc-01", method="DELETE")
    assert status == 409, {"status": status, "body": body}
    assert "disconnect it first" in body["detail"], body

    disconnect = request_json(base, "/agents/life-pc-01/disconnect", method="POST")
    assert disconnect["success"] is True, disconnect
    wait_until(
        lambda: (
            "life-pc-01"
            not in {
                agent["agent_id"] for agent in request_json(base, "/agents")["agents"]
            }
        ),
        label="life-pc-01 disconnected from online list",
    )
    history = wait_until(
        lambda: (
            request_json(base, "/agents/history?limit=20&offset=0")
            if next(
                (
                    item
                    for item in request_json(base, "/agents/history?limit=20&offset=0")[
                        "agents"
                    ]
                    if item["agent_id"] == "life-pc-01" and item["is_online"] is False
                ),
                None,
            )
            else None
        ),
        label="life-pc-01 persisted offline",
    )
    history_map = {item["agent_id"]: item for item in history["agents"]}
    assert history_map["life-pc-01"]["is_online"] is False, history_map["life-pc-01"]

    deleted = request_json(base, "/agents/life-pc-01", method="DELETE")
    assert deleted["success"] is True, deleted
    history = wait_until(
        lambda: (
            request_json(base, "/agents/history?limit=20&offset=0")
            if "life-pc-01"
            not in {
                item["agent_id"]
                for item in request_json(base, "/agents/history?limit=20&offset=0")[
                    "agents"
                ]
            }
            else None
        ),
        label="life-pc-01 removed from persisted history",
    )
    assert "life-pc-01" not in {item["agent_id"] for item in history["agents"]}, history

    retained_task = request_json(base, f"/tasks/{seed_task['task_id']}")
    assert retained_task["target_agent_id"] == "life-pc-01", retained_task
    assert retained_task["status"] == "succeeded", retained_task

    delete_audit = wait_until(
        lambda: next(
            (
                item
                for item in request_json(base, "/audits?limit=50&offset=0")["audits"]
                if item["action"] == "delete_agent"
                and item["target_id"] == "life-pc-01"
            ),
            None,
        ),
        label="delete audit record persisted",
    )
    assert "task/audit history retained" in (delete_audit["detail"] or ""), delete_audit

    harness.start_agent("life-pc-01")
    wait_until(
        lambda: {"life-pc-01", "life-pc-02"}.issubset(
            {agent["agent_id"] for agent in request_json(base, "/agents")["agents"]}
        ),
        label="life-pc-01 re-registered after deletion",
    )
    recreated = wait_until(
        lambda: next(
            (
                item
                for item in request_json(base, "/agents/history?limit=20&offset=0")[
                    "agents"
                ]
                if item["agent_id"] == "life-pc-01"
            ),
            None,
        ),
        label="life-pc-01 recreated in history",
    )
    assert recreated["is_online"] is True, recreated
    assert recreated["is_disabled"] is False, recreated

    audits = wait_until(
        lambda: request_json(base, "/audits?limit=20&offset=0"),
        label="lifecycle audit records",
    )
    actions = [item["action"] for item in audits["audits"]]
    assert "disconnect_agent" in actions, actions
    assert "delete_agent" in actions, actions
    assert "disable_agent" in actions, actions
    assert "enable_agent" in actions, actions

    return {
        "disabled_flag": disabled_record["is_disabled"],
        "enabled_flag": enabled_record["is_disabled"],
        "disabled_dispatch_conflict": blocked_dispatch_status,
        "delete_online_conflict": status,
        "retained_task_status": retained_task["status"],
        "disconnect_action": disconnect["detail"],
        "delete_action": deleted["detail"],
        "recreated_online": recreated["is_online"],
        "audit_actions": sorted(set(actions)),
    }


def main() -> int:
    ensure_binaries()
    harness = Harness("hermes-lifecycle-e2e-")
    try:
        harness.start_server()
        print(
            json.dumps(
                {"lifecycle_suite": run(harness), "base_url": harness.base},
                ensure_ascii=False,
                indent=2,
            )
        )
        return 0
    finally:
        harness.close()


if __name__ == "__main__":
    raise SystemExit(main())
