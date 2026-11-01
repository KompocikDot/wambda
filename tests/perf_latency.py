# /// script
# dependencies = ["requests", "rich"]
# ///
# Cold start and warm latency of the same "hello" handler:
#   A - native server in a Docker container (alpine)
#   B - Wasm module in plain `wasmtime serve` (runtime only, no platform)
#   C - Wasm module through the Wambda platform (auth, DB, S3 logging, nginx)
# Usage: docker build -t wambda-bench-native bench && uv run tests/perf_latency.py
import statistics
import subprocess
import time

import requests
from rich.console import Console
from rich.table import Table

from perf_common import BASE, HELLO_WASM, login, remove_containers, start_container, start_wasmtime, stop_process, wait_until_ready

COMPOSE = ["docker", "compose", "-f", "compose.prod.yml"]

COLD_RUNS = 20
RESTART_RUNS = 5
WARM_REQUESTS = 300
WARMUP_REQUESTS = 30

URL_A = "http://127.0.0.1:18080/hello"
URL_B = "http://127.0.0.1:18081/hello"


def percentile(values, p):
    values = sorted(values)
    return values[min(len(values) - 1, int(len(values) * p))]


rows = []


def measure(name, function):
    print(f"running: {name}...")
    ms = [t * 1000 for t in function()]
    rows.append([
        name,
        str(len(ms)),
        f"{statistics.median(ms):.1f}",
        f"{percentile(ms, 0.95):.1f}",
        f"{percentile(ms, 0.99):.1f}",
        f"{min(ms):.1f}",
        f"{max(ms):.1f}",
    ])


def warm_latency(get):
    for _ in range(WARMUP_REQUESTS):
        get()
    times = []
    for _ in range(WARM_REQUESTS):
        start = time.time()
        get()
        times.append(time.time() - start)
    return times


# ---------- A: Docker ----------
def cold_docker():
    times = []
    for _ in range(COLD_RUNS):
        start = time.time()
        start_container("perf-native", 18080)
        wait_until_ready(URL_A)
        times.append(time.time() - start)
        remove_containers("perf-native")
    return times


def warm_docker():
    start_container("perf-native", 18080)
    wait_until_ready(URL_A)
    http = requests.Session()
    times = warm_latency(lambda: http.get(URL_A))
    remove_containers("perf-native")
    return times


# ---------- B: wasmtime serve ----------
def cold_wasmtime():
    times = []
    for _ in range(COLD_RUNS):
        start = time.time()
        process = start_wasmtime(18081)
        wait_until_ready(URL_B)
        times.append(time.time() - start)
        stop_process(process)
    return times


def warm_wasmtime():
    process = start_wasmtime(18081)
    wait_until_ready(URL_B)
    http = requests.Session()
    times = warm_latency(lambda: http.get(URL_B))
    stop_process(process)
    return times


# ---------- C: Wambda ----------
session, headers, _ = login()

data = {"name": "perf-hello", "memoryLimit": 128, "timeout": 10, "envsStringified": ""}
function_id = None

for function in session.get(f"{BASE}/functions").json():
    if function["name"] == "perf-hello":
        function_id = function["uuid"]
        session.patch(f"{BASE}/functions/{function_id}", data=data, files={"wasm": open(HELLO_WASM, "rb")})
if function_id is None:
    function_id = session.post(f"{BASE}/functions", data=data, files={"wasm": open(HELLO_WASM, "rb")}).json()["uuid"]

execute_url = f"{BASE}/execute/{function_id}/hello"


def call_wambda():
    return requests.get(execute_url, headers=headers)


def cold_wambda_cache_miss():
    times = []
    for _ in range(COLD_RUNS):
        # editing the function (without a new module) drops the cached binary
        session.patch(f"{BASE}/functions/{function_id}", data=data)
        start = time.time()
        call_wambda()
        times.append(time.time() - start)
    return times


def cold_wambda_restart():
    times = []
    for _ in range(RESTART_RUNS):
        subprocess.run(COMPOSE + ["restart", "backend"], capture_output=True, check=True)
        wait_until_ready(f"{BASE}/health")
        start = time.time()
        call_wambda()
        times.append(time.time() - start)
    return times


def warm_wambda():
    call_wambda()
    http = requests.Session()
    return warm_latency(lambda: http.get(execute_url, headers=headers))


# cold = time to the first correct response, warm = one keep-alive connection, sequential
measure("cold A docker run -> first 200", cold_docker)
measure("cold B wasmtime serve -> first 200", cold_wasmtime)
measure("cold C wambda, binary not cached", cold_wambda_cache_miss)
measure("cold C wambda, after backend restart", cold_wambda_restart)

measure("warm A docker", warm_docker)
measure("warm B wasmtime serve", warm_wasmtime)
measure("warm C wambda (via nginx)", warm_wambda)

table = Table(title="Latency of the hello handler (ms)")
table.add_column("Test")
for column in ["n", "median", "p95", "p99", "min", "max"]:
    table.add_column(column, justify="right")
for row in rows:
    table.add_row(*row)
Console().print(table)
