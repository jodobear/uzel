#!/usr/bin/env bash
set -euo pipefail

if rg -n '(tauri|svelte)' crates --glob '*.rs' --glob 'Cargo.toml'; then
  echo 'boundary violation: reusable crates depend on UI/platform code' >&2
  exit 1
fi

node scripts/check-napplet-imports.mjs

echo 'BOUNDARIES_OK'
