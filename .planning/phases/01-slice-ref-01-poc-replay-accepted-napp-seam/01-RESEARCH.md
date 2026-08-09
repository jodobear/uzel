# Phase 01: SLICE-REF-01 — POC Replay & Accepted Napp Seam - Research

**Researched:** 2026-08-09  
**Domain:** Brownfield Linux POC preservation, evidence admission, and fail-closed Napp qualification
**Confidence:** MEDIUM

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Accepted Napp Candidate Gate
- **D-01:** Phase 1 may integrate only one exact, reachable, committed Napp candidate whose product-facing client, events, and testkit vectors are present in source and pass the phase's executable probes. — **Reversibility:** costly — A later candidate change requires repinning source and locks and replaying the trust, lifecycle, fixture, and platform evidence.
- **D-02:** Candidate absence or a falsified contract stops only the qualification/adaptation lane. Record the full Napp repository, exact observed commit, missing contract, and required acceptance evidence; do not preserve progress through a Uzel-only facade or legacy protocol expansion. Independent POC replay and ownership evidence continue.
- **D-03:** The accepted candidate must keep NMP as the sole Nostr engine/store and Napp as runtime authority. Uzel owns presentation, trusted-host integration, and product-visible recovery only.

### Replay and Ownership Evidence
- **D-04:** Replay the existing POC before extraction or seam replacement. Evidence must cover exact-build review, confirmation, launch, multi-surface composition, source binding, hostile denial, restart/reconciliation, deterministic Chromium, and real Weston/WebKit behavior.
- **D-05:** Produce one durable ownership disposition tied to exact files, tests, repositories, and commits. Classify each relevant POC element as retained Uzel product code, consumed Napp contract, neutral upstream candidate, compatibility-only seam, or obsolete POC behavior.
- **D-06:** Reconcile the legacy Work 07 record with source truth: PR #30 is merged at `19519c3`, while visible Debian 13 acceptance remains unresolved. Preserve the historical file as input evidence; do not silently overwrite or claim its human gate complete.

### Baseline Measurement
- **D-07:** Record reproducible measurements, commands, environment identity, and limitations for cold start, first visible frame, local profile render, idle CPU/RSS, WebView memory, resource flow, queue bounds, cancellation, and lifecycle recovery.
- **D-08:** Mark a measurement `unavailable` with reason when the current environment cannot produce it. Do not estimate or convert a qualitative observation into a numeric claim.
- **D-09:** Separate build, dependency-cache, fixture-mode, and release-runtime measurements so one cannot be presented as another.

### Recovery and Upstream Stops
- **D-10:** Lost replies, duplicate retries, stale sessions, partial launch, cancellation, restart reconciliation, and cleanup remain typed, bounded lifecycle outcomes. Preserve byte-identical replay inputs and idempotent stop/cancel behavior.
- **D-11:** On contradiction, stop at the smallest falsifying probe, preserve the current green POC, update the durable fact/disposition record, and resume only from an accepted reachable commit.
- **D-12:** Any reusable fix must use a dedicated branch in the corresponding `jodobear` fork and be recorded in `uzel-poc-validated-pack/docs/08-upstream-contributions.md` before Uzel depends on it.

### Scope, State, and Handoff
- **D-13:** Phase 1 accepts one Uzel instance and one active read profile as the proven baseline. Record collision behavior and scope fields; multi-instance/multi-profile implementation is later Napp/package work.
- **D-14:** NMP remains canonical for Nostr events, queries, relays, freshness, provenance, signing, and publication. Uzel may persist only bounded product/UI selection and transient projection/resource state. The ownership artifact records durability, retention, migration trigger, and writable owner.
- **D-15:** Migration uses one axis: preserve current-green POC, qualify exact Napp candidate, adapt the narrow Rust/Tauri boundary, replay parity evidence, then retire only proven-obsolete compatibility code. No dual store, dual write, or big-bang replacement. Rollback is an adapter/pin revert to the preserved POC baseline.
- **D-16:** The installed product-first root pack is proposed input until its selected files and templates are audited and committed. Planning may cite committed `.planning/` artifacts and this re-audit; implementation may not depend on uncommitted normative files.
- **D-17:** One plan maps to one repository-qualified issue and bounded PR. Mosaico/GSD handoffs carry exact heads, issue, decisions, evidence paths, blocker, and next probe; they reference canonical documents instead of copying the programme pack.
- **D-18:** Use the existing review sequence once per semantic candidate: affected local gates, one full local CodeRabbit, remote Codex, final GitHub CodeRabbit. Unchanged inputs do not rerun expensive gates; enforcement belongs to the later CI slice.

### the agent's Discretion
The planner may choose the exact document names, measurement harness layout, and probe grouping, provided every artifact remains exact-source-backed, replayable, bounded, and within this phase boundary.

### Deferred Ideas (OUT OF SCOPE)
- Canonical Nix release closure belongs to Phase 2.
- PR-fast, merge-group, and required-check aggregation belong to Phase 3.
- New profile/resource gaps and Social graph/feed work belong to Phases 4 and 5. Existing rich profile/follow/resource behavior is preservation evidence, not future work to rebuild. New Social implementation remains gated on accepted REF, PKG, and CI evidence.
- Files, Blossom, signing, wallets, authoring, richer media, ContextVM, Relatr, TUI, WASI, Android, and native napplets remain outside this milestone.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|---|---|---|
| REF-01 | Replay exact-build review, confirmation, launch, rich profile/follow rendering, and composition from the current merged POC at exact current pins before any Napp adaptation. | REF-01A preserves/replays the merged POC; accepted-candidate parity is REF-07/REF-01D and remains conditional on later qualification. [VERIFIED: REQUIREMENTS.md; 01-REAUDIT.md] |
| REF-02 | Prove source-created-surface binding and authority denial. | Retain trusted-shell source binding, native navigation policy, boundary tests, and hostile smoke markers. [VERIFIED: codebase grep] |
| REF-03 | Recover read identity, builds, useful local state, and ambiguous outcomes without duplicate truth. | Record the current bounded state/rollback/reconciliation contract and single-instance collision behaviour. [VERIFIED: codebase grep] |
| REF-04 | Replay Chromium and real Weston/WebKit evidence. | REF-01A records separate fixture/browser/native modes and raw artifacts. [VERIFIED: 01-REAUDIT.md] |
| REF-05 | Produce durable ownership classification. | REF-01B emits file-level disposition plus state/retention/bounds ledger. [VERIFIED: 01-PATTERNS.md] |
| REF-06 | Produce measured lifecycle/resource baseline when supported. | REF-01B takes minimal representative samples, separates setup/build/runtime, and records unavailable honestly. [VERIFIED: 01-REAUDIT.md] |
| REF-07 | Consume only accepted Napp client/events/testkit seam. | Napp qualification/handoff is fail-closed; REF-01D remains blocked until it passes. [VERIFIED: local Napp HEAD] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Preserve brownfield POC code/tests. Tauri 2 + Svelte is locked; Rust owns surface/layout/runtime state and Svelte presentation. [VERIFIED: AGENTS.md]
- Source-bind every napplet request before Napp. Do not import NMP/nampplets internals or duplicate runtime, grant, signer, cache, or Nostr state. [VERIFIED: AGENTS.md]
- Render local state first; keep UI non-blocking and WebViews bounded; names stay descriptive and at most 21 characters. [VERIFIED: AGENTS.md]
- Keep strict TypeScript, accessibility, Fallow, and real WebKit evidence. Use changed-scope PR-fast plus path-gated full native coverage. [VERIFIED: AGENTS.md]
- Nix owns canonical Linux release packaging; no ambient package/Flatpak workaround; Cargo and flake Napp pins must align; no PATH discovery of a runtime daemon. [VERIFIED: AGENTS.md]
- Pin exact committed Napp client/testkit SHA; do not invent contracts. One issue, one visible outcome, one bounded PR. [VERIFIED: AGENTS.md]
- Preserve the POC evidence method: exact commits/source/tests/probes, bounded typed errors, one focused worktree, and upstream-fork ledger before Uzel depends on reusable changes. [VERIFIED: uzel-poc-validated-pack/AGENTS.md]

## Summary

Phase 01 splits into executable preservation/admission work and a blocked integration workstream. The merged Uzel baseline is `19519c378c2e775c6ad4b042cfd9aadd89f766b9`; current planning head `3a571baa58e2fd7db8a2fbf78cd9a5093c359ac6` adds planning artifacts only. Fresh re-audit replay records 37 JavaScript tests, 55 Rust tests with two explicit live-network ignores, 34 Chromium cases, boundary checks, and clean-target Weston/WebKit smoke. [VERIFIED: 01-REAUDIT.md; git history]

Napp committed HEAD `0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e` contains planning material only. It has no committed executable client, product events, testkit, package output, or candidate-declared probe. This is an accepted absence for planning: it blocks only Napp qualification/adapter work, not POC replay, ownership/pressure evidence, root-document admission, or Debian visible acceptance. [VERIFIED: local Napp HEAD; 01-CONTEXT.md]

The planner should make five bounded Uzel plans plus one Napp-owned dependency handoff: REF-01A replay from clean/relocated target, REF-01B ownership/storage/retention/bounds evidence, REF-01C committed product-doc admission, Napp qualification/handoff, blocked REF-01D adapter parity, and REF-01E Debian 13 visible acceptance. No plan may change POC production code before a specific falsifying probe justifies it. [VERIFIED: 01-REAUDIT.md]

**Primary recommendation:** Execute REF-01A, B, C, and E independently; record Napp handoff as fail-closed; omit implementation tasks for REF-01D until D-01/D-16 are satisfied. [VERIFIED: 01-CONTEXT.md]

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|---|---|---|---|
| Exact fixture/build review, session lifecycle, replay | Rust runtime/backend | Native Tauri host | Current LinuxRunner owns exact build/session/recovery mechanisms. [VERIFIED: codebase grep] |
| Source-created surface binding | Trusted browser host | Rust runtime | Host verifies MessageEvent.source against created frame before runtime routing. [VERIFIED: codebase grep] |
| Native bridge and navigation denial | Native Tauri host | Browser tests | Tauri host owns invoke/navigation boundary; native smoke validates hostile denial. [VERIFIED: codebase grep] |
| Local product selection state | Runtime storage | Svelte projection | Runner stores bounded selected identity/generation; browser keeps only UI preferences/transient URLs. [VERIFIED: codebase grep] |
| Nostr data, freshness, provenance, signer/publication | NMP | Future Napp adapter | Uzel does not own a parallel Nostr store or projection authority. [VERIFIED: 01-CONTEXT.md] |
| Candidate client/events/testkit | Napp repository | Uzel consumer probes | Public contract authority belongs upstream; Uzel may only qualify and consume. [VERIFIED: local Napp HEAD] |
| Packaging and CI enforcement | Phase 2 / Phase 3 scope | Phase 01 evidence | Current flake is development shell only; no committed GitHub workflows exist. [VERIFIED: 01-REAUDIT.md] |

## Standard Stack

No new package is approved for Phase 01. Preserve exact current-green inputs and use their existing commands. [VERIFIED: 01-REAUDIT.md]

| Component | Exact baseline | Phase use | Rule |
|---|---|---|---|
| Uzel Rust workspace | edition 2024, Rust 1.89 | Runtime/protocol/recovery corpus. [VERIFIED: Cargo.toml] | No refactor or extraction in REF-01A/B. [VERIFIED: 01-REAUDIT.md] |
| Tauri | 2.11.5 | Existing native host and real-WebKit proof. [VERIFIED: Cargo.toml] | Keep trust proof; clean target exposes relocation defect. [VERIFIED: 01-REAUDIT.md] |
| Svelte + Playwright | Playwright 1.62.0 | Existing accessible deterministic renderer proof. [VERIFIED: package.json] | Do not call mocked output native evidence. [VERIFIED: apps/uzel/tests/ui/README.md] |
| nampplets/NMP POC pin | jodobear/nampplets e2f69f3; NMP 005dc2a in re-audit | Current-green implementation baseline. [VERIFIED: 01-REAUDIT.md] | Freeze; one-axis upgrades are later research. [VERIFIED: 01-REAUDIT.md] |
| Nix dev shell | nixpkgs 38a4887 | Pinned command environment. [VERIFIED: flake.nix; 01-REAUDIT.md] | Separate materialization/build from runtime measurements. [VERIFIED: D-09] |
| Napp public seam | none accepted | Dependency handoff only. [VERIFIED: local Napp HEAD] | Do not name paths/APIs in Uzel plan. [VERIFIED: D-01] |

**Installation:** None. Phase 01 must not add or update dependencies, so no package-legitimacy audit applies. [VERIFIED: 01-CONTEXT.md]

## Architecture Patterns

### System Architecture Diagram

~~~
exact Uzel POC head + exact fixture corpus + pinned Nix shell
                              |
                              v
     REF-01A: clean/relocated replay --> browser evidence
             |                    |        native Weston/WebKit evidence
             |--------------------|------------------------------|
             v                    v                              v
REF-01B: ownership/state/   REF-01C: audited product-doc   REF-01E: Debian human proof
         bounds/measurements         authority

Napp candidate qualification (independent read-only lane)
                              |
                     absent / falsified?
                         | yes             | no
                         v                 v
               dependency handoff   REF-01D narrow Rust/Tauri adapter
               stop this lane       + same replay parity matrix
                                          |
                                          v
                          no dual state; pin/adapter revert rollback
~~~

REF-01D is outside current executable plan waves. [VERIFIED: 01-REAUDIT.md]

### Pattern 1: Clean/Relocated Baseline Replay

**What:** Use one replay harness that runs named existing commands in separate modes, records full head, fixtures, environment, raw output hash/path, and verdict. [VERIFIED: 01-PATTERNS.md]
**Why:** The re-audit found a default-target Tauri metadata path from an older checkout; clean isolated target passed, so warm target success cannot prove relocation. [VERIFIED: 01-REAUDIT.md]

Required mode separation:

| Mode | Existing proof | Required output |
|---|---|---|
| fixture/unit | Rust, contract, conformance, asset/boundary tests | source head, fixture hashes, raw logs. [VERIFIED: package.json] |
| deterministic renderer | pnpm test:ui | per-scenario Chromium artifacts and teardown result. [VERIFIED: apps/uzel/tests/ui/README.md] |
| native runtime | pnpm smoke:linux | Weston/WebKit markers, daemon/shell/hostile results, redacted logs. [VERIFIED: scripts/linux-run-smoke.sh] |
| relocated target | clean cargo target plus native run | old-path absence and the same native markers. [VERIFIED: 01-REAUDIT.md] |

### Pattern 2: Typed Fail-Closed Recovery

**What:** Preserve operation identity/replay bounds, idempotent stop/cancel, and snapshot-based reconciliation rather than “retry until green.” [VERIFIED: codebase grep]
**Use:** REF-01A replay criteria and REF-01B ownership ledger only; do not change runtime recovery logic. [VERIFIED: 01-REAUDIT.md]

Current runner stops every newly created session when exactly one matching verified/running session cannot be established. It also rolls back identity activation when persisted state cannot be safely written. [VERIFIED: crates/napd/src/runner.rs]

### Pattern 3: Source-Bound Projection and Cleanup

**What:** Trusted shell maps received browser event source to the exact created frame, checks bounded envelopes, and releases object URLs/cancels pending work on disposal. [VERIFIED: codebase grep]
**Use:** Preserve as regression acceptance before/after any future adapter. [VERIFIED: 01-PATTERNS.md]

Tauri documents that overlapping WebView capabilities merge permissions; keep untrusted napplet frames out of broad/shared capability grants and retain source binding as an application-level control. [CITED: https://v2.tauri.app/security/capabilities/]

### Pattern 4: One-Axis Migration and Rollback

**What:** Preserve current-green POC → qualify exact candidate → adapt narrow Rust/Tauri boundary → replay same matrix → retire only proven obsolete compatibility code. [VERIFIED: D-15]
**Use:** REF-01D only after all gates; rollback is pin/adapter revert while preserved POC remains green. [VERIFIED: 01-REAUDIT.md]

Do not history-rewrite/extract POC during this phase. Git documents history rewriting as experimental and rewritten commits receive new object identity. [CITED: https://git-scm.com/docs/git-history]

### Anti-Patterns to Avoid

- Treating POC private crates/Unix protocol as public Napp contract. [VERIFIED: 01-REAUDIT.md]
- Inferring a candidate from Napp documents or dirty sibling files. [VERIFIED: local Napp HEAD]
- Calling Chromium evidence a WebKit/native result. [VERIFIED: apps/uzel/tests/ui/README.md]
- Turning clean-shell materialization time into runtime latency, or unavailable results into estimates. [VERIFIED: D-08/D-09]
- Rebuilding current rich profile/follow UI in Phase 01. [VERIFIED: 01-REAUDIT.md]
- Copying uncommitted product-first pack text into implementation plans as authority. [VERIFIED: D-16]

## Bounded MVP Plan Slices

| Slice | Status | Scope | Verification / exit |
|---|---|---|---|
| REF-01A | Ready | Clean/relocated replay of existing exact-build review, confirmation, multi-surface composition, profile/follow visible path, recovery, hostile denial, Chromium, Weston/WebKit; write replay manifest and Work 07 reconciliation. [VERIFIED: 01-REAUDIT.md] | All named required modes pass. A failed, missing, or `unavailable` required mode is recorded honestly per D-08 but leaves replay blocked/incomplete; default-target path leak is documented; no production diff. [VERIFIED: D-04/D-08; 01-REAUDIT.md] |
| REF-01B | Ready | File ownership disposition; state/retention/migration trigger map; bounds ledger; single-instance/read-profile collision probe; minimal resource/lifecycle measurements. [VERIFIED: 01-REAUDIT.md] | Every relevant POC element has owner/durability/bound/revalidate condition; all metrics raw or unavailable. [VERIFIED: 01-PATTERNS.md] |
| REF-01C | Ready, documentation-only | Select root product-first documents/templates, narrow root auditor to canonical files, audit and commit authority without product scope change. [VERIFIED: 01-REAUDIT.md] | Chosen authority set is committed and audited; unselected copies explicitly non-normative. [VERIFIED: D-16] |
| Napp qualification/handoff | Ready, fail-closed | Read only committed Napp tree/declared commands; capture missing/present source, events, vectors, version/lifecycle/pin evidence; write repository-qualified blocker if absent. [VERIFIED: 01-PATTERNS.md] | Any missing required evidence returns stop; no Uzel source edit. [VERIFIED: D-01/D-02] |
| REF-01D | **Blocked** | Narrow Rust/Tauri adapter and same-matrix parity only. [VERIFIED: 01-REAUDIT.md] | Requires committed authority docs plus exact Napp client/events/testkit candidate and all qualification probes. [VERIFIED: D-01/D-16] |
| REF-01E | Ready, human/platform | Debian 13 interactive visible acceptance for merged rich-profile fixtures. [VERIFIED: 01-REAUDIT.md] | Operator records direct visible acceptance or failure with exact head/environment; no architecture changes. [VERIFIED: D-06] |

Each executable slice maps to one repository-qualified issue and one bounded PR; do not combine A/B/C/E or candidate handoff with adapter implementation. [VERIFIED: D-17]

## Don't Hand-Roll

| Problem | Do Not Build | Use Instead | Reason |
|---|---|---|---|
| Missing consumer seam | Uzel façade, copied Napp daemon, expanded private protocol | Repository-qualified Napp handoff/qualification | Prevents permanent inversion of runtime authority. [VERIFIED: D-02/D-03] |
| Nostr truth | Uzel profile/follow/relay/event/freshness cache | NMP through future accepted Napp surface | NMP ownership is locked. [VERIFIED: D-14] |
| Surface/principal authority | Caller-selected payload field | Trusted host created-frame mapping + runtime token | Existing hostile/source tests prove this boundary. [VERIFIED: codebase grep] |
| Recovery | Generic retry manager | Existing typed replay/cancel/reconcile mechanisms | Current POC has bounded terminal outcomes and rollback. [VERIFIED: crates/napd/src/runner.rs] |
| Resource pressure | New framework or unbounded streams | Existing bounds ledger and project-specific probes | Limits already exist for active fixtures, reviews, envelopes, replay, state, and URLs. [VERIFIED: codebase grep] |
| CI architecture | Duplicate wrapper suites | Later measured CI composition/aggregator | Distinct evidence stays distinct; orchestration is Phase 3. [VERIFIED: 01-REAUDIT.md] |

## Runtime State Inventory

| Category | Items Found | Action Required |
|---|---|---|
| Stored data | Runner persists bounded versioned selected-identity/mode/generation state in uzel-state.json under XDG data/runtime root; NMP owns canonical Nostr state. [VERIFIED: crates/napd/src/runner.rs; apps/uzel-napd/src/main.rs] | REF-01B records owner, durability, schema, retention, migration trigger, and rollback; no migration/new store. [VERIFIED: D-14/D-15] |
| Browser state | Svelte persists orientation, split, preferences locally; trusted shell holds bounded transient object URLs and pending operations. [VERIFIED: codebase grep] | Classify as Uzel UI/transient state; prove cleanup and do not treat it as Nostr/runtime truth. [VERIFIED: D-14] |
| Live service config | Current repository has no Uzel service unit; user D-Bus was unavailable to earlier sandbox probe. [VERIFIED: codebase grep; 01-REAUDIT.md] | Record unavailable; Phase 2 package work owns service lifecycle. [VERIFIED: 01-REAUDIT.md] |
| OS-registered state | No Uzel desktop/service/plist artifact was found in repository. [VERIFIED: codebase grep] | Do not register or migrate anything in Phase 01. [VERIFIED: D-15] |
| Secrets/env vars | Observed Uzel variables configure XDG/Wayland/smoke paths; captured smoke logs redact Tauri invoke key material. [VERIFIED: scripts/linux-run-smoke.sh] | Keep redaction; do not introduce secret storage or rename unknown operator variables. [VERIFIED: codebase grep] |
| Build artifacts | Ignored acceptance/smoke artifacts and Cargo target metadata exist; default target leaked old absolute checkout path. [VERIFIED: 01-REAUDIT.md] | REF-01A uses clean/relocated target; evidence keeps raw logs but no stale generated output is promoted. [VERIFIED: 01-REAUDIT.md] |

## Common Pitfalls

### Candidate reachability mistaken for acceptance

Napp HEAD is a reachable committed documentation tree but has no consumable public implementation. Inspect exact committed exports and declared commands, then stop on absence. [VERIFIED: local Napp HEAD]

### POC replay mistaken for candidate parity

Current POC evidence remains valuable regression corpus, but it does not prove future Napp API/version/lifecycle compatibility. Keep baseline and candidate records separate. [VERIFIED: 01-REAUDIT.md]

### Relocation blind spot

Warm default Cargo target retained Tauri metadata from an older path; a clean target passed. Require clean/relocated proof and retain this failure as reproducibility evidence. [VERIFIED: 01-REAUDIT.md]

### Storage ownership drift

Selected identity and UI preferences may persist; Nostr profile/follow/freshness truth may not. The ledger must identify writable owner, retention, and migration trigger before adapter work. [VERIFIED: D-14]

### Unbounded future scope

Multi-instance/multi-profile, resource.bytesMany whole-operation deadline, TLS, OS child isolation, package closure, and CI aggregator remain later work. Record limits/gaps; do not implement them in this phase. [VERIFIED: 01-REAUDIT.md]

### Historical gate rewrite

STATUS retains stale PR #30 language while merged source is 19519c3; Debian visible acceptance remains open. Add dated reconciliation only. [VERIFIED: D-06; 01-REAUDIT.md]

## Code Examples

### Candidate qualification record

~~~markdown
repository: jodobear/napp
observed_commit: 0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e
commit_tree: b12b0c13b11ce5f64e4fd91025789ae692438f38
tree_inventory_sha256: 9d6f11acb50a13c68f77f5e4945598ba9c2a1b301c70017568fe360b16abd41b
approved_ref_reachability: recorded
working_tree_evidence: excluded
client_source: {status: missing, evidence_hash: null}
product_events_source: {status: missing, evidence_hash: null}
testkit_vectors: {status: missing, evidence_hash: null}
version_mismatch: {status: missing, evidence_hash: null}
lifecycle_recovery: {status: missing, evidence_hash: null}
pin_parity: {status: missing, evidence_hash: null}
declared_probes: {status: missing, safe_executed: [], skipped_unsafe: []}
result: stop
uzel_action: no source change; publish upstream dependency handoff
resume_when: exact committed candidate declares source and probes
~~~

Use fields discovered from exact candidate source only; do not hard-code future crate names or commands. [VERIFIED: 01-PATTERNS.md]

### Measurement record

~~~json
{
  "metric": "first_visible_frame",
  "mode": "release-runtime",
  "unit": "ms",
  "result": "unavailable",
  "reason": "named measurement method did not run on this host",
  "command": ["nix", "develop", "--command", "<fixed-probe>"],
  "environment": {
    "architecture": "x86_64",
    "kernel_release": "<captured>",
    "target": "clean"
  },
  "inputs": {
    "product_commit": "<full SHA>",
    "product_tree": "<tree SHA>",
    "fixture_inventory_sha256": "<SHA-256>"
  },
  "toolchain": {
    "flake_nix_blob": "<Git blob>",
    "flake_lock_blob": "<Git blob>",
    "nixpkgs_locked": "<full input revision>",
    "resolved_versions": {"nix": "<version>", "rustc": "<version>", "node": "<version>"}
  },
  "git_head": "<full SHA>",
  "candidate": null,
  "raw_record": {"id": "frame-1", "sha256": "<SHA-256>", "encoding": "base64"},
  "extractor": null,
  "limitation": "No named observable produced a numeric sample"
}
~~~

An unavailable record has no synthetic samples. Measured records additionally use one finite extractor from Plan 01-02 so the validator reproduces the numeric value from durable redacted bytes. [VERIFIED: 01-PATTERNS.md; 01-02-PLAN.md]

## State of the Art

| Old/unsafe approach | Current phase approach | Impact |
|---|---|---|
| Whole Phase 01 blocked by missing Napp | Split independent POC/admission lanes from blocked adapter lane | Preservation can deliver evidence now without private seam. [VERIFIED: 01-CONTEXT.md] |
| Warm shared target treated as reproducible | Clean/relocated target recorded separately | Detects absolute-path metadata leakage. [VERIFIED: 01-REAUDIT.md] |
| Scattered POC Markdown facts | Machine-readable disposition, bounds, replay/measurement records | Planner gains one exact audit surface. [VERIFIED: 01-PATTERNS.md] |
| Big-bang extraction/history rewrite | One-axis adapter/pin migration with revert | Preserves accepted POC/fixture provenance. [CITED: https://git-scm.com/docs/git-history] |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|---|---|---|
| A1 | Selected root product-first docs can be reduced to one non-conflicting committed authority set during REF-01C. [ASSUMED] | MVP Plan Slices | Medium; document conflict may require human decision. |
| A2 | Clean target plus relocated run is sufficient to characterize current absolute-path defect. [ASSUMED] | Replay | Medium; a separate toolchain defect may remain. |
| A3 | Future Napp source will expose a client/events/testkit shape suitable for Uzel consumption. [ASSUMED] | Candidate handoff | High; no Uzel adaptation may assume this. |

## Open Questions (RESOLVED)

1. **Which exact Napp candidate can satisfy D-01?**
   - Known: observed committed HEAD is docs-only. [VERIFIED: local Napp HEAD]
   - Gap: public source, events, testkit, version/mismatch, lifecycle, pin-parity, and declared probes. [VERIFIED: 01-REAUDIT.md]
   - Resolution: Record a repository-qualified Napp dependency; REF-01D is excluded from executable plans and resumes only after one exact committed candidate qualifies. [VERIFIED: D-01/D-02]

2. **Which root product-first docs become authoritative?**
   - Known: root pack files/templates are untracked, while planning derivatives are committed. [VERIFIED: git index]
   - Gap: selected canonical set and auditor scope. [VERIFIED: 01-REAUDIT.md]
   - Resolution: REF-01C resolves this at planning level through a bounded admission/audit/commit plan; current untracked files remain proposed input until that plan executes. [VERIFIED: D-16]

3. **What are current clean runtime baseline numbers?**
   - Known: fresh functional replays passed; no current cold/first-frame/RSS/WebView-memory baseline is recorded. [VERIFIED: 01-REAUDIT.md]
   - Resolution: REF-01B resolves the planning obligation with a bounded measurement plan; numeric values remain intentionally unknown until execution and unsupported values are recorded as unavailable. [VERIFIED: D-07/D-08/D-09]

4. **Does Debian 13 visually accept merged fixture behaviour?**
   - Known: headless/native evidence exists; interactive visible acceptance remains open. [VERIFIED: 01-REAUDIT.md]
   - Resolution: REF-01E resolves the planning obligation through a separate human/platform checkpoint; acceptance remains open until that execution records direct evidence. [VERIFIED: D-06]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|---|---|---|---|---|
| Nix | Pinned shell/replay | ✓ | 2.34.1 | — [VERIFIED: local probe] |
| Cargo/rustc | Rust POC replay | ✓ | 1.89.0 | — [VERIFIED: local probe] |
| Node | Renderer path | ✓ | v22.22.0 | Use through Nix shell. [VERIFIED: local probe] |
| pnpm | Existing frontend commands | ✗ ambient | — | Nix shell Corepack. [VERIFIED: local probe; flake.nix] |
| Weston | Real native smoke | ✗ ambient | — | Nix shell; otherwise unavailable record. [VERIFIED: local probe; flake.nix] |
| Debian 13 GUI | REF-01E | not verified here | — | Human/operator platform gate; no synthetic substitute. [VERIFIED: 01-REAUDIT.md] |
| Accepted Napp candidate | REF-01D | ✗ | none | Fail-closed handoff. [VERIFIED: local Napp HEAD] |

**Missing dependencies with no fallback:** accepted committed Napp candidate for adapter work. [VERIFIED: D-01]

## Security Domain

OWASP ASVS is a verification framework. This phase applies access control, session/lifecycle, validation, resource, and configuration checks to the POC boundary. [CITED: https://devguide.owasp.org/en/11-security-gap-analysis/01-guides/02-asvs/]

| ASVS Category | Applies | Phase control |
|---|---|---|
| V2 Authentication | No | Read identity is a context, not login; no authentication feature is added. [VERIFIED: uzel-poc-validated-pack/AGENTS.md] |
| V3 Session Management | Yes | Preserve typed review/confirm/launch/cancel/stop/reconcile/rollback tests. [VERIFIED: codebase grep] |
| V4 Access Control | Yes | Host-created source binding, native navigation policy, no caller-selected principal. [VERIFIED: codebase grep] |
| V5 Validation | Yes | Bounded typed envelopes, exact fixtures, socket/path checks, fail-closed candidate probe. [VERIFIED: codebase grep] |
| V6 Cryptography | No new work | Preserve exact signed fixture/build evidence; do not add crypto/signing. [VERIFIED: 01-CONTEXT.md] |
| V12 Files/resources | Yes | Retain resource bytes/object-URL bounds and cancellation/cleanup evidence. [VERIFIED: codebase grep] |
| V14 Configuration | Yes | Clean target, locked inputs, XDG/socket path evidence, root-doc authority admission. [VERIFIED: 01-REAUDIT.md] |

| Threat | STRIDE | Standard mitigation |
|---|---|---|
| Forged napplet message selects surface/principal | Spoofing/elevation | Trusted host maps source to created surface; runtime maps token to session. [VERIFIED: codebase grep] |
| Malformed/oversized request/resource exhausts retention | Tampering/DoS | Preserve maximum envelope/frame/replay/state/resource bounds and tests. [VERIFIED: codebase grep] |
| Browser frame gets native/network authority | Elevation | Opaque frame, denied Tauri bridge/navigation, hostile native/browser probes. [VERIFIED: 01-REAUDIT.md] |
| Candidate/pin substitution | Tampering | Exact repo+SHA, tree/export/vector/probe checks, no dirty source acceptance. [VERIFIED: D-01] |
| Partial launch/retry leaves extra runtime state | Integrity/DoS | Snapshot reconciliation stops ambiguous sessions; identity persistence rolls back. [VERIFIED: crates/napd/src/runner.rs] |

## Sources

### Primary (HIGH confidence)

- Phase context, re-audit, and pattern map — bounded lane scope, live re-audit results, issue slicing, artifact patterns, blockers. [VERIFIED: 01-CONTEXT.md; 01-REAUDIT.md; 01-PATTERNS.md]
- Uzel source/tests/fixtures — Rust runner/server/protocol, trusted shell, Tauri main, Chromium harness, Weston/WebKit smoke, fixture corpus. [VERIFIED: codebase grep]
- Git state — planning head 3a571baa58e2fd7db8a2fbf78cd9a5093c359ac6, production baseline 19519c378c2e775c6ad4b042cfd9aadd89f766b9, and untracked root pack status. [VERIFIED: local git]
- Napp committed HEAD 0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e — docs-only tree. [VERIFIED: local Napp HEAD]

### Secondary (MEDIUM confidence)

- [Tauri capabilities](https://v2.tauri.app/security/capabilities/) — capability scope/permission merging.
- [OWASP ASVS guide](https://devguide.owasp.org/en/11-security-gap-analysis/01-guides/02-asvs/) — verification-category framework.
- [Git history](https://git-scm.com/docs/git-history) — rewrite is experimental and changes history identity.

## Metadata

**Confidence breakdown:**

- POC replay/architecture: HIGH — code, fixture, harness, re-audit, and git evidence agree. [VERIFIED: codebase + 01-REAUDIT.md]
- MVP slice scope: HIGH — locked D-01 through D-18 and re-audit explicitly define ready/blocked lanes. [VERIFIED: 01-CONTEXT.md; 01-REAUDIT.md]
- Candidate seam: HIGH for current absence; LOW for future API shape. [VERIFIED: local Napp HEAD]
- Measurement values: LOW until REF-01B records raw current samples. [VERIFIED: 01-REAUDIT.md]

**Research date:** 2026-08-09  
**Valid until:** Re-run qualification on every Napp commit and re-run clean/relocated replay after toolchain/pin changes. [VERIFIED: D-01/D-15]
