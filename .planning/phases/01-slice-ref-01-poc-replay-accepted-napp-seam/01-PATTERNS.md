# Phase 01: SLICE-REF-01 — POC Replay & Accepted Napp Seam - Pattern Map

**Mapped:** 2026-08-09  
**Files analyzed:** 7 likely new evidence/harness artifacts; **production files:** 0  
**Analogs found:** 6 / 7

## Scope Gate

Local `jodobear/napp@0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e` has no accepted product-facing client, events, or testkit source. Candidate qualification is first checkpoint and must fail closed. Do not create a Uzel client facade, change POC protocol, repin Cargo/flake, or modify app/Rust sources until an exact committed candidate passes its own declared probes.

Graphify was queried first (`graphify-out/GRAPH_REPORT.md`): its POC acceptance, `LinuxRunner`, trusted-shell, and smoke-test communities match this map. Graph was built from `d7ad414b`, while workspace head is `26bcbe03b98bc3852f2be48e979cdf3665d86250`; source excerpts below take precedence.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `evidence/phase-01/candidate-qualification.md` | config/evidence record | batch | `uzel-poc-validated-pack/templates/fact.md` | exact |
| `evidence/phase-01/ownership-disposition.json` | config/data map | transform | `uzel-poc-validated-pack/docs/facts/FACT-013-hostile-linux-boundary.md` | partial |
| `evidence/phase-01/replay-manifest.json` | config/evidence record | batch | `apps/uzel/tests/ui/acceptance.test.mjs` | role-match |
| `evidence/phase-01/measurements/<mode>.json` | test evidence | batch | `apps/uzel/tests/ui/acceptance.test.mjs` | role-match |
| `evidence/phase-01/work-07-preservation.md` | evidence record | transform | `uzel-poc-validated-pack/STATUS.md` | exact |
| `scripts/phase-01-qualify-napp.sh` | test/harness | batch | `scripts/linux-run-smoke.sh` | role-match |
| `scripts/phase-01-replay.sh` | test/harness | batch | `scripts/linux-run-smoke.sh` | role-match |

`scripts/phase-01-*.sh` are discretionary names, not required API files. If planner keeps probes as documented commands instead, omit them; do not replace their fail-closed behavior with a Uzel integration.

## Pattern Assignments

### `evidence/phase-01/candidate-qualification.md` (config/evidence record, batch)

**Analog:** `uzel-poc-validated-pack/templates/fact.md`

**Record fields** (lines 1-10):
```markdown
# FACT-XXX — title
- **Claim:**
- **Classification:** verified fact | product decision | hypothesis | deferred
- **Exact source/pin:**
- **Probe/command:**
- **Observed result:**
- **Decision:**
- **Affected documents/code:**
- **Revalidate when:**
```

Copy fields, then record repository, full observed SHA, reachability, each missing/present public export, declared executable probe, raw-log path, and `stop` decision. No inferred crate names or commands.

### `evidence/phase-01/ownership-disposition.json` (config/data map, transform)

**Analog:** `uzel-poc-validated-pack/docs/facts/FACT-013-hostile-linux-boundary.md`

**Exact-source evidence style** (lines 3-10):
```markdown
- **Claim:** Under the exact pinned Linux POC, ...
- **Classification:** verified fact
- **Exact source/pin:** `jodobear/nampplets@...`; ...
- **Probe/command:** `pnpm smoke:linux` in the pinned Nix shell; ...
- **Observed result:** ...
- **Decision:** Accept ... for this exact Linux ... POC.
- **Affected documents/code:** ...
- **Revalidate when:** any ... source-binding rule changes.
```

Use one machine-readable row per POC element: `path`, `repository`, `commit`, `tests`, `disposition` (`retained-uzel`, `consumed-napp-contract`, `neutral-upstream-candidate`, `compatibility-only`, `obsolete`), `rationale`, `revalidate_when`. Candidate-dependent rows must say `unaccepted` now; no claimed consumed Napp API.

### `evidence/phase-01/replay-manifest.json` (config/evidence record, batch)

**Analog:** `apps/uzel/tests/ui/acceptance.test.mjs`

**Per-run immutable outcome pattern** (lines 301-320):
```javascript
const outcome = {
  name, scenario, viewport: { ...viewport }, mode: HARNESS_MODE, gitSha,
  startedAt, finishedAt: new Date().toISOString(), status: 'passed',
  console: guarded.consoleEntries,
  acknowledgedProblems: guarded.acknowledgedProblems,
  acknowledgedExternalRequests: guarded.acknowledgedExternalRequests,
};
await writeFile(join(directory, 'case.json'), `${JSON.stringify(outcome, null, 2)}\n`);
```

Manifest rows need full Uzel head, candidate object or `null`, exact fixture hashes, command, mode (`fixture`, `build`, `dependency-cache`, `release-runtime`), raw-output path/hash, verdict (`pass`, `fail`, `unavailable`), and limitation. Never overwrite evidence from another mode.

### `evidence/phase-01/measurements/<mode>.json` (test evidence, batch)

**Analog:** `apps/uzel/tests/ui/acceptance.test.mjs`

**Failure retention pattern** (lines 321-338):
```javascript
const outcome = { name, scenario, viewport: { ...viewport }, mode: HARNESS_MODE,
  gitSha, startedAt, finishedAt: new Date().toISOString(), status: 'failed',
  error: String(error) };
outcomes.push(outcome);
await dumpFailure(guarded.page, guarded, viewport, scenario, name, error, capturedState);
throw error;
```

Each metric record includes `metric`, `unit` (or `null`), `mode`, `result`, `samples`, `command`, `environment`, `git_head`, `candidate`, `raw_output`, `limitation`. For unavailable, require `result: "unavailable"`, `reason`, and no synthetic `samples`.

### `evidence/phase-01/work-07-preservation.md` (evidence record, transform)

**Analog:** `uzel-poc-validated-pack/STATUS.md`

**Historical-state pattern** (lines 27-35):
```text
slice: Work 07 active; issues #28 and #29 are implemented on feat/rich-profile-surfaces in PR #30
commands: pinned-Nix ... real Weston/WebKit `pnpm smoke:linux`; ...
observable result: ...
known limitation: ... visible Debian 13 interactive acceptance ...
next action: ... merge PR #30 only when ... review is clean, then run Debian 13 interactive acceptance ...
```

Add dated reconciliation only: exact file, observed local commit, legacy status assertion, conflict if any, and unresolved Debian/review gates. Do not edit this status, merge/close PR #30, or call a gate complete.

### `scripts/phase-01-qualify-napp.sh` (test/harness, batch)

**Analog:** `scripts/linux-run-smoke.sh`

**Strict setup + bounded cleanup** (lines 1-18, 90-131):
```bash
#!/usr/bin/env bash
set -euo pipefail
[[ "$STARTUP_TIMEOUT_SECONDS" =~ ^[1-9][0-9]*$ ]] \
  || { echo 'UZEL_SMOKE_STARTUP_TIMEOUT_SECONDS must be a positive integer' >&2; exit 2; }
trap cleanup EXIT
trap handle_interrupt INT
trap handle_terminate TERM
```

**Marker-based truth** (lines 172-179):
```bash
report_marker() {
  local name=$1
  local pattern=$2
  local status=missing
  if rg -q "$pattern" "$SMOKE_TMP/uzel.log"; then status=present; fi
  echo "LINUX_SMOKE_MARKER name=$name status=$status" >&2
}
```

Probe only exact candidate paths and commands discovered in that commit. Write raw tree/probe output to phase evidence, emit missing fields, exit non-zero on any required absence, and do not invoke Uzel adaptation.

### `scripts/phase-01-replay.sh` (test/harness, batch)

**Analog:** `scripts/linux-run-smoke.sh`

**Separate runtime evidence + success marker** (lines 254-278):
```bash
if runtime_markers_ready; then
  sleep 2
  if ! kill -0 "$DEV_PID" 2>/dev/null; then exit 1; fi
  echo "$SUCCESS_MARKER ... source_bound=multi hostile=denied ... compositor=weston-headless-gl"
  exit 0
fi
```

Maintain independently runnable named probes for Rust fixtures, Chromium acceptance, and Weston/WebKit smoke. Preserve logs with secret redaction; return unavailable with reason when host/toolchain cannot run a mode. Do not treat mocked Chromium result as WebKit evidence.

## Shared Patterns

### Fail-closed replay and bounded errors

**Source:** `crates/napd/src/server.rs` lines 113-131, 298-328, 396-412, 768-810

```rust
let replay_key = match replay_key(&request) {
    Ok(replay_key) => replay_key,
    Err(InvalidOperationId) => return (Response::error("invalid_operation_id", "..."), false),
};
if let Some(key) = replay_key.as_deref()
    && let Some(response) = self.replay.lookup(key, &request) { return (response, false); }
// operation id is bounded and ASCII restricted
Response::error("runtime_refused", bounded_detail(error.to_string()))
```

Apply to: qualification/replay command orchestration. Preserve byte-identical inputs/results, bound values, stop at first falsifier, retain raw evidence.

### Source binding and cleanup

**Source:** `apps/uzel/public/trusted-shell/trusted-shell.js` lines 309-317, 866-877

```javascript
if (!frame || event.source !== frame.contentWindow) return null;
if (!isBoundedEnvelope(event.data)) return null;
return event.data;
// dispose revokes URLs, cancels pending operations, rejects, then clears maps.
```

Apply to: replay criteria only. Keep trusted-host source binding and cleanup evidence; no napplet-selected surface identity.

### Reconciliation vs. partial launch

**Source:** `crates/napd/src/runner.rs` lines 908-939, 1178-1206

```rust
let session = reconcile_launched_session(&self.controller, snapshot,
    &existing_session_ids, &expected_author, &expected_d_tag, &expected_aggregate_hash)?;
// require exactly one newly-created matching running session; stop every ambiguous new one.
```

Apply to: replay matrix/recovery evidence. Record typed outcome, not generic success.

### Browser/native proof separation

**Sources:** `apps/uzel/tests/ui/acceptance.test.mjs` lines 634-640, 1126-1154; `scripts/linux-run-smoke.sh` lines 151-169, 277-278

Chromium records scenario-specific artifacts; Weston/WebKit accepts exact daemon, source-binding, hostile, and native-bridge markers. Store them as separate mode records.

## No Analog Found

| File | Role | Data Flow | Reason |
|---|---|---|---|
| `evidence/phase-01/ownership-disposition.json` | config/data map | transform | Existing ownership evidence is Markdown facts, not a machine-readable disposition table. Preserve fact-field semantics. |

## Metadata

**Analog search scope:** `uzel-poc-validated-pack/{templates,docs}`, `crates/napd`, `apps/uzel/{public,tests}`, `scripts`, `graphify-out`  
**Files scanned:** 12 direct analog/graph files  
**Pattern extraction date:** 2026-08-09
