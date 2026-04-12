#!/usr/bin/env python3
import json
import sys
from pathlib import Path

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import (
        Harness,
        agent_history_map,
        ensure_binaries,
        online_agent_ids,
        request_json,
        request_json_with_status,
        wait_agent_offline,
        wait_task_terminal,
        wait_until,
    )
else:
    from .common import (
        Harness,
        agent_history_map,
        ensure_binaries,
        online_agent_ids,
        request_json,
        request_json_with_status,
        wait_agent_offline,
        wait_task_terminal,
        wait_until,
    )


def run(harness):
    base = harness.base
    for agent_id in ["office-pc-01", "office-pc-02", "office-pc-03"]:
        if agent_id not in harness.agent_processes:
            harness.start_agent(agent_id)
    wait_until(
        lambda: len(request_json(base, "/agents")["agents"]) == 3,
        label="three online agents for edge suite",
    )

    disconnect_response = request_json(
        base, "/agents/office-pc-03/disconnect", method="POST"
    )
    assert disconnect_response["success"] is True, disconnect_response
    wait_agent_offline(base, "office-pc-03")
    harness.stop_agent("office-pc-03")

    history_map = agent_history_map(base, limit=10)
    assert history_map["office-pc-03"]["is_online"] is False, history_map[
        "office-pc-03"
    ]

    harness.start_agent("office-pc-03")
    wait_until(
        lambda: len(request_json(base, "/agents")["agents"]) == 3,
        label="agent reconnect",
    )

    status, bad_status = request_json_with_status(base, "/tasks?status=not-real")
    assert status == 400, {"status": status, "body": bad_status}

    harness.start_agent("offline-agent")
    wait_until(
        lambda: (
            "offline-agent"
            in {agent["agent_id"] for agent in request_json(base, "/agents")["agents"]}
        ),
        label="offline-agent initially online",
    )
    harness.stop_agent("offline-agent")
    wait_agent_offline(base, "offline-agent")
    offline_dispatch = request_json(
        base,
        "/agents/offline-agent/tasks",
        method="POST",
        body={"command": "whoami", "payload": None},
    )
    offline_final = wait_task_terminal(base, offline_dispatch["task_id"])
    assert offline_final["status"] == "failed", offline_final
    assert "not connected" in (offline_final["output"] or ""), offline_final

    prior_agents = request_json(base, "/agents")["agents"]
    old_session = next(
        agent for agent in prior_agents if agent["agent_id"] == "office-pc-02"
    )["session_id"]
    harness.start_agent("office-pc-02")
    wait_until(
        lambda: next(
            (
                agent
                for agent in request_json(base, "/agents")["agents"]
                if agent["agent_id"] == "office-pc-02"
                and agent["session_id"] != old_session
            ),
            None,
        ),
        label="duplicate agent id takeover",
    )
    current_agents = request_json(base, "/agents")["agents"]
    new_session = next(
        agent for agent in current_agents if agent["agent_id"] == "office-pc-02"
    )["session_id"]
    assert new_session != old_session, {
        "old_session": old_session,
        "new_session": new_session,
    }

    disconnect_long = request_json(
        base,
        "/agents/office-pc-01/tasks",
        method="POST",
        body={"command": "exec", "payload": "sleep 120"},
    )
    wait_until(
        lambda: (
            request_json(base, f"/tasks/{disconnect_long['task_id']}")["status"]
            in {"pending", "dispatched", "running"}
        ),
        label="disconnect regression task accepted",
    )
    request_json(base, "/agents/office-pc-01/disconnect", method="POST")
    failed = wait_task_terminal(base, disconnect_long["task_id"])
    assert failed["status"] == "failed", failed
    assert "disconnected" in (failed["output"] or ""), failed

    return {
        "bad_status_query": status,
        "offline_dispatch": offline_final["status"],
        "duplicate_agent_takeover": {
            "old_session": old_session,
            "new_session": new_session,
        },
        "disconnect_failure": failed["status"],
        "offline_agents_in_history": {
            agent_id: item["is_online"]
            for agent_id, item in agent_history_map(base, limit=20).items()
            if agent_id in {"office-pc-03", "offline-agent"}
        },
        "currently_online_agents": sorted(online_agent_ids(base)),
    }


def main() -> int:
    ensure_binaries()
    harness = Harness("hermes-edge-e2e-")
    try:
        harness.start_server()
        print(
            json.dumps(
                {"edge_suite": run(harness), "base_url": harness.base},
                ensure_ascii=False,
                indent=2,
            )
        )
        return 0
    finally:
        harness.close()


if __name__ == "__main__":
    raise SystemExit(main())
