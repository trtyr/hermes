#!/usr/bin/env python3
import json
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

if __package__ in (None, ''):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from e2e.common import Harness, ensure_binaries, request_json, wait_agent_offline, wait_agent_online, wait_broadcast_settled, wait_task_terminal, wait_until
else:
    from .common import Harness, ensure_binaries, request_json, wait_agent_offline, wait_agent_online, wait_broadcast_settled, wait_task_terminal, wait_until


def run(harness):
    base = harness.base
    agent_ids = [f'stress-pc-0{i}' for i in range(1, 6)]
    for agent_id in agent_ids:
        if agent_id not in harness.agent_processes:
            harness.start_agent(agent_id)
    initial_online = wait_until(
        lambda: request_json(base, '/agents')['agents']
        if set(agent_ids).issubset({agent['agent_id'] for agent in request_json(base, '/agents')['agents']})
        else None,
        label='stress agents online',
    )
    initial_online_ids = {agent['agent_id'] for agent in initial_online}

    with ThreadPoolExecutor(max_workers=8) as pool:
        single_dispatches = list(
            pool.map(
                lambda agent_id: request_json(
                    base,
                    f'/agents/{agent_id}/tasks',
                    method='POST',
                    body={'command': 'whoami', 'payload': None},
                ),
                agent_ids,
            )
        )
    single_results = [wait_task_terminal(base, item['task_id']) for item in single_dispatches]
    assert all(item['status'] == 'succeeded' for item in single_results), single_results

    with ThreadPoolExecutor(max_workers=2) as pool:
        broadcasts = list(
            pool.map(
                lambda _: request_json(base, '/tasks/broadcast', method='POST', body={'command': 'whoami', 'payload': None}),
                range(2),
            )
        )
    broadcast_results = [wait_broadcast_settled(base, item['task_id']) for item in broadcasts]
    assert all(item['status'] == 'succeeded' for item in broadcast_results), broadcast_results
    assert all(len(item['children']) == len(initial_online_ids) for item in broadcast_results), broadcast_results

    cancel_task = request_json(
        base,
        '/agents/stress-pc-01/tasks',
        method='POST',
        body={'command': 'exec', 'payload': 'sleep 20'},
    )
    disconnect_task = request_json(
        base,
        '/agents/stress-pc-02/tasks',
        method='POST',
        body={'command': 'exec', 'payload': 'sleep 20'},
    )
    finish_task = request_json(
        base,
        '/agents/stress-pc-03/tasks',
        method='POST',
        body={'command': 'exec', 'payload': 'sleep 1'},
    )
    for task_id in [cancel_task['task_id'], disconnect_task['task_id'], finish_task['task_id']]:
        wait_until(
            lambda task_id=task_id: request_json(base, f'/tasks/{task_id}')['status'] in {'pending', 'dispatched', 'running'},
            label=f'{task_id} active',
        )

    with ThreadPoolExecutor(max_workers=3) as pool:
        cancel_response_future = pool.submit(request_json, base, f"/tasks/{cancel_task['task_id']}", 'DELETE')
        disconnect_response_future = pool.submit(request_json, base, '/agents/stress-pc-02/disconnect', 'POST')
        command_session_future = pool.submit(request_json, base, '/agents/stress-pc-04/command-sessions', 'POST')
        cancel_response = cancel_response_future.result()
        disconnect_response = disconnect_response_future.result()
        command_session = command_session_future.result()['session']

    assert cancel_response['success'] is True, cancel_response
    assert disconnect_response['success'] is True, disconnect_response

    cancelled_final = wait_task_terminal(base, cancel_task['task_id'])
    disconnected_final = wait_task_terminal(base, disconnect_task['task_id'])
    succeeded_final = wait_task_terminal(base, finish_task['task_id'])
    assert cancelled_final['status'] == 'cancelled', cancelled_final
    assert disconnected_final['status'] == 'failed', disconnected_final
    assert 'disconnected' in (disconnected_final['output'] or ''), disconnected_final
    assert succeeded_final['status'] == 'succeeded', succeeded_final
    wait_agent_offline(base, 'stress-pc-02')

    session_id = command_session['command_session_id']
    with ThreadPoolExecutor(max_workers=2) as pool:
        pwd_result_future = pool.submit(
            request_json,
            base,
            f'/command-sessions/{session_id}/execute',
            'POST',
            {'line': 'pwd'},
        )
        whoami_result_future = pool.submit(
            request_json,
            base,
            f'/agents/stress-pc-05/tasks',
            'POST',
            {'command': 'whoami', 'payload': None},
        )
        pwd_result = pwd_result_future.result()['result']
        trailing_task = whoami_result_future.result()
    assert pwd_result['success'] is True, pwd_result
    trailing_final = wait_task_terminal(base, trailing_task['task_id'])
    assert trailing_final['status'] == 'succeeded', trailing_final

    close_result = request_json(base, f'/command-sessions/{session_id}/close', method='POST')
    assert close_result['success'] is True, close_result

    harness.start_agent('stress-pc-02')
    reconnected = wait_agent_online(base, 'stress-pc-02')

    final_agents = request_json(base, '/agents')['agents']
    final_agent_ids = {item['agent_id'] for item in final_agents}
    assert set(agent_ids).issubset(final_agent_ids), final_agents

    final_tasks = request_json(base, '/tasks?limit=200&offset=0')
    task_map = {item['task_id']: item for item in final_tasks['tasks']}
    assert cancel_task['task_id'] in task_map, task_map.keys()
    assert disconnect_task['task_id'] in task_map, task_map.keys()
    assert finish_task['task_id'] in task_map, task_map.keys()

    return {
        'single_dispatches': len(single_dispatches),
        'initial_online_count': len(initial_online_ids),
        'broadcasts': [item['task_id'] for item in broadcasts],
        'cancelled_task': cancelled_final['status'],
        'disconnected_task': disconnected_final['status'],
        'completed_task': succeeded_final['status'],
        'command_session_id': session_id,
        'reconnected_session_id': reconnected['session_id'],
        'online_agents': sorted(final_agent_ids),
    }


def main() -> int:
    ensure_binaries()
    harness = Harness('hermes-concurrent-stress-e2e-')
    try:
        harness.start_server()
        print(json.dumps({'concurrent_stress_suite': run(harness), 'base_url': harness.base}, ensure_ascii=False, indent=2))
        return 0
    finally:
        harness.close()


if __name__ == '__main__':
    raise SystemExit(main())
