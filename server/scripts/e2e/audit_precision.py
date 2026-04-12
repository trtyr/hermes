#!/usr/bin/env python3
import json
import shutil
import sys
import tempfile
from pathlib import Path

if __package__ in (None, ''):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import Harness, ensure_binaries, request_json, wait_agent_offline, wait_broadcast_settled, wait_task_terminal, wait_until
    from e2e.db_tools import load_audits, normalize_audits
else:
    from .common import Harness, ensure_binaries, request_json, wait_agent_offline, wait_broadcast_settled, wait_task_terminal, wait_until
    from .db_tools import load_audits, normalize_audits


def run(_=None):
    root = Path(tempfile.mkdtemp(prefix='hermes-audit-precision-e2e-'))
    sqlite_path = root / 'data' / 'server.db'
    operator = 'e2e-regression'
    try:
        harness = Harness('hermes-audit-precision-e2e-', temp_root=root)
        try:
            harness.start_server()
            base = harness.base

            for agent_id in ['audit-pc-01', 'audit-pc-02']:
                harness.start_agent(agent_id)
            wait_until(lambda: len(request_json(base, '/agents')['agents']) == 2, label='audit precision agents online')

            dispatch = request_json(base, '/agents/audit-pc-01/tasks', method='POST', body={'command': 'whoami', 'payload': None})
            dispatch_task = wait_task_terminal(base, dispatch['task_id'])
            assert dispatch_task['status'] == 'succeeded', dispatch_task

            broadcast = request_json(base, '/tasks/broadcast', method='POST', body={'command': 'whoami', 'payload': None})
            broadcast_task = wait_broadcast_settled(base, broadcast['task_id'])
            assert broadcast_task['status'] == 'succeeded', broadcast_task

            cancellable = request_json(base, '/agents/audit-pc-02/tasks', method='POST', body={'command': 'exec', 'payload': 'sleep 20'})
            wait_until(
                lambda: request_json(base, f"/tasks/{cancellable['task_id']}")['status'] in {'pending', 'dispatched', 'running'},
                label='audit cancellable task active',
            )
            cancelled = request_json(base, f"/tasks/{cancellable['task_id']}", method='DELETE')
            assert cancelled['success'] is True, cancelled
            cancelled_task = wait_task_terminal(base, cancellable['task_id'])
            assert cancelled_task['status'] == 'cancelled', cancelled_task

            created = request_json(base, '/agents/audit-pc-01/command-sessions', method='POST')
            session = created['session']
            session_id = session['command_session_id']
            executed = request_json(
                base,
                f'/command-sessions/{session_id}/execute',
                method='POST',
                body={'line': 'pwd'},
            )['result']
            closed = request_json(base, f'/command-sessions/{session_id}/close', method='POST')
            assert closed['success'] is True, closed

            disconnected = request_json(base, '/agents/audit-pc-01/disconnect', method='POST')
            assert disconnected['success'] is True, disconnected
            wait_agent_offline(base, 'audit-pc-01')

            disabled = request_json(base, '/agents/audit-pc-02/disable', method='POST')
            assert disabled['success'] is True, disabled
            wait_agent_offline(base, 'audit-pc-02')

            enabled = request_json(base, '/agents/audit-pc-02/enable', method='POST')
            assert enabled['success'] is True, enabled
            harness.start_agent('audit-pc-02')
            wait_until(
                lambda: 'audit-pc-02' in {agent['agent_id'] for agent in request_json(base, '/agents')['agents']},
                label='audit-pc-02 reconnected after enable',
            )
            disconnect_02 = request_json(base, '/agents/audit-pc-02/disconnect', method='POST')
            assert disconnect_02['success'] is True, disconnect_02
            wait_agent_offline(base, 'audit-pc-02')

            deleted = request_json(base, '/agents/audit-pc-02', method='DELETE')
            assert deleted['success'] is True, deleted

            api_audits = wait_until(
                lambda: request_json(base, '/audits?limit=100&offset=0')
                if contains_expected_actions(request_json(base, '/audits?limit=100&offset=0')['audits'])
                else None,
                label='expected audit records persisted',
            )
            db_audits = load_audits(sqlite_path)
            assert normalize_audits(api_audits['audits']) == normalize_audits(db_audits), {
                'api_audits': api_audits['audits'],
                'db_audits': db_audits,
            }

            assert_audit(api_audits['audits'], operator, 'dispatch_task', 'agent', 'audit-pc-01', 'command=whoami payload=')
            assert_audit(api_audits['audits'], operator, 'broadcast_task', 'task', broadcast['task_id'], 'command=whoami payload=')
            assert_audit(api_audits['audits'], operator, 'cancel_task', 'task', cancellable['task_id'], 'cancel requested')
            assert_audit(api_audits['audits'], operator, 'open_command_session', 'agent', 'audit-pc-01', f'command_session_id={session_id}')
            assert_audit(api_audits['audits'], operator, 'execute_command_session', 'command_session', session_id, 'line=pwd')
            assert_audit(api_audits['audits'], operator, 'execute_command_session', 'command_session', session_id, f'cwd_before={executed["cwd_before"]}')
            assert_audit(api_audits['audits'], operator, 'execute_command_session', 'command_session', session_id, f'cwd_after={executed["cwd_after"]}')
            assert_audit(api_audits['audits'], operator, 'execute_command_session', 'command_session', session_id, f'exit_code={executed["exit_code"]}')
            assert_audit(api_audits['audits'], operator, 'close_command_session', 'command_session', session_id, None)
            assert_audit(api_audits['audits'], operator, 'disconnect_agent', 'agent', 'audit-pc-01', None)
            assert_audit(
                api_audits['audits'],
                operator,
                'disable_agent',
                'agent',
                'audit-pc-02',
                'agent disabled; new registration and task dispatch blocked',
            )
            assert_audit(
                api_audits['audits'],
                operator,
                'enable_agent',
                'agent',
                'audit-pc-02',
                'agent enabled; registration and task dispatch allowed',
            )
            assert_audit(api_audits['audits'], operator, 'disconnect_agent', 'agent', 'audit-pc-02', None)
            assert_audit(
                api_audits['audits'],
                operator,
                'delete_agent',
                'agent',
                'audit-pc-02',
                'removed persisted agent record; task/audit history retained',
            )

            return {
                'checked_actions': [
                    'dispatch_task',
                    'broadcast_task',
                    'cancel_task',
                    'open_command_session',
                    'execute_command_session',
                    'close_command_session',
                    'disconnect_agent',
                    'disable_agent',
                    'enable_agent',
                    'delete_agent',
                ],
                'command_session_id': session_id,
                'broadcast_task_id': broadcast['task_id'],
                'cancelled_task_id': cancellable['task_id'],
            }
        finally:
            harness.close()
    finally:
        shutil.rmtree(root, ignore_errors=True)


def contains_expected_actions(audits: list[dict]) -> bool:
    actions = {item['action'] for item in audits}
    return {
        'dispatch_task',
        'broadcast_task',
        'cancel_task',
        'open_command_session',
        'execute_command_session',
        'close_command_session',
        'disconnect_agent',
        'disable_agent',
        'enable_agent',
        'delete_agent',
    }.issubset(actions)


def assert_audit(audits: list[dict], operator: str, action: str, target_kind: str, target_id: str, detail_contains: str | None):
    for item in audits:
        if (
            item['operator'] == operator
            and item['action'] == action
            and item['target_kind'] == target_kind
            and item['target_id'] == target_id
        ):
            detail = item['detail']
            if detail_contains is None:
                assert detail is None, item
            else:
                assert detail is not None and detail_contains in detail, item
            return
    raise AssertionError(
        {
            'missing_action': action,
            'target_kind': target_kind,
            'target_id': target_id,
            'detail_contains': detail_contains,
            'audits': audits,
        }
    )


def main() -> int:
    ensure_binaries()
    print(json.dumps({'audit_precision_suite': run()}, ensure_ascii=False, indent=2))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
