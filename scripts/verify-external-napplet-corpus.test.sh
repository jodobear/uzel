#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
verifier="$root/scripts/verify-external-napplet-corpus.sh"
temporary_corpus=$(mktemp -d)
whitespace_corpus=$(mktemp -d)
null_entry_corpus=$(mktemp -d)
snapshot_corpus=$(mktemp -d)
diagnostic_corpus=$(mktemp -d)
long_diagnostic_corpus=$(mktemp -d)

cleanup() {
  rm -rf -- \
    "$temporary_corpus" \
    "$whitespace_corpus" \
    "$null_entry_corpus" \
    "$snapshot_corpus" \
    "$diagnostic_corpus" \
    "$long_diagnostic_corpus"
}
trap cleanup EXIT

cp -R "$root/fixtures/external-napplet-corpus/." "$temporary_corpus/"
cp -R "$root/fixtures/external-napplet-corpus/." "$null_entry_corpus/"
cp -R "$root/fixtures/external-napplet-corpus/." "$snapshot_corpus/"
cp -R "$root/fixtures/external-napplet-corpus/." "$diagnostic_corpus/"
cp -R "$root/fixtures/external-napplet-corpus/." "$long_diagnostic_corpus/"
real_node=$(command -v node)
real_jq=$(command -v jq)
real_nak=$(command -v nak)

basename_output=$(
  cd "$root/fixtures/external-napplet-corpus"
  bash "$verifier" corpus.lock.json
)
if [[ $basename_output != *"EXTERNAL_NAPPLET_CORPUS_OK entries=4"* ]]; then
  echo "expected a basename lock argument to resolve event files from its directory" >&2
  echo "$basename_output" >&2
  exit 1
fi

private_candidate() {
  local target_variable=$1
  local offset=$2
  local suffix

  printf -v suffix '%010d' "$((($$ * 10) + offset))"
  printf -v "$target_variable" '/tmp/uzel-external-corpus.%s' "$suffix"
}

proposal_mktemp="$temporary_corpus/proposal-mktemp"
# These lines are emitted into the fake mktemp executable.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'printf '\''%s\n'\'' "$UZEL_MKTEMP_CANDIDATE"' > "$proposal_mktemp"
chmod +x "$proposal_mktemp"

failing_mktemp="$temporary_corpus/failing-mktemp"
printf '%s\n' '#!/usr/bin/env bash' 'exit 46' > "$failing_mktemp"
chmod +x "$failing_mktemp"
set +e
mktemp_failure_output=$(
  UZEL_MKTEMP_BIN="$failing_mktemp" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
mktemp_failure_status=$?
set -e
if [[ $mktemp_failure_status -ne 3 ||
  $mktemp_failure_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=mktemp-execution-failed"* ]]; then
  echo "expected mktemp proposal failure to be a typed infrastructure failure" >&2
  echo "$mktemp_failure_output" >&2
  exit 1
fi

hanging_mktemp="$temporary_corpus/hanging-mktemp"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'exec sleep 10' > "$hanging_mktemp"
chmod +x "$hanging_mktemp"
set +e
mktemp_timeout_output=$(
  UZEL_MKTEMP_BIN="$hanging_mktemp" \
    UZEL_MKTEMP_TIMEOUT_SECONDS=0.1 \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
mktemp_timeout_status=$?
set -e
if [[ $mktemp_timeout_status -ne 3 ||
  $mktemp_timeout_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=mktemp-timeout"* ]]; then
  echo "expected a hanging mktemp subprocess to time out as infrastructure failure" >&2
  echo "$mktemp_timeout_output" >&2
  exit 1
fi

continuous_mktemp="$temporary_corpus/continuous-mktemp"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'while :; do printf '\''/tmp/uzel-external-corpus.0000000000\n'\''; done' \
  > "$continuous_mktemp"
chmod +x "$continuous_mktemp"
set +e
mktemp_limit_output=$(
  UZEL_MKTEMP_BIN="$continuous_mktemp" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
mktemp_limit_status=$?
set -e
if [[ $mktemp_limit_status -ne 3 ||
  $mktemp_limit_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=mktemp-output-limit"* ]]; then
  echo "expected continuous mktemp output to hit the bounded candidate capture" >&2
  echo "$mktemp_limit_output" >&2
  exit 1
fi

existing_file_candidate=
private_candidate existing_file_candidate 6
printf '%s\n' 'existing file sentinel' > "$existing_file_candidate"
set +e
existing_file_output=$(
  UZEL_MKTEMP_BIN="$proposal_mktemp" \
    UZEL_MKTEMP_CANDIDATE="$existing_file_candidate" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
existing_file_status=$?
set -e
if [[ $existing_file_status -ne 3 ||
  $existing_file_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=mktemp-preexisting-path"* ||
  $(< "$existing_file_candidate") != "existing file sentinel" ]]; then
  echo "expected a pre-existing file candidate to remain untouched" >&2
  echo "$existing_file_output" >&2
  exit 1
fi
rm -f -- "$existing_file_candidate"

existing_directory_candidate=
private_candidate existing_directory_candidate 7
mkdir -m 700 -- "$existing_directory_candidate"
printf '%s\n' 'existing directory sentinel' > "$existing_directory_candidate/sentinel"
set +e
existing_directory_output=$(
  UZEL_MKTEMP_BIN="$proposal_mktemp" \
    UZEL_MKTEMP_CANDIDATE="$existing_directory_candidate" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
existing_directory_status=$?
set -e
if [[ $existing_directory_status -ne 3 ||
  $existing_directory_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=mktemp-preexisting-path"* ||
  $(< "$existing_directory_candidate/sentinel") != "existing directory sentinel" ]]; then
  echo "expected a pre-existing directory candidate to remain untouched" >&2
  echo "$existing_directory_output" >&2
  exit 1
fi
rm -rf -- "$existing_directory_candidate"

inserted_child_candidate=
private_candidate inserted_child_candidate 8
race_bin="$temporary_corpus/race-bin"
mkdir -p "$race_bin"
real_mkdir=$(command -v mkdir)
# This wrapper deterministically models a surviving helper child inserting a
# file after exclusive directory creation but before fixed-child creation.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -euo pipefail' \
  'candidate=' \
  'for argument in "$@"; do candidate=$argument; done' \
  '"$UZEL_REAL_MKDIR" "$@"' \
  'printf '\''%s\n'\'' '\''fork sentinel'\'' > "$candidate/subprocess-stderr"' \
  > "$race_bin/mkdir"
chmod +x "$race_bin/mkdir"
set +e
inserted_child_output=$(
  PATH="$race_bin:$PATH" \
    UZEL_MKTEMP_BIN="$proposal_mktemp" \
    UZEL_MKTEMP_CANDIDATE="$inserted_child_candidate" \
    UZEL_REAL_MKDIR="$real_mkdir" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
inserted_child_status=$?
set -e
if [[ $inserted_child_status -ne 3 ||
  $inserted_child_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=temp-file-exclusive-create-failed"* ||
  ! -f $inserted_child_candidate/subprocess-stderr ||
  $(< "$inserted_child_candidate/subprocess-stderr") != "fork sentinel" ]]; then
  echo "expected an inserted child sentinel to remain unchanged and unowned" >&2
  echo "$inserted_child_output" >&2
  exit 1
fi
rm -rf -- "$inserted_child_candidate"

assert_cleanup_failure() {
  local label=$1
  local rm_helper=$2
  local expected_code=$3
  local offset=$4
  local candidate
  local filename
  local output
  local status

  private_candidate candidate "$offset"
  set +e
  output=$(
    UZEL_MKTEMP_BIN="$proposal_mktemp" \
      UZEL_MKTEMP_CANDIDATE="$candidate" \
      UZEL_RM_BIN="$rm_helper" \
      UZEL_RM_TIMEOUT_SECONDS=0.1 \
      bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
  )
  status=$?
  set -e
  if [[ $status -ne 3 ||
    $output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=$expected_code"* ||
    $output == *"EXTERNAL_NAPPLET_CORPUS_OK"* ]]; then
    echo "expected $label cleanup to block success with typed infrastructure failure" >&2
    echo "$output" >&2
    exit 1
  fi
  for filename in entries event verified-snapshot subprocess-stdout subprocess-stderr; do
    if [[ ! -f $candidate/$filename ]]; then
      echo "expected $label cleanup fixture $filename to remain observable" >&2
      exit 1
    fi
  done
  rm -rf -- "$candidate"
}

failing_rm="$temporary_corpus/failing-rm"
printf '%s\n' '#!/usr/bin/env bash' 'exit 47' > "$failing_rm"
chmod +x "$failing_rm"
assert_cleanup_failure failing "$failing_rm" rm-execution-failed 1

hanging_rm="$temporary_corpus/hanging-rm"
printf '%s\n' '#!/usr/bin/env bash' 'exec sleep 10' > "$hanging_rm"
chmod +x "$hanging_rm"
assert_cleanup_failure hanging "$hanging_rm" rm-timeout 2

noop_rm="$temporary_corpus/noop-rm"
printf '%s\n' '#!/usr/bin/env bash' 'exit 0' > "$noop_rm"
chmod +x "$noop_rm"
assert_cleanup_failure incomplete "$noop_rm" rm-incomplete 3

preserved_failure_candidate=
private_candidate preserved_failure_candidate 4
typed_trust_node="$temporary_corpus/typed-trust-node"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'printf '\''%s\n'\'' '\''{"format":"uzel.external-napplet-corpus-result.v1","category":"trust","code":"invalid-lock","message":"fixture trust failure"}'\'' >&2' \
  'exit 2' > "$typed_trust_node"
chmod +x "$typed_trust_node"
set +e
preserved_failure_output=$(
  UZEL_MKTEMP_BIN="$proposal_mktemp" \
    UZEL_MKTEMP_CANDIDATE="$preserved_failure_candidate" \
    UZEL_NODE_BIN="$typed_trust_node" \
    UZEL_RM_BIN="$failing_rm" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
preserved_failure_status=$?
set -e
if [[ $preserved_failure_status -ne 2 ||
  $preserved_failure_output != *"EXTERNAL_NAPPLET_CORPUS_TRUST result="* ||
  $preserved_failure_output != *"EXTERNAL_NAPPLET_CORPUS_CLEANUP code=rm-execution-failed preserved_status=2"* ]]; then
  echo "expected cleanup failure to preserve original trust exit status" >&2
  echo "$preserved_failure_output" >&2
  exit 1
fi
rm -rf -- "$preserved_failure_candidate"

null_entry_lock="$null_entry_corpus/corpus.lock.json"
null_entry_lock_next="$null_entry_corpus/corpus.lock.next.json"
jq '.entries[0] = null' "$null_entry_lock" > "$null_entry_lock_next"
mv "$null_entry_lock_next" "$null_entry_lock"
set +e
null_entry_output=$(bash "$verifier" "$null_entry_lock" 2>&1)
null_entry_status=$?
set -e
if [[ $null_entry_status -ne 2 ||
  $null_entry_output != *'EXTERNAL_NAPPLET_CORPUS_TRUST result={"format":"uzel.external-napplet-corpus-result.v1","category":"trust","code":"invalid-lock"'* ]]; then
  echo "expected a null entry to be classified as trust failure" >&2
  echo "$null_entry_output" >&2
  exit 1
fi

diagnostic_lock="$diagnostic_corpus/corpus.lock.json"
diagnostic_lock_next="$diagnostic_corpus/corpus.lock.next.json"
jq '.entries[0].name = "hostile\nname\r"' "$diagnostic_lock" > "$diagnostic_lock_next"
mv "$diagnostic_lock_next" "$diagnostic_lock"
set +e
diagnostic_output=$(bash "$verifier" "$diagnostic_lock" 2>&1)
diagnostic_status=$?
set -e
if [[ $diagnostic_status -ne 2 ||
  $diagnostic_output == *$'\n'* ||
  $diagnostic_output == *$'\r'* ||
  $diagnostic_output != *'"message":"hostile\nname\r: name is not in the audited automation allowlist"'* ]]; then
  echo "expected hostile diagnostic controls to remain one encoded trust result" >&2
  echo "$diagnostic_output" >&2
  exit 1
fi

long_diagnostic_lock="$long_diagnostic_corpus/corpus.lock.json"
long_diagnostic_lock_next="$long_diagnostic_corpus/corpus.lock.next.json"
printf -v long_diagnostic_name '%*s' 1500 ''
long_diagnostic_name=${long_diagnostic_name// /x}
jq --arg name "$long_diagnostic_name" '.entries[0].name = $name' \
  "$long_diagnostic_lock" > "$long_diagnostic_lock_next"
mv "$long_diagnostic_lock_next" "$long_diagnostic_lock"
set +e
long_diagnostic_output=$(bash "$verifier" "$long_diagnostic_lock" 2>&1)
long_diagnostic_status=$?
set -e
if [[ $long_diagnostic_status -ne 2 ||
  $long_diagnostic_output != *'"code":"invalid-lock"'* ||
  $long_diagnostic_output != *"$long_diagnostic_name"* ||
  $long_diagnostic_output == *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE"* ]]; then
  echo "expected a 1500-character typed trust diagnostic to preserve its exact code" >&2
  echo "$long_diagnostic_output" >&2
  exit 1
fi

oversized_trust_result="$temporary_corpus/oversized-trust-result.json"
printf -v oversized_trust_message '%*s' 17000 ''
oversized_trust_message=${oversized_trust_message// /y}
jq -cn --arg message "$oversized_trust_message" \
  '{format:"uzel.external-napplet-corpus-result.v1",category:"trust",code:"invalid-lock",$message}' \
  > "$oversized_trust_result"
oversized_trust_node="$temporary_corpus/oversized-trust-node"
# These lines are emitted into the fake Node launcher.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'cat "$UZEL_TRUST_RESULT" >&2' \
  'exit 2' > "$oversized_trust_node"
chmod +x "$oversized_trust_node"
set +e
oversized_trust_output=$(
  UZEL_NODE_BIN="$oversized_trust_node" \
    UZEL_TRUST_RESULT="$oversized_trust_result" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
oversized_trust_status=$?
set -e
if [[ $oversized_trust_status -ne 3 ||
  $oversized_trust_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=node-invalid-trust-output"* ||
  $oversized_trust_output == *"EXTERNAL_NAPPLET_CORPUS_TRUST result="* ]]; then
  echo "expected an over-16KiB typed diagnostic to fail its bounded transport contract" >&2
  echo "$oversized_trust_output" >&2
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

untyped_exit_two_node="$temporary_corpus/untyped-exit-two-node"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'exit 2' > "$untyped_exit_two_node"
chmod +x "$untyped_exit_two_node"
set +e
untyped_exit_two_output=$(
  UZEL_NODE_BIN="$untyped_exit_two_node" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
untyped_exit_two_status=$?
set -e
if [[ $untyped_exit_two_status -ne 3 ||
  $untyped_exit_two_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=node-invalid-trust-output"* ]]; then
  echo "expected an untyped Node exit 2 to fail as infrastructure" >&2
  echo "$untyped_exit_two_output" >&2
  exit 1
fi

multiple_results_node="$temporary_corpus/multiple-results-node"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'result='\''{"format":"uzel.external-napplet-corpus-result.v1","category":"trust","code":"invalid-lock","message":"hostile"}'\''' \
  'printf '\''%s %s\n'\'' "$result" "$result" >&2' \
  'exit 2' > "$multiple_results_node"
chmod +x "$multiple_results_node"
set +e
multiple_results_output=$(
  UZEL_NODE_BIN="$multiple_results_node" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
multiple_results_status=$?
set -e
if [[ $multiple_results_status -ne 3 ||
  $multiple_results_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=node-invalid-trust-output"* ]]; then
  echo "expected multiple same-line Node results to fail exact-schema validation" >&2
  echo "$multiple_results_output" >&2
  exit 1
fi

typed_exit_two_node="$temporary_corpus/typed-exit-two-node"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'printf '\''%s\n'\'' '\''{"format":"uzel.external-napplet-corpus-result.v1","category":"trust","code":"invalid-lock","message":"hostile"}'\'' >&2' \
  'exit 2' > "$typed_exit_two_node"
chmod +x "$typed_exit_two_node"
continuous_trust_result_jq="$temporary_corpus/continuous-trust-result-jq"
# These lines are emitted into the fake jq script.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'for argument in "$@"; do' \
  '  if [[ $argument == *".[0].format == \"uzel.external-napplet-corpus-result.v1\""* ]]; then' \
  '    while :; do printf '\''%064d\n'\'' 0; done' \
  '  fi' \
  'done' \
  'exec "$UZEL_REAL_JQ" "$@"' > "$continuous_trust_result_jq"
chmod +x "$continuous_trust_result_jq"
set +e
trust_result_limit_output=$(
  UZEL_NODE_BIN="$typed_exit_two_node" \
    UZEL_JQ_BIN="$continuous_trust_result_jq" \
    UZEL_REAL_JQ="$real_jq" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
trust_result_limit_status=$?
set -e
if [[ $trust_result_limit_status -ne 3 ||
  $trust_result_limit_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=jq-output-limit"* ||
  $trust_result_limit_output == *"EXTERNAL_NAPPLET_CORPUS_TRUST result="* ]]; then
  echo "expected continuous trust-result jq stdout to hit the streaming output bound" >&2
  echo "$trust_result_limit_output" >&2
  exit 1
fi

zero_entry_node="$temporary_corpus/zero-entry-node"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -euo pipefail' \
  '"$UZEL_REAL_NODE" "$@" | "$UZEL_REAL_JQ" '\''.entries = [] | .lock.entries = []'\''' \
  > "$zero_entry_node"
chmod +x "$zero_entry_node"
set +e
zero_entry_output=$(
  UZEL_NODE_BIN="$zero_entry_node" \
    UZEL_REAL_NODE="$real_node" \
    UZEL_REAL_JQ="$real_jq" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
zero_entry_status=$?
set -e
if [[ $zero_entry_status -ne 3 ||
  $zero_entry_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=node-invalid-snapshot"* ||
  $zero_entry_output == *"EXTERNAL_NAPPLET_CORPUS_STRUCTURE_OK"* ]]; then
  echo "expected a self-consistent zero-entry Node snapshot to fail before processing" >&2
  echo "$zero_entry_output" >&2
  exit 1
fi

near_limit_node="$temporary_corpus/near-limit-node"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -euo pipefail' \
  '"$UZEL_REAL_NODE" "$@" | "$UZEL_REAL_JQ" '\''
    .entries |= map(
      .eventText = (
        .eventText[0:-2]
        + (" " * (16384 - (.eventText | utf8bytelength)))
        + "}\n"
      )
    )
    | if (tojson | utf8bytelength) <= 65536
      then error("near-limit snapshot did not exceed the retired cap")
      else .
      end
  '\''' \
  > "$near_limit_node"
chmod +x "$near_limit_node"
near_limit_output=$(
  UZEL_NODE_BIN="$near_limit_node" \
    UZEL_REAL_NODE="$real_node" \
    UZEL_REAL_JQ="$real_jq" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json"
)
if [[ $near_limit_output != *"EXTERNAL_NAPPLET_CORPUS_OK entries=4"* ]]; then
  echo "expected four valid 16,384-byte event texts to fit the derived snapshot cap" >&2
  echo "$near_limit_output" >&2
  exit 1
fi

continuous_node="$temporary_corpus/continuous-node"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -euo pipefail' \
  'if [[ $UZEL_NOISE_STREAM == stderr ]]; then exec 1>&2; fi' \
  'while :; do printf '\''%064d\n'\'' 0; done' \
  > "$continuous_node"
chmod +x "$continuous_node"
for noise_stream in stdout stderr; do
  set +e
  continuous_node_output=$(
    UZEL_NODE_BIN="$continuous_node" \
      UZEL_NOISE_STREAM="$noise_stream" \
      bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
  )
  continuous_node_status=$?
  set -e
  if [[ $continuous_node_status -ne 3 ||
    $continuous_node_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=node-output-limit"* ]]; then
    echo "expected continuous Node $noise_stream to hit the streaming output bound" >&2
    echo "$continuous_node_output" >&2
    exit 1
  fi
done

false_wc="$temporary_corpus/false-wc"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'exit 47' > "$false_wc"
chmod +x "$false_wc"
hanging_wc="$temporary_corpus/hanging-wc"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'exec sleep 10' > "$hanging_wc"
chmod +x "$hanging_wc"
noisy_wc="$temporary_corpus/noisy-wc"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'printf '\''not-a-number\nunexpected\n'\''' > "$noisy_wc"
chmod +x "$noisy_wc"
for retired_wc in "$false_wc" "$hanging_wc" "$noisy_wc"; do
  set +e
  retired_wc_output=$(
    UZEL_WC_BIN="$retired_wc" \
      timeout --kill-after=1 2 \
      bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
  )
  retired_wc_status=$?
  set -e
  if [[ $retired_wc_status -ne 0 ||
    $retired_wc_output != *"EXTERNAL_NAPPLET_CORPUS_OK entries=4"* ]]; then
    echo "expected retired hostile UZEL_WC_BIN to have no effect" >&2
    echo "$retired_wc_output" >&2
    exit 1
  fi
done

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

continuous_metadata_jq="$temporary_corpus/continuous-metadata-jq"
# These lines are emitted into the fake jq script.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'for argument in "$@"; do' \
  '  if [[ $argument == *".format, .lock.toolchain.nakVersion"* ]]; then' \
  '    while :; do printf '\''%064d\n'\'' 0; done' \
  '  fi' \
  'done' \
  'exec "$UZEL_REAL_JQ" "$@"' > "$continuous_metadata_jq"
chmod +x "$continuous_metadata_jq"
set +e
metadata_limit_output=$(
  UZEL_JQ_BIN="$continuous_metadata_jq" \
    UZEL_REAL_JQ="$real_jq" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
metadata_limit_status=$?
set -e
if [[ $metadata_limit_status -ne 3 ||
  $metadata_limit_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=jq-output-limit"* ||
  $metadata_limit_output == *"EXTERNAL_NAPPLET_CORPUS_STRUCTURE_OK"* ]]; then
  echo "expected continuous snapshot-metadata jq stdout to hit the streaming output bound" >&2
  echo "$metadata_limit_output" >&2
  exit 1
fi

continuous_entry_scalar_jq="$temporary_corpus/continuous-entry-scalar-jq"
# These lines are emitted into the fake jq script.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'for argument in "$@"; do' \
  '  if [[ $argument == "$UZEL_NOISE_FILTER | select(type == \"string\")" ]]; then' \
  '    if [[ $UZEL_NOISE_STREAM == stderr ]]; then exec 1>&2; fi' \
  '    while :; do printf '\''%064d\n'\'' 0; done' \
  '  fi' \
  'done' \
  'exec "$UZEL_REAL_JQ" "$@"' > "$continuous_entry_scalar_jq"
chmod +x "$continuous_entry_scalar_jq"
recording_nak="$temporary_corpus/recording-nak"
# These lines are emitted into the fake nak script.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'printf '\''%s\n'\'' "${1:-none}" >> "$UZEL_NAK_OPERATIONS"' \
  'exec "$UZEL_REAL_NAK" "$@"' > "$recording_nak"
chmod +x "$recording_nak"
for scalar_filter in .name .naddr; do
  for scalar_stream in stdout stderr; do
    nak_operations="$temporary_corpus/nak-${scalar_filter#.}-$scalar_stream"
    set +e
    entry_scalar_limit_output=$(
      UZEL_JQ_BIN="$continuous_entry_scalar_jq" \
        UZEL_NOISE_FILTER="$scalar_filter" \
        UZEL_NOISE_STREAM="$scalar_stream" \
        UZEL_REAL_JQ="$real_jq" \
        UZEL_NAK_BIN="$recording_nak" \
        UZEL_NAK_OPERATIONS="$nak_operations" \
        UZEL_REAL_NAK="$real_nak" \
        bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
    )
    entry_scalar_limit_status=$?
    set -e
    mapfile -t observed_nak_operations < "$nak_operations"
    if [[ $entry_scalar_limit_status -ne 3 ||
      $entry_scalar_limit_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=jq-output-limit"* ||
      $entry_scalar_limit_output == *"EXTERNAL_NAPPLET_CORPUS_OK"* ||
      ${#observed_nak_operations[@]} -ne 1 ||
      ${observed_nak_operations[0]} != "--version" ]]; then
      echo "expected continuous $scalar_filter jq $scalar_stream to stop before nak data operations" >&2
      echo "$entry_scalar_limit_output" >&2
      exit 1
    fi
  done
done

multiple_name_jq="$temporary_corpus/multiple-name-jq"
# These lines are emitted into the fake jq script.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'for argument in "$@"; do' \
  '  if [[ $argument == ".name | select(type == \"string\")" ]]; then' \
  '    printf '\''good-morning\nextra\n'\''' \
  '    exit 0' \
  '  fi' \
  'done' \
  'exec "$UZEL_REAL_JQ" "$@"' > "$multiple_name_jq"
chmod +x "$multiple_name_jq"
set +e
multiple_name_output=$(
  UZEL_JQ_BIN="$multiple_name_jq" \
    UZEL_REAL_JQ="$real_jq" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
multiple_name_status=$?
set -e
if [[ $multiple_name_status -ne 3 ||
  $multiple_name_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=jq-invalid-output"* ||
  $multiple_name_output == *"EXTERNAL_NAPPLET_CORPUS_OK"* ]]; then
  echo "expected multiple entry-name scalars to fail before use" >&2
  echo "$multiple_name_output" >&2
  exit 1
fi

nul_suffix_name_jq="$temporary_corpus/nul-suffix-name-jq"
# These lines are emitted into the fake jq script.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'for argument in "$@"; do' \
  '  if [[ $argument == ".name | select(type == \"string\")" ]]; then' \
  '    printf '\''good-morning\0suffix'\''' \
  '    exit 0' \
  '  fi' \
  'done' \
  'exec "$UZEL_REAL_JQ" "$@"' > "$nul_suffix_name_jq"
chmod +x "$nul_suffix_name_jq"
set +e
nul_suffix_name_output=$(
  UZEL_JQ_BIN="$nul_suffix_name_jq" \
    UZEL_REAL_JQ="$real_jq" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
nul_suffix_name_status=$?
set -e
if [[ $nul_suffix_name_status -ne 3 ||
  $nul_suffix_name_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=jq-invalid-output"* ||
  $nul_suffix_name_output == *"EXTERNAL_NAPPLET_CORPUS_OK"* ]]; then
  echo "expected a NUL-suffixed entry-name scalar to fail before use" >&2
  echo "$nul_suffix_name_output" >&2
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

continuous_nak="$temporary_corpus/continuous-nak"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -euo pipefail' \
  'emit_forever() {' \
  '  if [[ $UZEL_NOISE_STREAM == stderr ]]; then exec 1>&2; fi' \
  '  while :; do printf '\''%064d\n'\'' 0; done' \
  '}' \
  'case "${UZEL_NOISE_OPERATION}:${1:-}" in' \
  '  version:--version|decode:decode) emit_forever ;;' \
  'esac' \
  'case "${1:-}" in' \
  '  --version) echo "nak version 0.20.1" ;;' \
  '  verify) exit 0 ;;' \
  '  decode) exit 42 ;;' \
  '  *) exit 42 ;;' \
  'esac' > "$continuous_nak"
chmod +x "$continuous_nak"
for noise_operation in version decode; do
  for noise_stream in stdout stderr; do
    set +e
    continuous_nak_output=$(
      UZEL_NAK_BIN="$continuous_nak" \
        UZEL_NOISE_OPERATION="$noise_operation" \
        UZEL_NOISE_STREAM="$noise_stream" \
        bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
    )
    continuous_nak_status=$?
    set -e
    if [[ $continuous_nak_status -ne 3 ||
      $continuous_nak_output != *"EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=nak-output-limit"* ]]; then
      echo "expected continuous nak $noise_operation $noise_stream to hit the streaming output bound" >&2
      echo "$continuous_nak_output" >&2
      exit 1
    fi
  done
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

cp -R "$root/fixtures/external-napplet-corpus/." "$whitespace_corpus/"
rotating_node="$whitespace_corpus/rotating-node"
# These lines are emitted into the fake Node launcher.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -euo pipefail' \
  '"$UZEL_REAL_NODE" "$@" | "$UZEL_REAL_JQ" '\''
    .entries[0].eventText as $first
    | .entries[0].eventText = .entries[1].eventText
    | .entries[1].eventText = .entries[2].eventText
    | .entries[2].eventText = .entries[3].eventText
    | .entries[3].eventText = $first
  '\''' \
  > "$rotating_node"
chmod +x "$rotating_node"
set +e
rotating_output=$(
  UZEL_NODE_BIN="$rotating_node" \
    UZEL_REAL_NODE="$real_node" \
    UZEL_REAL_JQ="$real_jq" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json" 2>&1
)
rotating_status=$?
set -e
if [[ $rotating_status -ne 2 ||
  $rotating_output != *"EXTERNAL_NAPPLET_CORPUS_TRUST code=event-id-drift"* ||
  $rotating_output == *"EXTERNAL_NAPPLET_CORPUS_STRUCTURE_OK"* ]]; then
  echo "expected rotated valid signed events to fail audited event-id binding" >&2
  echo "$rotating_output" >&2
  exit 1
fi

whitespace_node="$whitespace_corpus/whitespace-node"
# These lines are emitted into the fake Node launcher.
# shellcheck disable=SC2016
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'set -euo pipefail' \
  '"$UZEL_REAL_NODE" "$@" | "$UZEL_REAL_JQ" '\''.entries |= map(.eventText = ("  \n" + .eventText + "  \n"))'\''' \
  > "$whitespace_node"
chmod +x "$whitespace_node"
whitespace_output=$(
  UZEL_NODE_BIN="$whitespace_node" \
    UZEL_REAL_NODE="$real_node" \
    UZEL_REAL_JQ="$real_jq" \
    bash "$verifier" "$root/fixtures/external-napplet-corpus/corpus.lock.json"
)
if [[ $whitespace_output != *"EXTERNAL_NAPPLET_CORPUS_OK entries=4"* ]]; then
  echo "expected harmless signed-event JSON whitespace to remain accepted" >&2
  echo "$whitespace_output" >&2
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

echo 'EXTERNAL_NAPPLET_CORPUS_CLASSIFICATION_TEST_OK trust=2 infrastructure=3 version=pinned setup=private-exclusive node=typed-bounded trust-result=bounded-preserved jq=bounded entry-scalars=exact-bounded nak=bounded event-id=bound event-whitespace=allowed snapshot=exact-four snapshot-cap=derived subprocess=stream-bounded size-probe=builtin cleanup=owned-bounded-observable'
