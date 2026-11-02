#!/usr/bin/env bash
# Prints the environment of the test run (append to results/env-<machine>.txt).
# Usage: bash tests/collect_env.sh > results/env-sut.txt
echo "date: $(date -u +%FT%TZ)"
echo "host: $(hostname)"
echo "git: $(git rev-parse --short HEAD) $(git status --porcelain | wc -l) modified file(s)"
echo "os: $(. /etc/os-release && echo "$PRETTY_NAME"), kernel $(uname -r), $(uname -m)"
lscpu | grep -E "Model name|^CPU\(s\)|Thread|Socket|MHz" || true
free -h | head -2
echo "docker: $(docker --version)"
echo "compose: $(docker compose version --short)"
echo "wasmtime: $(wasmtime --version 2>&1)"
echo "rustc: $(rustc --version 2>&1)"
echo "uv: $(uv --version 2>&1)"
echo "k6 image: $(docker image inspect grafana/k6 --format '{{index .RepoDigests 0}}' 2>&1)"
docker image ls --format 'image: {{.Repository}}:{{.Tag}} {{.ID}} {{.Size}}' | grep -E "faasm|wambda|postgres|rustfs|nginx" || true
echo "sysctl: somaxconn=$(sysctl -n net.core.somaxconn) tcp_tw_reuse=$(sysctl -n net.ipv4.tcp_tw_reuse) ip_local_port_range='$(sysctl -n net.ipv4.ip_local_port_range)'"
echo "ulimit -n: $(ulimit -n)"
echo "target host: ${TARGET_HOST:-localhost}"
