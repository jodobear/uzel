#!/usr/bin/env bash
set -euo pipefail

UZEL_NAPD_PID=

cleanup() {
  if [[ -n "$UZEL_NAPD_PID" ]]; then
    kill "$UZEL_NAPD_PID" 2>/dev/null || true
    wait "$UZEL_NAPD_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

NAPD_RELAY_ARGS=(
  --indexer-relay wss://purplepag.es
  --app-relay wss://purplepag.es
  --app-relay wss://nos.lol
)

cargo run -p uzel-napd -- \
  --live \
  "${NAPD_RELAY_ARGS[@]}" &
UZEL_NAPD_PID=$!

pnpm --dir apps/uzel tauri dev
