#!/usr/bin/env python3
import json
import sqlite3
import shutil
import sys
import tempfile
from pathlib import Path

if __package__ in (None, ''):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import Harness, ensure_binaries, request_json, wait_until
else:
    from .common import Harness, ensure_binaries, request_json, wait_until


def run(_=None):
    root = Path(tempfile.mkdtemp(prefix='hermes-db-e2e-'))
    sqlite_path = root / 'data' / 'server.db'
    try:
        first = Harness('hermes-db-e2e-', temp_root=root)
        try:
            first.start_server()
            base = first.base
            for agent_id in ['db-pc-01', 'db-pc-02']:
                first.start_agent(agent_id)
            wait_until(lambda: len(request_json(base, '/agents')['agents']) == 2, label='db suite agents online')

            running = request_json(base, '/agents/db-pc-01/tasks', method='POST', body={'command': 'exec', 'payload': 'sleep 20'})
            wait_until(lambda: request_json(base, f"/tasks/{running['task_id']}")['status'] in {'pending', 'dispatched', 'running'}, label='db running task')

            broadcast = request_json(base, '/tasks/broadcast', method='POST', body={'command': 'exec', 'payload': 'sleep 20'})
            wait_until(lambda: len(request_json(base, f"/tasks/{broadcast['task_id']}")['children']) == 2, label='db broadcast children created')
            wait_until(
                lambda: all(
                    request_json(base, f"/tasks/{child_id}")['status'] in {'pending', 'dispatched', 'running'}
                    for child_id in request_json(base, f"/tasks/{broadcast['task_id']}")['children']
                ),
                label='db broadcast children active before restart',
            )

            wait_until(
                lambda: persisted_task_ids(sqlite_path) >= {running['task_id'], broadcast['task_id']},
                label='db tasks persisted before restart',
            )
            wait_until(
                lambda: persisted_children_count(sqlite_path, broadcast['task_id']) == 2,
                label='db broadcast parent children persisted',
            )
            wait_until(
                lambda: persisted_online_agents(sqlite_path) == {'db-pc-01', 'db-pc-02'},
                label='db agents persisted before restart',
            )

            before_restart_agents = request_json(base, '/agents/history?limit=20&offset=0')
            before_restart_tasks = request_json(base, '/tasks?limit=20&offset=0')
            first.stop_server()
        finally:
            first.close()

        second = Harness('hermes-db-e2e-', temp_root=root)
        try:
            second.start_server()
            base = second.base

            after_restart_agents = request_json(base, '/agents/history?limit=20&offset=0')
            after_restart_tasks = request_json(base, '/tasks?limit=20&offset=0')

            agents_map = {item['agent_id']: item for item in after_restart_agents['agents']}
            assert agents_map['db-pc-01']['is_online'] is False, agents_map['db-pc-01']
            assert agents_map['db-pc-02']['is_online'] is False, agents_map['db-pc-02']

            tasks_map = {item['task_id']: item for item in after_restart_tasks['tasks']}
            assert tasks_map[running['task_id']]['status'] == 'failed', tasks_map[running['task_id']]
            assert has_restart_recovery_output(tasks_map[running['task_id']]), tasks_map[running['task_id']]

            parent = tasks_map[broadcast['task_id']]
            child_ids = parent['children']
            assert child_ids, parent
            child_statuses = [tasks_map[child_id]['status'] for child_id in child_ids]
            assert all(status == 'failed' for status in child_statuses), {'child_statuses': child_statuses, 'tasks': [tasks_map[c] for c in child_ids]}
            assert parent['status'] == 'failed', parent
            assert all(has_restart_recovery_output(tasks_map[child_id]) for child_id in child_ids), [tasks_map[child_id] for child_id in child_ids]

            return {
                'before_restart_online_flags': {item['agent_id']: item['is_online'] for item in before_restart_agents['agents']},
                'after_restart_online_flags': {item['agent_id']: item['is_online'] for item in after_restart_agents['agents']},
                'recovered_running_task': tasks_map[running['task_id']]['status'],
                'recovered_broadcast_parent': parent['status'],
                'recovered_broadcast_children': child_statuses,
                'tasks_before_restart': before_restart_tasks['total'],
                'tasks_after_restart': after_restart_tasks['total'],
            }
        finally:
            second.close()
    finally:
        shutil.rmtree(root, ignore_errors=True)


def persisted_task_ids(sqlite_path: Path) -> set[str]:
    connection = sqlite3.connect(sqlite_path)
    try:
        rows = connection.execute('select task_id from tasks').fetchall()
        return {row[0] for row in rows}
    finally:
        connection.close()


def persisted_online_agents(sqlite_path: Path) -> set[str]:
    connection = sqlite3.connect(sqlite_path)
    try:
        rows = connection.execute('select agent_id from agents where is_online = 1').fetchall()
        return {row[0] for row in rows}
    finally:
        connection.close()


def persisted_children_count(sqlite_path: Path, task_id: str) -> int:
    connection = sqlite3.connect(sqlite_path)
    try:
        row = connection.execute(
            'select children_json from tasks where task_id = ?',
            (task_id,),
        ).fetchone()
        if not row:
            return 0
        return len(json.loads(row[0]))
    finally:
        connection.close()


def has_restart_recovery_output(task: dict) -> bool:
    output = task.get('output') or ''
    return 'server restarted' in output or 'disconnected before reporting result' in output


def main() -> int:
    ensure_binaries()
    print(json.dumps({'database_suite': run()}, ensure_ascii=False, indent=2))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
