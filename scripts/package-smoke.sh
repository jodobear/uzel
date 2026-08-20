#!/usr/bin/env bash
set -euo pipefail

readonly NIX_FLAGS=(--extra-experimental-features 'nix-command flakes')
readonly NAMPPLETS_REV=e2f69f325a6b45213accdacfcc125e80e0687b4c
readonly TRUSTED_SHELL_REV=eefa9f9d8aa463b833b4d93723dd770f81408889
readonly TRUSTED_SHELL_SHA256=a3e6c18e8724329332bd15a039282a8a0bcf5ec93577b97752f46721df80fba3
readonly NMP_REV=005dc2a5f12aa414961b313d05ebb021934e385c
readonly PRODUCT_INPUTS=(Cargo.lock Cargo.toml package.json pnpm-lock.yaml pnpm-workspace.yaml rust-toolchain.toml apps crates fixtures napplets flake.nix flake.lock)
readonly PACKAGE_ENV=(env -u LD_LIBRARY_PATH -u XDG_DATA_DIRS -u GI_TYPELIB_PATH -u GIO_EXTRA_MODULES -u GTK_PATH -u LIBGL_DRIVERS_PATH -u __EGL_VENDOR_LIBRARY_FILENAMES)

repo_root=$(pwd -P)
launcher_only=${UZEL_PACKAGE_LAUNCHER_ONLY:-0}
mismatch_only=${UZEL_PACKAGE_MISMATCH_ONLY:-0}
[[ "$launcher_only" =~ ^[01]$ && "$mismatch_only" =~ ^[01]$ ]] \
  || { echo 'PACKAGE_SMOKE_FAILED probe modes must be 0 or 1' >&2; exit 2; }
[[ "$launcher_only" == 0 || "$mismatch_only" == 0 ]] \
  || { echo 'PACKAGE_SMOKE_FAILED launcher-only and mismatch-only modes are exclusive' >&2; exit 2; }

git diff HEAD --quiet -- "${PRODUCT_INPUTS[@]}" \
  || { echo 'PACKAGE_SMOKE_FAILED product inputs differ from committed HEAD' >&2; exit 1; }
bash scripts/check-pinned-assets.sh
rg -q "${NAMPPLETS_REV}" Cargo.toml
rg -q "${NAMPPLETS_REV}" Cargo.lock
rg -q "${TRUSTED_SHELL_REV}" apps/uzel/public/trusted-shell/README.md
rg -q "${TRUSTED_SHELL_SHA256}" apps/uzel/public/trusted-shell/trusted-shell-embedded.sha256
rg -q "${NMP_REV}" Cargo.lock

expected_store_path=$(nix "${NIX_FLAGS[@]}" eval --raw .#uzel.outPath)
store_path=${UZEL_PACKAGE_STORE_PATH:-}
if [[ -n "$store_path" ]]; then
  [[ "$store_path" == "$expected_store_path" ]] \
    || { echo 'PACKAGE_SMOKE_FAILED supplied store path does not match the current flake output' >&2; exit 1; }
else
  store_path=$(nix "${NIX_FLAGS[@]}" build --no-link --print-out-paths .#uzel)
fi
[[ "$store_path" == "$expected_store_path" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED realized store path does not match the current flake output' >&2; exit 1; }
git diff HEAD --quiet -- "${PRODUCT_INPUTS[@]}" \
  || { echo 'PACKAGE_SMOKE_FAILED product inputs changed during realization' >&2; exit 1; }
[[ "$store_path" == /nix/store/*-uzel-* ]] \
  || { echo 'PACKAGE_SMOKE_FAILED store path must identify the realized Uzel closure' >&2; exit 1; }
[[ -x "$store_path/bin/uzel" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED public packaged launcher is missing' >&2; exit 1; }
[[ -x "$store_path/libexec/uzel-shell" && -x "$store_path/libexec/uzel-napd" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED private shell or daemon is missing' >&2; exit 1; }
[[ $(find "$store_path/bin" -mindepth 1 -maxdepth 1 | wc -l) -eq 1 \
    && -f "$store_path/bin/uzel" && ! -L "$store_path/bin/uzel" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED only bin/uzel may be public' >&2; exit 1; }
[[ -f "$store_path/share/applications/uzel.desktop" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED desktop file is missing' >&2; exit 1; }
[[ -f "$store_path/share/icons/hicolor/512x512/apps/uzel.png" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED desktop icon is missing' >&2; exit 1; }

closure_size=$(nix "${NIX_FLAGS[@]}" path-info --closure-size "$store_path")
requisites=$(nix-store --query --requisites "$store_path")
references=$(nix-store --query --references "$store_path")
for runtime_ref in webkitgtk gtk+3; do
  printf '%s\n' "$requisites" | rg -F -q -- "-$runtime_ref-" \
    || { echo "PACKAGE_SMOKE_FAILED closure lacks $runtime_ref" >&2; exit 1; }
done
if printf '%s\n' "$requisites" | rg -q '/nix/store/[^/]*-(rustc|cargo|nodejs|pnpm)(-|$)'; then
  echo 'PACKAGE_SMOKE_FAILED closure contains build-only tooling' >&2
  exit 1
fi
printf 'PACKAGE_CLOSURE store_path=%s closure_size=%s\n' "$store_path" "$closure_size"
echo 'PACKAGE_CLOSURE_ASSERTIONS_OK runtime=gtk-webkit build_tools=absent'
printf '%s\n' "$requisites" | sort | sed 's#^#PACKAGE_REQUISITE #'
printf '%s\n' "$references" | sort | sed 's#^#PACKAGE_REFERENCE #'

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
failure_dir=${UZEL_PACKAGE_FAILURE_DIR:-"$repo_root/.artifacts/package-smoke-failure"}
case "$failure_dir" in
  /*) ;;
  *) failure_dir="$repo_root/$failure_dir" ;;
esac
mkdir -m 700 "$tmp/decoy"
printf '%s\n' '#!/usr/bin/env bash' \
  'touch "${UZEL_PACKAGE_DECOY_TOUCHED:?}"' \
  'exit 99' >"$tmp/decoy/uzel-napd"
chmod 700 "$tmp/decoy/uzel-napd"
export UZEL_PACKAGE_DECOY_TOUCHED="$tmp/decoy-executed"

if [[ "$launcher_only" == 0 && "$mismatch_only" == 0 ]]; then
  (
    cd "$tmp"
    PATH="$tmp/decoy:$PATH" \
      UZEL_SMOKE_NAME=package \
      UZEL_SMOKE_LAUNCHER="$store_path/bin/uzel" \
      UZEL_SMOKE_SCRUB_PACKAGE_ENV=1 \
      UZEL_SMOKE_ARTIFACT_DIR="$failure_dir" \
      bash "$repo_root/scripts/linux-run-smoke.sh"
  )
  [[ ! -e "$UZEL_PACKAGE_DECOY_TOUCHED" ]] \
    || { echo 'PACKAGE_SMOKE_FAILED ambient PATH daemon decoy executed' >&2; exit 1; }
fi

wait_for_socket() {
  local socket=$1
  for _ in $(seq 1 80); do
    [[ -S "$socket" ]] && return 0
    sleep 0.1
  done
  return 1
}

COMPAT_RESPONDER_PID=
start_compatible_responder() {
  local socket=$1
  python3 - "$socket" <<'PY' &
import json
import signal
import socket
import struct
import sys

server = socket.socket(socket.AF_UNIX)
server.bind(sys.argv[1])
server.listen(1)
server.settimeout(0.2)
signal.signal(signal.SIGTERM, lambda *_: sys.exit(0))
while True:
    try:
        connection, _ = server.accept()
    except TimeoutError:
        continue
    prefix = connection.recv(4)
    if len(prefix) == 4:
        length = struct.unpack('>I', prefix)[0]
        request = json.loads(connection.recv(length))
        if request == {'operation': 'hello', 'version': 0}:
            response = b'{"result":"hello","version":0}'
            connection.sendall(struct.pack('>I', len(response)) + response)
    connection.close()
PY
  COMPAT_RESPONDER_PID=$!
}

run_preexisting_path_probe() {
  local kind=$1
  local runtime_dir="$tmp/preexisting-$kind-runtime"
  local data_dir="$tmp/preexisting-$kind-data"
  local socket="$runtime_dir/uzel/napd.sock"
  local server_pid= status owner_alive=1 socket_alive identity_before identity_after diagnostic_present success_absent

  mkdir -p "$runtime_dir/uzel" "$data_dir"
  chmod 700 "$runtime_dir" "$runtime_dir/uzel" "$data_dir"
  case "$kind" in
    live-socket)
      start_compatible_responder "$socket"
      server_pid=$COMPAT_RESPONDER_PID
      wait_for_socket "$socket" || { kill -TERM "$server_pid" 2>/dev/null || true; wait "$server_pid" 2>/dev/null || true; return 1; }
      ;;
    stale-socket)
      python3 - "$socket" <<'PY'
import socket
import sys

server = socket.socket(socket.AF_UNIX)
server.bind(sys.argv[1])
server.close()
PY
      ;;
    file)
      : >"$socket"
      ;;
    symlink)
      : >"$runtime_dir/symlink-target"
      ln -s "$runtime_dir/symlink-target" "$socket"
      ;;
    *)
      echo "PACKAGE_SMOKE_FAILED unknown pre-existing path probe: $kind" >&2
      exit 2
      ;;
  esac
  identity_before=$(stat -c '%d:%i:%F' "$socket")
  set +e
  XDG_RUNTIME_DIR="$runtime_dir" XDG_DATA_HOME="$data_dir" \
    UZEL_LAUNCHER_TEST_HOLD_SECONDS=1 \
    "${PACKAGE_ENV[@]}" "$store_path/bin/uzel" >"$tmp/preexisting-$kind.log" 2>&1
  status=$?
  set -e
  socket_alive=0
  diagnostic_present=0
  success_absent=0
  [[ -e "$socket" || -L "$socket" ]] && socket_alive=1
  if [[ $socket_alive -eq 1 ]]; then
    identity_after=$(stat -c '%d:%i:%F' "$socket")
  else
    identity_after=missing
  fi
  rg -q 'refuses a pre-existing runtime socket' "$tmp/preexisting-$kind.log" && diagnostic_present=1
  ! rg -q '^UZEL_SHELL_READY$|PACKAGE_.*_OK' "$tmp/preexisting-$kind.log" && success_absent=1
  if [[ -n "$server_pid" ]]; then
    owner_alive=0
    kill -0 "$server_pid" 2>/dev/null && owner_alive=1
    kill -TERM "$server_pid" 2>/dev/null || true
    wait "$server_pid" 2>/dev/null || true
  fi
  rm -f -- "$socket"
  [[ $status -ne 0 ]] \
    || { echo 'PACKAGE_SMOKE_FAILED launcher accepted a foreign socket' >&2; exit 1; }
  [[ $owner_alive -eq 1 ]] \
    || { echo 'PACKAGE_SMOKE_FAILED launcher terminated the foreign socket owner' >&2; exit 1; }
  [[ $socket_alive -eq 1 ]] \
    || { echo "PACKAGE_SMOKE_FAILED launcher removed the pre-existing $kind path" >&2; exit 1; }
  [[ "$identity_after" == "$identity_before" ]] \
    || { echo "PACKAGE_SMOKE_FAILED launcher replaced the pre-existing $kind path" >&2; exit 1; }
  [[ $diagnostic_present -eq 1 ]] \
    || { echo 'PACKAGE_SMOKE_FAILED foreign socket refusal diagnostic missing' >&2; exit 1; }
  [[ $success_absent -eq 1 ]] \
    || { echo "PACKAGE_SMOKE_FAILED pre-existing $kind path reached a success marker" >&2; exit 1; }
  printf 'PACKAGE_PREEXISTING_PATH_OK kind=%s owner=preserved identity=preserved launcher=refused\n' "$kind"
}

run_postcheck_substitution_probe() {
  local runtime_dir="$tmp/postcheck-runtime"
  local data_dir="$tmp/postcheck-data"
  local socket="$runtime_dir/uzel/napd.sock"
  local launcher_pid= responder_pid= status= identity_before identity_after

  mkdir -p "$runtime_dir/uzel" "$data_dir"
  chmod 700 "$runtime_dir" "$runtime_dir/uzel" "$data_dir"
  XDG_RUNTIME_DIR="$runtime_dir" XDG_DATA_HOME="$data_dir" \
    UZEL_NAPD_TEST_PRE_BIND_DELAY_MS=3000 \
    UZEL_LAUNCHER_TEST_HOLD_SECONDS=1 \
    "${PACKAGE_ENV[@]}" "$store_path/bin/uzel" >"$tmp/postcheck.log" 2>&1 &
  launcher_pid=$!
  for _ in $(seq 1 80); do
    pgrep -P "$launcher_pid" >/dev/null && break
    kill -0 "$launcher_pid" 2>/dev/null || break
    sleep 0.05
  done
  pgrep -P "$launcher_pid" >/dev/null \
    || { cat "$tmp/postcheck.log" >&2; echo 'PACKAGE_SMOKE_FAILED delayed daemon did not start' >&2; exit 1; }
  start_compatible_responder "$socket"
  responder_pid=$COMPAT_RESPONDER_PID
  wait_for_socket "$socket" \
    || { kill -TERM "$launcher_pid" "$responder_pid" 2>/dev/null || true; wait "$launcher_pid" 2>/dev/null || true; wait "$responder_pid" 2>/dev/null || true; return 1; }
  identity_before=$(stat -c '%d:%i:%F' "$socket")
  set +e
  wait "$launcher_pid"
  status=$?
  set -e
  kill -0 "$responder_pid" 2>/dev/null \
    || { cat "$tmp/postcheck.log" >&2; echo 'PACKAGE_SMOKE_FAILED launcher terminated post-check responder' >&2; exit 1; }
  [[ -S "$socket" ]] \
    || { echo 'PACKAGE_SMOKE_FAILED launcher removed post-check responder socket' >&2; exit 1; }
  identity_after=$(stat -c '%d:%i:%F' "$socket")
  kill -TERM "$responder_pid" 2>/dev/null || true
  wait "$responder_pid" 2>/dev/null || true
  rm -f -- "$socket"
  [[ $status -ne 0 ]] \
    || { echo 'PACKAGE_SMOKE_FAILED launcher accepted post-check responder' >&2; exit 1; }
  [[ "$identity_after" == "$identity_before" ]] \
    || { echo 'PACKAGE_SMOKE_FAILED launcher replaced post-check responder socket' >&2; exit 1; }
  rg -q '^Error: ActiveSocket\(' "$tmp/postcheck.log" \
    || { cat "$tmp/postcheck.log" >&2; echo 'PACKAGE_SMOKE_FAILED post-check refusal diagnostic missing' >&2; exit 1; }
  ! rg -q '^UZEL_SHELL_READY$|PACKAGE_.*_OK' "$tmp/postcheck.log" \
    || { echo 'PACKAGE_SMOKE_FAILED post-check substitution reached readiness or success' >&2; exit 1; }
  printf 'PACKAGE_POSTCHECK_SUBSTITUTION_OK owner=preserved identity=preserved launcher=refused readiness=absent\n'
}

run_mismatch_probe() {
  local runtime_dir="$tmp/mismatch-runtime"
  local data_dir="$tmp/mismatch-data"
  local socket="$runtime_dir/uzel/napd.sock"
  local weston_pid responder_pid status responder_status

  mkdir -p "$runtime_dir/uzel" "$data_dir"
  chmod 700 "$runtime_dir" "$runtime_dir/uzel" "$data_dir"
  XDG_RUNTIME_DIR="$runtime_dir" weston \
    --backend=headless --renderer=gl --socket=wayland-uzel-mismatch \
    --idle-time=0 --no-config --log="$tmp/mismatch-weston.log" &
  weston_pid=$!
  wait_for_socket "$runtime_dir/wayland-uzel-mismatch" \
    || { kill -TERM "$weston_pid" 2>/dev/null || true; wait "$weston_pid" 2>/dev/null || true; return 1; }
  python3 - "$socket" <<'PY' >"$tmp/mismatch-responder.log" 2>&1 &
import json
import socket
import struct
import sys

server = socket.socket(socket.AF_UNIX)
server.bind(sys.argv[1])
server.listen(1)
server.settimeout(20)
connection, _ = server.accept()
connection.settimeout(20)
def read_exact(length):
    chunks = []
    remaining = length
    while remaining:
        chunk = connection.recv(remaining)
        if not chunk:
            raise SystemExit('truncated request')
        chunks.append(chunk)
        remaining -= len(chunk)
    return b''.join(chunks)

length = struct.unpack('>I', read_exact(4))[0]
request = json.loads(read_exact(length))
if request != {'operation': 'hello', 'version': 0}:
    raise SystemExit(f'unexpected request: {request!r}')
response = json.dumps({
    'result': 'error',
    'code': 'version_mismatch',
    'detail': 'injected package probe',
}, separators=(',', ':')).encode()
connection.sendall(struct.pack('>I', len(response)) + response)
connection.close()
server.close()
print('MISMATCH_RESPONDER_OK', flush=True)
PY
  responder_pid=$!
  wait_for_socket "$socket" \
    || { kill -TERM "$responder_pid" "$weston_pid" 2>/dev/null || true; wait "$responder_pid" 2>/dev/null || true; wait "$weston_pid" 2>/dev/null || true; return 1; }
  set +e
  timeout --signal=TERM --kill-after=5s 30s env \
    XDG_RUNTIME_DIR="$runtime_dir" XDG_DATA_HOME="$data_dir" \
    WAYLAND_DISPLAY=wayland-uzel-mismatch GDK_BACKEND=wayland NO_AT_BRIDGE=1 \
    "${PACKAGE_ENV[@]}" "$store_path/libexec/uzel-shell" >"$tmp/mismatch-shell.log" 2>&1
  status=$?
  for _ in $(seq 1 40); do
    ! kill -0 "$responder_pid" 2>/dev/null && break
    sleep 0.1
  done
  kill -TERM "$responder_pid" 2>/dev/null || true
  wait "$responder_pid"
  responder_status=$?
  kill -TERM "$weston_pid" 2>/dev/null || true
  wait "$weston_pid" 2>/dev/null || true
  set -e
  [[ $status -ne 0 ]] \
    || { echo 'PACKAGE_SMOKE_FAILED mismatched packaged shell reached success' >&2; exit 1; }
  [[ $status -ne 124 && $responder_status -eq 0 ]] \
    || { cat "$tmp/mismatch-responder.log" "$tmp/mismatch-shell.log" >&2; echo 'PACKAGE_SMOKE_FAILED mismatch responder was not exercised' >&2; exit 1; }
  rg -q '^MISMATCH_RESPONDER_OK$' "$tmp/mismatch-responder.log" \
    || { cat "$tmp/mismatch-responder.log" "$tmp/mismatch-shell.log" >&2; echo 'PACKAGE_SMOKE_FAILED mismatch responder proof missing' >&2; exit 1; }
  rg -q 'UZEL_SHELL_COMPATIBILITY_FAILED daemon refused version_mismatch: injected package probe' "$tmp/mismatch-shell.log" \
    || { cat "$tmp/mismatch-responder.log" "$tmp/mismatch-shell.log" >&2; echo 'PACKAGE_SMOKE_FAILED mismatch diagnostic missing' >&2; exit 1; }
  ! rg -q '^UZEL_SHELL_READY$' "$tmp/mismatch-shell.log" \
    || { echo 'PACKAGE_SMOKE_FAILED mismatched packaged shell reached readiness' >&2; exit 1; }
  printf 'PACKAGE_MISMATCH_OK responder=version-mismatch shell=refused ready=absent\n'
}

LIFECYCLE_PID=

if [[ "$mismatch_only" == 1 ]]; then
  run_mismatch_probe
  printf 'PACKAGE_MISMATCH_ONLY_OK shell=%s webkit=weston compatibility=refused full-smoke=not-run\n' \
    "$store_path/libexec/uzel-shell"
  exit 0
fi

run_preexisting_path_probe live-socket
run_preexisting_path_probe stale-socket
run_preexisting_path_probe file
run_preexisting_path_probe symlink
run_postcheck_substitution_probe

run_lifecycle_probe() {
  local name=$1
  local hold_seconds=${2:-30}
  local runtime_dir="$tmp/$name-runtime"
  local data_dir="$tmp/$name-data"
  local socket="$runtime_dir/uzel/napd.sock"
  local pid=

  mkdir -p "$runtime_dir" "$data_dir"
  chmod 700 "$runtime_dir" "$data_dir"
  # Job control gives the background launcher default INT disposition before
  # its own traps install; otherwise non-interactive Bash inherits SIGINT ignored.
  set -m
  "${PACKAGE_ENV[@]}" XDG_RUNTIME_DIR="$runtime_dir" XDG_DATA_HOME="$data_dir" \
    UZEL_LAUNCHER_TEST_HOLD_SECONDS="$hold_seconds" \
    "$store_path/bin/uzel" >"$tmp/$name.log" 2>&1 &
  pid=$!
  set +m
  if ! wait_for_socket "$socket"; then
    cat "$tmp/$name.log" >&2
    kill -TERM "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
    return 1
  fi
  if [[ $(stat -c '%a' "$runtime_dir/uzel") != 700 ]]; then
    echo 'PACKAGE_SMOKE_FAILED socket parent is not private' >&2
    kill -TERM "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
    return 1
  fi
  if [[ $(stat -c '%a' "$socket") != 600 ]]; then
    echo 'PACKAGE_SMOKE_FAILED socket is not private' >&2
    kill -TERM "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
    return 1
  fi
  LIFECYCLE_PID=$pid
}

run_lifecycle_probe repeat 2
repeat_pid=$LIFECYCLE_PID
wait "$repeat_pid"
[[ ! -e "$tmp/repeat-runtime/uzel/napd.sock" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED repeated launcher left a socket' >&2; exit 1; }
run_lifecycle_probe repeat 2
repeat_pid=$LIFECYCLE_PID
wait "$repeat_pid"
[[ ! -e "$tmp/repeat-runtime/uzel/napd.sock" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED second repeated launcher left a socket' >&2; exit 1; }

run_signal_probe() {
  local signal=$1
  local expected_status=$2
  local name="signal-${signal,,}"
  local pid child status
  local -a children=()

  run_lifecycle_probe "$name"
  pid=$LIFECYCLE_PID
  for _ in $(seq 1 80); do
    mapfile -t children < <(pgrep -P "$pid" || true)
    [[ ${#children[@]} -eq 2 ]] && break
    sleep 0.1
  done
  [[ ${#children[@]} -eq 2 ]] \
    || { echo "PACKAGE_SMOKE_FAILED $signal probe needs daemon and shell child" >&2; exit 1; }
  kill -s "$signal" "$pid"
  set +e
  wait "$pid"
  status=$?
  set -e
  [[ $status -eq $expected_status ]] \
    || { echo "PACKAGE_SMOKE_FAILED $signal launcher status=$status expected=$expected_status" >&2; exit 1; }
  for child in "${children[@]}"; do
    ! kill -0 "$child" 2>/dev/null \
      || { echo "PACKAGE_SMOKE_FAILED $signal child survived pid=$child" >&2; exit 1; }
  done
  [[ ! -e "$tmp/$name-runtime/uzel/napd.sock" ]] \
    || { echo "PACKAGE_SMOKE_FAILED $signal launcher left a socket" >&2; exit 1; }
  printf 'PACKAGE_SIGNAL_OK signal=%s status=%s children=reaped socket=retired\n' \
    "$signal" "$status"
}

run_signal_probe TERM 143
run_signal_probe INT 130

run_daemon_exit_probe() {
  local pid child daemon_pid= shell_pid= status
  local -a children=()

  run_lifecycle_probe daemon-exit
  pid=$LIFECYCLE_PID
  for _ in $(seq 1 80); do
    mapfile -t children < <(pgrep -P "$pid" || true)
    [[ ${#children[@]} -eq 2 ]] && break
    sleep 0.1
  done
  [[ ${#children[@]} -eq 2 ]] \
    || {
      echo 'PACKAGE_SMOKE_FAILED daemon-exit probe needs daemon and shell child' >&2
      kill -TERM "$pid" 2>/dev/null || true
      wait "$pid" 2>/dev/null || true
      exit 1
    }
  for child in "${children[@]}"; do
    case $(readlink -f "/proc/$child/exe") in
      */uzel-napd) daemon_pid=$child ;;
      *) shell_pid=$child ;;
    esac
  done
  [[ -n "$daemon_pid" && -n "$shell_pid" ]] \
    || {
      echo 'PACKAGE_SMOKE_FAILED daemon-exit probe could not identify both children' >&2
      kill -TERM "$pid" 2>/dev/null || true
      wait "$pid" 2>/dev/null || true
      exit 1
    }
  kill -STOP "$shell_pid"
  kill -KILL "$daemon_pid"
  set +e
  wait "$pid"
  status=$?
  set -e
  [[ $status -ne 0 ]] \
    || { echo 'PACKAGE_SMOKE_FAILED daemon-exit launcher succeeded' >&2; exit 1; }
  ! kill -0 "$shell_pid" 2>/dev/null \
    || { echo 'PACKAGE_SMOKE_FAILED daemon-exit shell child survived' >&2; exit 1; }
  [[ ! -e "$tmp/daemon-exit-runtime/uzel/napd.sock" ]] \
    || { echo 'PACKAGE_SMOKE_FAILED daemon-exit launcher left a socket' >&2; exit 1; }
  printf 'PACKAGE_DAEMON_EXIT_OK status=%s hung_shell=killed shell=reaped socket=retired\n' "$status"
}

run_daemon_exit_probe

run_lifecycle_probe concurrent
concurrent_pid=$LIFECYCLE_PID
if "${PACKAGE_ENV[@]}" XDG_RUNTIME_DIR="$tmp/concurrent-runtime" XDG_DATA_HOME="$tmp/concurrent-data" \
  UZEL_LAUNCHER_TEST_HOLD_SECONDS=1 "$store_path/bin/uzel" >"$tmp/concurrent-second.log" 2>&1; then
  echo 'PACKAGE_SMOKE_FAILED concurrent launcher reused private socket' >&2
  kill -TERM "$concurrent_pid" 2>/dev/null || true
  wait "$concurrent_pid" 2>/dev/null || true
  exit 1
fi
kill -0 "$concurrent_pid"
kill -TERM "$concurrent_pid"
wait "$concurrent_pid" || true

run_lifecycle_probe isolated-a
isolated_a_pid=$LIFECYCLE_PID
run_lifecycle_probe isolated-b
isolated_b_pid=$LIFECYCLE_PID
kill -0 "$isolated_a_pid"
kill -0 "$isolated_b_pid"
kill -TERM "$isolated_a_pid" "$isolated_b_pid"
wait "$isolated_a_pid" || true
wait "$isolated_b_pid" || true

printf 'PACKAGE_SOURCE nampplets=%s trusted_shell=%s embedded_sha256=%s nmp=%s lockfiles=unchanged assets=verified\n' \
  "$NAMPPLETS_REV" "$TRUSTED_SHELL_REV" "$TRUSTED_SHELL_SHA256" "$NMP_REV"
if [[ "$launcher_only" == 1 ]]; then
  printf 'PACKAGE_LAUNCHER_ONLY_OK launcher=%s daemon=absolute shell=absolute webkit=not-run\n' \
    "$store_path/bin/uzel"
else
  printf 'PACKAGE_RUNTIME launcher=%s daemon=absolute shell=absolute decoy=not-executed webkit=weston\n' \
    "$store_path/bin/uzel"
  echo 'PACKAGE_SMOKE_OK'
fi
