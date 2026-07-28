#!/usr/bin/env bash
set -euo pipefail

cargo run -p uzel-napd &
UZEL_NAPD_PID=$!

cleanup() {
  kill "$UZEL_NAPD_PID" 2>/dev/null || true
  wait "$UZEL_NAPD_PID" 2>/dev/null || true
}
trap cleanup EXIT INT TERM

pnpm --dir apps/uzel tauri dev
