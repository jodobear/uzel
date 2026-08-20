# Roadmap: Uzel

Shape and plan only the next selected GitHub issue under [`WORKFLOW.md`](../WORKFLOW.md).

## Phases

- [x] **Phase 1: POC replay and runtime recovery** — replay accepted behavior and prove current trusted-runtime recovery.
- [x] **Phase 2: Canonical Nix release** — ship one exact-pinned Linux artifact with its compatible runtime.
- [ ] **Phase 3: Large-napplet native alpha** — run one real multi-megabyte exact-build napplet through the packaged runtime with bounded transfer and cleanup.
- [ ] **Phase 4: Truthful native profile** — preserve local-first profile value, rendered resources, reconnect truth, and responsive cleanup.
- [ ] **Phase 5: Signed napplet install flow** — review, install, launch, and close a signed coordinate through one honest user flow.
- [ ] **Phase 6: Source-bound Social Home** — compose bounded graph, feed, and navigation napplets.

## Phase Details

### Phase 1: POC replay and runtime recovery

**Goal**: Replay current accepted behavior, prove source binding/restart/native recovery, and disposition protected incident evidence.
**Depends on**: Nothing
**Plans:** 1 completed plan

Plans:
- [x] 01-01-PLAN.md — accepted replay, recovery, evidence disposition, review, and merge complete.

**Delivery unit:** Issue #42 is closed and PR #43 merged reviewed head `63bd6aa` as
`577e02b`. The bounded closeout correction records the missing GSD summary, verification,
and next-phase pointer without reopening Phase 1 implementation.

- [x] Accepted behavior is reproducible or honestly dispositioned.
- [x] Protected incident evidence has a final compact durable retention disposition.
- [x] Current trusted-runtime source binding, restart recovery, and native WebKit recovery pass.

### Phase 2: Canonical Nix release

**Goal**: Ship one exact-pinned store-path Linux artifact with compatible native runtime dependencies.
**Depends on**: Phase 1
**Plans:** 1 completed plan

Plans:
- [x] 02-01-PLAN.md — canonical package, runtime compatibility, native recovery, review, and merge complete.

**Delivery unit:** Issue #46 is closed and PR #48 merged reviewed head `2c27277` as
`43e983f`.

- [x] One reproducible Linux artifact is produced from locked inputs.
- [x] Runtime compatibility is bound to the artifact.

### Phase 3: Large-napplet native alpha

**Goal**: Prove the packaged native runtime can safely load and run a useful multi-megabyte napplet instead of remaining a small-fixture demonstration.
**Depends on**: Phase 2
**Primary issue:** #47. Pull #18 cleanup behavior into this slice only where large transfer interruption exercises it.

- [ ] One finite configurable artifact policy governs verified read, staging, chunked transfer, reconstruction, and refusal.
- [ ] One real multi-megabyte exact-build napplet runs under packaged Weston/WebKit with source binding and sandbox restrictions intact.
- [ ] Interrupted or refused transfer leaves no surface, partial artifact, owned process, or socket behind.

### Phase 4: Truthful native profile

**Goal**: Make the packaged native app visibly useful offline and during network failure without duplicating NMP state.
**Depends on**: Phase 3
**Primary issue:** #12. Issues #16, #17, and the remaining #18 cleanup cases are linked
acceptance inputs, not separate Phase 4 delivery units.

- [ ] Local profile value survives unavailable networks.
- [ ] Profile resources render as real WebKit pixels with bounded failure and cancellation behavior.
- [ ] Refresh, relay loss, reconnect, stale, partial, and failed outcomes remain truthful and accessible.
- [ ] Slow NMP/resource work cannot block lifecycle cleanup.

### Phase 5: Signed napplet install flow

**Goal**: Turn an exact signed coordinate into a reviewed, installed, running, and cleanly closed native napplet through one coherent user journey.
**Depends on**: Phase 4
**Primary issue:** #13. Use at most one independently published fixture from #21 when it directly proves this flow; do not pull in the whole corpus.

- [ ] Raw `naddr1...` and `nostr:naddr1...` inputs reach the same exact review and install path.
- [ ] Invalid, wrong-kind, resolution, verification, permission, ambiguity, and cleanup failures are visibly distinct and fail closed.
- [ ] One independently published napplet completes review, install, launch, interaction, close, and restart reconciliation under real WebKit.

### Phase 6: Source-bound Social Home

**Goal**: Add bounded graph, feed, and navigation as independent composed napplets without duplicate runtime or Nostr state.
**Depends on**: Phase 5
**Primary work:** SOC-01 through SOC-06. #14 and #15 remain deferred unless they block this user flow.

- [ ] Graph, feed, and navigation compose through accepted boundaries.
- [ ] Runtime and Nostr state remain singly owned.
