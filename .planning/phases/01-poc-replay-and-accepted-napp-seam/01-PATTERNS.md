# Phase 01: POC replay and accepted Napp seam - Pattern Map

**Mapped:** 2026-08-13  
**Files analyzed:** 6 candidate files  
**Analogs found:** 6 / 6

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `scripts/ref-candidate-check.py` | utility / validator | transform | `scripts/ref-candidate-check.py` | exact (existing phase validator) |
| `evidence/phase-01/candidate-qualification.md` | config / evidence record | transform | `evidence/phase-01/napp-dependency.md` | exact |
| `evidence/phase-01/napp-dependency.md` | config / evidence record | transform | `evidence/phase-01/candidate-qualification.md` | exact |
| `evidence/phase-01/poc-replay.md` | config / evidence report | batch | `evidence/phase-01/candidate-qualification.md` | role-match |
| `evidence/phase-01/ownership-map.md` | config / evidence report | transform | `uzel-poc-validated-pack/docs/08-upstream-contributions.md` | role-match |
| `scripts/linux-run-smoke.sh` (only if replay gap needs a focused marker) | utility / acceptance harness | batch | `scripts/linux-run-smoke.sh` | exact |

`poc-replay.md` and `ownership-map.md` are inferred names for required durable Phase-01 evidence. Planner may use equivalent narrowly named evidence paths, but must keep them under `evidence/phase-01/` and bind claims to exact commands/heads.

## Pattern Assignments

### `scripts/ref-candidate-check.py` (utility / fail-closed transform)

**Analog:** existing `scripts/ref-candidate-check.py`.

**Imports and fixed-authority pattern** ([lines 1-46](../../../scripts/ref-candidate-check.py)):

```python
EXPECTED_COMMIT = "0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e"
EXPECTED_REPOSITORY = "jodobear/napp"
ROOT = Path(__file__).resolve().parents[1]
QUALIFICATION_PATH = "evidence/phase-01/candidate-qualification.md"
HANDOFF_PATH = "evidence/phase-01/napp-dependency.md"
```

Copy this fixed-path / fixed-commit approach. Do not accept candidate-supplied script, command, record path, or working-tree evidence.

**Confinement and atomic-write pattern** ([lines 95-152](../../../scripts/ref-candidate-check.py)):

```python
if relative.is_absolute() or any(part in ("", ".", "..") for part in relative.parts):
    raise CheckError("unsafe artifact path")
# Open every parent with O_NOFOLLOW, write a single-link 0600 temp file,
# fsync it, then os.replace(...) within the already-open directory fd.
```

Use only if checker still needs to emit its ignored committed-tree inventory. It is not a generic document writer.

**Exact-record validation and error pattern** ([lines 403-430](../../../scripts/ref-candidate-check.py)):

```python
if text.count(MARKER_BEGIN) != 1 or text.count(MARKER_END) != 1:
    raise CheckError("record requires exactly one marker-delimited canonical JSON block")
if canonical_record(value) != raw:
    raise CheckError("record JSON is not canonical")
committed = must_git(ROOT, [..., "HEAD:" + expected_relative])
if resolved.read_bytes() != committed:
    raise CheckError("record bytes differ from the reviewed HEAD blob")
```

Preserve fail-closed result validation ([lines 451-519](../../../scripts/ref-candidate-check.py)): exact schema keys, expected repo/origin/tree, committed record bytes, and invariant snapshots. Existing contract deliberately validates only `result == "stop"`; do not turn Phase 01 revalidation into implicit candidate acceptance.

**CLI / failure boundary** ([lines 687-705](../../../scripts/ref-candidate-check.py)):

```python
try:
    args = parser.parse_args()
    args.func(args)
except (CheckError, OSError, json.JSONDecodeError) as error:
    print("ref-candidate-check: " + str(error), file=sys.stderr)
    return 1
```

Keep commands explicit and errors typed. If validator is obsolete after replay, document evidence-backed deletion/disposition; do not broaden it.

---

### `evidence/phase-01/candidate-qualification.md` and `evidence/phase-01/napp-dependency.md` (evidence records / transform)

**Analogs:** each other; emitted and checked by `scripts/ref-candidate-check.py`.

**Record envelope** ([candidate qualification lines 1-7](../../../evidence/phase-01/candidate-qualification.md), [dependency handoff lines 1-7](../../../evidence/phase-01/napp-dependency.md)):

```markdown
# Napp Candidate Qualification

Committed-object evidence only; sibling working-tree material is excluded.

<!-- ref-candidate-record:begin -->
{canonical single-line JSON}
<!-- ref-candidate-record:end -->
```

Keep one marker-delimited, canonical JSON record; no handwritten fields inside marker. Current record's terminal outcome is a deliberate `stop`: missing committed source-backed admission evidence and a declared safe project probe. Revalidate rather than overwrite that disposition with a speculative seam.

**Handoff parity rule** ([`scripts/ref-candidate-check.py` lines 522-569](../../../scripts/ref-candidate-check.py)): handoff copies qualification provenance fields and verifies every copied field plus Plan-object parity. Use it only for repository-qualified dependency handoff; it grants neither publication nor adapter implementation.

---

### `evidence/phase-01/poc-replay.md` (evidence report / batch)

**Analog:** `scripts/linux-run-smoke.sh` as executable evidence producer; evidence record front matter above for exact-head binding.

**Bounded, secret-safe evidence preservation** ([lines 20-44](../../../scripts/linux-run-smoke.sh)):

```bash
SMOKE_TMP=$(mktemp -d)
FAILED_DIR=${UZEL_SMOKE_ARTIFACT_DIR:-uzel-poc-validated-pack/reports/probes/${SMOKE_NAME}-failed}

preserve_logs() {
  # redact the Tauri invoke key before saving Uzel log
}

preserve_failure() {
  preserve_logs "$FAILED_DIR"
  echo "Linux runtime smoke failed; logs preserved in $FAILED_DIR" >&2 || true
}
```

Report each REF-01--REF-06 check as `pass`, `failed`, or `unavailable`, with exact command, current commit, generated evidence location, owner, and revisit trigger. Never synthesize a pass from historic status text.

**Runtime proof markers** ([lines 142-203](../../../scripts/linux-run-smoke.sh)):

```bash
rg -q '^UZEL_SHELL_READY$' "$SMOKE_TMP/uzel.log"
rg -q '^UZEL_HOSTILE_PROBE_OK ... network_denials=13 sentinel_accepts=0 native_calls=0 source_bound=true$' "$SMOKE_TMP/uzel.log"
hostile_markers_are_ordered
```

Reuse named, exact markers and ordering checks for any focused replay amendment. Do not relax marker matching, leak invoke material, or replace real Weston/WebKit proof with Chromium-only evidence.

---

### `evidence/phase-01/ownership-map.md` (ownership report / transform)

**Analog:** `uzel-poc-validated-pack/docs/08-upstream-contributions.md`.

**Ownership map table pattern** ([lines 5-13](../../../uzel-poc-validated-pack/docs/08-upstream-contributions.md)):

```markdown
| Concern | Authority or implementation owner |
|---|---|
| Napplet protocol messages and lifecycle | `napplet/naps` |
| Rust Nostr data plane, relay work, cache, freshness, and evidence | `pablof7z/nmp` |
| Linux product composition | `jodobear/uzel`, later reusable `kehto/napd` seams |
```

Use a concrete table: retained Uzel behavior, Napp consumption/extraction requirement, upstream candidate, obsolete POC-only behavior, owner, exact source path/commit, and decision/revisit trigger. Preserve stated authority: NMP owns relay, event-store, freshness, provenance, and diagnostics; nampplets owns reusable native runtime. No Uzel duplicate cache/runtime/Nostr owner.

---

### POC fixtures and recovery checks (existing source; modify only when evidence proves a gap)

**Analogs:** `crates/napd/src/fixtures.rs`, `crates/napd/src/runner.rs`.

**Exact fixture catalog** ([`fixtures.rs` lines 5-16, 44-71](../../../crates/napd/src/fixtures.rs)):

```rust
pub struct FixtureDefinition {
    pub author: &'static str,
    pub d_tag: &'static str,
    pub aggregate_hash: &'static str,
    pub event: &'static [u8],
    pub index: &'static [u8],
}
const FIXTURES: [&FixtureDefinition; MAXIMUM_ACTIVE_FIXTURES] =
    [&GOOD_MORNING, &FOLLOW_LIST, &PROFILE_CARD, &HOSTILE_EGRESS];
```

**Verify → install → permission review → launch → source-bound session reconciliation** ([`runner.rs` lines 965-1096](../../../crates/napd/src/runner.rs)):

```rust
let verification = self.controller.verify_artifact(...);
let artifact = verification.artifact.ok_or_else(|| RunnerError::Verification(...))?;
if artifact.author() != fixture.author
    || artifact.d_tag().as_deref() != Some(fixture.d_tag)
    || artifact.aggregate_hash() != fixture.aggregate_hash { ... }
self.controller.install(Arc::clone(&artifact));
let permission = self.controller.permission_review(exact);
self.controller.launch(artifact, RuntimeExecutionProfile::Legacy);
```

If replay finds source changes necessary, adapt this existing narrow sequence; never introduce a second runtime, cache, Nostr path, signer, or persistence owner.

**Restart/recovery test pattern** ([`runner.rs` lines 1787-1818](../../../crates/napd/src/runner.rs)):

```rust
let first = runner.start_fixture().unwrap();
runner.stop_fixture(&first.surface_token).unwrap();
let mut restarted = LinuxRunner::open(temp.path()).unwrap();
let second = restarted.start_fixture().unwrap();
assert_ne!(first.surface_token, second.surface_token);
```

Use temporary state, explicit stop, reopen, and identity/session assertions. No global host state or silent recovery fallback.

## Shared Patterns

### Exact source binding

**Sources:** `scripts/check-pinned-assets.sh` lines 4-14 and `crates/napd/src/fixtures.rs` lines 83-101.

```bash
actual=$(sha256sum "$path")
actual=${actual%% *}
if [[ "$actual" != "$expected" ]]; then
  echo "pinned asset digest mismatch: $path" >&2
  exit 1
fi
```

Every replay claim must name exact commit, tree/blob/digest, and source path. Fixture source accepts only `/index.html`, matched digest, byte limit, and no redirects.

### Fail closed + narrow errors

**Sources:** `scripts/ref-candidate-check.py` lines 403-519; `crates/napd/src/runner.rs` lines 921-930, 1054-1069.

Refuse malformed / noncanonical evidence, refused snapshots, missing sessions, unmatched identities, and unavailable capabilities. Persist cause; never silently pass or widen a candidate contract.

### Evidence isolation and retention

**Source:** `scripts/linux-run-smoke.sh` lines 20-116.

Use per-run temp paths, cleanup traps, redact protected material before retained logs, preserve failed artifacts, and return nonzero on cleanup failure. Incident artifacts receive an explicit owner and revisit trigger or evidence-backed final disposition.

### Read-only maintenance tests

**Source:** `scripts/test-maintenance.py` lines 41-63.

```python
completed = run(["pnpm", "--silent", "docs:check"], checkout)
self.assertEqual(run(["git", "status", "--porcelain"], checkout).stdout, "")
result = json.loads(completed.stdout)
self.assertEqual(result["root"], ".")
```

Use isolated temporary checkout and assert no untracked/source mutation when a replay validator or documentation audit claims read-only behavior.

## No Analog Found

| File | Role | Data Flow | Reason |
|---|---|---|---|
| `evidence/phase-01/poc-replay.md` | evidence report | batch | No prior single REF-01--REF-06 consolidated replay report; compose from existing exact command markers. |
| `evidence/phase-01/ownership-map.md` | evidence report | transform | No prior retained/extract/obsolete POC classification matrix; use contribution-ledger ownership-table shape. |

## Metadata

**Analog search scope:** `scripts/`, `evidence/phase-01/`, `crates/napd/`, `apps/uzel/`, `uzel-poc-validated-pack/`  
**Files scanned:** 12  
**Pattern extraction date:** 2026-08-13
