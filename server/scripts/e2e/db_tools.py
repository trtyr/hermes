#!/usr/bin/env python3
import sqlite3
from pathlib import Path


def connect(sqlite_path: Path) -> sqlite3.Connection:
    connection = sqlite3.connect(sqlite_path)
    connection.row_factory = sqlite3.Row
    return connection


def load_agents(sqlite_path: Path) -> list[sqlite3.Row]:
    connection = connect(sqlite_path)
    try:
        return connection.execute(
            'select agent_id, session_id, hostname, username, tags_json, peer_addr, connected_at, last_seen, is_online, is_disabled, updated_at from agents order by agent_id asc'
        ).fetchall()
    finally:
        connection.close()


def load_agent(sqlite_path: Path, agent_id: str):
    connection = connect(sqlite_path)
    try:
        return connection.execute(
            'select agent_id, session_id, hostname, username, tags_json, peer_addr, connected_at, last_seen, is_online, is_disabled, updated_at from agents where agent_id = ?',
            (agent_id,),
        ).fetchone()
    finally:
        connection.close()


def load_task(sqlite_path: Path, task_id: str):
    connection = connect(sqlite_path)
    try:
        return connection.execute(
            'select task_id, parent_task_id, target_agent_id, command, payload, status, created_at, updated_at, success, output, children_json from tasks where task_id = ?',
            (task_id,),
        ).fetchone()
    finally:
        connection.close()


def load_tasks_if_status(sqlite_path: Path, task_ids: list[str], status: str):
    rows = [load_task(sqlite_path, task_id) for task_id in task_ids]
    if all(row is not None and row['status'] == status for row in rows):
        return rows
    return None


def load_audits(sqlite_path: Path) -> list[dict]:
    connection = connect(sqlite_path)
    try:
        rows = connection.execute(
            'select audit_id, operator, action, target_kind, target_id, detail, created_at from audits order by audit_id desc'
        ).fetchall()
        return [dict(row) for row in rows]
    finally:
        connection.close()


def normalize_audits(records: list[dict]) -> list[tuple]:
    return [
        (
            record['audit_id'],
            record['operator'],
            record['action'],
            record['target_kind'],
            record['target_id'],
            record['detail'],
            record['created_at'],
        )
        for record in records
    ]


def count_actions(records: list[dict]) -> dict[str, int]:
    counts = {}
    for record in records:
        counts[record['action']] = counts.get(record['action'], 0) + 1
    return counts
