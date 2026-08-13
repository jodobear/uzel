---
phase: 01-poc-replay-and-accepted-napp-seam
plan: 01
subsystem: evidence
tags: [poc-replay, nix, napp, source-authority, ownership]
requires: []
provides:
  - Exact-head POC replay disposition with source/pin preflight and unavailable-runtime evidence
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
  - "Record locked Nix materialization failure as unavailable evidence; never infer current runtime success from historical POC reports."
  - "Keep Napp candidate 0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e at result stop until its owner publishes committed qualifying evidence."
requirements-completed: [REF-01, REF-02, REF-03, REF-04, REF-05, REF-06]
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
- Ran each required baseline command once; recorded the Nix database-lock failure as current unavailable evidence instead of copying historic pass claims.
- Validated `jodobear/napp@0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e` with the fixed checker at `result: stop`, preserving sibling checkout exclusion and all nine missing admission categories.

## Task Commits

1. **Task 1: Replay and disposition one accepted POC path at exact authority** — `c4fabec` (docs)
2. **Task 2: Validate committed STOP and publish exact external prerequisite** — `b4294c2` (docs)

## Files Created

- `evidence/phase-01/poc-replay.md` — exact preflight, pin bindings, one baseline disposition, and REF-06 unavailable measurements.
- `evidence/phase-01/ownership-map.md` — authority boundaries plus protected ref/worktree disposition.
- `evidence/phase-01/napp-prerequisite.md` — candidate STOP, missing categories, absent-handoff disposition, and external next action.

## Decisions Made

- Treat `/nix/var/nix/db/big-lock` read-only failure as an unavailable runtime baseline, not a pass or a reason to repeat the six locked baseline commands.
- Keep NMP as sole Nostr owner; keep Napp/nampplets runtime ownership and Uzel product-policy ownership separate.
- Preserve REF-07 as blocked: issue #42 has no authority to accept or mutate the dirty Napp sibling checkout.

## Deviations from Plan

None — the plan explicitly requires honest failed/unavailable baseline dispositions and a terminal candidate STOP when their required conditions occur.

## Issues Encountered

- The one required baseline set could not acquire `/nix/var/nix/db/big-lock` in the sandbox; all six commands failed before `pnpm`. The evidence report records exact commands, cause, owner, and revisit trigger. They were not rerun.
- Final locked `pnpm docs:check` initially met the same sandbox boundary, then passed with authorized Nix database access: 47 Markdown documents, 78 links, 9 Mermaid blocks, zero errors, zero warnings.

## Known Stubs

None.

## Next Phase Readiness

- REF-01 through REF-06 evidence obligations are recorded; the runtime replay needs a future writable-Nix execution environment for current run results.
- REF-07 remains blocked. `jodobear/napp` must publish committed qualifying evidence before a later Phase 1 issue and plan may be created. No second plan exists.

## Self-Check: PASSED

Verified all three evidence documents, this summary, and the single STATE update exist. Verified task commits `c4fabec` and `b4294c2` are reachable.

*Phase: 01-poc-replay-and-accepted-napp-seam*  
*Completed: 2026-08-13*
