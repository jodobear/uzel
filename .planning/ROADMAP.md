# Roadmap: Uzel

Shape and plan only the next selected GitHub issue under [`WORKFLOW.md`](../WORKFLOW.md).

## Phases

- [ ] **Phase 1: POC replay and runtime recovery** — replay accepted behavior and prove current trusted-runtime recovery.
- [ ] **Phase 2: Canonical Nix release** — ship one exact-pinned Linux artifact with its compatible runtime.
- [ ] **Phase 3: Measured fail-closed delivery** — provide scoped evidence and exact merge-head gates.
- [ ] **Phase 4: Local profile and honest resource state** — preserve local-first value with accessible state truth.
- [ ] **Phase 5: Source-bound Social Home** — compose bounded graph, feed, and navigation napplets.

## Phase Details

### Phase 1: POC replay and runtime recovery

**Goal**: Replay current accepted behavior, prove source binding/restart/native recovery, and disposition protected incident evidence.
**Depends on**: Nothing
**Plans:** 1 current plan

Plans:
- [ ] 01-01-PLAN.md — Replay complete; protected disposition and REF-02, REF-03, and REF-04 remain active.

**Blocked dependencies:** Issue #42, its dedicated branch/worktree, and PR #43 remain the single
Phase 1 delivery unit; no second issue, plan, branch, or PR is created. Human-authorized
non-destructive durable preservation may resume independently. Current Uzel runtime recovery work
proceeds without an external framework gate.

- [x] Accepted behavior is reproducible or honestly dispositioned.
- [ ] Protected incident evidence has a final non-destructive retention disposition.
- [ ] Current trusted-runtime source binding, restart recovery, and native WebKit recovery pass.

### Phase 2: Canonical Nix release

**Goal**: Ship one exact-pinned store-path Linux artifact with compatible native runtime dependencies.
**Depends on**: Phase 1

- [ ] One reproducible Linux artifact is produced from locked inputs.
- [ ] Runtime compatibility is bound to the artifact.

### Phase 3: Measured fail-closed delivery

**Goal**: Provide scoped PR evidence and exact merge-head gates without duplicate receipt machinery.
**Depends on**: Phase 2

- [ ] Delivery evidence is scoped to affected behavior.
- [ ] Merge gates fail closed on the exact reviewed head.

### Phase 4: Local profile and honest resource state

**Goal**: Preserve local-first profile value through the trusted runtime boundary with accessible stale, partial, and failure truth.
**Depends on**: Phase 3

- [ ] Local profile value survives unavailable networks.
- [ ] Resource state communicates stale, partial, and failed outcomes accessibly.

### Phase 5: Source-bound Social Home

**Goal**: Add bounded graph, feed, and navigation as independent composed napplets without duplicate runtime or Nostr state.
**Depends on**: Phase 4

- [ ] Graph, feed, and navigation compose through accepted boundaries.
- [ ] Runtime and Nostr state remain singly owned.
