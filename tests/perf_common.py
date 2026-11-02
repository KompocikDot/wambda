# Helpers shared by the perf_*.py scripts (run them from the repository root).
# TARGET_HOST=<address> aims the scripts at another machine (the one running sut_targets.py), default is this one.
import os
import subprocess
import threading
import time

import requests

TARGET_HOST = os.environ.get("TARGET_HOST", "localhost")
REMOTE = TARGET_HOST != "localhost"
BASE = f"http://{TARGET_HOST}/api"
EMAIL = "perf@test.pl"
PASSWORD = "Test1234!pass"
HELLO_WASM = "bench/target/wasm32-wasip2/release/guest.wasm"
IMAGE = "wambda-bench-native"


def wait_until_ready(url):
    while True:
        try:
            if requests.get(url, timeout=1).status_code == 200:
                return
        except requests.RequestException:
            pass
        time.sleep(0.001)


def login():
    """Registers the test user (if needed) and returns (logged-in session, Authorization headers for /execute)."""
    session = requests.Session()
    session.post(f"{BASE}/auth/register", json={"fullName": "Perf", "email": EMAIL, "password": PASSWORD})
    session.post(f"{BASE}/auth/login", json={"email": EMAIL, "password": PASSWORD})
    api_key = session.get(f"{BASE}/auth/api-key").json()["apiKey"]
    return session, {"Authorization": f"Bearer {api_key}"}, api_key


def deploy(session, name, timeout=30):
    data = {"name": name, "memoryLimit": 128, "timeout": timeout, "envsStringified": ""}
    response = session.post(f"{BASE}/functions", data=data, files={"wasm": open(HELLO_WASM, "rb")})
    return response.json()["uuid"]


def start_container(name, port):
    # a leftover from an interrupted run would hold the name and the port
    subprocess.run(["docker", "rm", "-f", name], capture_output=True)
    result = subprocess.run(["docker", "run", "-d", "--name", name, "-p", f"{port}:8080", IMAGE],
                            capture_output=True, text=True)
    if result.returncode != 0:
        raise SystemExit(result.stderr)


def remove_containers(*names):
    subprocess.run(["docker", "rm", "-f", *names], capture_output=True)


def start_wasmtime(port, host="127.0.0.1"):
    return subprocess.Popen(["wasmtime", "serve", "-Scommon", "-Sp3", "-Wcomponent-model-async",
                             "--addr", f"{host}:{port}", HELLO_WASM],
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)


def stop_process(process):
    process.kill()
    process.wait()


def to_mib(text):
    for unit, factor in [("GiB", 1024), ("MiB", 1), ("KiB", 1 / 1024), ("B", 1 / 1024 / 1024)]:
        if text.endswith(unit):
            return float(text[: -len(unit)]) * factor
    raise ValueError(text)


def docker_stats():
    """name -> (cpu %, memory MiB) for every running container."""
    out = subprocess.run(["docker", "stats", "--no-stream", "--format", "{{.Name}} {{.CPUPerc}} {{.MemUsage}}"],
                         capture_output=True, text=True).stdout
    stats = {}
    for line in out.splitlines():
        name, cpu, mem = line.split()[:3]
        stats[name] = (float(cpu.rstrip("%")), to_mib(mem))
    return stats


def docker_sum(names):
    stats = docker_stats()
    return sum(stats[n][0] for n in names if n in stats), sum(stats[n][1] for n in names if n in stats)


def process_stats(pids):
    cpu, mem = 0, 0
    for pid in pids:
        out = subprocess.run(["ps", "-o", "%cpu=,rss=", "-p", str(pid)], capture_output=True, text=True).stdout.split()
        cpu += float(out[0])
        mem += int(out[1]) / 1024
    return cpu, mem


def closed_loop(url, request_headers, seconds, workers, sample):
    """Sends requests from `workers` parallel clients (a new one as soon as the previous finishes).
    sample() -> (cpu %, memory MiB) is polled meanwhile. Returns (req/s, average CPU %, max memory MiB)."""
    deadline = time.time() + seconds
    counts = [0] * workers

    def worker(i):
        http = requests.Session()
        while time.time() < deadline:
            if http.get(url, headers=request_headers).status_code == 200:
                counts[i] += 1

    threads = [threading.Thread(target=worker, args=(i,)) for i in range(workers)]
    for thread in threads:
        thread.start()
    samples = []
    while time.time() < deadline:
        samples.append(sample())
    for thread in threads:
        thread.join()
    return sum(counts) / seconds, sum(s[0] for s in samples) / len(samples), max(s[1] for s in samples)
