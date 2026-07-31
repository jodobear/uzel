#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
verifier="$root/scripts/verify-external-napplet-corpus.sh"
lock="$root/fixtures/external-napplet-corpus/corpus.lock.json"
temporary_directory=$(mktemp -d)
trap 'rm -rf -- "$temporary_directory"' EXIT
real_node=$(command -v node)
real_jq=$(command -v jq)

fail() {
  echo "$1" >&2
  if [[ -n ${2:-} ]]; then
    echo "$2" >&2
  fi
  exit 1
}

mutating_node="$temporary_directory/mutating-node"
# These lines are emitted into the fake Node launcher.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -euo pipefail' \
  '"$UZEL_REAL_NODE" "$@" | "$UZEL_REAL_JQ" "$UZEL_LOCK_MUTATION"' \
  > "$mutating_node"
chmod +x "$mutating_node"

assert_lock_mutation_refused() {
  local label=$1
  local mutation=$2
  local output
  local status
  set +e
  output=$(
    UZEL_NODE_BIN="$mutating_node" \
      UZEL_REAL_NODE="$real_node" \
      UZEL_REAL_JQ="$real_jq" \
      UZEL_LOCK_MUTATION="$mutation" \
      bash "$verifier" "$lock" 2>&1
  )
  status=$?
  set -e
  if [[ $status -ne 2 ||
    $output != *"EXTERNAL_NAPPLET_CORPUS_TRUST code=audited-lock-drift"* ||
    $output == *"EXTERNAL_NAPPLET_CORPUS_STRUCTURE_OK"* ]]; then
    fail "expected $label mutation to fail trust before STRUCTURE_OK" "$output"
  fi
}

assert_lock_mutation_refused source.commit \
  '.lock.source.commit = "0000000000000000000000000000000000000000"'
assert_lock_mutation_refused domains \
  '.lock.entries[0].domains = ["wallet"] | .entries[0].entry.domains = ["wallet"]'
assert_lock_mutation_refused eventId \
  '.lock.entries[0].eventId = ("0" * 64) | .entries[0].entry.eventId = ("0" * 64)'
assert_lock_mutation_refused safeAutomation \
  '.lock.entries[0].safeAutomation = "upload-and-publish" | .entries[0].entry.safeAutomation = "upload-and-publish"'

assert_sha256sum_infrastructure_failure() {
  local label=$1
  local helper=$2
  local expected_code=$3
  local output
  local status
  shift 3
  set +e
  output=$(env UZEL_SHA256SUM_BIN="$helper" "$@" bash "$verifier" "$lock" 2>&1)
  status=$?
  set -e
  if [[ $status -ne 3 ||
    $output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=$expected_code"* ||
    $output == *"EXTERNAL_NAPPLET_CORPUS_STRUCTURE_OK"* ]]; then
    fail "expected $label sha256sum to fail infrastructure before STRUCTURE_OK" "$output"
  fi
}

failing_sha256sum="$temporary_directory/failing-sha256sum"
printf '%s\n' '#!/usr/bin/env bash' 'exit 47' > "$failing_sha256sum"
chmod +x "$failing_sha256sum"
assert_sha256sum_infrastructure_failure failing "$failing_sha256sum" \
  sha256sum-execution-failed

malformed_sha256sum="$temporary_directory/malformed-sha256sum"
printf '%s\n' '#!/usr/bin/env bash' 'printf '\''not-a-digest\n'\''' > "$malformed_sha256sum"
chmod +x "$malformed_sha256sum"
assert_sha256sum_infrastructure_failure malformed "$malformed_sha256sum" \
  sha256sum-invalid-output

hanging_sha256sum="$temporary_directory/hanging-sha256sum"
printf '%s\n' '#!/usr/bin/env bash' 'exec sleep 10' > "$hanging_sha256sum"
chmod +x "$hanging_sha256sum"
assert_sha256sum_infrastructure_failure hanging "$hanging_sha256sum" \
  sha256sum-timeout UZEL_SHA256SUM_TIMEOUT_SECONDS=0.1

noisy_sha256sum="$temporary_directory/noisy-sha256sum"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'while :; do printf '\''%064d\n'\'' 0; done' \
  > "$noisy_sha256sum"
chmod +x "$noisy_sha256sum"
assert_sha256sum_infrastructure_failure noisy "$noisy_sha256sum" \
  sha256sum-output-limit

set +e
missing_output=$(
  UZEL_SHA256SUM_BIN=uzel-deliberately-missing-sha256sum \
    bash "$verifier" "$lock" 2>&1
)
missing_status=$?
set -e
if [[ $missing_status -ne 3 ||
  $missing_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=sha256sum-unavailable"* ]]; then
  fail "expected missing sha256sum to fail as infrastructure" "$missing_output"
fi

echo 'EXTERNAL_NAPPLET_CORPUS_LOCK_TEST_OK digest=canonical-complete mutations=4 sha256sum=bounded-classified'
