#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
verifier="$root/scripts/verify-external-napplet-corpus.sh"
temporary_corpus=$(mktemp -d)
lossless_corpus=$(mktemp -d)
null_entry_corpus=$(mktemp -d)
snapshot_corpus=$(mktemp -d)

cleanup() {
  rm -rf -- "$temporary_corpus" "$lossless_corpus" "$null_entry_corpus" "$snapshot_corpus"
}
trap cleanup EXIT

cp -R "$root/fixtures/external-napplet-corpus/." "$temporary_corpus/"
cp -R "$root/fixtures/external-napplet-corpus/." "$null_entry_corpus/"
cp -R "$root/fixtures/external-napplet-corpus/." "$snapshot_corpus/"

basename_output=$(
  cd "$root/fixtures/external-napplet-corpus"
  bash "$verifier" corpus.lock.json
)
if [[ $basename_output != *"EXTERNAL_NAPPLET_CORPUS_OK entries=4"* ]]; then
  echo "expected a basename lock argument to resolve event files from its directory" >&2
  echo "$basename_output" >&2
  exit 1
fi

null_entry_lock="$null_entry_corpus/corpus.lock.json"
null_entry_lock_next="$null_entry_corpus/corpus.lock.next.json"
jq '.entries[0] = null' "$null_entry_lock" > "$null_entry_lock_next"
mv "$null_entry_lock_next" "$null_entry_lock"
set +e
null_entry_output=$(bash "$verifier" "$null_entry_lock" 2>&1)
null_entry_status=$?
set -e
if [[ $null_entry_status -ne 2 || $null_entry_output != *"EXTERNAL_NAPPLET_CORPUS_TRUST code=invalid-lock"* ]]; then
  echo "expected a null entry to be classified as trust failure" >&2
  echo "$null_entry_output" >&2
  exit 1
fi

hanging_node="$temporary_corpus/hanging-node"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'exec sleep 10' > "$hanging_node"
chmod +x "$hanging_node"
set +e
node_timeout_output=$(
  UZEL_NODE_BIN="$hanging_node" \
    UZEL_NODE_TIMEOUT_SECONDS=0.1 \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
node_timeout_status=$?
set -e
if [[ $node_timeout_status -ne 3 || $node_timeout_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=node-timeout"* ]]; then
  echo "expected a hanging Node structural verifier to time out as infrastructure failure" >&2
  echo "$node_timeout_output" >&2
  exit 1
fi

hanging_jq="$temporary_corpus/hanging-jq"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'exec sleep 10' > "$hanging_jq"
chmod +x "$hanging_jq"
set +e
jq_timeout_output=$(
  UZEL_JQ_BIN="$hanging_jq" \
    UZEL_JQ_TIMEOUT_SECONDS=0.1 \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
jq_timeout_status=$?
set -e
if [[ $jq_timeout_status -ne 3 || $jq_timeout_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=jq-timeout"* ]]; then
  echo "expected a hanging jq verifier subprocess to time out as infrastructure failure" >&2
  echo "$jq_timeout_output" >&2
  exit 1
fi

set +e
invalid_jq_timeout_output=$(
  UZEL_JQ_TIMEOUT_SECONDS=0 \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
invalid_jq_timeout_status=$?
set -e
if [[ $invalid_jq_timeout_status -ne 3 || $invalid_jq_timeout_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=invalid-timeout message=jq timeout"* ]]; then
  echo "expected an invalid jq timeout to fail as infrastructure" >&2
  echo "$invalid_jq_timeout_output" >&2
  exit 1
fi

real_node=$(command -v node)
real_jq=$(command -v jq)
snapshot_lock="$snapshot_corpus/corpus.lock.json"
snapshot_lock_next="$snapshot_corpus/corpus.lock.next.json"
snapshot_event="$snapshot_corpus/events/good-morning.json"
snapshot_event_next="$snapshot_corpus/events/good-morning.next.json"
swap_after_verify_node="$snapshot_corpus/swap-after-verify-node"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -euo pipefail' \
  'set +e' \
  '"$UZEL_REAL_NODE" "$@"' \
  'node_status=$?' \
  'set -e' \
  'if [[ $node_status -eq 0 ]]; then' \
  '  "$UZEL_REAL_JQ" '\''.source.commit = "0000000000000000000000000000000000000000"'\'' "$UZEL_SWAP_LOCK" > "$UZEL_SWAP_LOCK_NEXT"' \
  '  mv -- "$UZEL_SWAP_LOCK_NEXT" "$UZEL_SWAP_LOCK"' \
  '  "$UZEL_REAL_JQ" '\''.sig = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"'\'' "$UZEL_SWAP_EVENT" > "$UZEL_SWAP_EVENT_NEXT"' \
  '  mv -- "$UZEL_SWAP_EVENT_NEXT" "$UZEL_SWAP_EVENT"' \
  'fi' \
  'exit "$node_status"' > "$swap_after_verify_node"
chmod +x "$swap_after_verify_node"
snapshot_output=$(
  UZEL_NODE_BIN="$swap_after_verify_node" \
    UZEL_REAL_NODE="$real_node" \
    UZEL_REAL_JQ="$real_jq" \
    UZEL_SWAP_LOCK="$snapshot_lock" \
    UZEL_SWAP_LOCK_NEXT="$snapshot_lock_next" \
    UZEL_SWAP_EVENT="$snapshot_event" \
    UZEL_SWAP_EVENT_NEXT="$snapshot_event_next" \
    bash "$verifier" "$snapshot_lock"
)
if [[ $snapshot_output != *"EXTERNAL_NAPPLET_CORPUS_OK entries=4 commit=aa4dc7a0799d95e3066b50055b29685d6e376045 mode=offline"* ]]; then
  echo "expected nak and reporting to consume the verified snapshot after path swaps" >&2
  echo "$snapshot_output" >&2
  exit 1
fi
if [[ $(jq -r '.source.commit' "$snapshot_lock") != "0000000000000000000000000000000000000000" ]]; then
  echo "expected the deterministic swap hook to replace the lock after verification" >&2
  exit 1
fi
if [[ $(jq -r '.sig' "$snapshot_event") != "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000" ]]; then
  echo "expected the deterministic swap hook to replace the event after verification" >&2
  exit 1
fi

bad_event="$temporary_corpus/events/good-morning.json"
bad_event_next="$temporary_corpus/events/good-morning.next.json"
jq '.sig = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"' \
  "$bad_event" > "$bad_event_next"
mv "$bad_event_next" "$bad_event"

set +e
trust_output=$(bash "$verifier" "$temporary_corpus/corpus.lock.json" 2>&1)
trust_status=$?
set -e
if [[ $trust_status -ne 2 || $trust_output != *"EXTERNAL_NAPPLET_CORPUS_TRUST code=invalid-event-signature"* ]]; then
  echo "expected invalid signature to be classified as trust failure" >&2
  echo "$trust_output" >&2
  exit 1
fi

set +e
infrastructure_output=$(
  UZEL_NAK_BIN=uzel-deliberately-missing-nak \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
infrastructure_status=$?
set -e
if [[ $infrastructure_status -ne 3 || $infrastructure_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=nak-unavailable"* ]]; then
  echo "expected missing nak to be classified as infrastructure failure" >&2
  echo "$infrastructure_output" >&2
  exit 1
fi

old_nak="$temporary_corpus/old-nak"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'echo "nak version 0.16.2"' > "$old_nak"
chmod +x "$old_nak"
set +e
version_output=$(
  UZEL_NAK_BIN="$old_nak" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
version_status=$?
set -e
if [[ $version_status -ne 3 || $version_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=nak-version-mismatch"* ]]; then
  echo "expected an unpinned nak version to be classified as infrastructure failure" >&2
  echo "$version_output" >&2
  exit 1
fi

hanging_nak="$temporary_corpus/hanging-nak"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -euo pipefail' \
  'case "${UZEL_HANG_OPERATION}:${1:-}" in' \
  '  version:--version|verify:verify|decode:decode) exec sleep 10 ;;' \
  'esac' \
  'case "${1:-}" in' \
  '  --version) echo "nak version 0.20.1" ;;' \
  '  verify) exit 0 ;;' \
  '  decode) exit 42 ;;' \
  '  *) exit 42 ;;' \
  'esac' > "$hanging_nak"
chmod +x "$hanging_nak"
for operation in version verify decode; do
  set +e
  timeout_output=$(
    UZEL_HANG_OPERATION="$operation" \
      UZEL_NAK_BIN="$hanging_nak" \
      UZEL_NAK_TIMEOUT_SECONDS=0.1 \
      bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
  )
  timeout_status=$?
  set -e
  if [[ $timeout_status -ne 3 || $timeout_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=nak-timeout"* ]]; then
    echo "expected hanging nak $operation to time out as infrastructure failure" >&2
    echo "$timeout_output" >&2
    exit 1
  fi
done

broken_enumeration_jq="$temporary_corpus/broken-enumeration-jq"
# This line is emitted into the fake jq script.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'for argument in "$@"; do' \
  '  if [[ $argument == *".entries[]"* ]]; then' \
  '    exit 44' \
  '  fi' \
  'done' \
  "exec $real_jq \"\$@\"" > "$broken_enumeration_jq"
chmod +x "$broken_enumeration_jq"
set +e
enumeration_output=$(
  UZEL_JQ_BIN="$broken_enumeration_jq" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
enumeration_status=$?
set -e
if [[ $enumeration_status -ne 3 || $enumeration_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=jq-execution-failed"* ]]; then
  echo "expected broken jq entry enumeration to be classified as infrastructure failure" >&2
  echo "$enumeration_output" >&2
  exit 1
fi

broken_comparison_jq="$temporary_corpus/broken-comparison-jq"
# This line is emitted into the fake jq script.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'for argument in "$@"; do' \
  '  if [[ $argument == *".pubkey =="* ]]; then' \
  '    exit 45' \
  '  fi' \
  'done' \
  "exec $real_jq \"\$@\"" > "$broken_comparison_jq"
chmod +x "$broken_comparison_jq"
set +e
comparison_output=$(
  UZEL_JQ_BIN="$broken_comparison_jq" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
comparison_status=$?
set -e
if [[ $comparison_status -ne 3 || $comparison_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=jq-execution-failed"* ]]; then
  echo "expected broken jq coordinate comparison to be classified as infrastructure failure" >&2
  echo "$comparison_output" >&2
  exit 1
fi

cp -R "$root/fixtures/external-napplet-corpus/." "$lossless_corpus/"
real_nak=$(command -v nak)
lossless_node="$lossless_corpus/lossless-node"
# These lines are emitted into the fake Node launcher.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -euo pipefail' \
  'snapshot=$("$UZEL_REAL_NODE" "$@")' \
  '"$UZEL_REAL_JQ" '\''
    .entries[0].entry.relayHints = []
    | .entries[1].entry.relayHints = ["wss://relay.example/path,segment"]
  '\'' <<< "$snapshot"' > "$lossless_node"
chmod +x "$lossless_node"

lossless_nak="$lossless_corpus/lossless-nak"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  "real_nak=$real_nak" \
  "real_jq=$real_jq" \
  'if [[ ${1:-} != "decode" ]]; then' \
  '  exec "$real_nak" "$@"' \
  'fi' \
  'decoded=$("$real_nak" "$@") || exit $?' \
  'identifier=$("$real_jq" -r .identifier <<< "$decoded") || exit $?' \
  'case "$identifier" in' \
  '  good-morning) relays="[]" ;;' \
  '  rubik-cube) relays="[\"wss://relay.example/path,segment\"]" ;;' \
  '  *) exec "$real_jq" . <<< "$decoded" ;;' \
  'esac' \
  '"$real_jq" --argjson relays "$relays" '\''.relays = $relays'\'' <<< "$decoded"' \
  > "$lossless_nak"
chmod +x "$lossless_nak"
lossless_output=$(
  UZEL_NODE_BIN="$lossless_node" \
    UZEL_REAL_NODE="$real_node" \
    UZEL_REAL_JQ="$real_jq" \
    UZEL_NAK_BIN="$lossless_nak" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json"
)
if [[ $lossless_output != *"EXTERNAL_NAPPLET_CORPUS_OK entries=4"* ]]; then
  echo "expected empty and comma-bearing relay arrays to survive entry transport" >&2
  echo "$lossless_output" >&2
  exit 1
fi

broken_nak="$temporary_corpus/broken-nak"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  "if [[ \${1:-} == \"--version\" ]]; then" \
  '  echo "nak version 0.20.1"' \
  '  exit 0' \
  'fi' \
  'exit 42' > "$broken_nak"
chmod +x "$broken_nak"
set +e
execution_output=$(
  UZEL_NAK_BIN="$broken_nak" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
execution_status=$?
set -e
if [[ $execution_status -ne 3 || $execution_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=nak-execution-failed"* ]]; then
  echo "expected a broken pinned-version nak to be classified as infrastructure failure" >&2
  echo "$execution_output" >&2
  exit 1
fi

broken_decode_nak="$temporary_corpus/broken-decode-nak"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  "if [[ \${1:-} == \"--version\" ]]; then" \
  '  echo "nak version 0.20.1"' \
  '  exit 0' \
  'fi' \
  "if [[ \${1:-} == \"verify\" ]]; then" \
  '  exit 0' \
  'fi' \
  'exit 43' > "$broken_decode_nak"
chmod +x "$broken_decode_nak"
set +e
decode_output=$(
  UZEL_NAK_BIN="$broken_decode_nak" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
decode_status=$?
set -e
if [[ $decode_status -ne 3 || $decode_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=nak-execution-failed"* ]]; then
  echo "expected a broken nak decoder to be classified as infrastructure failure" >&2
  echo "$decode_output" >&2
  exit 1
fi

invalid_output_nak="$temporary_corpus/invalid-output-nak"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  "if [[ \${1:-} == \"--version\" ]]; then" \
  '  echo "nak version 0.20.1"' \
  '  exit 0' \
  'fi' \
  "if [[ \${1:-} == \"verify\" ]]; then" \
  '  exit 0' \
  'fi' \
  'echo not-json' > "$invalid_output_nak"
chmod +x "$invalid_output_nak"
set +e
output_status_text=$(
  UZEL_NAK_BIN="$invalid_output_nak" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
output_status=$?
set -e
if [[ $output_status -ne 3 || $output_status_text != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=nak-invalid-output"* ]]; then
  echo "expected malformed nak output to be classified as infrastructure failure" >&2
  echo "$output_status_text" >&2
  exit 1
fi

real_nak=$(command -v nak)
multi_document_nak="$temporary_corpus/multi-document-nak"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  "real_nak=$real_nak" \
  'if [[ ${1:-} == "--version" ]]; then' \
  '  echo "nak version 0.20.1"' \
  '  exit 0' \
  'fi' \
  'if [[ ${1:-} == "verify" ]]; then' \
  '  exit 0' \
  'fi' \
  'echo "{}"' \
  'exec "$real_nak" "$@"' > "$multi_document_nak"
chmod +x "$multi_document_nak"
set +e
multi_document_output=$(
  UZEL_NAK_BIN="$multi_document_nak" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
multi_document_status=$?
set -e
if [[ $multi_document_status -ne 3 || $multi_document_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=nak-invalid-output"* ]]; then
  echo "expected a multi-document nak decode stream to be classified as infrastructure failure" >&2
  echo "$multi_document_output" >&2
  exit 1
fi

echo 'EXTERNAL_NAPPLET_CORPUS_CLASSIFICATION_TEST_OK trust=2 infrastructure=3 version=pinned node=bounded jq=bounded execution=bounded transport=lossless'
