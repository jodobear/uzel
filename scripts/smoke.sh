#!/usr/bin/env bash
set -euo pipefail

cargo run --quiet -p uzel-napd -- --check | rg '^UZEL_NAPD_READY role=runtime-authority$'
cargo test -p napd live_nmp_refreshes_then_restarts_cache_first_without_a_second_cache -- --ignored
pnpm --filter @uzel/shell build
cargo check --workspace

echo 'SLICE_04_DAEMON_NMP_SMOKE_OK'
