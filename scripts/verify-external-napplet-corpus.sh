#!/usr/bin/env bash
set -euo pipefail

script_directory=${BASH_SOURCE[0]%/*}
if [[ $script_directory == "${BASH_SOURCE[0]}" ]]; then
  script_directory=.
fi
root=$(cd "$script_directory/.." && pwd)
lock=${1:-"$root/fixtures/external-napplet-corpus/corpus.lock.json"}
nak_bin=${UZEL_NAK_BIN:-nak}
jq_bin=${UZEL_JQ_BIN:-jq}
sha256sum_bin=${UZEL_SHA256SUM_BIN:-sha256sum}
node_bin=${UZEL_NODE_BIN:-node}
timeout_bin=${UZEL_TIMEOUT_BIN:-timeout}
mktemp_bin=${UZEL_MKTEMP_BIN:-mktemp}
rm_bin=${UZEL_RM_BIN:-rm}
node_timeout_seconds=${UZEL_NODE_TIMEOUT_SECONDS:-10}
jq_timeout_seconds=${UZEL_JQ_TIMEOUT_SECONDS:-10}
sha256sum_timeout_seconds=${UZEL_SHA256SUM_TIMEOUT_SECONDS:-10}
nak_timeout_seconds=${UZEL_NAK_TIMEOUT_SECONDS:-10}
mktemp_timeout_seconds=${UZEL_MKTEMP_TIMEOUT_SECONDS:-10}
rm_timeout_seconds=${UZEL_RM_TIMEOUT_SECONDS:-10}
nak_output_limit_bytes=65536
canonical_lock_output_limit_bytes=65536
event_binding_output_limit_bytes=1024
snapshot_metadata_output_limit_bytes=1024
sha256sum_output_limit_bytes=1024
expected_lock_sha256=1994fc5940e51d0fd9a9567a1a82a0f836bd93ab496022cf43933eb69653c632
# Four 16,384-byte event texts can each double when embedded as JSON strings.
# Two more doubled evidence-file allowances cover the lock and duplicated entry
# metadata, and 65,536 bytes cover the fixed snapshot envelope/schema overhead.
node_output_limit_bytes=262144
entries_file=
verified_event_file=
verified_snapshot_file=
subprocess_stdout_file=
subprocess_stderr_file=
bounded_status=
bounded_output_exceeded=
cleanup_code=
cleanup_message=

infrastructure_failure() {
  echo "EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=$1 message=$2" >&2
  exit 3
}

trust_failure() {
  echo "EXTERNAL_NAPPLET_CORPUS_TRUST code=$1 message=$2" >&2
  exit 2
}

cleanup_files() {
  local rm_status
  local temporary_file
  local temporary_files=()

  for temporary_file in \
    "$entries_file" \
    "$verified_event_file" \
    "$verified_snapshot_file" \
    "$subprocess_stdout_file" \
    "$subprocess_stderr_file"; do
    if [[ -n $temporary_file ]]; then
      temporary_files+=("$temporary_file")
    fi
  done

  if [[ ${#temporary_files[@]} -eq 0 ]]; then
    return 0
  fi

  cleanup_code=
  cleanup_message=
  if "$timeout_bin" --kill-after=1 "$rm_timeout_seconds" \
    "$rm_bin" -f -- "${temporary_files[@]}" >/dev/null 2>&1; then
    rm_status=0
  else
    rm_status=$?
  fi

  if [[ $rm_status -eq 124 || $rm_status -eq 137 ]]; then
    cleanup_code=rm-timeout
    cleanup_message="temporary-file cleanup exceeded ${rm_timeout_seconds}s"
    return 1
  fi
  if [[ $rm_status -ne 0 ]]; then
    cleanup_code=rm-execution-failed
    cleanup_message="temporary-file cleanup failed with status $rm_status"
    return 1
  fi
  for temporary_file in "${temporary_files[@]}"; do
    if [[ -e $temporary_file || -L $temporary_file ]]; then
      cleanup_code=rm-incomplete
      cleanup_message="temporary-file cleanup left one or more files"
      return 1
    fi
  done

  entries_file=
  verified_event_file=
  verified_snapshot_file=
  subprocess_stdout_file=
  subprocess_stderr_file=
  return 0
}

cleanup_on_exit() {
  local original_status=$?
  trap - EXIT
  if ! cleanup_files; then
    if [[ $original_status -eq 0 ]]; then
      echo "EXTERNAL_NAPPLET_CORPUS_INFRASTRUCTURE code=$cleanup_code message=$cleanup_message" >&2
      exit 3
    fi
    echo "EXTERNAL_NAPPLET_CORPUS_CLEANUP code=$cleanup_code preserved_status=$original_status message=$cleanup_message" >&2
  fi
  exit "$original_status"
}

run_bounded_subprocess() {
  local stdout_file=$1
  local stderr_file=$2
  local timeout_seconds=$3
  local output_limit_bytes=$4
  # Both explicit caps are exact KiB multiples; Bash `ulimit -f` uses KiB.
  local file_limit_blocks=$((output_limit_bytes / 1024))
  shift 4

  : > "$stdout_file"
  : > "$stderr_file"
  set +e
  {
    (
      ulimit -c 0
      ulimit -f "$file_limit_blocks" || exit 125
      exec "$timeout_bin" --kill-after=1 "$timeout_seconds" "$@"
    ) > "$stdout_file" 2> "$stderr_file"
  } 2>/dev/null
  bounded_status=$?
  set -e
  bounded_output_exceeded=0
  if [[ $bounded_status -eq 153 ]]; then
    bounded_output_exceeded=1
  fi
}
trap cleanup_on_exit EXIT

create_temp_file() {
  local target_variable=$1
  local description=$2
  local candidate
  local mktemp_status

  set +e
  candidate=$("$timeout_bin" --kill-after=1 "$mktemp_timeout_seconds" "$mktemp_bin" 2>/dev/null)
  mktemp_status=$?
  set -e
  if [[ -n $candidate && $candidate != *$'\n'* && -f $candidate && ! -L $candidate ]]; then
    printf -v "$target_variable" '%s' "$candidate"
  fi
  if [[ $mktemp_status -eq 124 || $mktemp_status -eq 137 ]]; then
    infrastructure_failure mktemp-timeout "$description temporary-file creation exceeded ${mktemp_timeout_seconds}s"
  fi
  if [[ $mktemp_status -ne 0 ]]; then
    infrastructure_failure mktemp-execution-failed "$description temporary-file creation failed with status $mktemp_status"
  fi
  if [[ -z ${!target_variable} ]]; then
    infrastructure_failure mktemp-invalid-output "$description temporary-file creation returned no regular file"
  fi
}

command -v "$timeout_bin" >/dev/null 2>&1 || infrastructure_failure timeout-unavailable "timeout is required"
command -v "$rm_bin" >/dev/null 2>&1 || infrastructure_failure rm-unavailable "rm is required"
command -v "$mktemp_bin" >/dev/null 2>&1 || infrastructure_failure mktemp-unavailable "mktemp is required"
command -v "$node_bin" >/dev/null 2>&1 || infrastructure_failure node-unavailable "node is required"
command -v "$jq_bin" >/dev/null 2>&1 || infrastructure_failure jq-unavailable "jq is required"
command -v "$sha256sum_bin" >/dev/null 2>&1 || infrastructure_failure sha256sum-unavailable "sha256sum is required"
command -v "$nak_bin" >/dev/null 2>&1 || infrastructure_failure nak-unavailable "pinned nak is required"
if [[ ! $node_timeout_seconds =~ ^(([1-9][0-9]*)(\.[0-9]+)?|0\.[0-9]*[1-9][0-9]*)$ ]]; then
  infrastructure_failure invalid-timeout "node timeout must be a positive number of seconds"
fi
if [[ ! $jq_timeout_seconds =~ ^(([1-9][0-9]*)(\.[0-9]+)?|0\.[0-9]*[1-9][0-9]*)$ ]]; then
  infrastructure_failure invalid-timeout "jq timeout must be a positive number of seconds"
fi
if [[ ! $sha256sum_timeout_seconds =~ ^(([1-9][0-9]*)(\.[0-9]+)?|0\.[0-9]*[1-9][0-9]*)$ ]]; then
  infrastructure_failure invalid-timeout "sha256sum timeout must be a positive number of seconds"
fi
if [[ ! $nak_timeout_seconds =~ ^(([1-9][0-9]*)(\.[0-9]+)?|0\.[0-9]*[1-9][0-9]*)$ ]]; then
  infrastructure_failure invalid-timeout "nak timeout must be a positive number of seconds"
fi
if [[ ! $mktemp_timeout_seconds =~ ^(([1-9][0-9]*)(\.[0-9]+)?|0\.[0-9]*[1-9][0-9]*)$ ]]; then
  infrastructure_failure invalid-timeout "mktemp timeout must be a positive number of seconds"
fi
if [[ ! $rm_timeout_seconds =~ ^(([1-9][0-9]*)(\.[0-9]+)?|0\.[0-9]*[1-9][0-9]*)$ ]]; then
  infrastructure_failure invalid-timeout "rm timeout must be a positive number of seconds"
fi

create_temp_file entries_file entries
create_temp_file verified_event_file event
create_temp_file verified_snapshot_file verified-snapshot
create_temp_file subprocess_stdout_file subprocess-stdout
create_temp_file subprocess_stderr_file subprocess-stderr

run_bounded_subprocess "$verified_snapshot_file" "$subprocess_stderr_file" \
  "$node_timeout_seconds" "$node_output_limit_bytes" \
  "$node_bin" "$root/scripts/verify-external-napplet-corpus.mjs" --snapshot-json "$lock"
node_status=$bounded_status
if [[ $bounded_output_exceeded -eq 1 ]]; then
  infrastructure_failure node-output-limit \
    "corpus structure verifier exceeded ${node_output_limit_bytes} bytes on stdout or stderr"
fi
if [[ $node_status -eq 124 || $node_status -eq 137 ]]; then
  infrastructure_failure node-timeout "corpus structure verifier exceeded ${node_timeout_seconds}s"
fi
if [[ $node_status -eq 2 ]]; then
  node_trust_status=1
  if [[ ! -s $verified_snapshot_file ]]; then
    set +e
    node_trust_result=$("$timeout_bin" --kill-after=1 "$jq_timeout_seconds" "$jq_bin" -c -e -s \
      'select(
        length == 1
        and (.[0] | type) == "object"
        and ((.[0] | keys) == ["category", "code", "format", "message"])
        and .[0].format == "uzel.external-napplet-corpus-result.v1"
        and .[0].category == "trust"
        and (.[0].code | type == "string" and test("^[a-z][a-z0-9-]*$"))
        and (.[0].message | type == "string" and length > 0)
      ) | .[0]' "$subprocess_stderr_file")
    node_trust_status=$?
    set -e
  fi
  if [[ $node_trust_status -eq 124 || $node_trust_status -eq 137 ]]; then
    infrastructure_failure jq-timeout "Node trust-result validation exceeded ${jq_timeout_seconds}s"
  fi
  if [[ $node_trust_status -eq 0 ]]; then
    printf 'EXTERNAL_NAPPLET_CORPUS_TRUST result=%s\n' "$node_trust_result" >&2
    exit 2
  fi
  infrastructure_failure node-invalid-trust-output \
    "corpus structure verifier exited 2 without exactly one typed trust result"
fi
if [[ $node_status -ne 0 ]]; then
  infrastructure_failure node-execution-failed "corpus structure verifier failed with status $node_status"
fi
if [[ -s $subprocess_stderr_file ]]; then
  infrastructure_failure node-invalid-snapshot \
    "successful corpus structure verifier wrote unexpected stderr"
fi

run_bounded_subprocess "$subprocess_stdout_file" "$subprocess_stderr_file" \
  "$jq_timeout_seconds" "$snapshot_metadata_output_limit_bytes" \
  "$jq_bin" -r \
  '
    def exact_object($fields): type == "object" and keys == $fields;
    def string_array: type == "array" and all(.[]; type == "string");
    def audited_entry:
      exact_object([
        "artifact", "author", "createdAt", "dTag", "domains", "domainsSource",
        "eventFile", "eventId", "kind", "naddr", "name", "relayHints",
        "safeAutomation", "servers", "title"
      ])
      and (.artifact | exact_object(["aggregateSha256", "logicalPath", "sha256", "sizeBytes"]))
      and (.artifact.aggregateSha256 | type == "string")
      and (.artifact.logicalPath | type == "string")
      and (.artifact.sha256 | type == "string")
      and (.artifact.sizeBytes | type == "number")
      and (.author | type == "string")
      and (.createdAt | type == "number")
      and (.dTag | type == "string")
      and (.domains | string_array)
      and (.domainsSource | type == "string")
      and (.eventFile | type == "string")
      and (.eventId | type == "string")
      and (.kind | type == "number")
      and (.naddr | type == "string")
      and (.name | type == "string")
      and (.relayHints | string_array)
      and (.safeAutomation | type == "string")
      and (.servers | string_array)
      and (.title | type == "string");
    select(
      exact_object(["entries", "format", "lock"])
      and .format == "uzel.verified-external-napplet-corpus.v1"
      and (.lock | exact_object(["entries", "failurePolicy", "format", "publisher", "source", "toolchain"]))
      and .lock.format == "uzel.external-napplet-corpus.v1"
      and (.lock.publisher | type == "string")
      and (.lock.source | exact_object(["auditedOn", "commit", "commitUrl", "license", "licenseUrl", "publishedAt", "repository"]))
      and all(.lock.source[]; type == "string")
      and (.lock.toolchain | exact_object(["nakVersion"]))
      and (.lock.toolchain.nakVersion | type == "string")
      and (.lock.failurePolicy | exact_object(["infrastructure", "trust"]))
      and (.lock.failurePolicy.infrastructure | string_array)
      and (.lock.failurePolicy.trust | string_array)
      and (.lock.entries | type == "array" and length == 4)
      and (.entries | type == "array" and length == 4)
      and all(.entries[];
        exact_object(["entry", "eventText"])
        and (.entry | audited_entry)
        and (.eventText | type == "string" and utf8bytelength > 0 and utf8bytelength <= 16384)
      )
      and [.entries[].entry.name] == ["good-morning", "rubik-cube", "nap-feed", "wifi-map"]
      and [.entries[].entry] == .lock.entries
    )
    | .format, .lock.toolchain.nakVersion, .lock.source.commit, (.entries | length)
  ' "$verified_snapshot_file"
jq_status=$bounded_status
if [[ $bounded_output_exceeded -eq 1 ]]; then
  infrastructure_failure jq-output-limit \
    "verified snapshot metadata exceeded ${snapshot_metadata_output_limit_bytes} bytes on stdout or stderr"
fi
if [[ $jq_status -eq 124 || $jq_status -eq 137 ]]; then
  infrastructure_failure jq-timeout "reading verified snapshot metadata exceeded ${jq_timeout_seconds}s"
fi
if [[ $jq_status -eq 1 || $jq_status -eq 4 ]]; then
  infrastructure_failure node-invalid-snapshot \
    "successful corpus structure verifier returned an invalid snapshot"
fi
if [[ $jq_status -ne 0 ]]; then
  infrastructure_failure jq-execution-failed \
    "validating the successful corpus snapshot failed with status $jq_status"
fi
if [[ -s $subprocess_stderr_file ]]; then
  infrastructure_failure jq-invalid-output "verified snapshot metadata wrote unexpected stderr"
fi
snapshot_metadata=$(< "$subprocess_stdout_file")
mapfile -t metadata_lines <<< "$snapshot_metadata"
if [[ ${#metadata_lines[@]} -ne 4 || ${metadata_lines[0]} != "uzel.verified-external-napplet-corpus.v1" ]]; then
  infrastructure_failure node-invalid-snapshot "verified snapshot metadata was incomplete"
fi
expected_nak_version=${metadata_lines[1]}
source_commit=${metadata_lines[2]}
expected_entry_count=${metadata_lines[3]}

run_bounded_subprocess "$entries_file" "$subprocess_stderr_file" \
  "$jq_timeout_seconds" "$canonical_lock_output_limit_bytes" \
  "$jq_bin" -c -S -e '.lock' "$verified_snapshot_file"
canonical_lock_status=$bounded_status
if [[ $bounded_output_exceeded -eq 1 ]]; then
  infrastructure_failure jq-output-limit \
    "canonical lock exceeded ${canonical_lock_output_limit_bytes} bytes on stdout or stderr"
fi
if [[ $canonical_lock_status -eq 124 || $canonical_lock_status -eq 137 ]]; then
  infrastructure_failure jq-timeout "canonical lock serialization exceeded ${jq_timeout_seconds}s"
fi
if [[ $canonical_lock_status -ne 0 ]]; then
  infrastructure_failure jq-execution-failed \
    "canonical lock serialization failed with status $canonical_lock_status"
fi
if [[ ! -s $entries_file || -s $subprocess_stderr_file ]]; then
  infrastructure_failure jq-invalid-output "canonical lock serialization returned invalid output"
fi

run_bounded_subprocess "$subprocess_stdout_file" "$subprocess_stderr_file" \
  "$sha256sum_timeout_seconds" "$sha256sum_output_limit_bytes" \
  "$sha256sum_bin" "$entries_file"
sha256sum_status=$bounded_status
if [[ $bounded_output_exceeded -eq 1 ]]; then
  infrastructure_failure sha256sum-output-limit \
    "canonical lock digest exceeded ${sha256sum_output_limit_bytes} bytes on stdout or stderr"
fi
if [[ $sha256sum_status -eq 124 || $sha256sum_status -eq 137 ]]; then
  infrastructure_failure sha256sum-timeout \
    "canonical lock digest exceeded ${sha256sum_timeout_seconds}s"
fi
if [[ $sha256sum_status -ne 0 ]]; then
  infrastructure_failure sha256sum-execution-failed \
    "canonical lock digest failed with status $sha256sum_status"
fi
lock_digest_output=$(< "$subprocess_stdout_file")
lock_digest=${lock_digest_output%% *}
if [[ -s $subprocess_stderr_file || ! $lock_digest =~ ^[0-9a-f]{64}$ ||
  $lock_digest_output != "$lock_digest  $entries_file" ]]; then
  infrastructure_failure sha256sum-invalid-output \
    "canonical lock digest returned invalid output"
fi
if [[ $lock_digest != "$expected_lock_sha256" ]]; then
  trust_failure audited-lock-drift "canonical audited lock digest drifted"
fi

run_bounded_subprocess "$subprocess_stdout_file" "$subprocess_stderr_file" \
  "$jq_timeout_seconds" "$event_binding_output_limit_bytes" \
  "$jq_bin" -r \
  '
    [
      .entries[]
      | (.eventText | try fromjson catch null) as $event
      | if ($event | type) != "object"
        then "invalid-event-json"
        elif ($event.id | type) != "string" or $event.id != .entry.eventId
        then "event-id-drift"
        else "ok"
        end
    ]
    | if index("invalid-event-json")
      then "invalid-event-json"
      elif index("event-id-drift")
      then "event-id-drift"
      else "ok"
      end
  ' "$verified_snapshot_file"
event_binding_status=$bounded_status
if [[ $bounded_output_exceeded -eq 1 ]]; then
  infrastructure_failure jq-output-limit \
    "event-id binding exceeded ${event_binding_output_limit_bytes} bytes on stdout or stderr"
fi
if [[ $event_binding_status -eq 124 || $event_binding_status -eq 137 ]]; then
  infrastructure_failure jq-timeout "event-id binding exceeded ${jq_timeout_seconds}s"
fi
if [[ $event_binding_status -ne 0 ]]; then
  infrastructure_failure jq-execution-failed \
    "event-id binding failed with status $event_binding_status"
fi
event_binding_result=$(< "$subprocess_stdout_file")
if [[ -s $subprocess_stderr_file ]]; then
  infrastructure_failure jq-invalid-output "event-id binding wrote unexpected stderr"
fi
case "$event_binding_result" in
  ok) ;;
  invalid-event-json)
    trust_failure invalid-event-json "retained signed event is not one JSON object"
    ;;
  event-id-drift)
    trust_failure event-id-drift "retained signed event does not match audited event id"
    ;;
  *)
    infrastructure_failure jq-invalid-output "event-id binding returned invalid output"
    ;;
esac
echo "EXTERNAL_NAPPLET_CORPUS_STRUCTURE_OK entries=$expected_entry_count commit=$source_commit mode=offline"

run_bounded_subprocess "$subprocess_stdout_file" "$subprocess_stderr_file" \
  "$nak_timeout_seconds" "$nak_output_limit_bytes" "$nak_bin" --version
nak_version_status=$bounded_status
if [[ $bounded_output_exceeded -eq 1 ]]; then
  infrastructure_failure nak-output-limit \
    "nak --version exceeded ${nak_output_limit_bytes} bytes on stdout or stderr"
fi
if [[ $nak_version_status -eq 124 || $nak_version_status -eq 137 ]]; then
  infrastructure_failure nak-timeout "nak --version exceeded ${nak_timeout_seconds}s"
fi
if [[ $nak_version_status -ne 0 ]]; then
  infrastructure_failure nak-execution-failed "nak --version failed with status $nak_version_status"
fi
if [[ -s $subprocess_stderr_file ]]; then
  infrastructure_failure nak-invalid-output "nak --version wrote unexpected stderr"
fi
nak_version_output=$(< "$subprocess_stdout_file")
if [[ $nak_version_output != "nak version $expected_nak_version" ]]; then
  infrastructure_failure nak-version-mismatch "expected nak version $expected_nak_version"
fi

set +e
"$timeout_bin" --kill-after=1 "$jq_timeout_seconds" "$jq_bin" -r \
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
  "$verified_snapshot_file" > "$entries_file"
jq_status=$?
set -e
if [[ $jq_status -eq 124 || $jq_status -eq 137 ]]; then
  infrastructure_failure jq-timeout "entry enumeration exceeded ${jq_timeout_seconds}s"
fi
if [[ $jq_status -ne 0 ]]; then
  infrastructure_failure jq-execution-failed "entry enumeration failed with status $jq_status"
fi

entry_count=0
while IFS= read -r entry_json; do
  ((entry_count += 1))
  set +e
  name=$("$timeout_bin" --kill-after=1 "$jq_timeout_seconds" "$jq_bin" -r '.name' <<< "$entry_json")
  name_status=$?
  naddr=$("$timeout_bin" --kill-after=1 "$jq_timeout_seconds" "$jq_bin" -r '.naddr' <<< "$entry_json")
  naddr_status=$?
  set -e
  if [[ $name_status -eq 124 || $name_status -eq 137 ||
    $naddr_status -eq 124 || $naddr_status -eq 137 ]]; then
    infrastructure_failure jq-timeout "entry decoding exceeded ${jq_timeout_seconds}s"
  fi
  if [[ $name_status -ne 0 || $naddr_status -ne 0 ]]; then
    infrastructure_failure jq-execution-failed "entry decoding failed"
  fi
  set +e
  "$timeout_bin" --kill-after=1 "$jq_timeout_seconds" "$jq_bin" -j '.eventText' \
    <<< "$entry_json" > "$verified_event_file"
  event_status=$?
  set -e
  if [[ $event_status -eq 124 || $event_status -eq 137 ]]; then
    infrastructure_failure jq-timeout "$name verified event extraction exceeded ${jq_timeout_seconds}s"
  fi
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

  run_bounded_subprocess "$subprocess_stdout_file" "$subprocess_stderr_file" \
    "$nak_timeout_seconds" "$nak_output_limit_bytes" "$nak_bin" decode "$naddr"
  decode_status=$bounded_status
  if [[ $bounded_output_exceeded -eq 1 ]]; then
    infrastructure_failure nak-output-limit \
      "$name nak decode exceeded ${nak_output_limit_bytes} bytes on stdout or stderr"
  fi
  if [[ $decode_status -eq 124 || $decode_status -eq 137 ]]; then
    infrastructure_failure nak-timeout "$name nak decode exceeded ${nak_timeout_seconds}s"
  elif [[ $decode_status -ne 0 ]]; then
    if [[ $decode_status -eq 123 ]]; then
      trust_failure coordinate-drift "$name naddr failed nak decoding"
    fi
    infrastructure_failure nak-execution-failed "$name nak decode failed with status $decode_status"
  fi
  if [[ -s $subprocess_stderr_file ]]; then
    infrastructure_failure nak-invalid-output "$name nak decode wrote unexpected stderr"
  fi
  set +e
  "$timeout_bin" --kill-after=1 "$jq_timeout_seconds" "$jq_bin" -e -s \
    'length == 1 and (.[0] | type == "object")' \
    "$subprocess_stdout_file" >/dev/null 2>&1
  decoded_shape_status=$?
  set -e
  if [[ $decoded_shape_status -eq 124 || $decoded_shape_status -eq 137 ]]; then
    infrastructure_failure jq-timeout "$name nak output validation exceeded ${jq_timeout_seconds}s"
  fi
  if [[ $decoded_shape_status -ne 0 ]]; then
    infrastructure_failure nak-invalid-output "$name nak decode returned anything other than one JSON object"
  fi
  # jq expands these variables, not Bash.
  # shellcheck disable=SC2016
  set +e
  "$timeout_bin" --kill-after=1 "$jq_timeout_seconds" "$jq_bin" -e \
    --argjson locked "$entry_json" \
    '.pubkey == $locked.author
      and .kind == $locked.kind
      and .identifier == $locked.dTag
      and .relays == $locked.relayHints' \
    "$subprocess_stdout_file" >/dev/null
  coordinate_status=$?
  set -e
  if [[ $coordinate_status -eq 124 || $coordinate_status -eq 137 ]]; then
    infrastructure_failure jq-timeout "$name coordinate comparison exceeded ${jq_timeout_seconds}s"
  fi
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

if ! cleanup_files; then
  trap - EXIT
  infrastructure_failure "$cleanup_code" "$cleanup_message"
fi
trap - EXIT
echo "EXTERNAL_NAPPLET_CORPUS_OK entries=$entry_count commit=$source_commit mode=offline"
