#!/usr/bin/env bash
set -euo pipefail

SMOKE_NAME=${UZEL_SMOKE_NAME:-linux}
SUCCESS_MARKER=${UZEL_SMOKE_SUCCESS_MARKER:-LINUX_RUN_SMOKE_OK}
SMOKE_TMP=$(mktemp -d)
FAILED_DIR=${UZEL_SMOKE_ARTIFACT_DIR:-uzel-poc-validated-pack/reports/probes/${SMOKE_NAME}-failed}
EVIDENCE_DIR=${UZEL_SMOKE_EVIDENCE_DIR:-}
WESTON_PID=
DEV_PID=

# Invoked through the trap-called cleanup chain.
# shellcheck disable=SC2329
preserve_logs() {
  local output_dir=$1
  mkdir -p "$output_dir"
  cp "$SMOKE_TMP/weston.log" "$output_dir"/ 2>/dev/null || true
  if [[ -f "$SMOKE_TMP/uzel.log" ]]; then
    sed -E \
      's/(__TAURI_INVOKE_KEY__ expected )[^ ]+( but received invalid-child-key)/\1<redacted>\2/' \
      "$SMOKE_TMP/uzel.log" > "$output_dir/uzel.log"
  fi
}

# Invoked through cleanup.
# shellcheck disable=SC2329
preserve_failure() {
  preserve_logs "$FAILED_DIR"
  echo "Linux runtime smoke failed; logs preserved in $FAILED_DIR" >&2
}

# Invoked through the EXIT/INT/TERM trap.
# shellcheck disable=SC2329
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
  elif [[ -n "$EVIDENCE_DIR" ]]; then
    preserve_logs "$EVIDENCE_DIR"
  fi
  rm -rf "$SMOKE_TMP"
  exit "$status"
}
trap cleanup EXIT INT TERM

export XDG_RUNTIME_DIR="$SMOKE_TMP/runtime"
mkdir -m 700 "$XDG_RUNTIME_DIR"
export XDG_DATA_HOME="$SMOKE_TMP/data"
mkdir -m 700 "$XDG_DATA_HOME"
export WAYLAND_DISPLAY=wayland-uzel
export GDK_BACKEND=wayland
export NO_AT_BRIDGE=1
export UZEL_FIXTURE_RELAY_PORT=$((44000 + ($$ % 10000)))
export UZEL_RUN_HOSTILE_PROBE=1

hostile_markers_are_ordered() {
  local raw_line result_line success_line
  raw_line=$(rg -n '__TAURI_INVOKE_KEY__ expected .* but received invalid-child-key' "$SMOKE_TMP/uzel.log" | tail -1 | cut -d: -f1)
  result_line=$(rg -n '^UZEL_HOSTILE_RESULT_RECEIVED surface=uzel-hostile-egress-generation-3$' "$SMOKE_TMP/uzel.log" | tail -1 | cut -d: -f1)
  success_line=$(rg -n '^UZEL_HOSTILE_PROBE_OK surface=uzel-hostile-egress-generation-3 ' "$SMOKE_TMP/uzel.log" | tail -1 | cut -d: -f1)
  [[ -n "$raw_line" && -n "$result_line" && -n "$success_line" \
    && "$raw_line" -lt "$result_line" && "$result_line" -lt "$success_line" ]]
}

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
    && rg -q '^UZEL_SHELL_READY$' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_FIXTURE_VERIFIED fixture=profile-card aggregate=9ee2d7bfebcd1c56f9c8c0e4641402e2d9ab7bed8c97c5d480cc77c04d5690cc$' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_FIXTURE_VERIFIED fixture=follow-list aggregate=eaf4e565642e5cd055c8f69bea832d39701d04d3a820f5a5753f39bb3651ea9a$' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_NAP_SHELL_OK surface=uzel-profile-card-generation-1$' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_NAP_SHELL_OK surface=uzel-follow-list-generation-2$' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_SHELL_ACCEPTED surface=uzel-profile-card-generation-1$' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_SHELL_ACCEPTED surface=uzel-follow-list-generation-2$' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_ARTIFACT_RESPONDED type=identity.getFollows.result$' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_USER_MODE_OK diagnostics=hidden unsafe_controls=absent$' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_FIXTURE_VERIFIED fixture=hostile-egress aggregate=d29a7660cd37118f9d619a16854617b0d44b20d16b3f6a45b9f8e28ce5187a16$' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_HOSTILE_SENTINEL_READY control=accepted surface=uzel-hostile-egress-generation-3 url=http://127\.0\.0\.1:[0-9]+/uzel-hostile/[0-9]+-[0-9]+$' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_NAP_SHELL_OK surface=uzel-hostile-egress-generation-3$' "$SMOKE_TMP/uzel.log" \
    && rg -q '__TAURI_INVOKE_KEY__ expected .* but received invalid-child-key' "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_HOSTILE_PROBE_OK surface=uzel-hostile-egress-generation-3 network_denials=13 sentinel_accepts=0 native_calls=0 source_bound=true$' "$SMOKE_TMP/uzel.log" \
    && hostile_markers_are_ordered; then
    sleep 2
    kill -0 "$DEV_PID"
    echo "$SUCCESS_MARKER daemon=ready shell=ready exact_builds=3 nap_shell=3 shell_accepted=2 artifact=responded source_bound=multi hostile=denied sentinel=zero native=zero user_mode=hidden compositor=weston-headless-gl"
    exit 0
  fi
  kill -0 "$DEV_PID" 2>/dev/null
  sleep 0.25
done

echo 'Linux runtime smoke timed out waiting for readiness markers' >&2
exit 1
