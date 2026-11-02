# /// script
# dependencies = ["requests", "rich"]
# ///
# Throughput of the Wambda backend as a function of the CPU cores it may use (docker update --cpuset-cpus).
# Run on the machine under test. The load generator, nginx, Postgres and RustFS share the rest of the cores.
# Usage: uv run tests/perf_scaling.py
import subprocess

import requests
from perf_common import BASE, closed_loop, deploy, docker_sum, login
from rich.console import Console
from rich.table import Table

BACKEND = "faasm-backend-1"
SECONDS = 15
PATHS = ["fib/27"]  # CPU-bound; "hello" would be limited by the Python client, not the backend

cores = int(subprocess.run(["docker", "info", "--format", "{{.NCPU}}"], capture_output=True, text=True).stdout)
steps = sorted({k for k in [1, 2, 4, cores] if k <= cores})

session, headers, _ = login()
function_id = deploy(session, "perf-scaling")
requests.get(f"{BASE}/execute/{function_id}/hello", headers=headers)

rows = []
try:
    for k in steps:
        subprocess.run(["docker", "update", "--cpuset-cpus", f"0-{k - 1}", BACKEND], capture_output=True, check=True)
        for path in PATHS:
            print(f"running: backend on {k} core(s), /{path}...")
            workers = max(8, 4 * k)
            rate, cpu, _ = closed_loop(
                f"{BASE}/execute/{function_id}/{path}", headers, SECONDS, workers, lambda: docker_sum([BACKEND])
            )
            rows.append([str(k), f"/{path}", str(workers), f"{rate:.1f}", f"{cpu:.0f}"])
finally:
    subprocess.run(["docker", "update", "--cpuset-cpus", f"0-{cores - 1}", BACKEND], capture_output=True)
    session.delete(f"{BASE}/functions/{function_id}")

table = Table(title=f"Backend throughput vs. CPU cores ({SECONDS}s, closed loop)")
for column in ["Cores", "Path"]:
    table.add_column(column)
for column in ["Clients", "req/s", "backend CPU %"]:
    table.add_column(column, justify="right")
for row in rows:
    table.add_row(*row)
Console().print(table)
