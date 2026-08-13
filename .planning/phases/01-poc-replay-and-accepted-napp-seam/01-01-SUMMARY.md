---
phase: 01-poc-replay-and-accepted-napp-seam
plan: 01
subsystem: evidence
tags: [poc-replay, nix, napp, source-authority, ownership]
requires: []
provides:
  - Exact-head POC replay disposition with source/pin preflight and current locked candidate results
  - Ownership map preserving Uzel, NMP, Napp/nampplets, and protected-worktree boundaries
  - Reproducible committed Napp candidate STOP and external prerequisite packet
affects: [phase-01, napp-adapter, nix-packaging, ci, social-home]
actuals:
  tokens: 5029
  tasks: 2
  commits: 2
tech-stack:
  added: []
  patterns: [exact-head evidence, fail-closed committed-object qualification, locked prerequisite then candidate replay]
key-files:
  created:
    - evidence/phase-01/poc-replay.md
    - evidence/phase-01/ownership-map.md
    - evidence/phase-01/napp-prerequisite.md
  modified: []
key-decisions:
  - "All aggregate lanes pass after locked dependency setup, but REF-03 restart-state and REF-04 real-WebKit recovery coverage remain pending."
  - "Keep Napp candidate 0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e at result stop until its owner publishes committed qualifying evidence."
requirements-addressed: [REF-01, REF-02, REF-03, REF-04, REF-05, REF-06, REF-07]
status: complete
---

# Phase 01 Plan 01: POC replay and accepted Napp seam Summary

**Exact-head POC evidence and ownership map plus a reproducible Napp committed-object STOP, with REF-07 and Phase 1 deliberately left open.**

## Performance

- **Duration:** 1 h 11 min
- **Started:** 2026-08-13T16:57:52Z
- **Completed:** 2026-08-13T18:08:23Z
- **Tasks:** 2/2
- **Files modified:** 3 evidence documents

## Accomplishments

- Proved the accepted-commit replay path differs only by the locked-entrypoint `fixtures/README.md` documentation change, with fixture/pin parity intact.
- Ran the documented locked install, then each required baseline command once on exact head `d4c83a93` through the authorized host path; all six build/check/test/conformance/UI/Linux-smoke lanes passed.
- Validated `jodobear/napp@0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e` with the fixed checker at `result: stop`, preserving sibling checkout exclusion and all nine missing admission categories.
- Validated the canonical handoff against its recorded contract commit and approved archived plan object; `handoff: pass` does not change the candidate's qualification `stop`.

## Task Commits

1. **Task 1: Replay and disposition one accepted POC path at exact authority** — `c4fabec` (docs)
2. **Task 2: Validate committed STOP and publish exact external prerequisite** — `b4294c2` (docs)

## Files Created

- `evidence/phase-01/poc-replay.md` — exact preflight, pin bindings, passing baseline, and bounded REF-06 measurements.
- `evidence/phase-01/ownership-map.md` — authority boundaries plus protected ref/worktree disposition.
- `evidence/phase-01/napp-prerequisite.md` — candidate STOP, missing categories, validated handoff, and external next action.

## Decisions Made

- Treat the aggregate locked baseline as passing evidence for REF-01, REF-02, and REF-06; keep REF-03 pending for source-grounded restart-state replay and REF-04 pending for real WebKit recovery.
- Keep NMP as sole Nostr owner; keep Napp/nampplets runtime ownership and Uzel product-policy ownership separate.
- Preserve REF-07 as blocked: issue #42 has no authority to accept or mutate the dirty Napp sibling checkout.

## Deviations from Plan

None during execution — the accepted plan required honest failed/unavailable baseline dispositions and a terminal candidate STOP when their required conditions occur. After execution, Codex review collapsed the plan's command bodies into the lean `WORKFLOW.md` shape; the exact commands and results remain in the three evidence outputs.

## Issues Encountered

- Codex identified that the first authorized attempt omitted README's locked dependency prerequisite. After locked install passed without tracked changes, the one real post-prerequisite baseline passed all six lanes; the earlier precondition failure carries no product claim.
- Final affected-doc validation checked all seven changed Markdown files for H1 structure, relative links, and Mermaid starts (3 links, 0 errors), then passed `git diff --check`, GSD routing, qualification, and handoff checks. The brownfield pack's separate 47-document audit remains scoped only to that pack.

## Known Stubs

None.

## Next Phase Readiness

- REF-01, REF-02, REF-05, and REF-06 are complete with current source-bound results and explicit measurement limits.
- REF-03 and REF-04 remain pending on bounded runtime probes; REF-07 remains pending on Napp owner/source authority. Issue #42 and PR #43 stay the single blocked Phase 1 delivery unit.

## Self-Check: PASSED

Verified all three evidence documents, this summary, and the single STATE update exist. Verified task commits `c4fabec` and `b4294c2` are reachable.

*Phase: 01-poc-replay-and-accepted-napp-seam*  
*Completed: 2026-08-13*
