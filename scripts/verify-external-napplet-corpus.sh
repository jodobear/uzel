#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
lock=${1:-"$root/fixtures/external-napplet-corpus/corpus.lock.json"}
nak_bin=${UZEL_NAK_BIN:-nak}
jq_bin=${UZEL_JQ_BIN:-jq}
node_bin=${UZEL_NODE_BIN:-node}
timeout_bin=${UZEL_TIMEOUT_BIN:-timeout}
nak_timeout_seconds=${UZEL_NAK_TIMEOUT_SECONDS:-10}
entries_file=$(mktemp)
verified_event_file=$(mktemp)

cleanup() {
  rm -f -- "$entries_file" "$verified_event_file"
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

command -v "$node_bin" >/dev/null 2>&1 || infrastructure_failure node-unavailable "node is required"
command -v "$jq_bin" >/dev/null 2>&1 || infrastructure_failure jq-unavailable "jq is required"
command -v "$nak_bin" >/dev/null 2>&1 || infrastructure_failure nak-unavailable "pinned nak is required"
command -v "$timeout_bin" >/dev/null 2>&1 || infrastructure_failure timeout-unavailable "timeout is required"
if [[ ! $nak_timeout_seconds =~ ^(([1-9][0-9]*)(\.[0-9]+)?|0\.[0-9]*[1-9][0-9]*)$ ]]; then
  infrastructure_failure invalid-timeout "nak timeout must be a positive number of seconds"
fi

set +e
verified_snapshot=$("$node_bin" "$root/scripts/verify-external-napplet-corpus.mjs" --snapshot-json "$lock")
node_status=$?
set -e
if [[ $node_status -eq 2 ]]; then
  exit 2
fi
if [[ $node_status -ne 0 ]]; then
  infrastructure_failure node-execution-failed "corpus structure verifier failed with status $node_status"
fi

set +e
snapshot_metadata=$("$jq_bin" -r \
  '.format, .lock.toolchain.nakVersion, .lock.source.commit, (.entries | length)' \
  <<< "$verified_snapshot")
jq_status=$?
set -e
if [[ $jq_status -ne 0 ]]; then
  infrastructure_failure jq-execution-failed "reading verified snapshot metadata failed with status $jq_status"
fi
mapfile -t metadata_lines <<< "$snapshot_metadata"
if [[ ${#metadata_lines[@]} -ne 4 || ${metadata_lines[0]} != "uzel.verified-external-napplet-corpus.v1" ]]; then
  infrastructure_failure jq-invalid-output "verified snapshot metadata was incomplete"
fi
expected_nak_version=${metadata_lines[1]}
source_commit=${metadata_lines[2]}
expected_entry_count=${metadata_lines[3]}
echo "EXTERNAL_NAPPLET_CORPUS_STRUCTURE_OK entries=$expected_entry_count commit=$source_commit mode=offline"

set +e
nak_version_output=$("$timeout_bin" --kill-after=1 "$nak_timeout_seconds" "$nak_bin" --version 2>/dev/null)
nak_version_status=$?
set -e
if [[ $nak_version_status -eq 124 || $nak_version_status -eq 137 ]]; then
  infrastructure_failure nak-timeout "nak --version exceeded ${nak_timeout_seconds}s"
fi
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
        name: .entry.name,
        naddr: .entry.naddr,
        author: .entry.author,
        kind: .entry.kind,
        dTag: .entry.dTag,
        relayHints: .entry.relayHints,
        eventText
      }
    | @json' \
  <<< "$verified_snapshot" > "$entries_file"
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
  set -e
  if [[ $name_status -ne 0 || $naddr_status -ne 0 ]]; then
    infrastructure_failure jq-execution-failed "entry decoding failed"
  fi
  set +e
  "$jq_bin" -j '.eventText' <<< "$entry_json" > "$verified_event_file"
  event_status=$?
  set -e
  if [[ $event_status -ne 0 ]]; then
    infrastructure_failure jq-execution-failed "$name verified event extraction failed with status $event_status"
  fi
  set +e
  "$timeout_bin" --kill-after=1 "$nak_timeout_seconds" "$nak_bin" verify < "$verified_event_file" >/dev/null 2>&1
  verify_status=$?
  set -e
  if [[ $verify_status -eq 124 || $verify_status -eq 137 ]]; then
    infrastructure_failure nak-timeout "$name nak verify exceeded ${nak_timeout_seconds}s"
  elif [[ $verify_status -ne 0 ]]; then
    if [[ $verify_status -eq 123 ]]; then
      trust_failure invalid-event-signature "$name signed event failed nak verification"
    fi
    infrastructure_failure nak-execution-failed "$name nak verify failed with status $verify_status"
  fi

  set +e
  decoded=$("$timeout_bin" --kill-after=1 "$nak_timeout_seconds" "$nak_bin" decode "$naddr" 2>&1)
  decode_status=$?
  set -e
  if [[ $decode_status -eq 124 || $decode_status -eq 137 ]]; then
    infrastructure_failure nak-timeout "$name nak decode exceeded ${nak_timeout_seconds}s"
  elif [[ $decode_status -ne 0 ]]; then
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

if [[ $entry_count -ne $expected_entry_count ]]; then
  infrastructure_failure jq-invalid-output "entry enumeration returned $entry_count rows, expected $expected_entry_count"
fi

echo "EXTERNAL_NAPPLET_CORPUS_OK entries=$entry_count commit=$source_commit mode=offline"
