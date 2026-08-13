# Phase 1: POC replay and accepted Napp seam - Context

**Gathered:** 2026-08-13
**Status:** Ready for planning
**Source:** PRD Express Path (GitHub issue #42)

<domain>
## Phase Boundary

Replay the accepted Uzel POC at its exact current pins, give protected incident evidence a final disposition, and adopt only one Napp consumer seam that passes the existing bounded qualification gate.

</domain>

<decisions>
## Implementation Decisions

### Accepted behavior and evidence
- REF-01 through REF-07 must have executable, source-grounded checks in one concise GSD plan.
- Revalidate preserved exact-build, source-binding, restart/recovery, hostile-egress, native-bridge denial, fixture, and baseline evidence.
- Every protected incident artifact must be retained with an owner and revisit trigger or receive an explicit evidence-backed final disposition.
- Existing POC behavior must be reproduced or each unavailable result must be honestly recorded with exact evidence.

### Napp seam
- Classify retained Uzel behavior, Napp consumption or extraction needs, upstream candidates, and obsolete POC-only behavior.
- Qualify one exact committed Napp client, events, and testkit candidate before any Uzel seam adaptation.
- Candidate qualification must fail closed and bind the exact Napp commit used by source, locks, fixtures, and validation.
- Replay through the qualified seam without duplicate runtime, Nostr, trust, cache, signer, or persistence ownership.

### Delivery
- Use one issue (#42), one plan, one dedicated branch/worktree, and one draft PR.
- Use focused checks during work, one proportional candidate validation, then Codex and substantive CodeRabbit on one stable exact head.
- Exclude Phase 2 packaging, Phase 3 CI expansion, and later product capabilities.

### the agent's Discretion
- Exact task ordering, focused command selection, and whether preserved temporary validators remain useful after source revalidation.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Active authority
- `WORKFLOW.md` — lean delivery and exact-head review contract.
- `.planning/ROADMAP.md` — Phase 1 boundary and outcomes.
- `.planning/REQUIREMENTS.md` — REF-01 through REF-07 acceptance requirements.

### Bounded replay evidence, not active workflow authority
- `uzel-poc-validated-pack/AGENTS.md` — POC engineering, trust, and evidence rules.
- `uzel-poc-validated-pack/STATUS.md` — accepted POC evidence and current limitations.
- `uzel-poc-validated-pack/work/00-validate.md` — validated command categories.

### Existing Phase 1 evidence
- `evidence/phase-01/candidate-qualification.md` — candidate gate evidence to revalidate.
- `evidence/phase-01/napp-dependency.md` — exact dependency disposition to revalidate.
- `scripts/ref-candidate-check.py` — bounded candidate validator to retain or delete based on current use.

</canonical_refs>

<specifics>
## Specific Ideas

Issue: https://github.com/jodobear/uzel/issues/42

</specifics>

<deferred>
## Deferred Ideas

Canonical Nix release, CI expansion, profile/resource additions, Social Home, local files, Blossom, signing, wallets, and later-platform capabilities remain outside this phase.

</deferred>

---

*Phase: 01-poc-replay-and-accepted-napp-seam*
*Context gathered: 2026-08-13 via PRD Express Path*
