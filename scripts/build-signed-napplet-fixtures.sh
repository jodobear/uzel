#!/usr/bin/env bash
set -euo pipefail

: "${VITE_DEV_PRIVKEY_HEX:?set a disposable Nostr test key; never use a real identity key}"

temporary_directory="$(mktemp -d)"
trap 'rm -rf "$temporary_directory"' EXIT
cp -R --dereference node_modules/@napplet/cli "$temporary_directory/cli"

for entry in follow-list:follow-list profile-card:profile-card hostile-egress:egress-probe; do
  napplet="${entry%%:*}"
  d_tag="${entry##*:}"
  corepack pnpm@10.8.0 --filter "@jodobear/$napplet" build
  config="$temporary_directory/$napplet-config.json"
  plan="$temporary_directory/$napplet-plan.json"
  jq -n \
    --arg source "napplets/$napplet" \
    --arg d_tag "$d_tag" \
    '{
      version: 1,
      sourceDir: $source,
      relays: ["wss://relay.invalid"],
      blossomServers: ["https://blossom.invalid/"],
      defaultTarget: "named",
      named: [$d_tag],
      signing: {mode: "interactive"}
    }' > "$config"
  deno run \
    --config="$temporary_directory/cli/deno.json" \
    --lock=deno.lock \
    --frozen \
    --allow-read \
    --allow-env \
    --allow-net \
    "$temporary_directory/cli/src/cli.ts" \
    deploy \
    --config "$config" \
    --dry-run \
    --sec "$VITE_DEV_PRIVKEY_HEX" > "$plan"
  fixture="fixtures/$napplet"
  mkdir -p "$fixture"
  cp "napplets/$napplet/dist/index.html" "$fixture/index.html"
  jq '.manifests[0].signedEvent' "$plan" > "$fixture/event.json"
  expected_digest="$(jq -r '.tags[] | select(.[0] == "path" and .[1] == "/index.html") | .[2]' "$fixture/event.json")"
  actual_digest="$(sha256sum "$fixture/index.html")"
  actual_digest="${actual_digest%% *}"
  [[ "$actual_digest" == "$expected_digest" ]]
  jq -e \
    --arg d_tag "$d_tag" \
    '.kind == 35129 and .id and .sig and any(.tags[]; . == ["d", $d_tag]) and any(.tags[]; . == ["server", "https://blossom.invalid/"])' \
    "$fixture/event.json" >/dev/null
done

echo 'SIGNED_NAPPLET_FIXTURES_OK count=3 key=discarded-by-caller'
