#!/usr/bin/env bash
set -euo pipefail

readonly NIX_FLAGS=(--extra-experimental-features 'nix-command flakes')
readonly NAMPPLETS_REV=e2f69f325a6b45213accdacfcc125e80e0687b4c
readonly TRUSTED_SHELL_REV=eefa9f9d8aa463b833b4d93723dd770f81408889
readonly TRUSTED_SHELL_SHA256=a3e6c18e8724329332bd15a039282a8a0bcf5ec93577b97752f46721df80fba3
readonly NMP_REV=005dc2a5f12aa414961b313d05ebb021934e385c

repo_root=$(pwd -P)

git diff --exit-code -- Cargo.lock flake.lock pnpm-lock.yaml
git diff --exit-code -- Cargo.toml
bash scripts/check-pinned-assets.sh
rg -q "${NAMPPLETS_REV}" Cargo.toml
rg -q "${NAMPPLETS_REV}" Cargo.lock
rg -q "${TRUSTED_SHELL_REV}" apps/uzel/public/trusted-shell/README.md
rg -q "${TRUSTED_SHELL_SHA256}" apps/uzel/public/trusted-shell/trusted-shell-embedded.sha256
rg -q "${NMP_REV}" Cargo.lock

store_path=${UZEL_PACKAGE_STORE_PATH:-}
if [[ -z "$store_path" ]]; then
  store_path=$(nix "${NIX_FLAGS[@]}" build --no-link --print-out-paths .#uzel)
fi
[[ "$store_path" == /nix/store/*-uzel-* ]] \
  || { echo 'PACKAGE_SMOKE_FAILED store path must identify the realized Uzel closure' >&2; exit 1; }
[[ -x "$store_path/bin/uzel" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED public packaged launcher is missing' >&2; exit 1; }
[[ -x "$store_path/libexec/uzel-shell" && -x "$store_path/libexec/uzel-napd" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED private shell or daemon is missing' >&2; exit 1; }
[[ $(find "$store_path/bin" -maxdepth 1 -type f | wc -l) -eq 1 ]] \
  || { echo 'PACKAGE_SMOKE_FAILED only bin/uzel may be public' >&2; exit 1; }
[[ -f "$store_path/share/applications/uzel.desktop" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED desktop file is missing' >&2; exit 1; }
[[ -f "$store_path/share/icons/hicolor/512x512/apps/uzel.png" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED desktop icon is missing' >&2; exit 1; }

closure_size=$(nix "${NIX_FLAGS[@]}" path-info --closure-size "$store_path")
requisites=$(nix-store --query --requisites "$store_path")
references=$(nix-store --query --references "$store_path")
printf 'PACKAGE_CLOSURE store_path=%s closure_size=%s\n' "$store_path" "$closure_size"
printf '%s\n' "$requisites" | sort | sed 's#^#PACKAGE_REQUISITE #'
printf '%s\n' "$references" | sort | sed 's#^#PACKAGE_REFERENCE #'

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
failure_dir=${UZEL_PACKAGE_FAILURE_DIR:-"$tmp/failure"}
mkdir -m 700 "$tmp/decoy"
printf '%s\n' '#!/usr/bin/env bash' \
  'touch "${UZEL_PACKAGE_DECOY_TOUCHED:?}"' \
  'exit 99' >"$tmp/decoy/uzel-napd"
chmod 700 "$tmp/decoy/uzel-napd"
export UZEL_PACKAGE_DECOY_TOUCHED="$tmp/decoy-executed"

(
  cd "$tmp"
  PATH="$tmp/decoy:$PATH" \
    UZEL_SMOKE_NAME=package \
    UZEL_SMOKE_LAUNCHER="$store_path/bin/uzel" \
    UZEL_SMOKE_ARTIFACT_DIR="$failure_dir" \
    bash "$repo_root/scripts/linux-run-smoke.sh"
)
[[ ! -e "$UZEL_PACKAGE_DECOY_TOUCHED" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED ambient PATH daemon decoy executed' >&2; exit 1; }

wait_for_socket() {
  local socket=$1
  for _ in $(seq 1 80); do
    [[ -S "$socket" ]] && return 0
    sleep 0.1
  done
  return 1
}

LIFECYCLE_PID=

run_lifecycle_probe() {
  local name=$1
  local hold_seconds=${2:-30}
  local runtime_dir="$tmp/$name-runtime"
  local data_dir="$tmp/$name-data"
  local socket="$runtime_dir/uzel/napd.sock"
  local pid=

  mkdir -m 700 "$runtime_dir" "$data_dir"
  XDG_RUNTIME_DIR="$runtime_dir" XDG_DATA_HOME="$data_dir" \
    UZEL_LAUNCHER_TEST_HOLD_SECONDS="$hold_seconds" \
    "$store_path/bin/uzel" >"$tmp/$name.log" 2>&1 &
  pid=$!
  wait_for_socket "$socket" || { cat "$tmp/$name.log" >&2; return 1; }
  [[ $(stat -c '%a' "$runtime_dir/uzel") == 700 ]] \
    || { echo 'PACKAGE_SMOKE_FAILED socket parent is not private' >&2; return 1; }
  [[ $(stat -c '%a' "$socket") == 600 ]] \
    || { echo 'PACKAGE_SMOKE_FAILED socket is not private' >&2; return 1; }
  LIFECYCLE_PID=$pid
}

run_lifecycle_probe repeat 2
repeat_pid=$LIFECYCLE_PID
wait "$repeat_pid"
[[ ! -e "$tmp/repeat-runtime/uzel/napd.sock" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED repeated launcher left a socket' >&2; exit 1; }

run_lifecycle_probe interrupt
interrupt_pid=$LIFECYCLE_PID
kill -TERM "$interrupt_pid"
wait "$interrupt_pid" || true
[[ ! -e "$tmp/interrupt-runtime/uzel/napd.sock" ]] \
  || { echo 'PACKAGE_SMOKE_FAILED interrupted launcher left a socket' >&2; exit 1; }

run_lifecycle_probe concurrent
concurrent_pid=$LIFECYCLE_PID
if XDG_RUNTIME_DIR="$tmp/concurrent-runtime" XDG_DATA_HOME="$tmp/concurrent-data" \
  UZEL_LAUNCHER_TEST_HOLD_SECONDS=1 "$store_path/bin/uzel" >"$tmp/concurrent-second.log" 2>&1; then
  echo 'PACKAGE_SMOKE_FAILED concurrent launcher reused private socket' >&2
  kill -TERM "$concurrent_pid" 2>/dev/null || true
  wait "$concurrent_pid" 2>/dev/null || true
  exit 1
fi
kill -0 "$concurrent_pid"
kill -TERM "$concurrent_pid"
wait "$concurrent_pid" || true

run_lifecycle_probe isolated
isolated_pid=$LIFECYCLE_PID
kill -TERM "$isolated_pid"
wait "$isolated_pid" || true

printf 'PACKAGE_SOURCE nampplets=%s trusted_shell=%s embedded_sha256=%s nmp=%s lockfiles=unchanged assets=verified\n' \
  "$NAMPPLETS_REV" "$TRUSTED_SHELL_REV" "$TRUSTED_SHELL_SHA256" "$NMP_REV"
printf 'PACKAGE_RUNTIME launcher=%s daemon=absolute shell=absolute decoy=not-executed webkit=weston\n' \
  "$store_path/bin/uzel"
echo 'PACKAGE_SMOKE_OK'
