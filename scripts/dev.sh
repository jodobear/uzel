#!/usr/bin/env bash
set -euo pipefail

UZEL_RELAY_PID=
UZEL_NAPD_PID=
USE_FIXTURE_RELAY=${UZEL_USE_FIXTURE_RELAY:-0}

cleanup() {
  if [[ -n "$UZEL_NAPD_PID" ]]; then
    kill "$UZEL_NAPD_PID" 2>/dev/null || true
    wait "$UZEL_NAPD_PID" 2>/dev/null || true
  fi
  if [[ -n "$UZEL_RELAY_PID" ]]; then
    kill "$UZEL_RELAY_PID" 2>/dev/null || true
    wait "$UZEL_RELAY_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

if [[ "$USE_FIXTURE_RELAY" != 0 && "$USE_FIXTURE_RELAY" != 1 ]]; then
  echo 'UZEL_USE_FIXTURE_RELAY must be 0 or 1' >&2
  exit 1
fi

NAPD_RELAY_ARGS=(
  --indexer-relay wss://purplepag.es
  --indexer-relay wss://relay.primal.net
)

if [[ "$USE_FIXTURE_RELAY" == 1 ]]; then
  RELAY_PORT=${UZEL_FIXTURE_RELAY_PORT:-43129}
  if [[ ! "$RELAY_PORT" =~ ^[0-9]+$ ]] || (( RELAY_PORT < 1024 || RELAY_PORT > 65535 )); then
    echo 'UZEL_FIXTURE_RELAY_PORT must be an unprivileged TCP port' >&2
    exit 1
  fi

  nak serve \
    --hostname 127.0.0.1 \
    --port "$RELAY_PORT" \
    --events fixtures/nostr/live-events.jsonl &
  UZEL_RELAY_PID=$!

  for _ in $(seq 1 80); do
    if (exec 3<>"/dev/tcp/127.0.0.1/$RELAY_PORT") 2>/dev/null; then
      break
    fi
    kill -0 "$UZEL_RELAY_PID" 2>/dev/null
    sleep 0.05
  done
  if ! (exec 3<>"/dev/tcp/127.0.0.1/$RELAY_PORT") 2>/dev/null; then
    echo 'deterministic Nostr fixture relay did not become ready' >&2
    exit 1
  fi
  NAPD_RELAY_ARGS=(
    --fallback-relay "ws://127.0.0.1:$RELAY_PORT"
    --allow-local-relay-host 127.0.0.1
  )
fi

cargo run -p uzel-napd -- \
  --live \
  "${NAPD_RELAY_ARGS[@]}" &
UZEL_NAPD_PID=$!

pnpm --dir apps/uzel tauri dev
