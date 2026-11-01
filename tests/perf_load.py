# /// script
# dependencies = ["requests", "rich"]
# ///
# Prepares targets A (Docker), B (wasmtime serve), C (Wambda) and runs tests/perf_load.ts (k6) against each.
# Usage: docker build -t wambda-bench-native bench && uv run tests/perf_load.py
import os
import re
import subprocess

import requests
from perf_common import (
    BASE,
    deploy,
    login,
    remove_containers,
    start_container,
    start_wasmtime,
    stop_process,
    wait_until_ready,
)
from rich.console import Console
from rich.table import Table

K6_IMAGE = "grafana/k6"

# k6 runs in Docker and reaches the host through host.docker.internal (--add-host makes it work on Linux too)
HOST = "host.docker.internal"

DURATION = "10s"
WORKLOADS = [
    ("hello", [50, 200, 500]),
    ("fib/27", [50, 100, 200]),
]


rows = []


def run_k6(label, base_url, path, rate, api_key=""):
    print(f"running: {label} /{path} {rate}/s...")
    command = [
        "docker",
        "run",
        "--rm",
        "--add-host",
        "host.docker.internal:host-gateway",
        "-v",
        f"{os.getcwd()}/tests:/tests",
        "-e",
        f"BASE_URL={base_url}",
        "-e",
        f"REQ_PATH={path}",
        "-e",
        f"RATE={rate}",
        "-e",
        f"DURATION={DURATION}",
        "-e",
        f"API_KEY={api_key}",
        K6_IMAGE,
        "run",
        "--quiet",
        "/tests/perf_load.ts",
    ]
    out = subprocess.run(command, capture_output=True, text=True)
    text = out.stdout + out.stderr

    latency = re.search(
        r"http_req_duration.*?med=(\S+)\s+p\(95\)=(\S+)\s+p\(99\)=(\S+)\s+max=(\S+)",
        text,
    )
    reqs = re.search(r"http_reqs.*?(\d+)\s+([\d.]+)/s", text)
    failed = re.search(r"http_req_failed.*?([\d.]+)%", text)
    dropped = re.search(r"dropped_iterations.*?(\d+)", text)
    if not (latency and reqs):
        rows.append(
            [label, f"/{path}", f"{rate}/s", "[red]k6 output not recognized[/red]"]
        )
        print(text)
        return

    rows.append(
        [
            label,
            f"/{path}",
            f"{rate}/s",
            reqs.group(2),
            *latency.groups(),
            f"{failed.group(1) if failed else '0'}%",
            dropped.group(1) if dropped else "0",
        ]
    )


def run_target(label, base_url, api_key=""):
    for path, rates in WORKLOADS:
        for rate in rates:
            run_k6(label, base_url, path, rate, api_key)


session, headers, api_key = login()
function_id = deploy(session, "perf-load")

start_container("perf-native", 18080)
process = start_wasmtime(18081, host="0.0.0.0")
wait_until_ready("http://127.0.0.1:18080/hello")
wait_until_ready("http://127.0.0.1:18081/hello")
requests.get(f"{BASE}/execute/{function_id}/hello", headers=headers)

try:
    run_target("A docker", f"http://{HOST}:18080")
    run_target("B wasmtime serve", f"http://{HOST}:18081")
    run_target("C wambda (via nginx)", f"http://{HOST}/api/execute/{function_id}", api_key)
finally:
    stop_process(process)
    remove_containers("perf-native")
    session.delete(f"{BASE}/functions/{function_id}")

table = Table(title=f"k6 constant request rate, {DURATION} per run")
for column in ["Target", "Path", "Rate"]:
    table.add_column(column)
for column in ["Done req/s", "med", "p95", "p99", "max", "Failed", "Dropped"]:
    table.add_column(column, justify="right")
for row in rows:
    table.add_row(*row)
Console().print(table)
