#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
verifier="$root/scripts/verify-external-napplet-corpus.sh"
temporary_corpus=$(mktemp -d)
lossless_corpus=$(mktemp -d)

cleanup() {
  rm -rf -- "$temporary_corpus" "$lossless_corpus"
}
trap cleanup EXIT

cp -R "$root/fixtures/external-napplet-corpus/." "$temporary_corpus/"

basename_output=$(
  cd "$root/fixtures/external-napplet-corpus"
  bash "$verifier" corpus.lock.json
)
if [[ $basename_output != *"EXTERNAL_NAPPLET_CORPUS_OK entries=4"* ]]; then
  echo "expected a basename lock argument to resolve event files from its directory" >&2
  echo "$basename_output" >&2
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

real_jq=$(command -v jq)
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
lossless_lock="$lossless_corpus/corpus.lock.json"
lossless_lock_next="$lossless_corpus/corpus.lock.next.json"
jq \
  '.entries[0].relayHints = []
    | .entries[1].relayHints = ["wss://relay.example/path,segment"]' \
  "$lossless_lock" > "$lossless_lock_next"
mv "$lossless_lock_next" "$lossless_lock"

real_nak=$(command -v nak)
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
  UZEL_NAK_BIN="$lossless_nak" \
    bash "$verifier" "$lossless_lock"
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

echo 'EXTERNAL_NAPPLET_CORPUS_CLASSIFICATION_TEST_OK trust=2 infrastructure=3 version=pinned execution=bounded transport=lossless'
