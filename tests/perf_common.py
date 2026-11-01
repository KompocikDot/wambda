# Helpers shared by the perf_*.py scripts (run them from the repository root).
import subprocess
import time

import requests

BASE = "http://localhost/api"
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
    subprocess.run(["docker", "run", "-d", "--name", name, "-p", f"{port}:8080", IMAGE],
                   capture_output=True, check=True)


def remove_containers(*names):
    subprocess.run(["docker", "rm", "-f", *names], capture_output=True)


def start_wasmtime(port, host="127.0.0.1"):
    return subprocess.Popen(["wasmtime", "serve", "-Scommon", "-Sp3", "-Wcomponent-model-async",
                             "--addr", f"{host}:{port}", HELLO_WASM],
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)


def stop_process(process):
    process.kill()
    process.wait()
