#!/usr/bin/env python3
import json
import shutil
import sys
import tempfile
from pathlib import Path

if __package__ in (None, ''):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.db_tools import count_actions, load_agent, load_agents, load_audits, load_task, load_tasks_if_status, normalize_audits
    from e2e.common import (
        Harness,
        ensure_binaries,
        request_json,
        wait_agent_offline,
        wait_broadcast_settled,
        wait_task_terminal,
        wait_until,
    )
else:
    from .db_tools import count_actions, load_agent, load_agents, load_audits, load_task, load_tasks_if_status, normalize_audits
    from .common import (
        Harness,
        ensure_binaries,
        request_json,
        wait_agent_offline,
        wait_broadcast_settled,
        wait_task_terminal,
        wait_until,
    )


def run(_=None):
    root = Path(tempfile.mkdtemp(prefix='hermes-db-consistency-e2e-'))
    sqlite_path = root / 'data' / 'server.db'
    try:
        harness = Harness('hermes-db-consistency-e2e-', temp_root=root)
        try:
            harness.start_server()
            base = harness.base

            for agent_id in ['db-check-01', 'db-check-02']:
                harness.start_agent(agent_id)
            wait_until(lambda: len(request_json(base, '/agents')['agents']) == 2, label='db consistency agents online')

            initial_rows = load_agents(sqlite_path)
            assert {row['agent_id'] for row in initial_rows} == {'db-check-01', 'db-check-02'}, initial_rows
            assert all(row['is_online'] == 1 for row in initial_rows), initial_rows
            assert all(row['is_disabled'] == 0 for row in initial_rows), initial_rows
            assert all(row['session_id'] is not None for row in initial_rows), initial_rows

            single = request_json(base, '/agents/db-check-01/tasks', method='POST', body={'command': 'whoami', 'payload': None})
            single_final = wait_task_terminal(base, single['task_id'])
            assert single_final['status'] == 'succeeded', single_final

            single_row = load_task(sqlite_path, single['task_id'])
            assert single_row is not None, single['task_id']
            assert single_row['target_agent_id'] == 'db-check-01', single_row
            assert single_row['parent_task_id'] is None, single_row
            assert single_row['command'] == 'whoami', single_row
            assert single_row['status'] == 'succeeded', single_row
            assert single_row['success'] == 1, single_row
            assert json.loads(single_row['children_json']) == [], single_row

            broadcast = request_json(base, '/tasks/broadcast', method='POST', body={'command': 'whoami', 'payload': None})
            broadcast_final = wait_broadcast_settled(base, broadcast['task_id'])
            assert broadcast_final['status'] == 'succeeded', broadcast_final

            parent_row = wait_until(
                lambda: (
                    row
                    if (row := load_task(sqlite_path, broadcast['task_id'])) is not None and row['status'] == 'succeeded'
                    else None
                ),
                label='broadcast parent persisted as succeeded',
            )
            assert parent_row is not None, broadcast['task_id']
            child_ids = json.loads(parent_row['children_json'])
            assert len(child_ids) == 2, parent_row
            assert parent_row['target_agent_id'] is None, parent_row
            assert parent_row['parent_task_id'] is None, parent_row
            assert parent_row['status'] == 'succeeded', parent_row

            child_rows = wait_until(
                lambda: load_tasks_if_status(sqlite_path, child_ids, 'succeeded'),
                label='broadcast children persisted as succeeded',
            )
            assert all(row is not None for row in child_rows), child_rows
            assert {row['target_agent_id'] for row in child_rows} == {'db-check-01', 'db-check-02'}, child_rows
            assert all(row['parent_task_id'] == broadcast['task_id'] for row in child_rows), child_rows
            assert all(row['status'] == 'succeeded' for row in child_rows), child_rows
            assert all(row['success'] == 1 for row in child_rows), child_rows

            disconnect = request_json(base, '/agents/db-check-01/disconnect', method='POST')
            assert disconnect['success'] is True, disconnect
            db_check_01_offline = wait_agent_offline(base, 'db-check-01')
            row_after_disconnect = load_agent(sqlite_path, 'db-check-01')
            assert row_after_disconnect is not None, row_after_disconnect
            assert row_after_disconnect['is_online'] == 0, row_after_disconnect
            assert db_check_01_offline['is_online'] is False, db_check_01_offline

            disabled = request_json(base, '/agents/db-check-02/disable', method='POST')
            assert disabled['success'] is True, disabled
            db_check_02_offline = wait_agent_offline(base, 'db-check-02')
            row_after_disable = wait_until(
                lambda: (
                    row
                    if (row := load_agent(sqlite_path, 'db-check-02')) is not None and row['is_disabled'] == 1
                    else None
                ),
                label='db-check-02 disabled in sqlite',
            )
            assert row_after_disable['is_online'] == 0, row_after_disable
            assert row_after_disable['is_disabled'] == 1, row_after_disable
            assert db_check_02_offline['is_online'] is False, db_check_02_offline
            assert db_check_02_offline['is_disabled'] is True, db_check_02_offline

            deleted = request_json(base, '/agents/db-check-01', method='DELETE')
            assert deleted['success'] is True, deleted
            wait_until(lambda: load_agent(sqlite_path, 'db-check-01') is None, label='db-check-01 removed from sqlite')
            assert load_task(sqlite_path, single['task_id']) is not None, single['task_id']

            api_audits = request_json(base, '/audits?limit=50&offset=0')
            db_audits = load_audits(sqlite_path)
            assert api_audits['total'] == len(db_audits), {'api_total': api_audits['total'], 'db_total': len(db_audits)}
            assert normalize_audits(api_audits['audits']) == normalize_audits(db_audits), {
                'api_audits': api_audits['audits'],
                'db_audits': db_audits,
            }

            action_counts = count_actions(db_audits)
            required_actions = {'dispatch_task', 'broadcast_task', 'disconnect_agent', 'disable_agent', 'delete_agent'}
            assert required_actions.issubset(action_counts), action_counts

            remaining_agents = load_agents(sqlite_path)
            assert {row['agent_id'] for row in remaining_agents} == {'db-check-02'}, remaining_agents

            return {
                'remaining_agents': [row['agent_id'] for row in remaining_agents],
                'single_task_status': single_row['status'],
                'broadcast_children': child_ids,
                'disabled_agent_flags': {
                    'db-check-02': {
                        'is_online': bool(row_after_disable['is_online']),
                        'is_disabled': bool(row_after_disable['is_disabled']),
                    }
                },
                'audit_action_counts': action_counts,
            }
        finally:
            harness.close()
    finally:
        shutil.rmtree(root, ignore_errors=True)


def main() -> int:
    ensure_binaries()
    print(json.dumps({'database_consistency_suite': run()}, ensure_ascii=False, indent=2))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
