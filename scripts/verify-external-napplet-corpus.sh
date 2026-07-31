#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
lock=${1:-"$root/fixtures/external-napplet-corpus/corpus.lock.json"}
corpus_dir=${lock%/*}
nak_bin=${UZEL_NAK_BIN:-nak}

infrastructure_failure() {
  echo "EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=$1 message=$2" >&2
  exit 3
}

trust_failure() {
  echo "EXTERNAL_NAPPLET_CORPUS_TRUST code=$1 message=$2" >&2
  exit 2
}

command -v node >/dev/null 2>&1 || infrastructure_failure node-unavailable "node is required"
command -v jq >/dev/null 2>&1 || infrastructure_failure jq-unavailable "jq is required"
command -v "$nak_bin" >/dev/null 2>&1 || infrastructure_failure nak-unavailable "pinned nak is required"

node "$root/scripts/verify-external-napplet-corpus.mjs" "$lock"

while IFS=$'\t' read -r name naddr author kind d_tag relay_hints event_file; do
  event_path="$corpus_dir/$event_file"
  if ! "$nak_bin" verify < "$event_path" >/dev/null; then
    trust_failure invalid-event-signature "$name signed event failed nak verification"
  fi

  if ! decoded=$("$nak_bin" decode "$naddr"); then
    trust_failure coordinate-drift "$name naddr failed nak decoding"
  fi
  if ! jq -e \
    --arg author "$author" \
    --argjson kind "$kind" \
    --arg d_tag "$d_tag" \
    --arg relay_hints "$relay_hints" \
    '.pubkey == $author
      and .kind == $kind
      and .identifier == $d_tag
      and (.relays == ($relay_hints | split(",")))' \
    <<< "$decoded" >/dev/null; then
    trust_failure coordinate-drift "$name naddr does not encode locked coordinate"
  fi
done < <(
  jq -r \
    '.entries[]
      | [.name, .naddr, .author, (.kind | tostring), .dTag, (.relayHints | join(",")), .eventFile]
      | @tsv' \
    "$lock"
)

entry_count=$(jq '.entries | length' "$lock")
source_commit=$(jq -r '.source.commit' "$lock")
echo "EXTERNAL_NAPPLET_CORPUS_OK entries=$entry_count commit=$source_commit mode=offline"
