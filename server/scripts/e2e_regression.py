#!/usr/bin/env python3
import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from e2e.common import Harness, ensure_binaries
from e2e.agent_builds import run as run_agent_builds
from e2e.basic import run as run_basic
from e2e.auth import run as run_auth
from e2e.audit_precision import run as run_audit_precision
from e2e.edge import run as run_edge
from e2e.concurrent_stress import run as run_concurrent_stress
from e2e.database import run as run_database
from e2e.database_consistency import run as run_database_consistency
from e2e.database_interruptions import run as run_database_interruptions
from e2e.fault_matrix import run as run_fault_matrix
from e2e.https_listener import run as run_https_listener
from e2e.listeners import run as run_listeners
from e2e.lifecycle import run as run_lifecycle
from e2e.command_session import run as run_command_session
from e2e.full_chain import run as run_full_chain


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Hermes server regression suites")
    parser.add_argument(
        "suite",
        nargs="?",
        default="all",
        choices=[
            "all",
            "agent_builds",
            "basic",
            "auth",
            "audit_precision",
            "concurrent_stress",
            "edge",
            "database",
            "database_consistency",
            "database_interruptions",
            "fault_matrix",
            "https_listener",
            "listeners",
            "lifecycle",
            "command_session",
            "full_chain",
        ],
    )
    return parser.parse_args()


def main() -> int:
    ensure_binaries()
    args = parse_args()

    if args.suite == "auth":
        print(json.dumps({"auth_suite": run_auth()}, ensure_ascii=False, indent=2))
        return 0
    if args.suite == "audit_precision":
        print(
            json.dumps(
                {"audit_precision_suite": run_audit_precision()},
                ensure_ascii=False,
                indent=2,
            )
        )
        return 0
    if args.suite == "basic":
        harness = Harness("hermes-basic-e2e-")
        try:
            harness.start_server()
            print(
                json.dumps(
                    {"basic_suite": run_basic(harness), "base_url": harness.base},
                    ensure_ascii=False,
                    indent=2,
                )
            )
            return 0
        finally:
            harness.close()
    if args.suite == "agent_builds":
        harness = Harness("hermes-agent-builds-e2e-")
        try:
            harness.start_server()
            print(
                json.dumps(
                    {
                        "agent_builds_suite": run_agent_builds(harness),
                        "base_url": harness.base,
                    },
                    ensure_ascii=False,
                    indent=2,
                )
            )
            return 0
        finally:
            harness.close()
    if args.suite == "database":
        print(
            json.dumps({"database_suite": run_database()}, ensure_ascii=False, indent=2)
        )
        return 0
    if args.suite == "database_consistency":
        print(
            json.dumps(
                {"database_consistency_suite": run_database_consistency()},
                ensure_ascii=False,
                indent=2,
            )
        )
        return 0
    if args.suite == "database_interruptions":
        print(
            json.dumps(
                {"database_interruptions_suite": run_database_interruptions()},
                ensure_ascii=False,
                indent=2,
            )
        )
        return 0
    if args.suite == "fault_matrix":
        print(
            json.dumps(
                {"fault_matrix_suite": run_fault_matrix()}, ensure_ascii=False, indent=2
            )
        )
        return 0
    if args.suite == "edge":
        harness = Harness("hermes-edge-e2e-")
        try:
            harness.start_server()
            print(
                json.dumps(
                    {"edge_suite": run_edge(harness), "base_url": harness.base},
                    ensure_ascii=False,
                    indent=2,
                )
            )
            return 0
        finally:
            harness.close()
    if args.suite == "lifecycle":
        harness = Harness("hermes-lifecycle-e2e-")
        try:
            harness.start_server()
            print(
                json.dumps(
                    {
                        "lifecycle_suite": run_lifecycle(harness),
                        "base_url": harness.base,
                    },
                    ensure_ascii=False,
                    indent=2,
                )
            )
            return 0
        finally:
            harness.close()
    if args.suite == "command_session":
        harness = Harness("hermes-command-session-e2e-")
        try:
            harness.start_server()
            print(
                json.dumps(
                    {
                        "command_session_suite": run_command_session(harness),
                        "base_url": harness.base,
                    },
                    ensure_ascii=False,
                    indent=2,
                )
            )
            return 0
        finally:
            harness.close()
    if args.suite == "listeners":
        harness = Harness("hermes-listeners-e2e-")
        try:
            harness.start_server()
            print(
                json.dumps(
                    {
                        "listeners_suite": run_listeners(harness),
                        "base_url": harness.base,
                    },
                    ensure_ascii=False,
                    indent=2,
                )
            )
            return 0
        finally:
            harness.close()
    if args.suite == "https_listener":
        harness = Harness("hermes-https-e2e-")
        try:
            harness.start_server()
            print(
                json.dumps(
                    {
                        "https_listener_suite": run_https_listener(harness),
                        "base_url": harness.base,
                    },
                    ensure_ascii=False,
                    indent=2,
                )
            )
            return 0
        finally:
            harness.close()
    if args.suite == "concurrent_stress":
        harness = Harness("hermes-concurrent-stress-e2e-")
        try:
            harness.start_server()
            print(
                json.dumps(
                    {
                        "concurrent_stress_suite": run_concurrent_stress(harness),
                        "base_url": harness.base,
                    },
                    ensure_ascii=False,
                    indent=2,
                )
            )
            return 0
        finally:
            harness.close()
    if args.suite == "full_chain":
        harness = Harness("hermes-full-chain-e2e-")
        try:
            harness.start_server()
            print(
                json.dumps(
                    {
                        "full_chain_suite": run_full_chain(harness),
                        "base_url": harness.base,
                    },
                    ensure_ascii=False,
                    indent=2,
                )
            )
            return 0
        finally:
            harness.close()

    harness = Harness("hermes-e2e-")
    try:
        harness.start_server()
        result = {
            "base_url": harness.base,
            "tcp_port": harness.tcp_port,
            "api_port": harness.api_port,
            "basic_suite": run_basic(harness),
            "edge_suite": run_edge(harness),
            "lifecycle_suite": run_lifecycle(harness),
            "command_session_suite": run_command_session(harness),
            "concurrent_stress_suite": run_concurrent_stress(harness),
            "listeners_suite": run_listeners(harness),
            "agent_builds_suite": run_agent_builds(harness),
            "auth_suite": run_auth(),
            "audit_precision_suite": run_audit_precision(),
            "database_suite": run_database(),
            "database_consistency_suite": run_database_consistency(),
            "database_interruptions_suite": run_database_interruptions(),
            "fault_matrix_suite": run_fault_matrix(),
        }
        print(json.dumps(result, ensure_ascii=False, indent=2))
        return 0
    finally:
        harness.close()


if __name__ == "__main__":
    raise SystemExit(main())
