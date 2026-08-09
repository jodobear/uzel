# Phase 01: SLICE-REF-01 — POC Replay & Accepted Napp Seam - Research

**Researched:** 2026-08-09  
**Domain:** Linux POC evidence replay, recovery/resource baselining, Napp-consumer qualification  
**Confidence:** MEDIUM

> **Re-audit note:** `01-REAUDIT.md` supersedes this document where it treated candidate
> absence as blocking all Phase 1 work. Current-POC replay, ownership/pressure evidence,
> and document admission are independently plannable; only Napp adaptation is blocked.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Accepted Napp Candidate Gate
- **D-01:** Phase 1 may integrate only one exact, reachable, committed Napp candidate whose product-facing client, events, and testkit vectors are present in source and pass the phase's executable probes. — **Reversibility:** costly — A later candidate change requires repinning source and locks and replaying the trust, lifecycle, fixture, and platform evidence.
- **D-02:** Candidate absence or a falsified contract stops qualification/adaptation, not independent POC replay or ownership evidence. Record the full Napp repository, exact observed commit, missing contract, and required acceptance evidence; do not preserve progress through a Uzel-only facade or legacy protocol expansion.
- **D-03:** The accepted candidate must keep NMP as the sole Nostr engine/store and Napp as runtime authority. Uzel owns presentation, trusted-host integration, and product-visible recovery only.

### Replay and Ownership Evidence
- **D-04:** Replay the existing POC before extraction or seam replacement. Evidence must cover exact-build review, confirmation, launch, multi-surface composition, source binding, hostile denial, restart/reconciliation, deterministic Chromium, and real Weston/WebKit behavior.
- **D-05:** Produce one durable ownership disposition tied to exact files, tests, repositories, and commits. Classify each relevant POC element as retained Uzel product code, consumed Napp contract, neutral upstream candidate, compatibility-only seam, or obsolete POC behavior.
- **D-06:** Reconcile legacy Work 07 with source truth: PR #30 is merged at `19519c3`; visible Debian 13 acceptance remains unresolved. Preserve the historical record and do not claim its human gate complete.

### Baseline Measurement
- **D-07:** Record reproducible measurements, commands, environment identity, and limitations for cold start, first visible frame, local profile render, idle CPU/RSS, WebView memory, resource flow, queue bounds, cancellation, and lifecycle recovery.
- **D-08:** Mark a measurement `unavailable` with reason when the current environment cannot produce it. Do not estimate or convert a qualitative observation into a numeric claim.
- **D-09:** Separate build, dependency-cache, fixture-mode, and release-runtime measurements so one cannot be presented as another.

### Recovery and Upstream Stops
- **D-10:** Lost replies, duplicate retries, stale sessions, partial launch, cancellation, restart reconciliation, and cleanup remain typed, bounded lifecycle outcomes. Preserve byte-identical replay inputs and idempotent stop/cancel behavior.
- **D-11:** On contradiction, stop at the smallest falsifying probe, preserve the current green POC, update the durable fact/disposition record, and resume only from an accepted reachable commit.
- **D-12:** Any reusable fix must use a dedicated branch in the corresponding `jodobear` fork and be recorded in `uzel-poc-validated-pack/docs/08-upstream-contributions.md` before Uzel depends on it.

### the agent's Discretion
The planner may choose the exact document names, measurement harness layout, and probe grouping, provided every artifact remains exact-source-backed, replayable, bounded, and within this phase boundary.

### Deferred Ideas (OUT OF SCOPE)
- Canonical Nix release closure belongs to Phase 2.
- PR-fast, merge-group, and required-check aggregation belong to Phase 3.
- New profile/resource gaps and Social graph/feed work belong to Phases 4 and 5. Existing rich profile/follow behavior is Phase 1 preservation evidence, not a feature to rebuild.
- Files, Blossom, signing, wallets, authoring, richer media, ContextVM, Relatr, TUI, WASI, Android, and native napplets remain outside this milestone.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|---|---|---|
| REF-01 | Replay current merged POC at exact pins before adaptation. | Existing tests/fixtures and fresh clean-target replay support an independent preservation lane. [VERIFIED: live replay] |
| REF-02 | Trusted source-created surface binding. | Existing trusted shell matches MessageEvent.source to created frame; retain it and hostile proof. [VERIFIED: codebase grep] |
| REF-03 | Recovery without duplicate truth. | Existing runner persists bounded product state and reconciles cleanup from fresh status. [VERIFIED: codebase grep] |
| REF-04 | Chromium plus Weston/WebKit proof. | Existing renderer and native smoke are complementary. [VERIFIED: codebase grep] |
| REF-05 | Durable ownership map. | Use one machine-readable exact source/test/commit disposition map. [VERIFIED: codebase grep] |
| REF-06 | Measured baseline evidence. | Preserve raw samples, exact mode/head/environment, or record unavailable. [VERIFIED: codebase grep] |
| REF-07 | Public Napp seam only. | Local Napp has no implementation candidate; write dependency/handoff and stop. [VERIFIED: codebase grep] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Preserve proven POC code/tests; Tauri 2 + Svelte stays locked; Rust owns runtime-facing state and Svelte presentation. [VERIFIED: codebase grep]
- Source-bind every napplet request before Napp; never import NMP/nampplets internals or duplicate grants, caches, signers, runtime, profile, follow, relay, or event truth. [VERIFIED: codebase grep]
- Keep UI non-blocking, WebViews bounded, identifiers descriptive and at most 21 characters, strict TypeScript/accessibility/Fallow/real-WebKit evidence intact. [VERIFIED: codebase grep]
- Use Nix canonical artifact; align Cargo/flake Napp pins; never discover Napp daemon from PATH; pin exact committed client/testkit SHA. [VERIFIED: codebase grep]
- Use exact commits, sources, tests, and probes; reuse → adapt → contribute → build proven missing seam; no fake fallback, generic manager, or unbounded queue/retry. [VERIFIED: codebase grep]
- Treat frames as untrusted: no raw network, Tauri bridge, secrets, host paths, or caller-selected principal. [VERIFIED: codebase grep]
- Do not alter Work 07/PR #30 state. [VERIFIED: codebase grep]

## Summary

Phase 01 is a split evidence-and-integration gate. Current Uzel POC has concrete Rust protocol/runner tests, trusted-shell source binding, exact fixtures, rich profile/follow composition, deterministic Playwright/Chromium acceptance, and Weston/WebKit smoke. Replay and classify these before any candidate adaptation. [VERIFIED: live replay plus codebase]

No accepted Napp candidate exists locally. Local reachable jodobear/napp commit `0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e` contains planning documents only; its dirty sibling audit is not accepted implementation. Reachable committed history has no Cargo/package manifest, Rust source tree, public client, events implementation, testkit, Nix output, or executable candidate probe. This blocks adaptation, not POC preservation. [VERIFIED: live repository inspection]

Plan POC replay, ownership/pressure evidence, and document admission now. Plan candidate qualification as an independent fail-closed probe; if source/probes remain absent, record repository jodobear/napp, observed full SHA, missing client/events/testkit/probe evidence, and resumable upstream handoff. Do not alter Uzel production code, add façade, widen POC protocol, or repin. [VERIFIED: re-audit]

**Primary recommendation:** Plan baseline preservation plus a fail-closed Napp handoff; condition all adaptation work on a later accepted committed candidate. [VERIFIED: codebase grep]

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|---|---|---|---|
| Candidate provenance and pin parity | Dependency/build | Runtime API | SHA, source exports, testkit, Cargo, and flake must agree before consumption. [VERIFIED: codebase grep] |
| Review, launch, lifecycle | Runtime/backend | Trusted host | Existing Rust runner owns session/lifecycle truth. [VERIFIED: codebase grep] |
| Source authorization | Trusted browser host | Runtime/backend | Host maps created frame to surface before routing. [VERIFIED: codebase grep] |
| Recovery/state | Runtime/storage | Presentation | Runner owns persisted product state and reconciliation. [VERIFIED: codebase grep] |
| Renderer/native proof | Browser/client | Native host | Chromium tests UI; Weston/WebKit tests native projection/isolation. [VERIFIED: codebase grep] |
| Resource/lifecycle metrics | Native harness | Browser harness | Measure process/WebView/queue/cancel/recovery with raw evidence. [VERIFIED: codebase grep] |

## Standard Stack

No new dependency is approved or recommended in this phase. [VERIFIED: codebase grep]

| Component | Version / Pin | Purpose | Directive |
|---|---|---|---|
| Existing Rust workspace | edition 2024; Rust 1.89 | POC runtime/protocol/native baseline. [VERIFIED: codebase grep] | Preserve and replay only. [VERIFIED: codebase grep] |
| Tauri | =2.11.5 | Native shell/IPC boundary. [VERIFIED: codebase grep] | Retain exact POC trust evidence. [VERIFIED: codebase grep] |
| Playwright | 1.62.0 | Deterministic renderer evidence. [VERIFIED: codebase grep] | Keep separate from native proof. [VERIFIED: codebase grep] |
| jodobear/nampplets POC pin | e2f69f325a6b45213accdacfcc125e80e0687b4c | Current implementation below POC napd. [VERIFIED: codebase grep] | Baseline only, not Napp consumer seam. [VERIFIED: codebase grep] |
| Napp client/events/testkit | none accepted | Future public seam. [VERIFIED: codebase grep] | Fail closed. [VERIFIED: codebase grep] |

**Installation:** None. No package legitimacy audit applies because this phase must not install or add a package. [VERIFIED: codebase grep]

## Architecture Patterns

### System Architecture Diagram

~~~
exact POC head + fixtures + toolchain
              |
              v
       baseline replay / raw evidence
              |
              v
Napp repo + SHA --> source export probe --> client + events + testkit?
                                              | yes
                                              v
                         candidate probes + POC replay matrix
                                              |
                                      all required pass?
                                       | yes      | no
                                       v          v
                           accepted seam record   smallest falsifier,
                           + Cargo/flake parity   preserve POC, stop
                                       |
                                       v
trusted host source binding --> accepted client --> Napp --> NMP
~~~

Candidate branch always precedes adaptation. A failure produces a repository-qualified dependency record, never substitute code. [VERIFIED: codebase grep]

### Recommended Evidence Structure

~~~
evidence/phase-01/
  candidate-qualification.md
  ownership-disposition.json
  replay-manifest.json
  measurements/
  work-07-preservation.md
~~~

Filenames are discretionary; each record must bind source SHA, command, fixture/hash, raw result, mode, and limitation. [VERIFIED: codebase grep]

### Pattern 1: Candidate Gate Before Adaptation

**What:** Verify exact repository+commit, source exports, committed vectors, declared test/probe command, and replay matrix before using a Napp surface. [VERIFIED: codebase grep]  
**When:** First Wave and after any candidate/pin/fixture/platform change. [VERIFIED: codebase grep]

~~~bash
git -C "$candidate_repo" cat-file -e "$candidate_sha^{commit}"
git -C "$candidate_repo" ls-tree -r --name-only "$candidate_sha" > candidate-files.txt
rg -q 'crates/napp-client/' candidate-files.txt
rg -q 'crates/napp-testkit/' candidate-files.txt
test -f "$candidate_repo/Cargo.toml"
# Run only executable commands declared by that exact candidate.
~~~

Local Napp fails this skeleton because no implementation source paths exist in its reachable history. [VERIFIED: codebase grep]

### Pattern 2: Evidence-Preserving Replay

**What:** Record exact heads, candidate, fixture hashes, command, tool identity, mode, raw output, result, and limitation together. [VERIFIED: codebase grep]  
**When:** Before candidate work and after each changed trust/lifecycle/resource seam. [VERIFIED: codebase grep]

~~~json
{
  "git_head": "<full Uzel SHA>",
  "candidate": { "repository": "<owner/repo>", "commit": "<full SHA>" },
  "mode": "fixture | build | dependency-cache | release-runtime",
  "result": "pass | fail | unavailable",
  "raw_output": "relative/path/to/log"
}
~~~

This record shape is recommended only; it does not assert a future Napp API. [ASSUMED]

### Pattern 3: Source-Bound Envelope Projection

**What:** Resolve browser message source to host-created frame, then pass only runtime-owned surface identity onward. [VERIFIED: codebase grep]  
**When:** Every napplet-originated request, cancellation, resource terminal, and cross-napplet delivery. [VERIFIED: codebase grep]

Tauri capabilities scope IPC by WebView/window and overlapping capabilities merge permissions; untrusted frames must not receive broad/shared capability access, while source binding remains an application control. [CITED: https://v2.tauri.app/security/capabilities/]

### Anti-Patterns to Avoid

- Candidate-by-documentation: architecture plans are not public source, testkit, or executable proof. [VERIFIED: codebase grep]
- POC-private IPC promotion: do not present crates/napd-protocol as Napp public API. [VERIFIED: codebase grep]
- One-browser proof: mocked Chromium cannot replace real Weston/WebKit. [VERIFIED: codebase grep]
- Metric substitution: unavailable measurement is not an estimate. [VERIFIED: codebase grep]
- Historical overwrite: preserve Work 07 record and add dated reconciliation only. [VERIFIED: codebase grep]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---|---|---|---|
| Missing Napp seam | Uzel façade/expanded private protocol | Repository-qualified Napp dependency and stop gate | Avoid permanent POC-private contract. [VERIFIED: codebase grep] |
| Nostr state | Uzel profile/follow/relay/event store | NMP through accepted Napp surface | NMP is sole Nostr engine/store. [VERIFIED: codebase grep] |
| Frame authority | Caller-selected surface/principal | Host-created frame source mapping | Payload identity can be forged. [VERIFIED: codebase grep] |
| Bounds | New queue/retry/resource path | Existing bounded POC proof, then accepted Napp mechanism | Existing limits/tests cover envelopes, frames, replay, resources, cancellation. [VERIFIED: codebase grep] |
| Runtime toolchain | Ambient host tools/PATH daemon | Pinned Nix shell/closure | Host lacks ambient pnpm and Weston. [VERIFIED: codebase grep] |

## Runtime State Inventory

| Category | Items Found | Action Required |
|---|---|---|
| Stored data | Uzel daemon defaults to XDG data root and runner persists bounded uzel-state.json; NMP truth stays behind runtime. [VERIFIED: codebase grep] | No migration. Use controlled ephemeral-XDG replay; map schema/owner before later transition. [VERIFIED: codebase grep] |
| Live service config | No Uzel service artifact in repo; sandbox denied user D-Bus inventory. [VERIFIED: codebase grep] | Record unavailable and run read-only inventory on operator host later. [VERIFIED: codebase grep] |
| OS-registered state | No Uzel service, desktop, plist, or PM2 artifact found in repo. [VERIFIED: codebase grep] | No re-registration; retain sandbox limitation. [VERIFIED: codebase grep] |
| Secrets/env vars | No Uzel/Napp secret file found; observed values are XDG/Wayland/test settings. [VERIFIED: codebase grep] | Do not rename/create env keys; redact Tauri invoke key in captured logs. [VERIFIED: codebase grep] |
| Build artifacts | Ignored UI artifacts and Nix dev inputs exist; no Napp executable/package exists locally. [VERIFIED: codebase grep] | Preserve as evidence, not product/testkit artifact. [VERIFIED: codebase grep] |

## Common Pitfalls

### Candidate reachability mistaken for acceptance

Local Napp SHA is reachable but docs-only. Prevent this by requiring source exports, vectors, executable probe, and replay evidence; absence is hard stop. [VERIFIED: codebase grep]

### POC evidence mistaken for Napp evidence

Current nampplets/napd proof is valuable baseline but not accepted Napp client/events/testkit proof. Label baseline and candidate results separately. [VERIFIED: codebase grep]

### Chromium substitutes for native proof

Renderer harness explicitly mocks native replies; preserve both renderer and Weston/WebKit results. [VERIFIED: codebase grep]

### Estimated resource numbers

Host lacks ambient pnpm and Weston. Record exact modes and raw samples or unavailable, never qualitative-to-numeric conversion. [VERIFIED: codebase grep]

### Work 07 historical drift

Legacy STATUS says PR #30 active while local Uzel history contains 19519c378c2e775c6ad4b042cfd9aadd89f766b9 with PR #30 merge subject. Preserve both observations and do not claim Debian gate complete. [VERIFIED: codebase grep]

## Code Examples

### Required current stop record

~~~markdown
repository: jodobear/napp
observed_commit: 0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e
reachable: yes
public_client_source: missing
public_events_source: missing
committed_testkit_vectors: missing
candidate_probe_command: missing
decision: stop; do not alter Uzel
resume_when: source plus probes satisfy qualification matrix
~~~

This is current local evidence, not a forecast of Napp state. [VERIFIED: codebase grep]

### Measurement discipline

~~~json
{
  "metric": "WebView RSS after idle",
  "mode": "release-runtime",
  "result": "unavailable",
  "reason": "real-WebKit runner unavailable in observed host",
  "head": "<full Uzel SHA>",
  "candidate": null
}
~~~

Unavailable is compliant; invented numeric values are not. [VERIFIED: codebase grep]

## State of the Art

| Old Approach | Current Approach | Impact |
|---|---|---|
| POC napd/private protocol as baseline | Public Napp seam only after accepted candidate | Prevent accidental product promotion of private IPC. [VERIFIED: codebase grep] |
| Browser-only confidence | Renderer plus Weston/WebKit smoke | Retains independent UI/native trust evidence. [VERIFIED: codebase grep] |
| Ambient tool assumption | Locked Nix shell | Reproducible flake inputs and controlled toolchain. [CITED: https://nix.dev/manual/nix/2.26/command-ref/new-cli/nix3-flake.html] |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|---|---|---|
| A1 | Evidence JSON layout is suitable for planner tooling. [ASSUMED] | Architecture Patterns | Low; format can change. |
| A2 | Future Napp crates will use planned client/testkit names. [ASSUMED] | Candidate gate | High; never bake names into Uzel before source verifies them. |

## Open Questions

1. **Exact accepted Napp candidate?**
   - Known: local Napp SHA is reachable but no source candidate exists. [VERIFIED: codebase grep]
   - Gap: full repository+SHA, public exports, testkit, output, commands. [VERIFIED: codebase grep]
   - Plan: stop after baseline evidence and hand off to Napp. [VERIFIED: codebase grep]
2. **What can target host measure?**
   - Known: Nix/Cargo/rustc/Node exist; ambient pnpm/Weston do not. [VERIFIED: codebase grep]
   - Plan: one probe per mode; retain raw output; classify pass/fail/unavailable. [VERIFIED: codebase grep]
3. **Current Work 07 remote status?**
   - Known: historical STATUS conflicts with local merge observation. [VERIFIED: codebase grep]
   - Plan: preserve and date reconciliation; no remote action. [VERIFIED: codebase grep]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|---|---|---|---|---|
| Nix | Pinned POC shell/native replay | ✓ | 2.34.1 | — [VERIFIED: codebase grep] |
| Cargo/rustc | Rust baseline | ✓ | 1.89.0 | — [VERIFIED: codebase grep] |
| Node | Frontend harness | ✓ | v22.22.0 | Nix shell. [VERIFIED: codebase grep] |
| pnpm | Frontend commands | ✗ ambient | — | Nix Corepack shell. [VERIFIED: codebase grep] |
| Weston | Real WebKit smoke | ✗ ambient | — | Nix shell or unavailable. [VERIFIED: codebase grep] |
| User D-Bus | Live unit inventory | ✗ sandbox | — | Operator-host probe. [VERIFIED: codebase grep] |
| Napp candidate | REF-01/07 | ✗ | none | Fail closed. [VERIFIED: codebase grep] |

**Missing dependencies with no fallback:** accepted committed Napp client/events/testkit candidate. [VERIFIED: codebase grep]

## Security Domain

OWASP ASVS provides a security verification framework; this phase must consider access control and input validation/sanitization. [CITED: https://devguide.owasp.org/en/11-security-gap-analysis/01-guides/02-asvs/]

| ASVS Category | Applies | Standard Control |
|---|---|---|
| V2 Authentication | No | Read identity is not login; no auth feature enters this phase. [VERIFIED: codebase grep] |
| V3 Session Management | Yes | Preserve session/retry/reconcile/stop evidence; no browser-owned runtime truth. [VERIFIED: codebase grep] |
| V4 Access Control | Yes | Source binding, runtime-owned surface identity, no caller-selected principal. [VERIFIED: codebase grep] |
| V5 Validation | Yes | Typed/size/schema/exact-fixture/source checks before routing. [VERIFIED: codebase grep] |
| V6 Cryptography | No new work | No custom signing/crypto; preserve exact-build/fixture inputs. [VERIFIED: codebase grep] |
| V12 Files/resources | Yes | Bounded resource bytes, cleanup, no frame filesystem authority. [VERIFIED: codebase grep] |

| Threat | STRIDE | Mitigation |
|---|---|---|
| Forged surface/principal | Spoofing/elevation | Created-frame source binding; native derives surface. [VERIFIED: codebase grep] |
| Oversized malformed envelope | Tampering/DoS | Preserve existing limits and tests. [VERIFIED: codebase grep] |
| Broad shared WebView capability | Elevation | No broad/overlapping capability for untrusted frames. [CITED: https://v2.tauri.app/security/capabilities/] |
| Candidate substitution/pin drift | Tampering | Exact repo+SHA, export/testkit/probe matrix, Cargo/flake parity. [VERIFIED: codebase grep] |

## Sources

### Primary (HIGH confidence)

- Local Uzel POC source/tests: Cargo workspace, napd runner/server/protocol, trusted shell, native host, renderer acceptance, Linux smoke. [VERIFIED: codebase grep]
- Local planning/POC artifacts: CONTEXT, REQUIREMENTS, STATE, consumer/source baseline, handoff, Work 07, STATUS, contribution ledger. [VERIFIED: codebase grep]
- Local Napp checkout/history at 0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e. [VERIFIED: codebase grep]

### Secondary (MEDIUM confidence)

- [Tauri capabilities](https://v2.tauri.app/security/capabilities/) — WebView IPC permissions and merging caveat.
- [Tauri security](https://v2.tauri.app/security/) — frontend/core boundary.
- [Nix flake reference](https://nix.dev/manual/nix/2.26/command-ref/new-cli/nix3-flake.html) — lock-file reproducibility.
- [OWASP ASVS guide](https://devguide.owasp.org/en/11-security-gap-analysis/01-guides/02-asvs/) — category applicability.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — all are existing local POC inputs; no package added. [VERIFIED: codebase grep]
- Architecture: HIGH — source binding, bounds, recovery, and harnesses exist locally. [VERIFIED: codebase grep]
- Candidate seam: HIGH for present rejection; LOW for future API. [VERIFIED: codebase grep]
- Pitfalls: HIGH — observed absent candidate, test separation, tool gaps, and historical drift. [VERIFIED: codebase grep]

**Research date:** 2026-08-09  
**Valid until:** Re-run candidate qualification before any integration; current rejection changes when Napp gains source. [VERIFIED: codebase grep]
