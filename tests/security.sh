#!/usr/bin/env bash
# Security and isolation tests of the platform (variant C) -> results/security.csv
# Every attack is a separate Wasm module (bench/attacks); it answers 200 when the sandbox held and 403 when it did not.
# Expected statuses: 200 sandbox held, 422 guest trapped or hit a resource limit, 408 killed on timeout,
# 401 no or invalid API key, 404 not the user's function.
# Build the modules first: bash bench/attacks/build.sh
# The platform must be running: docker compose -f compose.prod.yml up -d --build --wait
# Usage: bash tests/security.sh   (exit code 1 when any test fails)
source "$(dirname "$0")/lib.sh"
TEST=security
FAILED=0
OTHER_COOKIES=$(mktemp)
FORBIDDEN_COOKIE='Cookie: session=secret-cookie'   # sent with every call, the platform must not pass it on

cleanup() {
  rm -f "$OTHER_COOKIES"
  platform_delete_all
}

# report <case> <expected> <received> <passed: yes|no> <seconds>
report() {
  printf '%-4s %-48s expected %-14s received %-14s %6.2f s\n' "$([[ $4 == yes ]] && echo ok || echo FAIL)" "$1" "$2" "$3" "$5"
  echo "$1,$2,$3,$4,$5" >>"$CSV"
  [[ $4 == yes ]] || FAILED=$((FAILED + 1))
}

# check <case> <function id> <path> <expected status> [text the body must contain]
check() {
  local passed=no
  call_function "$2" "$3" -H "$FORBIDDEN_COOKIE"
  if [[ $STATUS == "$4" && $BODY == *"${5:-}"* ]]; then passed=yes; fi
  report "$1" "$4" "$STATUS" "$passed" "$CALL_SECONDS"
}

# check_http <case> <expected status> <curl args>: any request, only the status is compared
check_http() {
  local name=$1 expected=$2 out status seconds passed=no
  shift 2
  out=$(curl -s -m 60 -o /dev/null -w '%{http_code} %{time_total}' "$@")
  read -r status seconds <<<"$out"
  if [[ $status == "$expected" ]]; then passed=yes; fi
  report "$name" "$expected" "$status" "$passed" "$seconds"
}

# attack <case> <module> <memory MB> <timeout s> <envs> <expected status> [text the body must contain]
attack() {
  platform_deploy "$1" "$ATTACKS_DIR/$2.wasm" "$3" "$4" "$5"
  check "$1" "$FUNCTION_ID" run "$6" "${7:-}"
}

csv_init "case,expected,received,passed,seconds"
platform_login

echo "--- resource limits and isolation of a function"
attack memory-limit      attack-memory-bomb    64 10 ""            422
attack trap              attack-trap-div-zero  64 10 ""            422
attack infinite-loop     attack-infinite-loop  64  5 ""            408
attack filesystem        attack-fs-escape      64 10 ""            200 "no unexpected access"
attack environment       attack-env-leak       64 10 "Rk9P:YmFy,"  200
attack forbidden-headers attack-header-leak    64 10 ""            200

echo "--- SSRF: requests from a function to internal addresses must be refused"
platform_deploy ssrf "$ATTACKS_DIR/attack-ssrf.wasm" 64 10
ssrf_id=$FUNCTION_ID
for target in 127.0.0.1 localhost localtest.me 169.254.169.254 10.0.0.1 192.168.0.1 100.64.0.1 s3:9000 db:5432; do
  check "ssrf $target" "$ssrf_id" "run?url=http://$target/" 200 "blocked"
done
check "ssrf control: public address is reachable" "$ssrf_id" "run?url=http://example.com/&allow=1" 200

echo "--- fairness: 16 infinite loops must not block other functions"
platform_deploy fairness-hello
hello_id=$FUNCTION_ID
platform_deploy fairness-loop "$ATTACKS_DIR/attack-infinite-loop.wasm" 64 5
loop_id=$FUNCTION_ID
for _ in $(seq 16); do
  curl -s -m 30 -o /dev/null -H "Authorization: Bearer $API_KEY" "$BASE/execute/$loop_id/run" &
done
sleep 1
worst=0
fairness_passed=yes
for _ in $(seq 5); do
  call_function "$hello_id" hello
  [[ $STATUS == 200 ]] || fairness_passed=no
  worst=$(calc "[$worst, $CALL_SECONDS] | max")
  sleep 0.5
done
wait
[[ $(calc "$worst < 1") == true ]] || fairness_passed=no
report "fairness: slowest hello during 16 loops" "200 in <1 s" "200 in ${worst} s" "$fairness_passed" "$worst"

echo "--- isolation between users"
other_key=$(register_and_login "$OTHER_COOKIES" other-tests@wambda.local)
foreign_function="$BASE/functions/$hello_id"
foreign_execute="$BASE/execute/$hello_id/hello"
check_http "execute without API key" 401 "$foreign_execute"
check_http "execute with invalid API key" 401 -H "Authorization: Bearer invalid" "$foreign_execute"
check_http "execute with another user's API key" 404 -H "Authorization: Bearer $other_key" "$foreign_execute"
check_http "read another user's function" 404 -b "$OTHER_COOKIES" "$foreign_function"
check_http "update another user's function" 404 -b "$OTHER_COOKIES" -X PATCH \
  -F name=stolen -F memoryLimit=64 -F timeout=5 -F envsStringified= -F "wasm=@$HELLO_WASM" "$foreign_function"
check_http "delete another user's function" 404 -b "$OTHER_COOKIES" -X DELETE "$foreign_function"
check_http "control: owner executes own function" 200 -H "Authorization: Bearer $API_KEY" "$foreign_execute"

echo
echo "failed tests: $FAILED, results: $CSV"
exit $((FAILED > 0))
