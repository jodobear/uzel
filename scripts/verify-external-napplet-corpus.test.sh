#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
verifier="$root/scripts/verify-external-napplet-corpus.sh"
temporary_corpus=$(mktemp -d)

cleanup() {
  rm -rf -- "$temporary_corpus"
}
trap cleanup EXIT

cp -R "$root/fixtures/external-napplet-corpus/." "$temporary_corpus/"

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

echo 'EXTERNAL_NAPPLET_CORPUS_CLASSIFICATION_TEST_OK trust=2 infrastructure=3 version=pinned execution=bounded'
