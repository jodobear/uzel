#!/usr/bin/env bash
set -euo pipefail

if rg -n '(tauri|svelte)' crates --glob '*.rs' --glob 'Cargo.toml'; then
  echo 'boundary violation: reusable crates depend on UI/platform code' >&2
  exit 1
fi

forbidden_import_pattern='\b(import\s*([^\x27"`]*\(|[^\x27"`]*from\s*)?|export\s+[^\x27"`]*from\s*)[\x27"`][^\x27"`]*(uzel|napd|tauri)'
forbidden_dependency_pattern='"(@tauri|uzel|napd)[^"]*"\s*:'

for prohibited_import in \
  "import 'uzel/runtime';" \
  'import("@tauri-apps/api");' \
  'import(`@tauri-apps/api`);' \
  "import /* lazy */ ('../../../apps/uzel/src/main.ts');" \
  $'import // lazy\n("../../../apps/uzel/src/main.ts");' \
  "export { runtime } from 'napd/runtime';"; do
  if ! printf '%s\n' "$prohibited_import" | rg -U -q "$forbidden_import_pattern"; then
    echo "boundary self-test failed: $prohibited_import" >&2
    exit 1
  fi
done

if rg -U -n -e "$forbidden_import_pattern" -e "$forbidden_dependency_pattern" \
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
