#!/usr/bin/env bash
# Phase 1, on the machine under test alone: cold start, resources, scaling, security.
# Every repetition starts from a clean stack. (Network measurements: run_load.sh, phase 2.)
# Usage: bash tests/run_sut.sh [runs]   -> results/sut-*.txt
set -u
export COLUMNS=220
RUNS=${1:-5}
mkdir -p results
bash tests/collect_env.sh > results/env-sut.txt

for i in $(seq 1 "$RUNS"); do
  docker compose -f compose.prod.yml down -v
  docker compose -f compose.prod.yml up -d --build --wait
  sleep 10
  uv run tests/attacks.py > "results/sut-attacks-$i.txt"
  uv run tests/perf_latency.py > "results/sut-latency-$i.txt"
  uv run tests/perf_resources.py > "results/sut-resources-$i.txt"
  uv run tests/perf_scaling.py > "results/sut-scaling-$i.txt"
done
