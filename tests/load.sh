#!/usr/bin/env bash
# Throughput and CPU of the variants under a few-second CPU-bound request (GET /matmul/{n}) -> results/load.csv
# CLIENTS parallel clients send REQUESTS requests; CPU time is read from the containers before and after.
# Variant C is measured twice in the same window: the backend alone and the whole stack (backend, nginx, db, s3).
# The Docker daemon, containerd, shims and proxies (Linux only) are measured in the same window as <variant>_docker.
# Peak memory is the highest value among samples taken every half a second during the test.
# Usage: bash tests/load.sh
source "$(dirname "$0")/lib.sh"
TEST=load
RUNS=${RUNS:-5}
CLIENTS=8
REQUESTS=16
SAMPLES=$(mktemp)

cleanup() {
  remove_containers load-a load-b
  rm -f "$SAMPLES"
}

# Each meter prints a line "<label> <cpu seconds so far> <memory MiB>" per measured target.
meter_a() {
  echo "A $(container_usage load-a)"
  echo "A_docker $(docker_overhead_usage)"
}
meter_b() {
  echo "B $(container_usage load-b)"
  echo "B_docker $(docker_overhead_usage)"
}
meter_c() {
  echo "C_backend $(backend_usage)"
  echo "C_stack $(stack_usage)"
  echo "C_docker $(docker_overhead_usage)"
}

sample_memory_every_half_second() {
  while true; do
    "$1" >>"$SAMPLES"
    sleep 0.5
  done
}

# run_load <meter> <run> <url> [oha args]
run_load() {
  local meter=$1 run=$2 url=$3 report
  shift 3
  : >"$SAMPLES"

  oha_json -c 1 -n 1 "$@" "$url" >/dev/null   # warm-up
  local before after
  before=$($meter)
  sample_memory_every_half_second "$meter" &
  local sampler=$!
  report=$(oha_json -c "$CLIENTS" -n "$REQUESTS" "$@" "$url")
  kill "$sampler"
  after=$($meter)

  local duration throughput
  duration=$(jq .summary.total <<<"$report")
  throughput=$(jq .summary.requestsPerSec <<<"$report")

  local label cpu_before cpu_after cpu
  while read -r label cpu_before _; do
    cpu_after=$(grep "^$label " <<<"$after" | cut -d' ' -f2)
    cpu=$(calc "$cpu_after - $cpu_before")
    record "$label" "$run" throughput "$throughput" req/s
    record "$label" "$run" cpu_percent "$(calc "$cpu / $duration * 100")" %
    record "$label" "$run" cpu_per_request "$(calc "$cpu / $REQUESTS")" s
    record "$label" "$run" peak_memory "$(grep "^$label " "$SAMPLES" | cut -d' ' -f3 | sort -g | tail -1)" MiB
  done <<<"$before"
}

# The handler must return the same answer in every variant.
check_answer() {
  local answer=$1 expected=${EXPECTED_ANSWER:-}
  EXPECTED_ANSWER=${expected:-$answer}
  [[ $answer == "$EXPECTED_ANSWER" ]] || die "wrong answer: $answer (expected $EXPECTED_ANSWER)"
}

csv_init
remove_containers load-a load-b
platform_login
platform_deploy load
auth="Authorization: Bearer $API_KEY"

for run in $(seq 1 "$RUNS"); do
  echo "run $run of $RUNS"

  start_container load-a "$IMAGE_A" 18080
  wait_until_ok http://127.0.0.1:18080/hello
  check_answer "$(curl -s "http://127.0.0.1:18080/matmul/$MATMUL_N")"
  run_load meter_a "$run" "http://127.0.0.1:18080/matmul/$MATMUL_N"
  remove_containers load-a

  start_container load-b "$IMAGE_B" 18081
  wait_until_ok http://127.0.0.1:18081/hello
  check_answer "$(curl -s "http://127.0.0.1:18081/matmul/$MATMUL_N")"
  run_load meter_b "$run" "http://127.0.0.1:18081/matmul/$MATMUL_N"
  remove_containers load-b

  call_function "$FUNCTION_ID" "matmul/$MATMUL_N"
  check_answer "$BODY"
  run_load meter_c "$run" "$BASE/execute/$FUNCTION_ID/matmul/$MATMUL_N" -H "$auth"
done

platform_delete "$FUNCTION_ID"
echo "results: $CSV"
