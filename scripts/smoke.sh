#!/usr/bin/env bash
set -euo pipefail

cargo run --quiet -p uzel-napd -- --check | rg '^UZEL_NAPD_READY role=runtime-authority$'
pnpm --filter @uzel/shell build
cargo check --workspace

echo 'SLICE_02_BUILD_SMOKE_OK'
