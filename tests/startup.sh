#!/usr/bin/env bash
# Cold start and warm start of the /hello handler -> results/startup.csv
#   cold start: A, B - `docker run` until the first answer; B_process - `wasmtime serve` without Docker;
#               C - the first call of a function: right after its deploy and after a restart of the backend
#   warm start: latency of sequential requests over one connection (after a warm-up)
# The platform must be running: docker compose -f compose.prod.yml up -d --build --wait
# Usage: bash tests/startup.sh
source "$(dirname "$0")/lib.sh"
TEST=startup
RUNS=${RUNS:-10}
WARMUP_REQUESTS=30
MEASURED_REQUESTS=300

cleanup() {
  remove_containers startup-a startup-b
  if [[ -n ${WASMTIME_PID:-} ]]; then kill "$WASMTIME_PID" 2>/dev/null || true; fi
  rm -f "${COMPILED:-}"
}

# warm_start <variant> <run> <url> [oha args]
warm_start() {
  local variant=$1 run=$2 url=$3 report
  shift 3
  oha_json -c 1 -n "$WARMUP_REQUESTS" "$@" "$url" >/dev/null
  report=$(oha_json -c 1 -n "$MEASURED_REQUESTS" "$@" "$url")
  record "$variant" "$run" warm_p50 "$(jq '.latencyPercentiles.p50 * 1000' <<<"$report")" ms
  record "$variant" "$run" warm_p95 "$(jq '.latencyPercentiles.p95 * 1000' <<<"$report")" ms
}

# Variants A and B: a container started from scratch.
# measure_container <variant> <name> <image> <port> <run>
measure_container() {
  local variant=$1 name=$2 image=$3 port=$4 run=$5 start end
  local url="http://127.0.0.1:$port/hello"
  start=$(now_us)
  start_container "$name" "$image" "$port"
  wait_until_ok "$url"
  end=$(now_us)
  record "$variant" "$run" cold_start "$(elapsed_ms "$start" "$end")" ms
  warm_start "$variant" "$run" "$url"
  remove_containers "$name"
}

# wasmtime started as a plain process (precompiled, as in the image of variant B).
measure_wasmtime_process() {
  local run=$1 start end
  start=$(now_us)
  wasmtime serve -Scommon -Sp3 -Wcomponent-model-async --allow-precompiled \
    --addr 127.0.0.1:18082 "$COMPILED" >/dev/null 2>&1 &
  WASMTIME_PID=$!
  wait_until_ok http://127.0.0.1:18082/hello
  end=$(now_us)
  record B_process "$run" cold_start "$(elapsed_ms "$start" "$end")" ms
  kill "$WASMTIME_PID"
  wait "$WASMTIME_PID" 2>/dev/null || true
  WASMTIME_PID=
}

measure_platform() {
  local run=$1

  platform_deploy "startup-$run"
  local function_id=$FUNCTION_ID
  record C "$run" deploy "$DEPLOY_MS" ms

  local auth="Authorization: Bearer $API_KEY"
  local url="$BASE/execute/$function_id/hello"
  record C "$run" cold_start "$(request_ms -H "$auth" "$url")" ms

  restart_backend
  record C "$run" cold_start_after_restart "$(request_ms -H "$auth" "$url")" ms

  warm_start C "$run" "$url" -H "$auth"
  platform_delete "$function_id"
}

csv_init
remove_containers startup-a startup-b
COMPILED=$(mktemp)
wasmtime compile -Sp3 -Wcomponent-model-async "$HELLO_WASM" -o "$COMPILED"
platform_login

for run in $(seq 1 "$RUNS"); do
  echo "run $run of $RUNS"
  measure_container A startup-a "$IMAGE_A" 18080 "$run"
  measure_container B startup-b "$IMAGE_B" 18081 "$run"
  measure_wasmtime_process "$run"
  measure_platform "$run"
done
echo "results: $CSV"
