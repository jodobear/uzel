#!/usr/bin/env bash
set -euo pipefail

if rg -n '(tauri|svelte)' crates --glob '*.rs' --glob 'Cargo.toml'; then
  echo 'boundary violation: reusable crates depend on UI/platform code' >&2
  exit 1
fi

if rg -n '(uzel|napd|tauri)' napplets --glob '*.ts' --glob '*.js' --glob '*.svelte' --glob 'package.json'; then
  echo 'boundary violation: portable napplet depends on Uzel/runtime code' >&2
  exit 1
fi

echo 'BOUNDARIES_OK'
