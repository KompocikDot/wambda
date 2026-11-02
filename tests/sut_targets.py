# /// script
# dependencies = ["requests"]
# ///
# Run on the machine under test (e.g. in tmux) while the load machine runs perf_load.py / perf_latency.py
# with TARGET_HOST=<this machine>. Starts A (Docker, :18080) and B (wasmtime serve, :18081), stops them on Ctrl-C.
# Usage: docker build -t wambda-bench-native bench && uv run tests/sut_targets.py
import time

from perf_common import (
    remove_containers,
    start_container,
    start_wasmtime,
    stop_process,
    wait_until_ready,
)

start_container("perf-native", 18080)
process = start_wasmtime(18081, host="0.0.0.0")
try:
    wait_until_ready("http://127.0.0.1:18080/hello")
    wait_until_ready("http://127.0.0.1:18081/hello")
    print("A on :18080 and B on :18081 are ready, Ctrl-C to stop")
    while True:
        time.sleep(1)
except KeyboardInterrupt:
    pass
finally:
    stop_process(process)
    remove_containers("perf-native")
