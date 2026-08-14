# Phase 1: POC replay and runtime recovery - Context

**Gathered:** 2026-08-13
**Reoriented:** 2026-08-14
**Status:** Complete
**Source:** GitHub issue #42 and human product-architecture correction

<domain>
## Phase Boundary

Replay the accepted Uzel POC at exact current pins, establish source binding at Uzel's current trusted runtime boundary, observe restart and native WebKit recovery, and leave protected incident evidence untouched until a human authorizes durable disposition.
</domain>

<decisions>
## Implementation Decisions

### Current product boundary
- Uzel owns product runtime, composition policy, trusted surfaces, and its private daemon.
- NMP remains sole Nostr authority.
- Current source/locks prove `jodobear/nampplets` and `pablof7z/nmp` as upstreams; other repositories require concrete source or reusable-fix evidence.
- REF-01 through REF-06 use executable, source-grounded checks; no external framework admission gate remains.

### Observable completion
- REF-02 closes only when current source and probes prove source binding and denied authority at Uzel's runtime boundary.
- REF-03 closes only when a restart replay observes recovered identity, follows, installed exact builds, and ambiguous lifecycle state.
- REF-04 closes only when deterministic Chromium and real Weston/WebKit recovery evidence pass.
- Failed or unavailable observations remain exact blockers; intent never counts as completion.

### Delivery
- Use issue #42, one plan, the existing branch/worktree, and draft PR #43.
- Use focused checks during debugging, then one proportional affected validation and one stable-head Codex/CodeRabbit review cycle.
- Exclude Phase 2 packaging, Phase 3 CI expansion, later product capabilities, Graphify refresh, and new process machinery.

### the agent's Discretion
- Exact focused probes and smallest changes to existing runner/test/smoke surfaces.
</decisions>

<canonical_refs>
## Canonical References

### Active authority
- `WORKFLOW.md`
- `.planning/PROJECT.md`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- current source, imports, locks, and passing executable probes

### Bounded replay evidence
- `uzel-poc-validated-pack/AGENTS.md`
- `uzel-poc-validated-pack/STATUS.md`
- `uzel-poc-validated-pack/work/00-validate.md`
- `evidence/phase-01/poc-replay.md`
- `evidence/phase-01/ownership-map.md`
</canonical_refs>

<specifics>
## Specific Ideas

Issue: https://github.com/jodobear/uzel/issues/42
</specifics>

<deferred>
## Deferred Ideas

Canonical Nix release, CI expansion, profile/resource additions, Social Home, local files, Blossom, signing, wallets, later platforms, and protected-evidence movement remain outside this autonomous slice.
</deferred>

---
*Phase: 01-poc-replay-and-runtime-recovery*
