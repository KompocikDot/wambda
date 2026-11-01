# /// script
# dependencies = ["requests", "rich"]
# requires-python = ">=3.11"
# ///


# 200 = test passed (sandbox held)
# 403 = test failed (attack succeeded)
# 422 = guest crashed or hit a resource limit (trap, memory) - contained by the platform
# 408 = guest killed on timeout
# 500 = unexpected platform error
import threading
import time

import requests
from rich.console import Console
from rich.live import Live
from rich.table import Table

BASE = "http://localhost/api"
EMAIL = "attacks@test.pl"
PASSWORD = "Test1234!pass"
DIST = "bench/attacks/dist"
HELLO_WASM = "bench/target/wasm32-wasip2/release/guest.wasm"

session = requests.Session()
session.post(
    f"{BASE}/auth/register",
    json={"fullName": "Attacks", "email": EMAIL, "password": PASSWORD},
)
session.post(f"{BASE}/auth/login", json={"email": EMAIL, "password": PASSWORD})
api_key = session.get(f"{BASE}/auth/api-key").json()["apiKey"]

console = Console()
table = Table(title="Tests")
table.add_column("Test name")
table.add_column("Expected status code")
table.add_column("Received status code")
table.add_column("Execution duration")
table.add_column("Test passed")


def colored(ok: bool) -> str:
    return "[green]True[/green]" if ok else "[red]False[/red]"


live = Live(table, console=console, refresh_per_second=4)
live.start()


def deploy(name, wasm_path, memory, timeout, envs=""):
    data = {
        "name": name,
        "memoryLimit": memory,
        "timeout": timeout,
        "envsStringified": envs,
    }
    files = {"wasm": open(wasm_path, "rb")}

    for function in session.get(f"{BASE}/functions").json():
        if function["name"] == name:
            session.patch(
                f"{BASE}/functions/{function['uuid']}", data=data, files=files
            )
            return function["uuid"]

    return session.post(f"{BASE}/functions", data=data, files=files).json()["uuid"]


def execute(function_id, path, timeout=300):
    # Cookie is set to check if API removes forbidden headers such as authorization or cookie
    headers = {"Authorization": f"Bearer {api_key}", "Cookie": "session=secret-cookie"}
    return requests.get(
        f"{BASE}/execute/{function_id}/{path}", headers=headers, timeout=timeout
    )


# name, memory (MB), timeout (s), expected status, envs (base64), text the body must contain
cases = [
    ("attack-memory-bomb", 64, 10, 422, "", ""),
    ("attack-trap-div-zero", 64, 10, 422, "", ""),
    ("attack-infinite-loop", 64, 5, 408, "", ""),
    ("attack-fs-escape", 64, 10, 200, "", ""),
    ("attack-env-leak", 64, 10, 200, "Rk9P:YmFy,", "FOO=bar"),
    ("attack-header-leak", 64, 10, 200, "", ""),
]


for name, memory, timeout, expected, envs, contains in cases:
    function_id = deploy(name, f"{DIST}/{name}.wasm", memory, timeout, envs)

    start = time.time()
    response = execute(function_id, "run")
    seconds = time.time() - start

    ok = response.status_code == expected and contains in response.text
    table.add_row(
        name,
        str(expected),
        str(response.status_code),
        f"{seconds:.2f}",
        colored(ok),
    )


# SSRF: the guest sends a request to the given address, internal ones must be refused (403).
# Last row is a control: a public address has to stay reachable (200), otherwise the filter blocks everything.
ssrf_id = deploy("attack-ssrf", f"{DIST}/attack-ssrf.wasm", 64, 10)
ssrf_targets = [
    ("http://127.0.0.1/", ""),
    ("http://localhost/", ""),
    ("http://localtest.me/", ""),
    ("http://169.254.169.254/latest/meta-data/", ""),
    ("http://10.0.0.1/", ""),
    ("http://192.168.0.1/", ""),
    ("http://100.64.0.1/", ""),
    ("http://s3:9000/health", ""),
    ("http://db:5432/", ""),
    ("http://example.com/", "&allow=1"),
]

for url, extra in ssrf_targets:
    expected = 200
    start = time.time()
    response = execute(ssrf_id, f"run?url={url}{extra}")
    seconds = time.time() - start

    table.add_row(
        f"ssrf {url[7:30]}",
        str(expected),
        str(response.status_code),
        f"{seconds:.2f}",
        colored(response.status_code == expected),
    )


# run 16 executions at once to test if they multiple executions won't slow down others too much
loop_id = deploy("attack-infinite-loop", f"{DIST}/attack-infinite-loop.wasm", 64, 5)
hello_id = deploy("baseline-hello", HELLO_WASM, 64, 10)

threads = []
for _ in range(16):
    thread = threading.Thread(target=execute, args=(loop_id, "run", 30))
    thread.start()
    threads.append(thread)
time.sleep(1)

worst = 0
for _ in range(5):
    start = time.time()
    execute(hello_id, "hello")
    worst = max(worst, time.time() - start)
    time.sleep(0.5)

for thread in threads:
    thread.join()

table.add_row(
    "baseline-hello",
    "<1s",
    f"{worst:.2f}",
    f"{worst:.2f}",
    colored(worst < 1),
)

live.stop()
