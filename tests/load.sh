#!/usr/bin/env bash
# Throughput and CPU of the variants under load, in two workloads (CLIENTS parallel clients, CPU time read before and after):
#   load.csv  - a few-second CPU-bound request (GET /matmul/{n}); the platform overhead is invisible next to the computation
#   hello.csv - a trivial request (GET /hello) for 10 s; shows the cost of the platform itself per request
#               (throughput only for C: for A and B the load generator is the limit)
# Variant C is measured twice in the same window: the backend alone and the whole stack (backend, nginx, db, s3).
# The Docker daemon, containerd, shims and proxies (Linux only) are measured in the same window as <variant>_docker.
# Peak memory is the highest value among samples taken every half a second during the test.
# Usage: bash tests/load.sh
source "$(dirname "$0")/lib.sh"
TEST=load
RUNS=${RUNS:-5}
CLIENTS=8
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

# run_load <meter> <run> <url> [oha args]; LOAD_LENGTH says how long to load ("-n 16" requests or "-z 10s")
run_load() {
  local meter=$1 run=$2 url=$3 report
  shift 3
  : >"$SAMPLES"

  oha_json -c 1 -n 1 "$@" "$url" >/dev/null   # warm-up
  local before after
  before=$($meter)
  sample_memory_every_half_second "$meter" &
  local sampler=$!
  report=$(oha_json -c "$CLIENTS" $LOAD_LENGTH "$@" "$url")
  kill "$sampler"
  after=$($meter)
  echo "$after" >>"$SAMPLES"   # a short test may end before the first sample

  local duration throughput requests
  duration=$(jq .summary.total <<<"$report")
  throughput=$(jq .summary.requestsPerSec <<<"$report")
  requests=$(jq '[.statusCodeDistribution[]] | add' <<<"$report")

  local label cpu_before cpu_after cpu
  while read -r label cpu_before _; do
    cpu_after=$(grep "^$label " <<<"$after" | cut -d' ' -f2)
    cpu=$(calc "$cpu_after - $cpu_before")
    [[ " $THROUGHPUT_FOR " == *" $label "* ]] && record "$label" "$run" throughput "$throughput" req/s
    record "$label" "$run" cpu_percent "$(calc "$cpu / $duration * 100")" %
    record "$label" "$run" cpu_per_request "$(calc "$cpu / $requests * 1000")" ms
    # the memory of the Docker processes is measured in memory.csv; here it would be an absolute value
    [[ $label == *_docker ]] || record "$label" "$run" peak_memory "$(grep "^$label " "$SAMPLES" | cut -d' ' -f3 | sort -g | tail -1)" MiB
  done <<<"$before"
}

# The handler must return the same answer in every variant.
check_answer() {
  local answer=$1 expected=${EXPECTED_ANSWER:-}
  EXPECTED_ANSWER=${expected:-$answer}
  [[ $answer == "$EXPECTED_ANSWER" ]] || die "wrong answer: $answer (expected $EXPECTED_ANSWER)"
}

# measure_all <test name> <path> <load length> <variants whose throughput is recorded>
# Runs every variant RUNS times and writes results/<test name>.csv
measure_all() {
  TEST=$1
  local path=$2
  LOAD_LENGTH=$3
  THROUGHPUT_FOR=$4
  csv_init
  restart_backend   # the memory of the backend must not depend on the tests that ran before

  for run in $(seq 1 "$RUNS"); do
    echo "$TEST: run $run of $RUNS"

    start_container load-a "$IMAGE_A" 18080
    wait_until_ok http://127.0.0.1:18080/hello
    check_answer "$(curl -s "http://127.0.0.1:18080/$path")"
    run_load meter_a "$run" "http://127.0.0.1:18080/$path"
    remove_containers load-a

    start_container load-b "$IMAGE_B" 18081
    wait_until_ok http://127.0.0.1:18081/hello
    check_answer "$(curl -s "http://127.0.0.1:18081/$path")"
    run_load meter_b "$run" "http://127.0.0.1:18081/$path"
    remove_containers load-b

    call_function "$FUNCTION_ID" "$path"
    check_answer "$BODY"
    run_load meter_c "$run" "$BASE/execute/$FUNCTION_ID/$path" -H "$auth"
  done
  echo "results: $CSV"
}

remove_containers load-a load-b
platform_login
platform_deploy load
auth="Authorization: Bearer $API_KEY"

EXPECTED_ANSWER=
measure_all load "matmul/$MATMUL_N" "-n 16" "A B C_backend"
EXPECTED_ANSWER=
# the load generator is the limit for A and B, so only their CPU per request is comparable
measure_all hello hello "-z 10s" "C_backend"

platform_delete "$FUNCTION_ID"
