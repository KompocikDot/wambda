#!/usr/bin/env bash
# Memory used by N functions, each called once (the idle state) -> results/memory.csv
#   A - N containers, one per function; A_docker is the growth of the Docker processes (Linux only):
#       the daemon, containerd, a shim per container and a proxy per published port
#   C - N functions in the one backend process; the memory is the growth of the backend container
#       after a clean restart. The fixed cost of the platform (idle_backend, idle_stack) is recorded too.
# The platform must be running: docker compose -f compose.prod.yml up -d --build --wait
# Usage: bash tests/memory.sh
source "$(dirname "$0")/lib.sh"
TEST=memory
RUNS=${RUNS:-5}
COUNTS="1 10 50"   # must be ascending
SETTLE_SECONDS=3   # pause before reading the memory, after the last call
FUNCTIONS=()

cleanup() {
  remove_containers $(docker ps -aq --filter name=memory-a-)
  for id in "${FUNCTIONS[@]}"; do platform_delete "$id"; done
}

# record_total <variant> <run> <n> <MiB>: the total and the share of one function
record_total() {
  record "$1" "$2" "total_n$3" "$4" MiB
  record "$1" "$2" "per_function_n$3" "$(calc "$4 / $3")" MiB
}

# Variant A: n containers, each answers one request.
measure_containers() {
  local run=$1 n=$2 total=0 i memory docker_before docker_after
  docker_before=$(docker_overhead_usage | cut -d' ' -f2)
  for i in $(seq 1 "$n"); do
    start_container "memory-a-$i" "$IMAGE_A" $((19000 + i))
    wait_until_ok "http://127.0.0.1:$((19000 + i))/hello"
  done
  sleep "$SETTLE_SECONDS"
  for i in $(seq 1 "$n"); do
    memory=$(container_usage "memory-a-$i" | cut -d' ' -f2)
    total=$(calc "$total + $memory")
  done
  docker_after=$(docker_overhead_usage | cut -d' ' -f2)
  record_total A "$run" "$n" "$total"
  record_total A_docker "$run" "$n" "$(calc "$docker_after - $docker_before")"
  remove_containers $(docker ps -aq --filter name=memory-a-)
}

# Variant C: functions are added up to n and each is called once; the backend's growth is measured.
measure_platform() {
  local run=$1 before after deployed=0 n id

  restart_backend
  sleep "$SETTLE_SECONDS"
  before=$(backend_usage | cut -d' ' -f2)
  record C_backend "$run" idle_backend "$before" MiB
  record C_stack "$run" idle_stack "$(stack_usage | cut -d' ' -f2)" MiB

  for n in $COUNTS; do
    while ((deployed < n)); do
      platform_deploy "memory-$deployed"
      FUNCTIONS+=("$FUNCTION_ID")
      call_function "$FUNCTION_ID" hello
      [[ $STATUS == 200 ]] || die "call failed: $STATUS $BODY"
      ((++deployed))
    done
    sleep "$SETTLE_SECONDS"
    after=$(backend_usage | cut -d' ' -f2)
    record_total C_backend "$run" "$n" "$(calc "$after - $before")"
  done

  for id in "${FUNCTIONS[@]}"; do platform_delete "$id"; done
  FUNCTIONS=()
}

csv_init
cleanup
platform_login

for run in $(seq 1 "$RUNS"); do
  echo "run $run of $RUNS"
  for n in $COUNTS; do
    measure_containers "$run" "$n"
  done
  measure_platform "$run"
done
echo "results: $CSV"
