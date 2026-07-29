#!/usr/bin/env bash
set -euo pipefail

IMAGE_TAG=${UZEL_DEBIAN_SMOKE_IMAGE:-uzel-debian-toolchain:local}

cleanup() {
  local status=$?
  trap - EXIT
  podman image rm "$IMAGE_TAG" >/dev/null 2>&1 || true
  exit "$status"
}
trap cleanup EXIT

podman build \
  --file Containerfile.debian \
  --target toolchain \
  --tag "$IMAGE_TAG" \
  .

podman run --rm \
  --volume "$PWD:/workspace:O" \
  --workdir /workspace \
  --env CARGO_INCREMENTAL=0 \
  --env CARGO_PROFILE_DEV_DEBUG=0 \
  "$IMAGE_TAG" \
  sh -c '
    node --version
    pnpm --version
    rustc --version
    pnpm install --frozen-lockfile
    pnpm --filter @uzel/shell build
    cargo test -p uzel --locked
    cargo build --workspace --locked
  '

echo "DEBIAN_BUILD_SMOKE_OK image=$IMAGE_TAG source_mount=overlay"
