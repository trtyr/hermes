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
        wait_until,
        wait_task_terminal,
        wait_broadcast_settled,
    )
else:
    from .common import (
        Harness,
        ensure_binaries,
        request_json,
        wait_until,
        wait_task_terminal,
        wait_broadcast_settled,
    )


def run(harness):
    base = harness.base
    for agent_id in ["office-pc-01", "office-pc-02", "office-pc-03"]:
        if agent_id not in harness.agent_processes:
            harness.start_agent(agent_id)

    agents_online = wait_until(
        lambda: request_json(base, "/agents")["agents"],
        label="agents endpoint non-empty",
    )
    wait_until(
        lambda: len(request_json(base, "/agents")["agents"]) == 3,
        label="three online agents",
    )

    single = request_json(
        base,
        "/agents/office-pc-01/tasks",
        method="POST",
        body={"command": "hostname", "payload": None},
    )
    single_final = wait_task_terminal(base, single["task_id"])
    assert single_final["status"] == "succeeded", single_final

    broadcast = request_json(
        base,
        "/tasks/broadcast",
        method="POST",
        body={"command": "whoami", "payload": None},
    )
    parent = wait_broadcast_settled(base, broadcast["task_id"])
    assert parent["status"] == "succeeded", parent
    for child_id in parent["children"]:
        child = request_json(base, f"/tasks/{child_id}")
        assert child["status"] == "succeeded", child

    long_task = request_json(
        base,
        "/agents/office-pc-02/tasks",
        method="POST",
        body={"command": "exec", "payload": "sleep 20"},
    )
    wait_until(
        lambda: (
            request_json(base, f"/tasks/{long_task['task_id']}")["status"]
            in {"pending", "dispatched", "running"}
        ),
        label="long task accepted",
    )
    cancel_response = request_json(
        base, f"/tasks/{long_task['task_id']}", method="DELETE"
    )
    assert cancel_response["success"] is True, cancel_response
    cancelled = wait_task_terminal(base, long_task["task_id"])
    assert cancelled["status"] == "cancelled", cancelled

    required_actions = {"dispatch_task", "broadcast_task", "cancel_task"}
    audits = wait_until(
        lambda: request_json(base, "/audits?limit=20&offset=0"),
        label="basic audit records",
    )
    audit_actions = {item["action"] for item in audits["audits"]}
    if not required_actions.issubset(audit_actions):
        audits = wait_until(
            lambda: (
                request_json(base, "/audits?limit=20&offset=0")
                if required_actions.issubset(
                    {
                        item["action"]
                        for item in request_json(base, "/audits?limit=20&offset=0")[
                            "audits"
                        ]
                    }
                )
                else None
            ),
            label="basic audit actions persisted",
        )
        audit_actions = {item["action"] for item in audits["audits"]}
    missing = required_actions - audit_actions
    assert not missing, {
        "missing_audits": sorted(missing),
        "audit_actions": sorted(audit_actions),
    }

    return {
        "online_agents_initial": [agent["agent_id"] for agent in agents_online],
        "single_task": single_final["status"],
        "broadcast_parent": parent["status"],
        "cancelled_task": cancelled["status"],
        "audit_actions": sorted(audit_actions),
    }


def main() -> int:
    ensure_binaries()
    harness = Harness("hermes-basic-e2e-")
    try:
        harness.start_server()
        print(
            json.dumps(
                {"basic_suite": run(harness), "base_url": harness.base},
                ensure_ascii=False,
                indent=2,
            )
        )
        return 0
    finally:
        harness.close()


if __name__ == "__main__":
    raise SystemExit(main())
