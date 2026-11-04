#!/usr/bin/env bash
# Builds everything, starts a clean platform and runs all tests; raw results go to results/*.csv.
# Usage (from the repository root): bash tests/run_all.sh
set -euo pipefail

mkdir -p results

# the handler (Wasm component and native server), the attack modules and the images of variants A and B
cargo build --release --target wasm32-wasip2 --manifest-path bench/Cargo.toml -p guest
bash bench/attacks/build.sh
docker build -t wambda-bench-native bench
docker build -f bench/Dockerfile.wasmtime -t wambda-bench-wasmtime bench

# the platform on a clean database
docker compose -f compose.prod.yml down -v
docker compose -f compose.prod.yml up -d --build --wait
sleep 10

for test in startup load memory security; do
  bash "tests/$test.sh"
done

# after the builds, so that the image list in env.txt is complete
bash tests/env.sh >results/env.txt
python3 tests/summarize.py
