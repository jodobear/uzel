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

reexec_with_nix_group() {
  command -v newgrp >/dev/null \
    || fail 'newgrp missing after Debian login package validation'
  [[ "${UZEL_DEBIAN13_GROUP_REEXEC:-}" != 1 ]] \
    || fail 'newgrp re-exec did not activate nix-users'
  export UZEL_DEBIAN13_GROUP_REEXEC=1
  export UZEL_DEBIAN13_REEXEC_SCRIPT="$ROOT/scripts/debian13-live-test.sh"
  export UZEL_DEBIAN13_REEXEC_MODE="$MODE"
  echo 'DEBIAN13_GROUP_REEXEC group=nix-users action=continue-without-logout'
  if (( ASSUME_YES == 1 )); then
    # Variables intentionally expand in the shell started by newgrp.
    # shellcheck disable=SC2016
    exec newgrp -c \
      'exec bash "$UZEL_DEBIAN13_REEXEC_SCRIPT" "$UZEL_DEBIAN13_REEXEC_MODE" --yes' \
      nix-users
  fi
  # Variables intentionally expand in the shell started by newgrp.
  # shellcheck disable=SC2016
  exec newgrp -c \
    'exec bash "$UZEL_DEBIAN13_REEXEC_SCRIPT" "$UZEL_DEBIAN13_REEXEC_MODE"' \
    nix-users
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
  set +e
  bash scripts/debian13-setup.sh "${SETUP_ARGS[@]}"
  SETUP_STATUS=$?
  set -e
  case "$SETUP_STATUS" in
    0) ;;
    4) reexec_with_nix_group ;;
    *) exit "$SETUP_STATUS" ;;
  esac

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

for command in node pnpm rustc cargo nak weston rg setsid timeout; do
  command -v "$command" >/dev/null || fail "$command missing from pinned Nix shell"
done

STARTUP_TIMEOUT_SECONDS=${UZEL_SMOKE_STARTUP_TIMEOUT_SECONDS:-600}
[[ "$STARTUP_TIMEOUT_SECONDS" =~ ^[1-9][0-9]*$ ]] \
  || fail 'UZEL_SMOKE_STARTUP_TIMEOUT_SECONDS must be a positive integer'

EVIDENCE_DIR=
PREBUILD_LOG=
if [[ "$MODE" == headless ]]; then
  EVIDENCE_ROOT="$ROOT/.artifacts/debian13-live"
  RUN_ID=$(date -u +%Y%m%dT%H%M%SZ)
  mkdir -p "$EVIDENCE_ROOT"
  EVIDENCE_DIR=$(mktemp -d "$EVIDENCE_ROOT/$RUN_ID.XXXXXX")
  PREBUILD_LOG="$EVIDENCE_DIR/prebuild.log"
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
  echo "DEBIAN13_EVIDENCE_CREATED path=$EVIDENCE_DIR"
fi

ACTIVE_CHILD_PID=
ACTIVE_CHILD_GROUP=0

# Invoked through signal traps.
# shellcheck disable=SC2329
forward_signal() {
  local signal=$1
  local status=$2
  trap - INT TERM
  if [[ -n "$ACTIVE_CHILD_PID" ]]; then
    if (( ACTIVE_CHILD_GROUP == 1 )); then
      kill -s "$signal" -- "-$ACTIVE_CHILD_PID" 2>/dev/null || true
    else
      kill -s "$signal" "$ACTIVE_CHILD_PID" 2>/dev/null || true
    fi
    wait "$ACTIVE_CHILD_PID" 2>/dev/null || true
  fi
  exit "$status"
}

trap 'forward_signal INT 130' INT
trap 'forward_signal TERM 143' TERM

record_prebuild() {
  local message=$1
  printf '%s\n' "$message"
  if [[ -n "$PREBUILD_LOG" ]]; then
    printf '%s\n' "$message" >> "$PREBUILD_LOG"
  fi
}

STARTUP_STARTED_AT=$SECONDS

run_startup_step() {
  local step=$1
  shift
  local elapsed=$((SECONDS - STARTUP_STARTED_AT))
  local remaining=$((STARTUP_TIMEOUT_SECONDS - elapsed))
  local step_status

  if (( remaining <= 0 )); then
    record_prebuild "DEBIAN13_STARTUP_TIMEOUT step=$step limit_seconds=$STARTUP_TIMEOUT_SECONDS"
    return 124
  fi

  record_prebuild "DEBIAN13_STARTUP_STEP_BEGIN step=$step remaining_seconds=$remaining"
  ACTIVE_CHILD_GROUP=1
  if [[ -n "$PREBUILD_LOG" ]]; then
    setsid timeout --signal=TERM --kill-after=30s "${remaining}s" "$@" \
      > >(tee -a "$PREBUILD_LOG") 2>&1 &
  else
    setsid timeout --signal=TERM --kill-after=30s "${remaining}s" "$@" &
  fi
  ACTIVE_CHILD_PID=$!
  set +e
  wait "$ACTIVE_CHILD_PID"
  step_status=$?
  set -e
  ACTIVE_CHILD_PID=
  ACTIVE_CHILD_GROUP=0

  if [[ $step_status -eq 124 || $step_status -eq 137 ]]; then
    record_prebuild "DEBIAN13_STARTUP_TIMEOUT step=$step limit_seconds=$STARTUP_TIMEOUT_SECONDS"
    return 124
  fi
  if [[ $step_status -ne 0 ]]; then
    record_prebuild "DEBIAN13_STARTUP_STEP_ERROR step=$step status=$step_status"
    return "$step_status"
  fi
  record_prebuild "DEBIAN13_STARTUP_STEP_OK step=$step"
}

echo "DEBIAN13_TOOLCHAIN node=$(node --version) pnpm=$(pnpm --version) rustc=$(rustc --version | tr ' ' '-') tauri=$(cargo tauri --version | tr ' ' '-') weston=$(weston --version | tr ' ' '-')"
run_startup_step pnpm-install pnpm install --frozen-lockfile
run_startup_step pinned-assets bash scripts/check-pinned-assets.sh
echo 'DEBIAN13_BUILD_BEGIN workspace=locked'
export CARGO_INCREMENTAL=0
export CARGO_PROFILE_DEV_DEBUG=0
run_startup_step shell-build pnpm --filter @uzel/shell build
run_startup_step workspace-build cargo build --workspace --locked
echo 'DEBIAN13_BUILD_OK workspace=locked'

if [[ "$MODE" == interactive ]]; then
  [[ -n "${WAYLAND_DISPLAY:-}${DISPLAY:-}" ]] \
    || fail 'interactive mode requires graphical desktop session; use headless mode over SSH'
  echo 'DEBIAN13_INTERACTIVE_READY close Uzel window or press Ctrl-C to stop daemon and fixture relay'
  exec pnpm dev
fi

export UZEL_SMOKE_NAME=debian13-live
export UZEL_SMOKE_SUCCESS_MARKER=DEBIAN13_LIVE_SMOKE_OK
export UZEL_SMOKE_ARTIFACT_DIR="$EVIDENCE_DIR/failure"
export UZEL_SMOKE_EVIDENCE_DIR="$EVIDENCE_DIR"
STARTUP_ELAPSED=$((SECONDS - STARTUP_STARTED_AT))
STARTUP_REMAINING=$((STARTUP_TIMEOUT_SECONDS - STARTUP_ELAPSED))
if (( STARTUP_REMAINING <= 0 )); then
  record_prebuild "DEBIAN13_STARTUP_TIMEOUT step=runtime-start limit_seconds=$STARTUP_TIMEOUT_SECONDS"
  exit 124
fi
export UZEL_SMOKE_STARTUP_TIMEOUT_SECONDS=$STARTUP_REMAINING

ACTIVE_CHILD_GROUP=0
bash scripts/linux-run-smoke.sh > >(tee "$EVIDENCE_DIR/run.log") 2>&1 &
ACTIVE_CHILD_PID=$!
set +e
wait "$ACTIVE_CHILD_PID"
SMOKE_STATUS=$?
set -e
ACTIVE_CHILD_PID=
trap - INT TERM
[[ $SMOKE_STATUS -eq 0 ]] || exit "$SMOKE_STATUS"
echo "DEBIAN13_EVIDENCE_OK path=$EVIDENCE_DIR"
