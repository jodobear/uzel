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
  patterns: [exact-head evidence, fail-closed committed-object qualification, unavailable-not-inferred runtime reporting]
key-files:
  created:
    - evidence/phase-01/poc-replay.md
    - evidence/phase-01/ownership-map.md
    - evidence/phase-01/napp-prerequisite.md
  modified: []
key-decisions:
  - "Record exact locked candidate results: test passes, while clean-worktree JS dependency materialization and Linux smoke readiness fail."
  - "Keep Napp candidate 0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e at result stop until its owner publishes committed qualifying evidence."
requirements-addressed: [REF-01, REF-02, REF-03, REF-04, REF-05, REF-06, REF-07]
status: complete
---

# Phase 01 Plan 01: POC replay and accepted Napp seam Summary

**Exact-head POC evidence and ownership map plus a reproducible Napp committed-object STOP, with REF-07 and Phase 1 deliberately left open.**

## Performance

- **Duration:** 6 min
- **Started:** 2026-08-13T16:57:52Z
- **Completed:** 2026-08-13T17:03:40Z
- **Tasks:** 2/2
- **Files modified:** 3 evidence documents

## Accomplishments

- Proved the accepted-commit replay path differs only by the locked-entrypoint `fixtures/README.md` documentation change, with fixture/pin parity intact.
- Re-ran each required baseline command once on exact head `b307a297` through the authorized host path: Nix and pnpm entered, `pnpm test` passed, and the other five lanes produced current actionable failures instead of sandbox preflight errors.
- Validated `jodobear/napp@0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e` with the fixed checker at `result: stop`, preserving sibling checkout exclusion and all nine missing admission categories.
- Validated the canonical handoff against its recorded contract commit and approved archived plan object; `handoff: pass` does not change the candidate's qualification `stop`.

## Task Commits

1. **Task 1: Replay and disposition one accepted POC path at exact authority** — `c4fabec` (docs)
2. **Task 2: Validate committed STOP and publish exact external prerequisite** — `b4294c2` (docs)

## Files Created

- `evidence/phase-01/poc-replay.md` — exact preflight, pin bindings, one baseline disposition, and REF-06 unavailable measurements.
- `evidence/phase-01/ownership-map.md` — authority boundaries plus protected ref/worktree disposition.
- `evidence/phase-01/napp-prerequisite.md` — candidate STOP, missing categories, absent-handoff disposition, and external next action.

## Decisions Made

- Treat the exact-head locked result as partial: test passes; clean-worktree JS dependency materialization and Linux smoke readiness remain failed candidate gates.
- Keep NMP as sole Nostr owner; keep Napp/nampplets runtime ownership and Uzel product-policy ownership separate.
- Preserve REF-07 as blocked: issue #42 has no authority to accept or mutate the dirty Napp sibling checkout.

## Deviations from Plan

None during execution — the accepted plan required honest failed/unavailable baseline dispositions and a terminal candidate STOP when their required conditions occur. After execution, Codex review collapsed the plan's command bodies into the lean `WORKFLOW.md` shape; the exact commands and results remain in the three evidence outputs.

## Issues Encountered

- The authorized exact-head retry entered pnpm. Build/check/conformance/UI failed because workspace JS dependencies were absent in the clean worktree; Linux smoke exited before readiness; `pnpm test` passed. The evidence report records exact outcomes, owners, and revisit triggers. No product command was rerun while editing.
- Final locked `pnpm docs:check` initially met the same sandbox boundary, then passed with authorized Nix database access: 47 Markdown documents, 78 links, 9 Mermaid blocks, zero errors, zero warnings.

## Known Stubs

None.

## Next Phase Readiness

- REF-01 through REF-06 evidence obligations are recorded with current results; locked JS dependency materialization and Linux runtime readiness still need correction before candidate acceptance.
- REF-01 through REF-04, REF-06, and REF-07 remain pending. Issue #42 and PR #43 stay the single blocked Phase 1 delivery unit; Napp owner/source-authority evidence remains an independent external prerequisite.

## Self-Check: PASSED

Verified all three evidence documents, this summary, and the single STATE update exist. Verified task commits `c4fabec` and `b4294c2` are reachable.

*Phase: 01-poc-replay-and-accepted-napp-seam*  
*Completed: 2026-08-13*
