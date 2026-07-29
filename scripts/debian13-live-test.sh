#!/usr/bin/env bash
set -euo pipefail

MODE=${1:-headless}
ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

fail() {
  echo "DEBIAN13_LIVE_ERROR $*" >&2
  exit 1
}

[[ "$MODE" == headless || "$MODE" == interactive ]] \
  || fail 'usage: scripts/debian13-live-test.sh [headless|interactive]'

if [[ "${UZEL_DEBIAN13_NIX_SHELL:-}" != 1 ]]; then
  bash scripts/debian13-setup.sh --check
  exec nix --extra-experimental-features 'nix-command flakes' \
    develop --command env UZEL_DEBIAN13_NIX_SHELL=1 \
    bash scripts/debian13-live-test.sh "$MODE"
fi

for command in node pnpm rustc cargo nak weston rg; do
  command -v "$command" >/dev/null || fail "$command missing from pinned Nix shell"
done

echo "DEBIAN13_TOOLCHAIN node=$(node --version) pnpm=$(pnpm --version) rustc=$(rustc --version | tr ' ' '-') tauri=$(cargo tauri --version | tr ' ' '-') weston=$(weston --version | tr ' ' '-')"
pnpm install --frozen-lockfile
bash scripts/check-pinned-assets.sh
echo 'DEBIAN13_BUILD_BEGIN workspace=locked'
pnpm --filter @uzel/shell build
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 cargo build --workspace --locked
echo 'DEBIAN13_BUILD_OK workspace=locked'

if [[ "$MODE" == interactive ]]; then
  [[ -n "${WAYLAND_DISPLAY:-}${DISPLAY:-}" ]] \
    || fail 'interactive mode requires graphical desktop session; use headless mode over SSH'
  echo 'DEBIAN13_INTERACTIVE_READY close Uzel window or press Ctrl-C to stop daemon and fixture relay'
  exec pnpm dev
fi

EVIDENCE_ROOT="$ROOT/.artifacts/debian13-live"
RUN_ID=$(date -u +%Y%m%dT%H%M%SZ)
EVIDENCE_DIR="$EVIDENCE_ROOT/$RUN_ID"
mkdir -p "$EVIDENCE_DIR"
{
  echo "git_commit=$(git rev-parse HEAD)"
  echo "kernel=$(uname -srmo)"
  echo "nix=$(nix --version)"
  echo "node=$(node --version)"
  echo "pnpm=$(pnpm --version)"
  echo "rustc=$(rustc --version)"
  echo "tauri=$(cargo tauri --version)"
  echo "webkitgtk=$(pkg-config --modversion webkit2gtk-4.1)"
  echo "weston=$(weston --version)"
} > "$EVIDENCE_DIR/environment.txt"

export UZEL_SMOKE_NAME=debian13-live
export UZEL_SMOKE_SUCCESS_MARKER=DEBIAN13_LIVE_SMOKE_OK
export UZEL_SMOKE_ARTIFACT_DIR="$EVIDENCE_DIR/failure"
export UZEL_SMOKE_EVIDENCE_DIR="$EVIDENCE_DIR"

set -o pipefail
bash scripts/linux-run-smoke.sh 2>&1 | tee "$EVIDENCE_DIR/run.log"
echo "DEBIAN13_EVIDENCE_OK path=$EVIDENCE_DIR"
