#!/usr/bin/env bash
set -euo pipefail

MODE=${1:---check}
ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

fail() {
  echo "DEBIAN13_SETUP_ERROR $*" >&2
  exit 1
}

[[ "$MODE" == "--check" || "$MODE" == "--install" ]] \
  || fail 'usage: scripts/debian13-setup.sh [--check|--install]'
[[ -r /etc/os-release ]] || fail '/etc/os-release missing'

# shellcheck disable=SC1091
source /etc/os-release
[[ "${ID:-}" == debian && "${VERSION_ID:-}" == 13 ]] \
  || fail "requires Debian 13; found ID=${ID:-unknown} VERSION_ID=${VERSION_ID:-unknown}"
[[ "$(uname -m)" == x86_64 ]] \
  || fail "current flake supports x86_64-linux only; found $(uname -m)"
[[ $EUID -ne 0 ]] || fail 'run as normal desktop user, not root'

if [[ "$MODE" == "--install" ]]; then
  command -v sudo >/dev/null || fail 'sudo missing'
  sudo apt-get update
  sudo apt-get install --yes git nix-setup-systemd
  if [[ " $(id -nG) " != *" nix-users "* ]]; then
    sudo /sbin/adduser "$USER" nix-users
    echo 'DEBIAN13_SETUP_RELOGIN_REQUIRED group=nix-users'
    echo 'Log out completely, log in, then rerun: bash scripts/debian13-setup.sh --check'
    exit 2
  fi
fi

command -v git >/dev/null || fail 'git missing; run with --install'
command -v nix >/dev/null || fail 'nix missing; run with --install'
[[ " $(id -nG) " == *" nix-users "* ]] \
  || fail 'current login lacks nix-users; log out and in after --install'

nix --version
nix --extra-experimental-features 'nix-command flakes' \
  flake metadata --no-write-lock-file . >/dev/null

echo 'DEBIAN13_SETUP_OK os=debian-13 arch=x86_64 nix=ready flake=locked'
