#!/usr/bin/env bash
set -euo pipefail

MODE=
ASSUME_YES=0
ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

fail() {
  echo "DEBIAN13_LIVE_ERROR $*" >&2
  exit 1
}

for argument in "$@"; do
  case "$argument" in
    headless|interactive)
      [[ -z "$MODE" ]] || fail 'choose exactly one of headless or interactive'
      MODE=$argument
      ;;
    --yes) ASSUME_YES=1 ;;
    *) fail 'usage: scripts/debian13-live-test.sh [headless|interactive] [--yes]' ;;
  esac
done
MODE=${MODE:-headless}

if [[ "${UZEL_DEBIAN13_NIX_SHELL:-}" != 1 ]]; then
  SETUP_ARGS=(--install)
  (( ASSUME_YES == 1 )) && SETUP_ARGS+=(--yes)
  bash scripts/debian13-setup.sh "${SETUP_ARGS[@]}"

  echo 'DEBIAN13_NIX_PLAN source=flake.lock tools=node,pnpm,rust,cargo,tauri,nak,weston,webkitgtk,mesa,ripgrep'
  NIX_NEEDS_APPROVAL=1
  if nix --extra-experimental-features 'nix-command flakes' \
    flake metadata --offline --no-write-lock-file . >/dev/null 2>&1; then
    NIX_PLAN=$(nix --extra-experimental-features 'nix-command flakes' \
      build --offline .#devShells.x86_64-linux.default --dry-run --no-link 2>&1)
    printf '%s\n' "$NIX_PLAN"
    NIX_NEEDS_APPROVAL=0
    [[ "$NIX_PLAN" == *'will be fetched'* ]] && NIX_NEEDS_APPROVAL=1
    while IFS= read -r plan_line; do
      if [[ "$plan_line" =~ ^[[:space:]]+/nix/store/.*\.drv$ \
        && ! "$plan_line" =~ -nix-shell\.drv$ ]]; then
        NIX_NEEDS_APPROVAL=1
      fi
    done <<< "$NIX_PLAN"
  else
    echo 'DEBIAN13_NIX_INPUTS status=missing source=flake.lock'
  fi
  if (( NIX_NEEDS_APPROVAL == 1 )); then
    if (( ASSUME_YES == 0 )); then
      [[ -t 0 ]] || fail 'Nix dependency approval requires interactive terminal; rerun with --yes for unattended install'
      read -r -p 'Realize listed locked Nix dependencies? [y/N] ' approval
      case "$approval" in
        y|Y|yes|YES) ;;
        *) echo 'DEBIAN13_LIVE_CANCELLED no_nix_dependencies_realized'; exit 3 ;;
      esac
    fi
  else
    echo 'DEBIAN13_NIX_NO_CHANGES closure=ready shell_derivation=ephemeral'
  fi

  INNER_ARGS=("$MODE")
  (( ASSUME_YES == 1 )) && INNER_ARGS+=(--yes)
  exec nix --extra-experimental-features 'nix-command flakes' \
    develop --command env UZEL_DEBIAN13_NIX_SHELL=1 \
    bash scripts/debian13-live-test.sh "${INNER_ARGS[@]}"
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
mkdir -p "$EVIDENCE_ROOT"
EVIDENCE_DIR=$(mktemp -d "$EVIDENCE_ROOT/$RUN_ID.XXXXXX")
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
