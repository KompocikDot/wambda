#!/usr/bin/env bash
# Prints the environment of the test run: bash tests/env.sh > results/env.txt
echo "date: $(date -u +%FT%TZ)"
echo "host: $(hostname)"
echo "git: $(git rev-parse --short HEAD), $(git status --porcelain | wc -l) modified file(s)"
echo "os: $(. /etc/os-release && echo "$PRETTY_NAME"), kernel $(uname -r), $(uname -m)"
lscpu | grep -E "Model name|^CPU\(s\)" || true
free -h | head -2
echo "docker: $(docker --version)"
echo "compose: $(docker compose version --short)"
echo "wasmtime: $(wasmtime --version 2>&1)"
echo "rustc: $(rustc --version 2>&1)"
echo "oha: $(oha --version 2>&1)"
docker image ls --format 'image: {{.Repository}}:{{.Tag}} {{.ID}} {{.Size}}' | grep -E "faasm|wambda|postgres|rustfs|nginx" || true
