#!/usr/bin/env bash
# Runs on the LOAD machine; measures the machine under test over the network.
# The stack on the SUT must be up and `uv run tests/sut_targets.py` must be running there.
# Usage: TARGET_HOST=<address> bash tests/run_load.sh [runs]   -> results/load-*.txt
set -eu
export COLUMNS=220
export DURATION=${DURATION:-30s}
RUNS=${1:-5}
mkdir -p results
bash tests/collect_env.sh > results/env-load.txt
docker pull grafana/k6

for i in $(seq 1 "$RUNS"); do
  uv run tests/perf_latency.py > "results/load-latency-$i.txt"
  uv run tests/perf_load.py > "results/load-k6-$i.txt"
done
