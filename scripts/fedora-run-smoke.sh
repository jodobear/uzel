#!/usr/bin/env bash
set -euo pipefail

export UZEL_SMOKE_NAME=slice-06-fedora
export UZEL_SMOKE_SUCCESS_MARKER=FEDORA_RUN_SMOKE_OK
exec bash scripts/linux-run-smoke.sh
