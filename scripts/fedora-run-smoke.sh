#!/usr/bin/env bash
set -euo pipefail

SMOKE_TMP=$(mktemp -d)
FAILED_DIR=${UZEL_SMOKE_ARTIFACT_DIR:-uzel-poc-validated-pack/reports/probes/slice-01-fedora-failed}
WESTON_PID=
DEV_PID=

preserve_failure() {
  mkdir -p "$FAILED_DIR"
  cp "$SMOKE_TMP"/*.log "$FAILED_DIR"/ 2>/dev/null || true
  echo "Fedora smoke failed; logs preserved in $FAILED_DIR" >&2
}

cleanup() {
  local status=$?
  trap - EXIT INT TERM
  if [[ -n "$DEV_PID" ]]; then
    kill -- -"$DEV_PID" 2>/dev/null || true
    wait "$DEV_PID" 2>/dev/null || true
  fi
  if [[ -n "$WESTON_PID" ]]; then
    kill "$WESTON_PID" 2>/dev/null || true
    wait "$WESTON_PID" 2>/dev/null || true
  fi
  if [[ $status -ne 0 ]]; then
    preserve_failure
  fi
  rm -rf "$SMOKE_TMP"
  exit "$status"
}
trap cleanup EXIT INT TERM

export XDG_RUNTIME_DIR="$SMOKE_TMP/runtime"
mkdir -m 700 "$XDG_RUNTIME_DIR"
export WAYLAND_DISPLAY=wayland-uzel
export GDK_BACKEND=wayland
export NO_AT_BRIDGE=1

weston \
  --backend=headless \
  --renderer=gl \
  --socket="$WAYLAND_DISPLAY" \
  --idle-time=0 \
  --no-config \
  --log="$SMOKE_TMP/weston.log" &
WESTON_PID=$!

for _ in $(seq 1 80); do
  [[ -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ]] && break
  kill -0 "$WESTON_PID" 2>/dev/null
  sleep 0.25
done
[[ -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ]]

setsid pnpm dev >"$SMOKE_TMP/uzel.log" 2>&1 &
DEV_PID=$!

for _ in $(seq 1 240); do
  if rg -q '^UZEL_NAPD_READY role=runtime-authority$' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_SHELL_READY$' "$SMOKE_TMP/uzel.log"; then
    sleep 2
    kill -0 "$DEV_PID"
    echo 'FEDORA_RUN_SMOKE_OK daemon=ready shell=ready compositor=weston-headless-gl'
    exit 0
  fi
  kill -0 "$DEV_PID" 2>/dev/null
  sleep 0.25
done

echo 'Fedora smoke timed out waiting for readiness markers' >&2
exit 1
