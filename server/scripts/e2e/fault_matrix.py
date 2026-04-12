#!/usr/bin/env python3
import json
import socket
import sys
from pathlib import Path

if __package__ in (None, ''):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import (
        Harness,
        ensure_binaries,
        request_json,
        wait_agent_offline,
        wait_agent_online,
        wait_task_terminal,
        wait_until,
    )
    from e2e.edge import run as run_edge
    from e2e.lifecycle import run as run_lifecycle
    from e2e.concurrent_stress import run as run_concurrent_stress
    from e2e.database_interruptions import run as run_database_interruptions
else:
    from .common import (
        Harness,
        ensure_binaries,
        request_json,
        wait_agent_offline,
        wait_agent_online,
        wait_task_terminal,
        wait_until,
    )
    from .edge import run as run_edge
    from .lifecycle import run as run_lifecycle
    from .concurrent_stress import run as run_concurrent_stress
    from .database_interruptions import run as run_database_interruptions


def run_suite(name: str, runner):
    harness = Harness(f'hermes-{name}-e2e-')
    try:
        harness.start_server()
        return runner(harness)
    finally:
        harness.close()


def run_heartbeat_timeout():
    harness = Harness('hermes-heartbeat-timeout-e2e-')
    sock = None
    try:
        harness.start_server()
        base = harness.base
        agent_id = 'timeout-pc-01'

        sock = socket.create_connection(('127.0.0.1', harness.tcp_port), timeout=5)
        register = {
            'type': 'register',
            'agent_id': agent_id,
            'hostname': 'timeout-host',
            'username': 'tester',
            'protocol_version': 2,
            'os': 'macos',
            'arch': 'arm64',
            'pid': 4242,
            'internal_ip': '127.0.0.1',
            'tags': ['fault-matrix', 'heartbeat-timeout'],
            'sleep_interval': 1,
            'jitter': 0,
            'token': None,
            'session_nonce': None,
            'auth_response': None,
        }
        sock.sendall((json.dumps(register) + '\n').encode())

        online = wait_agent_online(base, agent_id, timeout=10.0)
        assert online['agent_id'] == agent_id, online

        task = request_json(
            base,
            f'/agents/{agent_id}/tasks',
            method='POST',
            body={'command': 'whoami', 'payload': None},
        )
        wait_until(
            lambda: request_json(base, f"/tasks/{task['task_id']}")['status'] in {'pending', 'dispatched', 'running'},
            timeout=10.0,
            label='heartbeat-timeout task active',
        )

        offline = wait_agent_offline(base, agent_id, timeout=15.0)
        failed = wait_task_terminal(base, task['task_id'], timeout=15.0)

        assert offline['is_online'] is False, offline
        assert failed['status'] == 'failed', failed
        assert 'heartbeat timed out' in (failed['output'] or ''), failed

        return {
            'agent_id': agent_id,
            'offline_confirmed': offline['is_online'] is False,
            'timed_out_task': failed['status'],
            'timed_out_output': failed['output'],
        }
    finally:
        if sock is not None:
            sock.close()
        harness.close()


def run():
    return {
        'matrix': [
            {
                'case': 'duplicate_agent_takeover',
                'source': 'edge',
                'result': run_suite('fault-edge', run_edge)['duplicate_agent_takeover'],
            },
            {
                'case': 'active_task_fails_on_operator_disconnect',
                'source': 'edge',
                'result': run_suite('fault-edge-disconnect', run_edge)['disconnect_failure'],
            },
            {
                'case': 'disabled_agent_cannot_reregister',
                'source': 'lifecycle',
                'result': run_suite('fault-lifecycle', run_lifecycle)['disabled_dispatch_conflict'],
            },
            {
                'case': 'disconnect_reconnect_under_load',
                'source': 'concurrent_stress',
                'result': run_suite('fault-concurrent', run_concurrent_stress),
            },
            {
                'case': 'server_restart_recovers_interrupted_tasks',
                'source': 'database_interruptions',
                'result': run_database_interruptions(),
            },
            {
                'case': 'heartbeat_timeout_marks_agent_offline_and_fails_task',
                'source': 'fault_matrix',
                'result': run_heartbeat_timeout(),
            },
        ]
    }


def main() -> int:
    ensure_binaries()
    print(json.dumps({'fault_matrix_suite': run()}, ensure_ascii=False, indent=2))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
