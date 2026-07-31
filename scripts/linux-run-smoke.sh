#!/usr/bin/env bash
set -euo pipefail

SMOKE_NAME=${UZEL_SMOKE_NAME:-linux}
SUCCESS_MARKER=${UZEL_SMOKE_SUCCESS_MARKER:-LINUX_RUN_SMOKE_OK}
STARTUP_TIMEOUT_SECONDS=${UZEL_SMOKE_STARTUP_TIMEOUT_SECONDS:-600}
RUNTIME_TIMEOUT_SECONDS=${UZEL_SMOKE_RUNTIME_TIMEOUT_SECONDS:-120}
SHUTDOWN_GRACE_SECONDS=${UZEL_SMOKE_SHUTDOWN_GRACE_SECONDS:-5}
PROFILE_AGGREGATE=0b1fc0eb4909819f48514b81464708b609c5a9872f7e43454d273a87f98e530e
FOLLOW_AGGREGATE=4a3fa9c5e22a53c6fd41474ad55eb262494c6bc4c832833f3222ca3b6e78cd48
NAPD_READY_PATTERN='UZEL_NAPD_READY role=runtime-authority$'

[[ "$STARTUP_TIMEOUT_SECONDS" =~ ^[1-9][0-9]*$ ]] \
  || { echo 'UZEL_SMOKE_STARTUP_TIMEOUT_SECONDS must be a positive integer' >&2; exit 2; }
[[ "$RUNTIME_TIMEOUT_SECONDS" =~ ^[1-9][0-9]*$ ]] \
  || { echo 'UZEL_SMOKE_RUNTIME_TIMEOUT_SECONDS must be a positive integer' >&2; exit 2; }
[[ "$SHUTDOWN_GRACE_SECONDS" =~ ^[1-9][0-9]*$ ]] \
  || { echo 'UZEL_SMOKE_SHUTDOWN_GRACE_SECONDS must be a positive integer' >&2; exit 2; }

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
  echo "Linux runtime smoke failed; logs preserved in $FAILED_DIR" >&2 || true
}

# Invoked through cleanup.
# shellcheck disable=SC2329
stop_child() {
  local pid=$1
  local scope=$2
  local signal_target=$pid
  local watchdog_pid
  local alive=0

  if [[ "$scope" == group ]]; then
    signal_target="-$pid"
  fi

  kill -TERM -- "$signal_target" 2>/dev/null || true
  (
    sleep "$SHUTDOWN_GRACE_SECONDS"
    kill -KILL -- "$signal_target" 2>/dev/null || true
  ) &
  watchdog_pid=$!

  wait "$pid" 2>/dev/null || true

  if [[ "$scope" == group ]] && kill -0 -- "$signal_target" 2>/dev/null; then
    wait "$watchdog_pid" 2>/dev/null || true
  else
    kill "$watchdog_pid" 2>/dev/null || true
    wait "$watchdog_pid" 2>/dev/null || true
  fi

  for _ in $(seq 1 20); do
    if ! kill -0 -- "$signal_target" 2>/dev/null; then
      alive=0
      break
    fi
    alive=1
    sleep 0.1
  done

  if (( alive == 1 )); then
    echo "Linux smoke cleanup could not stop $scope $pid" >&2 || true
    return 1
  fi
}

# Invoked through the EXIT trap.
# shellcheck disable=SC2329
cleanup() {
  local status=$?
  local cleanup_failed=0
  trap - EXIT INT TERM
  if [[ -n "$DEV_PID" ]]; then
    if ! stop_child "$DEV_PID" group; then
      cleanup_failed=1
    fi
  fi
  if [[ -n "$WESTON_PID" ]]; then
    if ! stop_child "$WESTON_PID" process; then
      cleanup_failed=1
    fi
  fi
  if [[ $status -eq 0 && $cleanup_failed -ne 0 ]]; then
    status=1
  fi
  if [[ $status -ne 0 ]]; then
    preserve_failure
  elif [[ -n "$EVIDENCE_DIR" ]]; then
    preserve_logs "$EVIDENCE_DIR"
  fi
  rm -rf "$SMOKE_TMP"
  exit "$status"
}

# Invoked through signal traps; exit status is then captured by cleanup's EXIT trap.
# shellcheck disable=SC2329
handle_interrupt() {
  exit 130
}

# shellcheck disable=SC2329
handle_terminate() {
  exit 143
}

trap cleanup EXIT
trap handle_interrupt INT
trap handle_terminate TERM

export XDG_RUNTIME_DIR="$SMOKE_TMP/runtime"
mkdir -m 700 "$XDG_RUNTIME_DIR"
export XDG_DATA_HOME="$SMOKE_TMP/data"
mkdir -m 700 "$XDG_DATA_HOME"
export WAYLAND_DISPLAY=wayland-uzel
export GDK_BACKEND=wayland
export NO_AT_BRIDGE=1
export UZEL_RUN_HOSTILE_PROBE=1

hostile_markers_are_ordered() {
  local raw_line result_line success_line
  raw_line=$(rg -n '__TAURI_INVOKE_KEY__ expected .* but received invalid-child-key' "$SMOKE_TMP/uzel.log" | tail -1 | cut -d: -f1)
  result_line=$(rg -n '^UZEL_HOSTILE_RESULT_RECEIVED surface=uzel-hostile-egress-generation-3$' "$SMOKE_TMP/uzel.log" | tail -1 | cut -d: -f1)
  success_line=$(rg -n '^UZEL_HOSTILE_PROBE_OK surface=uzel-hostile-egress-generation-3 ' "$SMOKE_TMP/uzel.log" | tail -1 | cut -d: -f1)
  [[ -n "$raw_line" && -n "$result_line" && -n "$success_line" \
    && "$raw_line" -lt "$result_line" && "$result_line" -lt "$success_line" ]]
}

runtime_markers_ready() {
  # napd and Tauri share this log. Concurrent Cargo progress can prefix the
  # daemon's line, so require the exact marker at line end without a start anchor.
  rg -q "$NAPD_READY_PATTERN" "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_SHELL_READY$' "$SMOKE_TMP/uzel.log" \
    && rg -q "^UZEL_FIXTURE_VERIFIED fixture=profile-card aggregate=${PROFILE_AGGREGATE}$" "$SMOKE_TMP/uzel.log" \
    && rg -q "^UZEL_FIXTURE_VERIFIED fixture=follow-list aggregate=${FOLLOW_AGGREGATE}$" "$SMOKE_TMP/uzel.log" \
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
    && hostile_markers_are_ordered
}

report_marker() {
  local name=$1
  local pattern=$2
  local status=missing
  if rg -q "$pattern" "$SMOKE_TMP/uzel.log"; then
    status=present
  fi
  echo "LINUX_SMOKE_MARKER name=$name status=$status" >&2
}

report_marker_state() {
  report_marker napd_ready "$NAPD_READY_PATTERN"
  report_marker shell_ready '^UZEL_SHELL_READY$'
  report_marker profile_fixture "^UZEL_FIXTURE_VERIFIED fixture=profile-card aggregate=${PROFILE_AGGREGATE}$"
  report_marker follow_fixture "^UZEL_FIXTURE_VERIFIED fixture=follow-list aggregate=${FOLLOW_AGGREGATE}$"
  report_marker profile_nap_shell '^UZEL_NAP_SHELL_OK surface=uzel-profile-card-generation-1$'
  report_marker follow_nap_shell '^UZEL_NAP_SHELL_OK surface=uzel-follow-list-generation-2$'
  report_marker profile_accepted '^UZEL_SHELL_ACCEPTED surface=uzel-profile-card-generation-1$'
  report_marker follow_accepted '^UZEL_SHELL_ACCEPTED surface=uzel-follow-list-generation-2$'
  report_marker artifact_response '^UZEL_ARTIFACT_RESPONDED type=identity.getFollows.result$'
  report_marker user_mode '^UZEL_USER_MODE_OK diagnostics=hidden unsafe_controls=absent$'
  report_marker hostile_fixture '^UZEL_FIXTURE_VERIFIED fixture=hostile-egress aggregate=d29a7660cd37118f9d619a16854617b0d44b20d16b3f6a45b9f8e28ce5187a16$'
  report_marker hostile_sentinel '^UZEL_HOSTILE_SENTINEL_READY control=accepted surface=uzel-hostile-egress-generation-3 url=http://127\.0\.0\.1:[0-9]+/uzel-hostile/[0-9]+-[0-9]+$'
  report_marker hostile_nap_shell '^UZEL_NAP_SHELL_OK surface=uzel-hostile-egress-generation-3$'
  report_marker hostile_native_rejection '__TAURI_INVOKE_KEY__ expected .* but received invalid-child-key'
  report_marker hostile_probe '^UZEL_HOSTILE_PROBE_OK surface=uzel-hostile-egress-generation-3 network_denials=13 sentinel_accepts=0 native_calls=0 source_bound=true$'
  if hostile_markers_are_ordered; then
    echo 'LINUX_SMOKE_MARKER name=hostile_order status=present' >&2
  else
    echo 'LINUX_SMOKE_MARKER name=hostile_order status=missing' >&2
  fi
}

SMOKE_STARTED_AT=$SECONDS
echo "LINUX_SMOKE_PHASE phase=startup timeout_seconds=$STARTUP_TIMEOUT_SECONDS"

startup_deadline_expired() {
  (( SECONDS - SMOKE_STARTED_AT >= STARTUP_TIMEOUT_SECONDS ))
}

runtime_deadline_expired() {
  [[ -n "$RUNTIME_STARTED_AT" ]] \
    && (( SECONDS - RUNTIME_STARTED_AT >= RUNTIME_TIMEOUT_SECONDS ))
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
  if startup_deadline_expired; then
    echo "Linux smoke timed out during compositor startup after ${STARTUP_TIMEOUT_SECONDS}s" >&2
    exit 1
  fi
  kill -0 "$WESTON_PID" 2>/dev/null
  sleep 0.25
done
[[ -S "$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY" ]]

if startup_deadline_expired; then
  echo "Linux smoke timed out during compositor startup after ${STARTUP_TIMEOUT_SECONDS}s" >&2
  exit 1
fi

setsid pnpm dev >"$SMOKE_TMP/uzel.log" 2>&1 &
DEV_PID=$!

RUNTIME_STARTED_AT=

while true; do
  if [[ -z "$RUNTIME_STARTED_AT" ]] && startup_deadline_expired; then
    echo "Linux smoke timed out during build/startup before UZEL_SHELL_READY after ${STARTUP_TIMEOUT_SECONDS}s" >&2
    report_marker_state
    exit 1
  fi

  if runtime_deadline_expired; then
    echo "Linux smoke timed out after UZEL_SHELL_READY after ${RUNTIME_TIMEOUT_SECONDS}s" >&2
    report_marker_state
    exit 1
  fi

  if [[ -z "$RUNTIME_STARTED_AT" ]] && rg -q '^UZEL_SHELL_READY$' "$SMOKE_TMP/uzel.log"; then
    RUNTIME_STARTED_AT=$SECONDS
    echo "LINUX_SMOKE_PHASE phase=runtime timeout_seconds=$RUNTIME_TIMEOUT_SECONDS"
  fi

  if runtime_markers_ready; then
    sleep 2
    if runtime_deadline_expired; then
      echo "Linux smoke timed out after UZEL_SHELL_READY after ${RUNTIME_TIMEOUT_SECONDS}s" >&2
      report_marker_state
      exit 1
    fi
    if ! kill -0 "$DEV_PID" 2>/dev/null; then
      echo 'Linux runtime exited after readiness markers' >&2
      report_marker_state
      exit 1
    fi
    echo "$SUCCESS_MARKER daemon=ready shell=ready exact_builds=3 nap_shell=3 shell_accepted=2 artifact=responded source_bound=multi hostile=denied sentinel=zero native=zero user_mode=hidden compositor=weston-headless-gl"
    exit 0
  fi

  if ! kill -0 "$DEV_PID" 2>/dev/null; then
    echo 'Linux runtime exited before readiness markers' >&2
    report_marker_state
    exit 1
  fi

  sleep 0.25
done
