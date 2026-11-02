# /// script
# dependencies = ["requests", "rich"]
# ///
# Resource usage of the same "fib" handler: A = Docker (native), B = `wasmtime serve`, C = Wambda platform.
# Usage: docker build -t wambda-bench-native bench && uv run tests/perf_resources.py
import os
import subprocess
import time

import requests
from perf_common import (
    BASE,
    HELLO_WASM,
    IMAGE,
    closed_loop,
    deploy,
    docker_sum,
    login,
    process_stats,
    remove_containers,
    start_container,
    start_wasmtime,
    stop_process,
    wait_until_ready,
)
from rich.console import Console
from rich.table import Table

BACKEND = "faasm-backend-1"
STACK = ["faasm-backend-1", "faasm-nginx-1", "faasm-db-1", "faasm-s3-1"]

DENSITY = [1, 10, 50]
LOAD_SECONDS = 15
LOAD_WORKERS = 8
LOAD_PATH = "fib/27"


session, headers, _ = login()


# section title -> column names; the first column is text, the rest are numbers
SECTIONS = {
    "Size": ["Item", "Size"],
    "Idle memory (one function, no traffic)": ["Target", "Memory MiB"],
    f"Load ({LOAD_WORKERS} parallel clients, {LOAD_SECONDS}s, GET /{LOAD_PATH}, closed loop)": [
        "Target",
        "req/s",
        "avg CPU %",
        "max mem MiB",
    ],
    "Density (N idle functions, each called once)": [
        "Target",
        "N",
        "Total MiB",
        "Each MiB",
        "Started in s",
    ],
}
SIZE, IDLE, LOAD, DENSITY_SECTION = SECTIONS
rows = {section: [] for section in SECTIONS}


def add(section, *values):
    rows[section].append(values)


# ---------- sizes ----------
image_bytes = int(
    subprocess.run(
        ["docker", "image", "inspect", IMAGE, "-f", "{{.Size}}"],
        capture_output=True,
        text=True,
    ).stdout
)
backend_image = subprocess.run(
    ["docker", "images", "faasm-backend", "--format", "{{.Size}}"],
    capture_output=True,
    text=True,
).stdout.strip()
add(
    SIZE,
    "A docker image (alpine + native binary)",
    f"{image_bytes / 1024 / 1024:.1f} MiB",
)
add(SIZE, "B/C wasm module", f"{os.path.getsize(HELLO_WASM) / 1024:.0f} KiB")
add(SIZE, "C platform image (shared by all functions)", backend_image)

# ---------- idle memory ----------
print("preparing idle memory measurement...")
start_container("perf-native", 18080)
wait_until_ready("http://127.0.0.1:18080/hello")
process = start_wasmtime(18081)
wait_until_ready("http://127.0.0.1:18081/hello")
function_id = deploy(session, "perf-idle")
requests.get(f"{BASE}/execute/{function_id}/hello", headers=headers)
time.sleep(3)

add(IDLE, "A docker container", f"{docker_sum(['perf-native'])[1]:.1f}")
add(IDLE, "B wasmtime serve process", f"{process_stats([process.pid])[1]:.1f}")
add(IDLE, "C backend (whole platform)", f"{docker_sum([BACKEND])[1]:.1f}")
add(IDLE, "C whole stack (+nginx, db, s3)", f"{docker_sum(STACK)[1]:.1f}")


# ---------- load: CPU, memory, throughput ----------
def run_load(label, url, request_headers, sample):
    print(f"load: {label}...")
    rate, cpu, mem = closed_loop(url, request_headers, LOAD_SECONDS, LOAD_WORKERS, sample)
    add(LOAD, label, f"{rate:.1f}", f"{cpu:.1f}", f"{mem:.1f}")


run_load(
    "A docker",
    f"http://127.0.0.1:18080/{LOAD_PATH}",
    {},
    lambda: docker_sum(["perf-native"]),
)
run_load(
    "B wasmtime serve",
    f"http://127.0.0.1:18081/{LOAD_PATH}",
    {},
    lambda: process_stats([process.pid]),
)
run_load(
    "C wambda, backend only",
    f"{BASE}/execute/{function_id}/{LOAD_PATH}",
    headers,
    lambda: docker_sum([BACKEND]),
)
run_load(
    "C wambda, whole stack",
    f"{BASE}/execute/{function_id}/{LOAD_PATH}",
    headers,
    lambda: docker_sum(STACK),
)

stop_process(process)
remove_containers("perf-native")
session.delete(f"{BASE}/functions/{function_id}")

# ---------- density ----------
for n in DENSITY:
    print(f"density: N={n}, A docker containers...")
    names = [f"perf-d-{i}" for i in range(n)]
    start = time.time()
    for i, name in enumerate(names):
        start_container(name, 18100 + i)
    for i in range(n):
        wait_until_ready(f"http://127.0.0.1:{18100 + i}/hello")
    started = time.time() - start
    time.sleep(2)
    mem = docker_sum(names)[1]
    remove_containers(*names)
    add(
        DENSITY_SECTION,
        "A docker containers",
        str(n),
        f"{mem:.1f}",
        f"{mem / n:.1f}",
        f"{started:.1f}",
    )

    print(f"density: N={n}, B wasmtime processes...")
    start = time.time()
    processes = [start_wasmtime(18300 + i) for i in range(n)]
    for i in range(n):
        wait_until_ready(f"http://127.0.0.1:{18300 + i}/hello")
    started = time.time() - start
    time.sleep(2)
    mem = process_stats([p.pid for p in processes])[1]
    for p in processes:
        stop_process(p)
    add(
        DENSITY_SECTION,
        "B wasmtime processes",
        str(n),
        f"{mem:.1f}",
        f"{mem / n:.1f}",
        f"{started:.1f}",
    )

    # C: N functions in one backend, memory is the growth of the backend (deploy + one call each)
    print(f"density: N={n}, C wambda functions...")
    before = docker_sum([BACKEND])[1]
    start = time.time()
    ids = [deploy(session, f"perf-d-{i}") for i in range(n)]
    for function_id in ids:
        requests.get(f"{BASE}/execute/{function_id}/hello", headers=headers)
    started = time.time() - start
    time.sleep(2)
    growth = docker_sum([BACKEND])[1] - before
    for function_id in ids:
        session.delete(f"{BASE}/functions/{function_id}")
    add(
        DENSITY_SECTION,
        "C wambda (backend growth)",
        str(n),
        f"{growth:.1f}",
        f"{growth / n:.1f}",
        f"{started:.1f}",
    )

for section, columns in SECTIONS.items():
    table = Table(title=section)
    for i, column in enumerate(columns):
        table.add_column(column, justify="left" if i == 0 else "right")
    for row in rows[section]:
        table.add_row(*row)
    Console().print(table)
