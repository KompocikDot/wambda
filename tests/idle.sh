#!/usr/bin/env bash
# Memory of the freshly started platform with no functions -> results/idle.csv
# Run it right after `docker compose -f compose.prod.yml up -d --build --wait`, before the other tests:
# the database and the object storage grow with every deployed function.
# Usage: bash tests/idle.sh
source "$(dirname "$0")/lib.sh"
TEST=idle

csv_init
record C_backend 1 idle_memory "$(backend_usage | cut -d' ' -f2)" MiB
record C_stack 1 idle_memory "$(stack_usage | cut -d' ' -f2)" MiB
echo "results: $CSV"
