#!/usr/bin/env bash
set -euo pipefail

if rg -n '(tauri|svelte)' crates --glob '*.rs' --glob 'Cargo.toml'; then
  echo 'boundary violation: reusable crates depend on UI/platform code' >&2
  exit 1
fi

if rg -n \
  "((import|export).*from|import\\()\\s*['\"][^'\"]*(uzel|napd|tauri)|\"(@tauri|uzel|napd)[^\"]*\"\\s*:" \
  napplets --glob '*.ts' --glob '*.js' --glob '*.svelte' --glob 'package.json'; then
  echo 'boundary violation: portable napplet depends on Uzel/runtime code' >&2
  exit 1
fi

if rg -n '(fetch|XMLHttpRequest|WebSocket|EventSource|sendBeacon|serviceWorker)' \
  napplets/follow-list napplets/profile-card \
  --glob '*.ts' --glob '*.js' --glob '*.svelte'; then
  echo 'boundary violation: product napplet uses direct browser network authority' >&2
  exit 1
fi

echo 'BOUNDARIES_OK'
