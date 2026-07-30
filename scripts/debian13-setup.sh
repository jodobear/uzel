#!/usr/bin/env bash
set -euo pipefail

MODE=
ASSUME_YES=0
ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"
CURRENT_USER=$(id -un)

fail() {
  echo "DEBIAN13_SETUP_ERROR $*" >&2
  exit 1
}

for argument in "$@"; do
  case "$argument" in
    --check|--install)
      [[ -z "$MODE" ]] || fail 'choose exactly one of --check or --install'
      MODE=$argument
      ;;
    --yes)
      ASSUME_YES=1
      ;;
    *) fail 'usage: scripts/debian13-setup.sh [--check|--install] [--yes]' ;;
  esac
done
MODE=${MODE:---check}
[[ "$MODE" == --install || $ASSUME_YES -eq 0 ]] \
  || fail '--yes is valid only with --install'
[[ -r /etc/os-release ]] || fail '/etc/os-release missing'

# shellcheck disable=SC1091
source /etc/os-release
[[ "${ID:-}" == debian && "${VERSION_ID:-}" == 13 ]] \
  || fail "requires Debian 13; found ID=${ID:-unknown} VERSION_ID=${VERSION_ID:-unknown}"
[[ "$(uname -m)" == x86_64 ]] \
  || fail "current flake supports x86_64-linux only; found $(uname -m)"
[[ $EUID -ne 0 ]] || fail 'run as normal desktop user, not root'
command -v dpkg-query >/dev/null || fail 'dpkg-query missing on Debian host'
command -v getent >/dev/null || fail 'getent missing on Debian host'
command -v timeout >/dev/null || fail 'timeout missing on Debian host'

APT_PACKAGES=(git login nix-setup-systemd)
MISSING_APT=()
for package in "${APT_PACKAGES[@]}"; do
  if [[ "$(dpkg-query -W -f='${Status}' "$package" 2>/dev/null || true)" == 'install ok installed' ]]; then
    echo "DEBIAN13_DEPENDENCY type=apt name=$package status=installed"
  else
    echo "DEBIAN13_DEPENDENCY type=apt name=$package status=missing"
    MISSING_APT+=("$package")
  fi
done

CONFIGURED_GROUPS=$(id -nG "$CURRENT_USER" 2>/dev/null || true)
ACTIVE_GROUPS=$(id -nG)
GROUP_CONFIGURED=0
GROUP_ACTIVE=0
[[ " $CONFIGURED_GROUPS " == *" nix-users "* ]] && GROUP_CONFIGURED=1
[[ " $ACTIVE_GROUPS " == *" nix-users "* ]] && GROUP_ACTIVE=1
echo "DEBIAN13_DEPENDENCY type=group name=nix-users configured=$GROUP_CONFIGURED active=$GROUP_ACTIVE"
echo 'DEBIAN13_DEPENDENCY type=nix-closure status=resolved-by-flake tools=node,pnpm,rust,cargo,tauri,nak,weston,webkitgtk,mesa,ripgrep'

refresh_nix_builder_state() {
  local builder_number builder_user
  NIX_BUILD_GROUP_PRESENT=0
  NIX_BUILD_USERS_READY=0
  NIX_BUILDERS_READY=0
  getent group nixbld >/dev/null 2>&1 && NIX_BUILD_GROUP_PRESENT=1
  for builder_number in {1..10}; do
    builder_user="nixbld$builder_number"
    if id "$builder_user" >/dev/null 2>&1 \
      && [[ "$(id -gn "$builder_user" 2>/dev/null || true)" == nixbld ]]; then
      (( NIX_BUILD_USERS_READY += 1 ))
    fi
  done
  if (( NIX_BUILD_GROUP_PRESENT == 1 && NIX_BUILD_USERS_READY == 10 )); then
    NIX_BUILDERS_READY=1
  fi
}

print_nix_builder_state() {
  echo "DEBIAN13_DEPENDENCY type=nix-builders group=nixbld present=$NIX_BUILD_GROUP_PRESENT users=$NIX_BUILD_USERS_READY/10"
}

refresh_nix_builder_state
print_nix_builder_state

command -v systemctl >/dev/null \
  || fail 'systemctl missing; nix-setup-systemd requires a systemd host'
systemctl show --property=Version --value >/dev/null 2>&1 \
  || fail 'systemd system manager unreachable; boot Debian with systemd before using nix-setup-systemd'

NIX_DAEMON_UNIT=nix-daemon.socket
NIX_DAEMON_SOCKET=/nix/var/nix/daemon-socket/socket

refresh_nix_daemon_state() {
  NIX_DAEMON_LOAD=$(systemctl show "$NIX_DAEMON_UNIT" --property=LoadState --value 2>/dev/null || true)
  NIX_DAEMON_ACTIVE=$(systemctl is-active "$NIX_DAEMON_UNIT" 2>/dev/null || true)
  NIX_DAEMON_ENABLED=$(systemctl is-enabled "$NIX_DAEMON_UNIT" 2>/dev/null || true)
  NIX_DAEMON_SOCKET_STATE=missing
  NIX_DAEMON_CLIENT=not-tested
  NIX_DAEMON_READY=0
  if [[ -S "$NIX_DAEMON_SOCKET" ]]; then
    NIX_DAEMON_SOCKET_STATE=visible
  elif [[ -d "${NIX_DAEMON_SOCKET%/*}" && ! -x "${NIX_DAEMON_SOCKET%/*}" ]]; then
    NIX_DAEMON_SOCKET_STATE=restricted
  fi

  if [[ "$NIX_DAEMON_LOAD" == loaded \
    && "$NIX_DAEMON_ACTIVE" == active \
    && "$NIX_DAEMON_ENABLED" == enabled ]]; then
    NIX_DAEMON_READY=1
    if (( GROUP_ACTIVE == 1 )) && command -v nix >/dev/null; then
      if timeout --foreground --signal=TERM --kill-after=2s 10s \
        nix --extra-experimental-features nix-command \
        store info --store daemon >/dev/null 2>&1; then
        NIX_DAEMON_CLIENT=ok
      else
        NIX_DAEMON_CLIENT=failed
        NIX_DAEMON_READY=0
      fi
    fi
  fi
}

print_nix_daemon_state() {
  echo "DEBIAN13_DEPENDENCY type=nix-daemon unit=$NIX_DAEMON_UNIT load=${NIX_DAEMON_LOAD:-unknown} active=${NIX_DAEMON_ACTIVE:-unknown} enabled=${NIX_DAEMON_ENABLED:-unknown} socket=$NIX_DAEMON_SOCKET_STATE client=$NIX_DAEMON_CLIENT"
}

wait_for_nix_daemon_socket() {
  local attempt
  for (( attempt = 0; attempt < 50; attempt += 1 )); do
    sudo test -S "$NIX_DAEMON_SOCKET" && return 0
    sleep 0.1
  done
  return 1
}

refresh_nix_daemon_state
print_nix_daemon_state

if [[ "$MODE" == --check ]]; then
  if (( ${#MISSING_APT[@]} > 0 )); then
    echo 'DEBIAN13_SETUP_ACTION_REQUIRED run=bash scripts/debian13-setup.sh --install' >&2
    exit 2
  fi
  if (( GROUP_CONFIGURED == 0 )); then
    echo 'DEBIAN13_SETUP_ACTION_REQUIRED group=nix-users run=bash scripts/debian13-setup.sh --install' >&2
    exit 2
  fi
  if (( NIX_BUILDERS_READY == 0 )); then
    echo 'DEBIAN13_SETUP_ACTION_REQUIRED builders=nixbld action=dpkg-reconfigure:nix-setup-systemd run=bash scripts/debian13-setup.sh --install' >&2
    exit 2
  fi
  if (( NIX_DAEMON_READY == 0 )); then
    echo 'DEBIAN13_SETUP_ACTION_REQUIRED unit=nix-daemon.socket action=enable-rebind run=bash scripts/debian13-setup.sh --install' >&2
    exit 2
  fi
  if (( GROUP_ACTIVE == 0 )); then
    echo 'DEBIAN13_SETUP_GROUP_REEXEC_REQUIRED group=nix-users' >&2
    exit 4
  fi
else
  NEEDS_CHANGE=0
  (( ${#MISSING_APT[@]} > 0 || GROUP_CONFIGURED == 0 \
    || NIX_BUILDERS_READY == 0 || NIX_DAEMON_READY == 0 )) && NEEDS_CHANGE=1

  if (( NEEDS_CHANGE == 1 )); then
    APT_PLAN=none
    GROUP_PLAN=none
    BUILDER_PLAN=none
    DAEMON_PLAN=none
    (( ${#MISSING_APT[@]} > 0 )) && APT_PLAN="${MISSING_APT[*]}"
    (( GROUP_CONFIGURED == 0 )) && GROUP_PLAN="$CURRENT_USER:nix-users"
    (( NIX_BUILDERS_READY == 0 )) && BUILDER_PLAN=reconfigure:nix-setup-systemd
    (( NIX_BUILDERS_READY == 0 || NIX_DAEMON_READY == 0 )) \
      && DAEMON_PLAN=enable-rebind:nix-daemon.socket
    echo "DEBIAN13_SETUP_PLAN apt_install=$APT_PLAN group_add=$GROUP_PLAN nix_builders=$BUILDER_PLAN nix_daemon=$DAEMON_PLAN"
    command -v sudo >/dev/null || fail 'sudo missing; root must install listed apt packages'
    if (( ASSUME_YES == 0 )); then
      [[ -t 0 ]] || fail 'approval requires interactive terminal; rerun with --yes for unattended install'
      read -r -p 'Apply listed Debian system changes? [y/N] ' approval
      case "$approval" in
        y|Y|yes|YES) ;;
        *) echo 'DEBIAN13_SETUP_CANCELLED no_changes_applied'; exit 3 ;;
      esac
    fi

    if (( ${#MISSING_APT[@]} > 0 )); then
      sudo apt-get update
      sudo apt-get install --yes "${MISSING_APT[@]}"
    fi
    if [[ " $(id -nG "$CURRENT_USER" 2>/dev/null || true) " != *" nix-users "* ]]; then
      sudo /sbin/adduser "$CURRENT_USER" nix-users
    fi
    if [[ "$BUILDER_PLAN" != none ]]; then
      [[ -x /usr/sbin/dpkg-reconfigure ]] \
        || fail '/usr/sbin/dpkg-reconfigure missing; cannot repair Debian-owned Nix build users'
      sudo /usr/bin/env DEBIAN_FRONTEND=noninteractive \
        /usr/sbin/dpkg-reconfigure nix-setup-systemd
      refresh_nix_builder_state
      print_nix_builder_state
      (( NIX_BUILDERS_READY == 1 )) \
        || fail "nix-setup-systemd reconfigure did not restore nixbld1..nixbld10; ready=$NIX_BUILD_USERS_READY/10"
    fi
    if [[ "$DAEMON_PLAN" != none ]]; then
      sudo systemctl daemon-reload
      NIX_DAEMON_LOAD=$(systemctl show "$NIX_DAEMON_UNIT" --property=LoadState --value 2>/dev/null || true)
      [[ "$NIX_DAEMON_LOAD" != masked ]] \
        || fail 'nix-daemon.socket is masked; inspect local system policy before unmasking it'
      [[ "$NIX_DAEMON_LOAD" == loaded ]] \
        || fail "nix-daemon.socket unavailable after package install and daemon-reload; load=$NIX_DAEMON_LOAD"
      sudo systemctl enable "$NIX_DAEMON_UNIT"
      # Explicit stop/start closes any retained socket file descriptor before
      # systemd binds a new filesystem endpoint. A restart may retain it.
      sudo systemctl stop nix-daemon.socket nix-daemon.service
      sudo systemctl start nix-daemon.socket
      wait_for_nix_daemon_socket \
        || fail "nix-daemon.socket did not create $NIX_DAEMON_SOCKET within 5 seconds"
    fi
  else
    echo 'DEBIAN13_SETUP_NO_CHANGES system_dependencies=ready'
  fi

  refresh_nix_builder_state
  print_nix_builder_state
  (( NIX_BUILDERS_READY == 1 )) \
    || fail "Nix build users not ready after setup; group=$NIX_BUILD_GROUP_PRESENT users=$NIX_BUILD_USERS_READY/10"
  refresh_nix_daemon_state
  print_nix_daemon_state
  (( NIX_DAEMON_READY == 1 )) \
    || fail "nix-daemon.socket not ready after setup; load=${NIX_DAEMON_LOAD:-unknown} active=${NIX_DAEMON_ACTIVE:-unknown} enabled=${NIX_DAEMON_ENABLED:-unknown} socket=$NIX_DAEMON_SOCKET_STATE client=$NIX_DAEMON_CLIENT"

  if [[ " $(id -nG) " != *" nix-users "* ]]; then
    echo 'DEBIAN13_SETUP_GROUP_REEXEC_REQUIRED group=nix-users'
    exit 4
  fi
fi

command -v git >/dev/null || fail 'git command missing after package validation'
command -v nix >/dev/null || fail 'nix command missing after package validation'
[[ -f flake.nix && -f flake.lock ]] || fail 'flake.nix or flake.lock missing'
nix --version
if nix --extra-experimental-features 'nix-command flakes' \
  flake metadata --offline --no-write-lock-file . >/dev/null 2>&1; then
  echo 'DEBIAN13_DEPENDENCY type=nix-inputs status=cached'
else
  echo 'DEBIAN13_DEPENDENCY type=nix-inputs status=missing approval=required-by-live-test'
fi

echo 'DEBIAN13_SETUP_OK os=debian-13 arch=x86_64 nix=ready flake=locked'
