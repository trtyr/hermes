#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

if __package__ in (None, ''):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import Harness, ensure_binaries, request_json, spawn, terminate_process, wait_until
else:
    from .common import Harness, ensure_binaries, request_json, spawn, terminate_process, wait_until


def run(harness):
    base = harness.base
    listeners = wait_until(
        lambda: request_json(base, '/listeners?limit=20&offset=0')
        if request_json(base, '/listeners?limit=20&offset=0')['listeners']
        else None,
        label='listeners available for agent build',
    )
    default_listener = listeners['listeners'][0]

    created = request_json(
        base,
        f"/listeners/{default_listener['listener_id']}/agent-builds",
        method='POST',
        body={
            'profile': 'release',
        },
        timeout=120,
    )['build']
    assert created['status'] == 'succeeded', created
    assert created['listener_id'] == default_listener['listener_id'], created
    assert created['artifact_path'], created
    assert created['artifact_name'], created
    assert Path(created['artifact_path']).exists(), created

    created_via_legacy = request_json(
        base,
        '/agent-builds',
        method='POST',
        body={
            'listener_id': default_listener['listener_id'],
            'profile': 'release',
        },
        timeout=120,
    )['build']
    assert created_via_legacy['status'] == 'succeeded', created_via_legacy

    fetched = request_json(base, f"/agent-builds/{created['build_id']}")
    assert fetched['build_id'] == created['build_id'], fetched
    assert fetched['status'] == 'succeeded', fetched
    assert fetched['server_addr'] == f"{default_listener['bind_host']}:{default_listener['bind_port']}", fetched
    assert 'manifest=' in (fetched.get('detail') or ''), fetched
    manifest_path = Path(fetched['detail'].split('manifest=', 1)[1])
    assert manifest_path.exists(), manifest_path
    manifest = json.loads(manifest_path.read_text())
    assert manifest['listener_id'] == default_listener['listener_id'], manifest
    assert manifest['embedded_server_addr'] == fetched['server_addr'], manifest
    assert manifest['server_addr_binding'] == 'compile_time_only', manifest
    assert 'HERMES_SERVER_ADDR' in manifest['ignored_runtime_overrides'], manifest
    assert manifest['runtime_overrides'] == [], manifest
    assert manifest['artifact_path'] == created['artifact_path'], manifest

    listed = request_json(base, '/agent-builds?limit=20&offset=0')
    assert listed['total'] >= 1, listed
    assert any(item['build_id'] == created['build_id'] for item in listed['builds']), listed

    artifact_size = os.path.getsize(created['artifact_path'])
    assert artifact_size > 0, artifact_size

    wrong_server_addr = '127.0.0.1:9'
    env = os.environ.copy()
    env['HERMES_SERVER_ADDR'] = wrong_server_addr
    launch_name = f"built-agent-{created['build_id']}"
    launch_path = Path(created['artifact_path']).parent / launch_name
    launch_path.write_bytes(Path(created['artifact_path']).read_bytes())
    launch_path.chmod(0o755)
    built_proc = spawn([str(launch_path)], Path(created['artifact_path']).parent, env=env)
    try:
        online = wait_until(
            lambda: next(
                (agent for agent in request_json(base, '/agents')['agents'] if agent['agent_id'] == launch_name),
                None,
            ),
            label='compiled agent online via embedded server_addr',
        )
        assert online['agent_id'] == launch_name, online
    finally:
        terminate_process(built_proc)

    return {
        'build_id': created['build_id'],
        'target_triple': created['target_triple'],
        'artifact_name': created['artifact_name'],
        'artifact_size': artifact_size,
        'manifest_path': str(manifest_path),
    }


def main() -> int:
    ensure_binaries()
    harness = Harness('hermes-agent-builds-e2e-')
    try:
        harness.start_server()
        print(json.dumps({'agent_builds_suite': run(harness), 'base_url': harness.base}, ensure_ascii=False, indent=2))
        return 0
    finally:
        harness.close()


if __name__ == '__main__':
    raise SystemExit(main())
