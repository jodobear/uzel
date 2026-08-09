# Roadmap: Uzel

## Overview

This MVP preserves the proven local-first napplet POC behind an accepted Napp seam, makes its exact runtime reproducible in the canonical Nix artifact, establishes fail-closed delivery evidence, then delivers a source-bound Social Home in two visible slices. The roadmap is sequential as the durable execution record, while REF and PKG may be worked independently and CI may measure early before its package/merge-full contract consumes PKG.

## Phases

**Phase Numbering:**
- Integer phases are planned milestone work.
- Decimal phases are urgent insertions only.

- [ ] **Phase 1: SLICE-REF-01 — POC Replay & Accepted Napp Seam** - Preserve evidence and establish the only permitted consumer seam for Social work.
- [ ] **Phase 2: SLICE-PKG-01 — Canonical Nix Release Closure** - Deliver the exact-pinned, store-path Linux artifact.
- [ ] **Phase 3: SLICE-CI-01 — Measured Fail-Closed Delivery Lanes** - Make scoped PR and merge-queue evidence observable and fail closed.
- [ ] **Phase 4: SLICE-SOC-01 — Local Profile, Resources & Honest State** - Show local-first profile and resource state through the accepted seam.
- [ ] **Phase 5: SLICE-SOC-02 — Source-Bound Graph, Feed & Navigation** - Complete the composable Social Home experience.

## Phase Details

### Phase 1: SLICE-REF-01 — POC Replay & Accepted Napp Seam
**Goal**: Developers and reviewers can prove the preserved POC works through one accepted Napp consumer seam without recreating runtime or Nostr authority.
**Mode:** mvp
**Depends on**: Nothing (first phase; Social Napp gate)
**Requirements**: REF-01, REF-02, REF-03, REF-04, REF-05, REF-06, REF-07
**Success Criteria** (what must be TRUE):
  1. Developer can replay exact-build review, confirmation, launch, and multi-surface composition against one accepted committed Napp client/testkit candidate.
  2. Release reviewer can verify source-created-surface binding and denial of raw frame network, Tauri bridge, paths, secrets, and caller-selected principals before requests reach Napp.
  3. Operator can restart Uzel and recover selected read identity, exact builds, useful local profile/follow state, and ambiguous lifecycle outcomes without duplicate runtime or Nostr state.
  4. Maintainer can inspect replayable Chromium and Weston/WebKit evidence, a durable ownership disposition, baseline lifecycle/resource/performance measurements, and repository-qualified missing-contract dependencies instead of private substitutes.
**Plans**: TBD

### Phase 2: SLICE-PKG-01 — Canonical Nix Release Closure
**Goal**: Linux users and release reviewers can build, inspect, and run Uzel as a self-contained Nix artifact with its exact compatible Napp runtime.
**Mode:** mvp
**Depends on**: Nothing (independent early work; recorded after Phase 1 for sequential execution)
**Requirements**: PKG-01, PKG-02, PKG-03, PKG-04, PKG-05
**Success Criteria** (what must be TRUE):
  1. Linux user can build `packages.uzel` and default package/app from locked `x86_64-linux` inputs, with the development shell and package-dependent checks exposed by the flake.
  2. Release reviewer can prove Cargo, both lockfiles, fixtures, and the packaged runtime closure identify the same accepted full Napp commit.
  3. Linux user can start the package from its Nix store path without the checkout, development shell, ambient system packages, or arbitrary `nappd` discovery from `PATH`.
  4. Operator receives a clear incompatible-client/runtime failure rather than a silently incompatible daemon, and release reviewer can inspect the closure, size, Napp runtime reference, desktop assets, and applicable package/WebKit smoke evidence.
**Plans**: TBD

### Phase 3: SLICE-CI-01 — Measured Fail-Closed Delivery Lanes
**Goal**: Contributors and merge authority receive measured, exact-head delivery evidence that cannot pass through missing or skipped required work.
**Mode:** mvp
**Depends on**: Phase 2 for final package preflight and merge-full contract; current-lane measurement and initial PR-fast work may proceed independently of Phases 1–2.
**Requirements**: CI-01, CI-02, CI-03, CI-04, CI-05, CI-06, CI-07
**Success Criteria** (what must be TRUE):
  1. Contributor receives conservative changed-scope PR-fast results for docs, frontend, Rust, contracts, native-host/security, and package/toolchain changes, with shared or uncertain inputs promoted to full scope.
  2. Contributor receives formatter, typecheck, unit, Fallow, Rust, and targeted Chromium evidence only when applicable; package/toolchain and host/security/native/package changes also receive their required Nix or Weston/WebKit proof.
  3. Merge queue validates the exact `merge_group` SHA with the canonical Nix package and full required suite before GitHub merges.
  4. One stable required aggregator fails when a classified job is missing, cancelled, failed, or unexpectedly skipped instead of accepting path-filter silence.
  5. Maintainer can inspect p50/p95 and cache/cancellation measurements before retaining CI complexity, while merge authority can bind local CodeRabbit, remote Codex, final CodeRabbit, lane, disposition, and zero-blocker evidence to the required exact head SHA.
**Plans**: TBD

### Phase 4: SLICE-SOC-01 — Local Profile, Resources & Honest State
**Goal**: Users can view and operate accessible local-first profile and resource state while its runtime freshness and recovery limits remain honest.
**Mode:** mvp
**Depends on**: Phase 1 accepted Napp candidate, Phase 2 canonical package, and Phase 3 required M0 delivery evidence.
**Execution gate**: Not executable until the accepted committed Napp candidate and required REF, PKG, and CI M0 gates are accepted; no private seam may be invented to start Social work.
**Requirements**: PROF-01, PROF-02, PROF-03, PROF-04, PROF-05
**Success Criteria** (what must be TRUE):
  1. User sees assigned profile data from Napp/NMP-projected local state before background refresh, without a Uzel-owned profile or freshness cache.
  2. User sees mediated profile-resource loading, success, failure, cancellation, and recovery while existing concurrency, byte, retry, object-URL, and viewport cleanup bounds hold.
  3. User can distinguish local, refreshing, stale, partial, blocked, and diagnosable error states while valid local profile data remains visible through refresh failure.
  4. Keyboard and assistive-technology users can operate profile and resource state with stable focus, semantic controls, accessible names, and non-disruptive status announcements.
  5. Developer can opt into compact pressure, cache, freshness, and failure diagnostics without exposing secrets, authority tokens, raw runtime internals, or creating another state owner.
**Plans**: TBD
**UI hint**: yes

### Phase 5: SLICE-SOC-02 — Source-Bound Graph, Feed & Navigation
**Goal**: Users can navigate one composable Social Home whose local graph, feed, and profile selection stay source-bound, useful, and accessible through recovery.
**Mode:** mvp
**Depends on**: Phase 4 and its accepted REF, PKG, and CI M0 gates.
**Execution gate**: Inherits the Phase 4 Social execution gate; only the accepted Napp seam and required delivery evidence may carry graph/feed/navigation work.
**Requirements**: SOC-01, SOC-02, SOC-03, SOC-04, SOC-05, SOC-06
**Success Criteria** (what must be TRUE):
  1. User sees known follows and graph edges from Napp/NMP-projected local state first, with explicit incomplete, stale, and refreshing semantics rather than a global-completeness claim.
  2. User sees a local feed and navigation view that retains useful items and selection through refresh, partial results, failure, and recovery, with its local projection ordering described.
  3. User can select a profile from graph or feed navigation through source-bound, runtime-mediated `napplet:profile/open` routing that cannot choose its own principal or bypass the trusted host.
  4. Profile, graph, and feed remain independent exact-build napplets composed into one visible Social Home without direct frame peering or duplicate runtime/Nostr data paths.
  5. Keyboard and assistive-technology users can navigate graph, feed, and profile selection with stable focus and accessible state changes; release reviewer can verify bounded surfaces, queues, subscriptions, resources, retries, cancellation, recovery, hostile-egress, and affected real-WebKit behavior.
**Plans**: TBD
**UI hint**: yes

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. SLICE-REF-01 — POC Replay & Accepted Napp Seam | 0/TBD | Not started | - |
| 2. SLICE-PKG-01 — Canonical Nix Release Closure | 0/TBD | Not started | - |
| 3. SLICE-CI-01 — Measured Fail-Closed Delivery Lanes | 0/TBD | Not started | - |
| 4. SLICE-SOC-01 — Local Profile, Resources & Honest State | 0/TBD | Not started | - |
| 5. SLICE-SOC-02 — Source-Bound Graph, Feed & Navigation | 0/TBD | Not started | - |
