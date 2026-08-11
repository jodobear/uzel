# Phase 1: SLICE-REF-01 — POC Replay & Accepted Napp Seam - Context

**Gathered:** 2026-08-09
**Status:** Conditionally ready — preservation lanes ready; Napp adapter lane blocked

<domain>
## Phase Boundary

Preserve and replay the current Linux POC, prove its trust and recovery properties, measure a small current lifecycle/resource baseline, and establish the single accepted committed Napp client/events/testkit seam that later Uzel work may consume. Current-POC replay, ownership, pressure, and documentation-admission work may proceed without that candidate. Candidate adaptation may not. This phase does not rebuild existing rich profile/follow behavior, build new Social Home features, package Uzel for release, redesign Napp, or replace any missing Napp/NMP contract inside Uzel.

</domain>

<decisions>
## Implementation Decisions

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

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone and Consumer Contract
- `.planning/phases/01-slice-ref-01-poc-replay-accepted-napp-seam/01-REAUDIT.md` — Exact source audit, classifications, corrected scope, blockers, and readiness verdict.
- `.planning/PROJECT.md` — Product boundary, validated POC capabilities, constraints, and key decisions.
- `.planning/REQUIREMENTS.md` — Locked Phase 1 requirements REF-01 through REF-07.
- `.planning/ROADMAP.md` — Phase goal, dependencies, and success criteria.
- `docs/10-gsd-seed.md` — Proposed milestone ordering and Social execution gate; currently uncommitted.
- `docs/11-issue-seed.md` — Proposed initial Slice IDs and dependency intent; currently uncommitted.
- `PROGRAMME_CONTRACT.md` — Proposed product-first Uzel/Napp ownership rule; currently uncommitted.
- `NAPP_CONSUMER_PROFILE.md` — Proposed client/events/testkit-only consumption and pin rule; currently uncommitted.
- `SOURCE_BASELINE.md` — Proposed reviewed snapshot and upgrade rule; currently uncommitted and requiring live refresh before an upgrade.

### POC Evidence and Active State
- `evidence/POC_HANDOFF.md` — Proven POC outcome, candidate seams, surviving invariants, lessons, and unresolved human gate.
- `uzel-poc-validated-pack/AGENTS.md` — Evidence method, ownership, trust, stop rules, and upstream contribution policy.
- `uzel-poc-validated-pack/STATUS.md` — Durable legacy POC state, active Work 07/PR #30 status, exact evidence, and remaining Debian acceptance gate.
- `uzel-poc-validated-pack/docs/01-validation.md` — Gate 0 evidence and hard-stop conditions.
- `uzel-poc-validated-pack/docs/08-upstream-contributions.md` — Existing fork commits, validation, PR state, and repin rules.

### Existing Code and Tests
- `crates/napd/src/runner.rs` — Exact-build review/confirmation, runtime composition, lifecycle recovery, surface/session mapping, and response routing.
- `crates/napd/src/server.rs` — Private socket, replay cache, request serialization, and verified asset transfer.
- `crates/napd/src/fixtures.rs` — Signed exact fixture events, artifacts, identities, and limits.
- `apps/uzel/public/trusted-shell/trusted-shell.js` — Inert materialization, source-created-surface binding, and bounded NAP projection.
- `apps/uzel/src-tauri/src/main.rs` — Trusted native client boundary and navigation policy.
- `apps/uzel/tests/ui/acceptance.test.mjs` — Deterministic renderer, recovery, fault, and hostile-boundary evidence.
- `scripts/linux-run-smoke.sh` — Real Weston/WebKit integration and hostile-egress evidence.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `LinuxRunner` and its review/launch/reconciliation tests in `crates/napd/src/runner.rs`: primary executable reference for mapping the POC onto an accepted Napp client/testkit.
- Replay cache and bounded Unix protocol in `crates/napd/src/server.rs` and `crates/napd-protocol/src/lib.rs`: lifecycle ambiguity evidence; not presumed to be the final product-facing Napp contract.
- Exact signed fixtures in `crates/napd/src/fixtures.rs` and `fixtures/`: stable regression corpus for pin and source-binding replay.
- Chromium acceptance and Weston/WebKit smoke harnesses: complementary evidence realities that must remain separate.

### Established Patterns
- Ports-and-adapters ownership: Svelte/Tauri presentation delegates runtime truth to the daemon-owned runtime controller and NMP.
- Exact-build authority: reviewed publisher/build bytes and runtime-assigned identity determine executable authority.
- Fail-closed projection: opaque napplet frames receive only source-bound, capability-projected envelopes; no raw network or native bridge.
- Bounded recovery: operation IDs, replay, cancellation, cleanup, restart state, queues, and resources have explicit ceilings and terminal outcomes.

### Integration Points
- Replace or adapt only the narrow client/event/testkit boundary between Tauri/Uzel and the accepted Napp candidate.
- Keep trusted-host surface mapping before requests reach Napp; do not shift caller-principal selection into napplet code.
- Map neutral gaps to the exact Napp repository and fork workflow; keep product recovery/presentation in Uzel.

</code_context>

<specifics>
## Specific Ideas

- Use one machine-readable disposition table plus human rationale rather than scattered extraction notes.
- Bind every replay and measurement record to exact repository heads, commands, toolchain, environment, and artifact hashes.
- Run POC replay and documentation admission independently. Make candidate acceptance the first checkpoint of adapter implementation so missing contracts cannot trigger speculative Uzel code.

</specifics>

<deferred>
## Deferred Ideas

- Canonical Nix release closure belongs to Phase 2.
- PR-fast, merge-group, and required-check aggregation belong to Phase 3.
- New profile/resource gaps and Social graph/feed work belong to Phases 4 and 5. Existing rich profile/follow/resource behavior is preservation evidence, not future work to rebuild. New Social implementation remains gated on accepted REF, PKG, and CI evidence.
- Files, Blossom, signing, wallets, authoring, richer media, ContextVM, Relatr, TUI, WASI, Android, and native napplets remain outside this milestone.

</deferred>

---

*Phase: 1-SLICE-REF-01 — POC Replay & Accepted Napp Seam*
*Context gathered: 2026-08-09*
