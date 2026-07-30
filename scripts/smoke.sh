#!/usr/bin/env bash
set -euo pipefail

cargo run --quiet -p uzel-napd -- --check | rg '^UZEL_NAPD_READY role=runtime-authority$'
LIVE_IDENTITY_TEST=runner::tests::public_identity_profile_follows_and_picture_cross_only_native_providers
LIVE_IDENTITY_OUTPUT=$(cargo test -p napd "$LIVE_IDENTITY_TEST" -- --ignored --exact 2>&1)
printf '%s\n' "$LIVE_IDENTITY_OUTPUT"
rg -q '^running 1 test$' <<<"$LIVE_IDENTITY_OUTPUT"
rg -q "^test ${LIVE_IDENTITY_TEST} \.\.\. ok$" <<<"$LIVE_IDENTITY_OUTPUT"
cargo test -p napd daemon_routes_inc_delivery_to_the_other_exact_surface
pnpm --filter @uzel/shell build
cargo check --workspace

echo 'SLICE_05_COMPOSED_DEMO_SMOKE_OK'
