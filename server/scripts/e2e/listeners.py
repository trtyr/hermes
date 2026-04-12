#!/usr/bin/env python3
import json
import sys
from pathlib import Path

if __package__ in (None, ''):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import Harness, ensure_binaries, request_json, request_json_with_status, wait_until
else:
    from .common import Harness, ensure_binaries, request_json, request_json_with_status, wait_until


def run(harness):
    base = harness.base

    initial = wait_until(
        lambda: request_json(base, '/listeners?limit=20&offset=0')
        if request_json(base, '/listeners?limit=20&offset=0')['listeners']
        else None,
        label='default listener bootstrapped',
    )
    default_listener = initial['listeners'][0]
    assert default_listener['kind'] == 'tcp_json', default_listener
    assert default_listener['enabled'] is True, default_listener

    created = request_json(
        base,
        '/listeners',
        method='POST',
        body={
            'name': 'office-backup',
            'kind': 'tcp_json',
            'bind_host': '127.0.0.1',
            'bind_port': harness.tcp_port + 1000,
            'enabled': False,
            'config': {},
        },
    )['listener']
    listener_id = created['listener_id']
    assert created['enabled'] is False, created
    assert created['runtime_status'] == 'stopped', created

    fetched = request_json(base, f'/listeners/{listener_id}')
    assert fetched['name'] == 'office-backup', fetched

    updated = request_json(
        base,
        f'/listeners/{listener_id}',
        method='PATCH',
        body={'name': 'office-backup-renamed'},
    )['listener']
    assert updated['name'] == 'office-backup-renamed', updated

    enabled = request_json(base, f'/listeners/{listener_id}/enable', method='POST')['listener']
    assert enabled['enabled'] is True, enabled
    running = wait_until(
        lambda: request_json(base, f'/listeners/{listener_id}')
        if request_json(base, f'/listeners/{listener_id}')['runtime_status'] == 'running'
        else None,
        timeout=10.0,
        label='secondary listener running',
    )
    assert running['enabled'] is True, running

    harness.start_agent('listener-pc-01', server_addr=f"127.0.0.1:{created['bind_port']}")
    wait_until(
        lambda: 'listener-pc-01' in {agent['agent_id'] for agent in request_json(base, '/agents')['agents']},
        label='agent connected via managed listener',
    )

    disabled = request_json(base, f'/listeners/{listener_id}/disable', method='POST')['listener']
    assert disabled['enabled'] is False, disabled
    stopped = wait_until(
        lambda: request_json(base, f'/listeners/{listener_id}')
        if request_json(base, f'/listeners/{listener_id}')['runtime_status'] == 'stopped'
        else None,
        timeout=10.0,
        label='secondary listener stopped',
    )
    assert stopped['enabled'] is False, stopped

    status, conflict = request_json_with_status(base, f'/listeners/{default_listener["listener_id"]}', method='DELETE')
    assert status == 409, {'status': status, 'body': conflict}

    deleted = request_json(base, f'/listeners/{listener_id}', method='DELETE')
    assert deleted['success'] is True, deleted
    final = request_json(base, '/listeners?limit=20&offset=0')
    assert all(item['listener_id'] != listener_id for item in final['listeners']), final

    return {
        'default_listener_id': default_listener['listener_id'],
        'managed_listener_id': listener_id,
        'managed_listener_runtime': running['runtime_status'],
        'delete_enabled_conflict': status,
        'final_listener_total': final['total'],
    }


def main() -> int:
    ensure_binaries()
    harness = Harness('hermes-listeners-e2e-')
    try:
        harness.start_server()
        print(json.dumps({'listeners_suite': run(harness), 'base_url': harness.base}, ensure_ascii=False, indent=2))
        return 0
    finally:
        harness.close()


if __name__ == '__main__':
    raise SystemExit(main())
