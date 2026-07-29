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

if [[ "$MODE" == --check ]]; then
  if (( ${#MISSING_APT[@]} > 0 )); then
    echo 'DEBIAN13_SETUP_ACTION_REQUIRED run=bash scripts/debian13-setup.sh --install' >&2
    exit 2
  fi
  if (( GROUP_CONFIGURED == 0 )); then
    echo 'DEBIAN13_SETUP_ACTION_REQUIRED group=nix-users run=bash scripts/debian13-setup.sh --install' >&2
    exit 2
  fi
  if (( GROUP_ACTIVE == 0 )); then
    echo 'DEBIAN13_SETUP_GROUP_REEXEC_REQUIRED group=nix-users' >&2
    exit 4
  fi
else
  NEEDS_CHANGE=0
  (( ${#MISSING_APT[@]} > 0 || GROUP_CONFIGURED == 0 )) && NEEDS_CHANGE=1

  if (( NEEDS_CHANGE == 1 )); then
    APT_PLAN=none
    GROUP_PLAN=none
    (( ${#MISSING_APT[@]} > 0 )) && APT_PLAN="${MISSING_APT[*]}"
    (( GROUP_CONFIGURED == 0 )) && GROUP_PLAN="$CURRENT_USER:nix-users"
    echo "DEBIAN13_SETUP_PLAN apt_install=$APT_PLAN group_add=$GROUP_PLAN"
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
  else
    echo 'DEBIAN13_SETUP_NO_CHANGES system_dependencies=ready'
  fi

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
