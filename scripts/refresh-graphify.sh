#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CACHE="$ROOT/graphify-out/cache"

cleanup_cache() {
  rm -rf -- "$CACHE"
}

trap cleanup_cache EXIT
cleanup_cache
cd "$ROOT"
graphify update .
cleanup_cache
python3 scripts/normalize-graphify-output.py
python3 scripts/check-graphify-portability.py
