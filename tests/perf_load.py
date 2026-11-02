# /// script
# dependencies = ["requests", "rich"]
# ///
# Prepares targets A (Docker), B (wasmtime serve), C (Wambda) and runs tests/perf_load.ts (k6) against each.
# Local: docker build -t wambda-bench-native bench && uv run tests/perf_load.py
# Two machines: run tests/sut_targets.py and the compose stack on the machine under test, then on this one:
#   TARGET_HOST=<address> DURATION=30s uv run tests/perf_load.py
import os
import re
import subprocess

import requests
from perf_common import (
    BASE,
    REMOTE,
    TARGET_HOST,
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
HOST = TARGET_HOST if REMOTE else "host.docker.internal"

DURATION = os.environ.get("DURATION", "10s")
WARMUP = "5s"
WORKLOADS = [
    ("hello", [50, 200, 500]),
    ("fib/27", [50, 100, 200]),
]
# platform without Wasm execution: nginx + Axum only
HEALTH_WORKLOADS = [("health", [200, 500])]


rows = []


def k6(base_url, path, rate, duration, api_key=""):
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
        f"DURATION={duration}",
        "-e",
        f"API_KEY={api_key}",
        K6_IMAGE,
        "run",
        "--quiet",
        "/tests/perf_load.ts",
    ]
    out = subprocess.run(command, capture_output=True, text=True)
    return out.stdout + out.stderr


def run_k6(label, base_url, path, rate, api_key=""):
    print(f"running: {label} /{path} {rate}/s...")
    text = k6(base_url, path, rate, DURATION, api_key)

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


def run_target(label, base_url, workloads, api_key=""):
    print(f"warm-up: {label}...")
    k6(base_url, workloads[0][0], 50, WARMUP, api_key)
    for path, rates in workloads:
        for rate in rates:
            run_k6(label, base_url, path, rate, api_key)


session, headers, api_key = login()
function_id = deploy(session, "perf-load")

process = None
if not REMOTE:
    start_container("perf-native", 18080)
    process = start_wasmtime(18081, host="0.0.0.0")
wait_until_ready(f"http://{TARGET_HOST}:18080/hello")
wait_until_ready(f"http://{TARGET_HOST}:18081/hello")
requests.get(f"{BASE}/execute/{function_id}/hello", headers=headers)

try:
    run_target("A docker", f"http://{HOST}:18080", WORKLOADS)
    run_target("B wasmtime serve", f"http://{HOST}:18081", WORKLOADS)
    run_target("C wambda (via nginx)", f"http://{HOST}/api/execute/{function_id}", WORKLOADS, api_key)
    run_target("C nginx + Axum only", f"http://{HOST}/api", HEALTH_WORKLOADS)
finally:
    if process:
        stop_process(process)
        remove_containers("perf-native")
    session.delete(f"{BASE}/functions/{function_id}")

table = Table(title=f"k6 constant request rate, {DURATION} per run, target {TARGET_HOST}")
for column in ["Target", "Path", "Rate"]:
    table.add_column(column)
for column in ["Done req/s", "med", "p95", "p99", "max", "Failed", "Dropped"]:
    table.add_column(column, justify="right")
for row in rows:
    table.add_row(*row)
Console().print(table)
