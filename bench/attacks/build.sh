#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

mkdir -p dist
for feature in attack-memory-bomb attack-trap-div-zero attack-infinite-loop attack-fs-escape attack-env-leak attack-header-leak attack-ssrf; do
    cargo build -q --release --target wasm32-wasip2 --features "$feature"
    cp ../target/wasm32-wasip2/release/attacks.wasm "dist/$feature.wasm"
done
