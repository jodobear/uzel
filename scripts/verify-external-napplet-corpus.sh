#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
lock=${1:-"$root/fixtures/external-napplet-corpus/corpus.lock.json"}
corpus_dir=$(dirname -- "$lock")
nak_bin=${UZEL_NAK_BIN:-nak}
jq_bin=${UZEL_JQ_BIN:-jq}
entries_file=$(mktemp)

cleanup() {
  rm -f -- "$entries_file"
}
trap cleanup EXIT

infrastructure_failure() {
  echo "EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=$1 message=$2" >&2
  exit 3
}

trust_failure() {
  echo "EXTERNAL_NAPPLET_CORPUS_TRUST code=$1 message=$2" >&2
  exit 2
}

command -v node >/dev/null 2>&1 || infrastructure_failure node-unavailable "node is required"
command -v "$jq_bin" >/dev/null 2>&1 || infrastructure_failure jq-unavailable "jq is required"
command -v "$nak_bin" >/dev/null 2>&1 || infrastructure_failure nak-unavailable "pinned nak is required"

set +e
node "$root/scripts/verify-external-napplet-corpus.mjs" "$lock"
node_status=$?
set -e
if [[ $node_status -eq 2 ]]; then
  exit 2
fi
if [[ $node_status -ne 0 ]]; then
  infrastructure_failure node-execution-failed "corpus structure verifier failed with status $node_status"
fi

set +e
expected_nak_version=$("$jq_bin" -r '.toolchain.nakVersion' "$lock")
jq_status=$?
set -e
if [[ $jq_status -ne 0 ]]; then
  infrastructure_failure jq-execution-failed "reading the pinned nak version failed with status $jq_status"
fi
set +e
nak_version_output=$("$nak_bin" --version 2>/dev/null)
nak_version_status=$?
set -e
if [[ $nak_version_status -ne 0 ]]; then
  infrastructure_failure nak-execution-failed "nak --version failed with status $nak_version_status"
fi
if [[ $nak_version_output != "nak version $expected_nak_version" ]]; then
  infrastructure_failure nak-version-mismatch "expected nak version $expected_nak_version"
fi

set +e
"$jq_bin" -r \
  '.entries[]
    | {
        name,
        naddr,
        author,
        kind,
        dTag,
        relayHints,
        eventFile
      }
    | @json' \
  "$lock" > "$entries_file"
jq_status=$?
set -e
if [[ $jq_status -ne 0 ]]; then
  infrastructure_failure jq-execution-failed "entry enumeration failed with status $jq_status"
fi

entry_count=0
while IFS= read -r entry_json; do
  ((entry_count += 1))
  set +e
  name=$("$jq_bin" -r '.name' <<< "$entry_json")
  name_status=$?
  naddr=$("$jq_bin" -r '.naddr' <<< "$entry_json")
  naddr_status=$?
  event_file=$("$jq_bin" -r '.eventFile' <<< "$entry_json")
  event_file_status=$?
  set -e
  if [[ $name_status -ne 0 || $naddr_status -ne 0 || $event_file_status -ne 0 ]]; then
    infrastructure_failure jq-execution-failed "entry decoding failed"
  fi
  event_path="$corpus_dir/$event_file"
  set +e
  "$nak_bin" verify < "$event_path" >/dev/null 2>&1
  verify_status=$?
  set -e
  if [[ $verify_status -ne 0 ]]; then
    if [[ $verify_status -eq 123 ]]; then
      trust_failure invalid-event-signature "$name signed event failed nak verification"
    fi
    infrastructure_failure nak-execution-failed "$name nak verify failed with status $verify_status"
  fi

  set +e
  decoded=$("$nak_bin" decode "$naddr" 2>&1)
  decode_status=$?
  set -e
  if [[ $decode_status -ne 0 ]]; then
    if [[ $decode_status -eq 123 ]]; then
      trust_failure coordinate-drift "$name naddr failed nak decoding"
    fi
    infrastructure_failure nak-execution-failed "$name nak decode failed with status $decode_status"
  fi
  if ! "$jq_bin" -e -s \
    'length == 1 and (.[0] | type == "object")' \
    <<< "$decoded" >/dev/null 2>&1; then
    infrastructure_failure nak-invalid-output "$name nak decode returned anything other than one JSON object"
  fi
  # jq expands these variables, not Bash.
  # shellcheck disable=SC2016
  set +e
  "$jq_bin" -e \
    --argjson locked "$entry_json" \
    '.pubkey == $locked.author
      and .kind == $locked.kind
      and .identifier == $locked.dTag
      and .relays == $locked.relayHints' \
    <<< "$decoded" >/dev/null
  coordinate_status=$?
  set -e
  if [[ $coordinate_status -eq 1 ]]; then
    trust_failure coordinate-drift "$name naddr does not encode locked coordinate"
  fi
  if [[ $coordinate_status -ne 0 ]]; then
    infrastructure_failure jq-execution-failed "$name coordinate comparison failed with status $coordinate_status"
  fi
done < "$entries_file"

if [[ $entry_count -eq 0 ]]; then
  infrastructure_failure jq-invalid-output "entry enumeration returned no rows"
fi

set +e
source_commit=$("$jq_bin" -r '.source.commit' "$lock")
jq_status=$?
set -e
if [[ $jq_status -ne 0 ]]; then
  infrastructure_failure jq-execution-failed "reading the source commit failed with status $jq_status"
fi
echo "EXTERNAL_NAPPLET_CORPUS_OK entries=$entry_count commit=$source_commit mode=offline"
