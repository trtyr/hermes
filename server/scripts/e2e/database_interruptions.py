#!/usr/bin/env python3
import json
import shutil
import sys
import tempfile
from pathlib import Path

if __package__ in (None, ''):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import Harness, ensure_binaries, request_json, wait_agent_offline, wait_until
    from e2e.db_tools import load_agent, load_task
else:
    from .common import Harness, ensure_binaries, request_json, wait_agent_offline, wait_until
    from .db_tools import load_agent, load_task


def run(_=None):
    root = Path(tempfile.mkdtemp(prefix='hermes-db-interrupt-e2e-'))
    sqlite_path = root / 'data' / 'server.db'
    try:
        harness = Harness('hermes-db-interrupt-e2e-', temp_root=root)
        try:
            harness.start_server()
            base = harness.base

            for agent_id in ['interrupt-pc-01', 'interrupt-pc-02', 'interrupt-pc-03']:
                harness.start_agent(agent_id)
            wait_until(lambda: len(request_json(base, '/agents')['agents']) == 3, label='interrupt agents online')

            agent_kill_task = request_json(
                base,
                '/agents/interrupt-pc-01/tasks',
                method='POST',
                body={'command': 'exec', 'payload': 'sleep 20'},
            )
            wait_until(
                lambda: request_json(base, f"/tasks/{agent_kill_task['task_id']}")['status'] in {'pending', 'dispatched', 'running'},
                label='agent kill task active',
            )

            harness.stop_agent('interrupt-pc-01')
            api_offline = wait_agent_offline(base, 'interrupt-pc-01')
            api_failed = wait_until(
                lambda: request_json(base, f"/tasks/{agent_kill_task['task_id']}")
                if request_json(base, f"/tasks/{agent_kill_task['task_id']}")['status'] == 'failed'
                else None,
                label='agent kill task failed in api',
            )

            db_offline = wait_until(
                lambda: (
                    row
                    if (row := load_agent(sqlite_path, 'interrupt-pc-01')) is not None and row['is_online'] == 0
                    else None
                ),
                label='interrupt-pc-01 offline in db',
            )
            db_failed = wait_until(
                lambda: (
                    row
                    if (row := load_task(sqlite_path, agent_kill_task['task_id'])) is not None and row['status'] == 'failed'
                    else None
                ),
                label='agent kill task failed in db',
            )
            assert 'disconnected before reporting result' in (api_failed['output'] or ''), api_failed
            assert 'disconnected before reporting result' in (db_failed['output'] or ''), dict(db_failed)
            assert api_offline['is_online'] is False, api_offline
            assert db_offline['is_online'] == 0, dict(db_offline)

            restart_single = request_json(
                base,
                '/agents/interrupt-pc-02/tasks',
                method='POST',
                body={'command': 'exec', 'payload': 'sleep 20'},
            )
            restart_broadcast = request_json(
                base,
                '/tasks/broadcast',
                method='POST',
                body={'command': 'exec', 'payload': 'sleep 20'},
            )
            wait_until(
                lambda: request_json(base, f"/tasks/{restart_single['task_id']}")['status'] in {'pending', 'dispatched', 'running'},
                label='restart single task active',
            )
            restart_parent = wait_until(
                lambda: request_json(base, f"/tasks/{restart_broadcast['task_id']}")
                if len(request_json(base, f"/tasks/{restart_broadcast['task_id']}")['children']) == 2
                else None,
                label='restart broadcast children created',
            )
            restart_child_ids = restart_parent['children']
            wait_until(
                lambda: all(
                    request_json(base, f'/tasks/{child_id}')['status'] in {'pending', 'dispatched', 'running'}
                    for child_id in restart_child_ids
                ),
                label='restart broadcast children active',
            )
            wait_until(
                lambda: (
                    row
                    if (row := load_task(sqlite_path, restart_broadcast['task_id'])) is not None and len(json.loads(row['children_json'])) == 2
                    else None
                ),
                label='restart broadcast persisted before server kill',
            )

            harness.stop_server()
            harness.stop_agent('interrupt-pc-02')
            harness.stop_agent('interrupt-pc-03')
            harness.start_server()

            api_history = wait_until(
                lambda: request_json(base, '/agents/history?limit=20&offset=0')
                if all(
                    item['is_online'] is False
                    for item in request_json(base, '/agents/history?limit=20&offset=0')['agents']
                    if item['agent_id'] in {'interrupt-pc-02', 'interrupt-pc-03'}
                )
                else None,
                label='agents offline after server restart',
            )
            api_tasks = request_json(base, '/tasks?limit=20&offset=0')
            api_task_map = {item['task_id']: item for item in api_tasks['tasks']}

            db_restart_single = load_task(sqlite_path, restart_single['task_id'])
            db_restart_parent = load_task(sqlite_path, restart_broadcast['task_id'])
            db_restart_children = [load_task(sqlite_path, child_id) for child_id in restart_child_ids]

            assert api_task_map[restart_single['task_id']]['status'] == 'failed', api_task_map[restart_single['task_id']]
            assert db_restart_single is not None and db_restart_single['status'] == 'failed', dict(db_restart_single) if db_restart_single else None
            assert has_restart_recovery_output(api_task_map[restart_single['task_id']]['output']), api_task_map[restart_single['task_id']]
            assert has_restart_recovery_output(db_restart_single['output']), dict(db_restart_single)

            assert api_task_map[restart_broadcast['task_id']]['status'] == 'failed', api_task_map[restart_broadcast['task_id']]
            assert db_restart_parent is not None and db_restart_parent['status'] == 'failed', dict(db_restart_parent) if db_restart_parent else None
            assert len(json.loads(db_restart_parent['children_json'])) == 2, dict(db_restart_parent)

            for child_id, db_child in zip(restart_child_ids, db_restart_children):
                api_child = api_task_map[child_id]
                assert api_child['status'] == 'failed', api_child
                assert db_child is not None and db_child['status'] == 'failed', dict(db_child) if db_child else None
                assert has_restart_recovery_output(api_child['output']), api_child
                assert has_restart_recovery_output(db_child['output']), dict(db_child)

            history_map = {item['agent_id']: item for item in api_history['agents']}
            db_agent_02 = load_agent(sqlite_path, 'interrupt-pc-02')
            db_agent_03 = load_agent(sqlite_path, 'interrupt-pc-03')
            assert history_map['interrupt-pc-02']['is_online'] is False, history_map['interrupt-pc-02']
            assert history_map['interrupt-pc-03']['is_online'] is False, history_map['interrupt-pc-03']
            assert db_agent_02 is not None and db_agent_02['is_online'] == 0, dict(db_agent_02) if db_agent_02 else None
            assert db_agent_03 is not None and db_agent_03['is_online'] == 0, dict(db_agent_03) if db_agent_03 else None

            return {
                'agent_kill_task': api_failed['status'],
                'agent_kill_output_checked': True,
                'restart_single_status': api_task_map[restart_single['task_id']]['status'],
                'restart_broadcast_parent_status': api_task_map[restart_broadcast['task_id']]['status'],
                'restart_broadcast_child_statuses': [api_task_map[child_id]['status'] for child_id in restart_child_ids],
                'offline_agents_after_restart': {
                    'interrupt-pc-02': bool(history_map['interrupt-pc-02']['is_online']),
                    'interrupt-pc-03': bool(history_map['interrupt-pc-03']['is_online']),
                },
            }
        finally:
            harness.close()
    finally:
        shutil.rmtree(root, ignore_errors=True)


def has_restart_recovery_output(output: str | None) -> bool:
    text = output or ''
    return 'server restarted' in text or 'disconnected before reporting result' in text


def main() -> int:
    ensure_binaries()
    print(json.dumps({'database_interruptions_suite': run()}, ensure_ascii=False, indent=2))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
