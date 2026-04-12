#!/usr/bin/env python3
import json
import sys
import time
from pathlib import Path

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import (
        Harness,
        ensure_binaries,
        request_json,
        request_json_with_status,
        wait_until,
        terminate_process,
    )
else:
    from .common import (
        Harness,
        ensure_binaries,
        request_json,
        request_json_with_status,
        wait_until,
        terminate_process,
    )


def run(_=None):
    harness = Harness(
        "hermes-auth-e2e-", api_token="api-secret", agent_token="agent-secret"
    )
    try:
        harness.start_server()
        base = harness.base

        status, body = request_json_with_status(base, "/tasks")
        assert status == 401, {"status": status, "body": body}

        status, body = request_json_with_status(
            base, "/tasks", headers={"Authorization": "Bearer wrong-token"}
        )
        assert status == 401, {"status": status, "body": body}

        status, body = request_json_with_status(
            base, "/tasks", headers={"Authorization": "Bearer api-secret"}
        )
        assert status == 200, {"status": status, "body": body}

        invalid_proc = harness.start_agent("bad-agent")
        time.sleep(1.5)
        agents = request_json(base, "/agents", headers={"x-api-token": "api-secret"})
        assert len(agents["agents"]) == 0, agents
        terminate_process(invalid_proc)

        harness.start_agent("secure-agent", token="agent-secret")
        wait_until(
            lambda: (
                len(
                    request_json(
                        base, "/agents", headers={"x-api-token": "api-secret"}
                    )["agents"]
                )
                == 1
            ),
            label="authorized agent online",
        )

        status, body = request_json_with_status(
            base, "/tasks", headers={"x-api-token": "api-secret"}
        )
        assert status == 200, {"status": status, "body": body}

        return {
            "api_unauthorized": 401,
            "api_wrong_token": 401,
            "api_authorized": 200,
            "agent_rejected_without_token": True,
            "agent_accepted_with_token": True,
        }
    finally:
        harness.close()


def main() -> int:
    ensure_binaries()
    print(json.dumps({"auth_suite": run()}, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
