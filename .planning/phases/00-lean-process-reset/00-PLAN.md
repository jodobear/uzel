# Lean process reset

## Outcome

Uzel has one lean GSD delivery authority and no active abandoned-planning machinery before
product work resumes.

## Appetite

One documentation/process-only PR from current `origin/master`.

## Boundaries

- Preserve product source, tests, fixtures, dependency locks, POC evidence, upstream
  contribution records, canonical Graphify output, and protected incident material.
- Change no product behavior or dependency pin.

## No-gos

- No old Phase 1 resume, replay execution, new validators, receipts, custom review refs,
  or speculative future plans.
- No merge or mutation of PR #36 or deletion of protected incident branches/worktrees.

## Risks

- Removing evidence needed for later incident disposition.
- Leaving a second normative workflow active.
- Publishing private incident metadata in GitHub artifacts.

## Dependencies

- GitHub issue #37.
- Current `origin/master` and live PR #36 state.

## Acceptance checks

- One canonical workflow covers phase ownership, concise planning, validation cadence,
  same-head review gates, finding disposition, and autonomous continuation.
- Locked flake plus `pnpm` is the sole normal command entrypoint; existing domain locks stay
  authoritative.
- `.planning/STATE.md` contains only the current pointer fields.
- Draft PR description contains exact-path `KEEP`, `DELETE`, and
  `PRESERVE-TEMPORARILY` inventory before deletions.
- Every `DELETE` path is absent while protected/product/evidence paths remain.
- Canonical Graphify output is refreshed once after stable edits and committed separately.
- Final exact head passes affected checks and required reviews before merge; PR #36 is
  closed as superseded only after replacement merge.
