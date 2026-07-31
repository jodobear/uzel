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

echo 'EXTERNAL_NAPPLET_CORPUS_CLASSIFICATION_TEST_OK trust=2 infrastructure=3'
