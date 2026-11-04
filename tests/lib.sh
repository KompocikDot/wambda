#!/usr/bin/env bash
# Helpers shared by the test scripts (they `source` this file). Run the scripts from the repository root.
# Needs: bash 5, curl, jq, docker, wasmtime, oha (https://github.com/hatoo/oha).
#
# Variants under test (the same handler in each):
#   A - native server in a Docker container (image wambda-bench-native)
#   B - the Wasm component in `wasmtime serve`, in a Docker container (image wambda-bench-wasmtime)
#   C - the Wasm component deployed to the Wambda platform (docker compose stack)
set -euo pipefail
export LC_ALL=C   # decimal point instead of comma in EPOCHREALTIME and in numbers printed by curl

BASE=http://localhost/api
COMPOSE="docker compose -f compose.prod.yml"
DOCKER_SOCKET=${DOCKER_SOCKET:-/var/run/docker.sock}
RESULTS=results
HELLO_WASM=bench/target/wasm32-wasip2/release/guest.wasm
ATTACKS_DIR=bench/attacks/dist
IMAGE_A=wambda-bench-native
IMAGE_B=wambda-bench-wasmtime
MATMUL_N=${MATMUL_N:-1000}   # size of the matrices in /matmul/{n}, chosen so a request takes a few seconds

TEST_EMAIL=tests@wambda.local
TEST_PASSWORD='Test1234!pass'

COOKIES=$(mktemp)     # session cookie of the logged-in test user
BODY_FILE=$(mktemp)   # body of the last response of call_function

# A script can define cleanup() to remove what it started.
on_exit() {
  if declare -F cleanup >/dev/null; then cleanup; fi
  rm -f "$COOKIES" "$BODY_FILE"
}
trap on_exit EXIT

die() {
  echo "error: $*" >&2
  exit 1
}

# ---------- numbers and time ----------

# Arithmetic on decimal numbers: calc '1.5 * 2'
calc() {
  jq -n "$1"
}

now_us() {
  echo "${EPOCHREALTIME/./}"
}

# Milliseconds between two now_us values.
elapsed_ms() {
  calc "($2 - $1) / 1000"
}

# ---------- results (CSV) ----------

# Starts the CSV file of the test named in $TEST: results/<test>.csv (header optional)
csv_init() {
  mkdir -p "$RESULTS"
  CSV="$RESULTS/$TEST.csv"
  echo "${1:-test,variant,run,metric,value,unit}" >"$CSV"
}

# record <variant> <run> <metric> <value> <unit>
record() {
  echo "$TEST,$1,$2,$3,$4,$5" >>"$CSV"
}

# ---------- HTTP ----------

# Polls the url until it answers 2xx. Arguments are passed to curl (the url is the last one).
wait_until_ok() {
  local deadline=$((SECONDS + 60))
  until curl -sf -o /dev/null "$@"; do
    ((SECONDS < deadline)) || die "no answer from ${*: -1}"
  done
}

# Milliseconds taken by one request (curl's time_total). Arguments are passed to curl.
request_ms() {
  local seconds
  seconds=$(curl -s -o /dev/null -w '%{time_total}' "$@")
  calc "$seconds * 1000"
}

# Runs the oha load generator and prints its JSON report; stops the script if a request failed.
oha_json() {
  local report
  report=$(oha --no-tui --output-format json "$@")
  jq -e '.summary.successRate == 1' <<<"$report" >/dev/null || die "failed requests: $report"
  echo "$report"
}

# ---------- containers (variants A and B) ----------

# start_container <name> <image> <host port>
start_container() {
  docker run -d --name "$1" -p "$3:8080" "$2" >/dev/null
}

remove_containers() {
  docker rm -f "$@" >/dev/null 2>&1 || true
}

# "<cpu seconds used so far> <memory in MiB>" of a container, from the Docker API.
# Memory is what `docker stats` shows (without the page cache).
container_usage() {
  curl -s --unix-socket "$DOCKER_SOCKET" "http://localhost/containers/$1/stats?stream=false&one-shot=true" |
    jq -r '"\(.cpu_stats.cpu_usage.total_usage / 1e9) \((.memory_stats.usage - (.memory_stats.stats.inactive_file // 0)) / 1048576)"'
}

# ---------- Docker itself ----------

# "<cpu seconds used so far> <memory in MiB>" of the processes that exist because containers run:
# dockerd, containerd, a containerd-shim per container and a docker-proxy per published port.
# It reads /proc, so it works on Linux only (on other systems it prints "0 0").
docker_overhead_usage() {
  local ticks=0 pages=0 pid fields resident
  for pid in $(pgrep '^(dockerd|containerd|docker-proxy)' || true); do
    { read -r -a fields </proc/"$pid"/stat && read -r _ resident _ </proc/"$pid"/statm; } 2>/dev/null || continue
    ticks=$((ticks + fields[13] + fields[14]))   # utime + stime
    pages=$((pages + resident))
  done
  echo "$(calc "$ticks / $(getconf CLK_TCK)") $(calc "$pages * $(getconf PAGESIZE) / 1048576")"
}

# ---------- platform (variant C) ----------

service_container() {
  $COMPOSE ps -q "$1"
}

# Usage of the backend container alone and of all containers of the stack together (db, s3, nginx, backend).
backend_usage() {
  container_usage "$(service_container backend)"
}

stack_usage() {
  local cpu=0 memory=0 container_cpu container_memory id
  for id in $($COMPOSE ps -q); do
    read -r container_cpu container_memory < <(container_usage "$id")
    cpu=$(calc "$cpu + $container_cpu")
    memory=$(calc "$memory + $container_memory")
  done
  echo "$cpu $memory"
}

restart_backend() {
  $COMPOSE restart backend >/dev/null 2>&1
  wait_until_ok "$BASE/health"
}

# register_and_login <cookie file> <email>: prints the API key of the user
register_and_login() {
  local json="{\"fullName\": \"Tests\", \"email\": \"$2\", \"password\": \"$TEST_PASSWORD\"}"
  curl -s -o /dev/null -c "$1" -H 'Content-Type: application/json' -d "$json" "$BASE/auth/register"
  curl -s -o /dev/null -c "$1" -H 'Content-Type: application/json' -d "$json" "$BASE/auth/login"
  curl -s -b "$1" "$BASE/auth/api-key" | jq -r .apiKey
}

# Logs in the test user (API_KEY and the cookie are used by the functions below) and deletes its old functions.
platform_login() {
  API_KEY=$(register_and_login "$COOKIES" "$TEST_EMAIL")
  platform_delete_all
}

platform_delete_all() {
  local id
  for id in $(curl -s -b "$COOKIES" "$BASE/functions" | jq -r '.[].uuid'); do
    platform_delete "$id"
  done
}

# platform_deploy <name> [wasm file] [memory MB] [timeout s] [envs]
# Sets FUNCTION_ID and DEPLOY_MS (it does not print, so that it can measure itself).
platform_deploy() {
  local response start end
  start=$(now_us)
  response=$(curl -s -b "$COOKIES" \
    -F "name=$1" -F "memoryLimit=${3:-128}" -F "timeout=${4:-30}" -F "envsStringified=${5:-}" \
    -F "wasm=@${2:-$HELLO_WASM}" "$BASE/functions")
  end=$(now_us)
  DEPLOY_MS=$(elapsed_ms "$start" "$end")
  FUNCTION_ID=$(jq -r .uuid <<<"$response")
  [[ $FUNCTION_ID != null ]] || die "deploy of $1 failed: $response"
}

platform_delete() {
  curl -s -o /dev/null -b "$COOKIES" -X DELETE "$BASE/functions/$1"
}

# call_function <function id> <path> [curl args]: sets STATUS, BODY and CALL_SECONDS of the response.
call_function() {
  local id=$1 path=$2 out
  shift 2
  out=$(curl -s -m 60 -o "$BODY_FILE" -w '%{http_code} %{time_total}' \
    -H "Authorization: Bearer $API_KEY" "$@" "$BASE/execute/$id/$path")
  read -r STATUS CALL_SECONDS <<<"$out"
  BODY=$(<"$BODY_FILE")
}
