#!/usr/bin/env python3
import json
import os
import sys
from pathlib import Path

if __package__ in (None, ''):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import Harness, ensure_binaries, request_json, wait_until
else:
    from .common import Harness, ensure_binaries, request_json, wait_until


def run(harness):
    base = harness.base
    agent_id = 'cmd-pc-01'
    harness.start_agent(agent_id)
    wait_until(
        lambda: agent_id in {agent['agent_id'] for agent in request_json(base, '/agents')['agents']},
        label='command session agent online',
    )

    created = request_json(base, f'/agents/{agent_id}/command-sessions', method='POST')
    session = created['session']
    session_id = session['command_session_id']
    assert session['status'] == 'open', session
    assert session['cwd'], session

    listed = request_json(base, '/command-sessions?limit=20&offset=0')
    assert any(item['command_session_id'] == session_id for item in listed['sessions']), listed

    pwd_before = request_json(
        base,
        f'/command-sessions/{session_id}/execute',
        method='POST',
        body={'line': 'pwd'},
    )['result']
    assert pwd_before['success'] is True, pwd_before
    assert pwd_before['cwd_before'] == pwd_before['cwd_after'], pwd_before

    if os.name == 'nt':
        target = 'cd ..'
    else:
        target = 'cd /tmp'

    changed = request_json(
        base,
        f'/command-sessions/{session_id}/execute',
        method='POST',
        body={'line': target},
    )['result']
    assert changed['success'] is True, changed
    assert changed['cwd_after'], changed
    if os.name != 'nt':
        assert changed['cwd_after'] == '/tmp', changed

    pwd_after = request_json(
        base,
        f'/command-sessions/{session_id}/execute',
        method='POST',
        body={'line': 'pwd'},
    )['result']
    assert pwd_after['success'] is True, pwd_after
    assert pwd_after['stdout'].strip() == changed['cwd_after'], pwd_after

    listed_open = request_json(base, '/command-sessions?status=open&limit=20&offset=0')
    assert any(item['command_session_id'] == session_id for item in listed_open['sessions']), listed_open

    closed = request_json(base, f'/command-sessions/{session_id}/close', method='POST')['session']
    assert closed['status'] == 'closed', closed

    final = request_json(base, f'/command-sessions/{session_id}')
    assert final['status'] == 'closed', final

    return {
        'command_session_id': session_id,
        'initial_cwd': session['cwd'],
        'cwd_after_cd': changed['cwd_after'],
        'closed_status': final['status'],
    }


def main() -> int:
    ensure_binaries()
    harness = Harness('hermes-command-session-e2e-')
    try:
        harness.start_server()
        print(json.dumps({'command_session_suite': run(harness), 'base_url': harness.base}, ensure_ascii=False, indent=2))
        return 0
    finally:
        harness.close()


if __name__ == '__main__':
    raise SystemExit(main())
