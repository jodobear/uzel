---
phase: 01-slice-ref-01-poc-replay-accepted-napp-seam
plan: 04
subsystem: dependency-qualification
tags: [napp, git, provenance, evidence, graphify]
requires: []
provides:
  - Fail-closed qualification for one exact committed Napp candidate
  - Repository-qualified Napp dependency handoff with adapter preconditions
affects: [REF-01D, adapter-research, napp]
tech-stack:
  added: []
  patterns: [canonical JSON evidence, literal Git argv admission, immutable parity binding]
key-files:
  created: [scripts/ref-candidate-check.py, evidence/phase-01/candidate-qualification.md, evidence/phase-01/napp-dependency.md]
  modified: [graphify-out/graph.json, graphify-out/GRAPH_REPORT.md]
key-decisions:
  - "Current Napp candidate is a repository-qualified stop, not an adapter seam."
  - "Plan-01 parity binds to its recorded immutable Uzel commit, not later HEAD changes."
requirements-completed: []
coverage:
  - id: D1
    description: Fail-closed candidate qualification and safe-probe grammar
    requirement: REF-07
    verification:
      - kind: unit
        ref: python3 scripts/ref-candidate-check.py self-test
        status: pass
      - kind: integration
        ref: python3 scripts/ref-candidate-check.py qualification --repo /workspace/projects/napplets/napp-uzel/napp --expected-repository jodobear/napp --expected-commit 0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e --record evidence/phase-01/candidate-qualification.md --expected-result stop
        status: pass
    human_judgment: false
  - id: D2
    description: Structurally equal dependency handoff and blocked adapter resume contract
    requirement: REF-07
    verification:
      - kind: integration
        ref: python3 scripts/ref-candidate-check.py handoff --repo . --napp-repo /workspace/projects/napplets/napp-uzel/napp --qualification evidence/phase-01/candidate-qualification.md --handoff evidence/phase-01/napp-dependency.md --plan .planning/phases/01-slice-ref-01-poc-replay-accepted-napp-seam/01-01-PLAN.superseded.md
        status: pass
    human_judgment: false
duration: 7m 28s
completed: 2026-08-09
status: complete
---

# Historical superseded Plan 04 record — non-authoritative

This archived record describes an earlier Napp-qualification experiment only. It is not an active Phase-1 plan, does not satisfy any current Phase-1 gate, does not authorize REF-01D, and must not be used as current Napp ownership, candidate, review, or transition evidence. The Napp handoff below is historical context only; current v4 authority is the active seven-plan chain and its explicit Plan-07 transition closeout gate.

# Phase 01 Plan 04: Napp Qualification and Handoff Summary

**One exact reachable Napp commit is recorded as an evidence-backed stop, with a machine-checkable dependency handoff instead of an invented adapter.**

## Performance

- **Duration:** 7m 28s
- **Started:** 2026-08-09T23:09:20Z
- **Completed:** 2026-08-09T23:16:48Z
- **Tasks:** 2/2
- **Files modified:** 9

## Accomplishments

- Qualified committed `0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e` from `jodobear/napp`; its tree is `b12b0c13b11ce5f64e4fd91025789ae692438f38` and inventory digest is `9d6f11acb50a13c68f77f5e4945598ba9c2a1b301c70017568fe360b16abd41b`.
- Added a standard-library validator with fixed Git read grammar, byte-preserving `tree.bin` inventory, pinned committed blobs, and mutation snapshots.
- Published a structurally equal Napp handoff: adapter work remains blocked on committed authority, a qualified candidate, and admitted Plan-01 replay evidence with an exact manifest digest.
- Refreshed tracked Graphify artifacts after the checker change; no production, dependency, fixture, lock, remote, PR, or sibling-repository state was changed by this plan.

## Task Commits

1. **Task 1: Qualify the exact committed Napp candidate or stop** — `fca2280` (feat)
2. **Task 2: Publish repository-qualified dependency and adapter resume contract** — `1239809` (docs)
3. **Rule 1 validator correction** — `eb3713b` (fix)
4. **Required Graphify refresh** — `ec6da8b` (chore)

## Files Created/Modified

- `scripts/ref-candidate-check.py` — literal-argv checker, snapshots, and equality validator.
- `evidence/phase-01/candidate-qualification.md` — canonical committed-candidate stop record.
- `evidence/phase-01/napp-dependency.md` — canonical repository-qualified handoff and resume contract.
- `graphify-out/graph.json` — refreshed code graph, with companion report, labels, manifest, HTML, and stat index.

## Decisions Made

- Candidate absence is valid evidence: it blocks only REF-01D adaptation.
- The Plan-01 parity contract is immutable at its recorded handoff commit; subsequent documentation or graph commits cannot invalidate it.
- D-18 remains candidate/pending only. Publication, PR/issue creation, merge, and external review need separate authorization.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Re-baselined concurrent sibling Napp state drift**
- **Found during:** Task 2
- **Issue:** Another process changed Napp's index/object snapshot while this plan held its first evidence record.
- **Fix:** Re-ran the same committed-object-only qualification and regenerated the handoff; no sibling working-tree material was read as candidate evidence or modified.
- **Files modified:** `evidence/phase-01/candidate-qualification.md`, `evidence/phase-01/napp-dependency.md`
- **Verification:** Current Napp snapshot matched the regenerated post-snapshot before commit.
- **Committed in:** `1239809`

**2. [Rule 1 - Bug] Bound parity validation to the recorded Uzel commit**
- **Found during:** Post-Task-2 verification
- **Issue:** Validator compared the handoff contract to mutable `HEAD`, so its own later commit invalidated a valid handoff.
- **Fix:** Resolve the committed Plan-01 blob from the recorded handoff SHA and require that SHA to equal the recorded Uzel head.
- **Files modified:** `scripts/ref-candidate-check.py`
- **Verification:** Full qualification and handoff commands pass after the fix.
- **Committed in:** `eb3713b`

**Total deviations:** 2 auto-fixed (Rule 1: 1; Rule 3: 1). No scope expansion.

## Issues Encountered

- Sandbox Git metadata was read-only; exact-path commits used approved elevated Git access.
- Graphify created 99 untracked AST cache entries. They were generated by this run, removed before handoff, and not committed.

## Known Stubs

None.

## Threat Flags

None. The plan adds local evidence validation only; it exposes no endpoint, auth path, file-serving path, or schema trust boundary.

## Next Phase Readiness

REF-01D remains blocked. Napp owners must supply committed behavior-backed client/events/testkit/version/lifecycle/pin evidence and an approved sandbox contract. Then run `$gsd-plan-phase 1 --research` before any narrow Rust/Tauri adapter plan.

## D-18 Review Handoff

```json
{"affected_local_gates":[{"command":"python3 -c 'from pathlib import Path; compile(Path(\"scripts/ref-candidate-check.py\").read_text(encoding=\"utf-8\"), \"scripts/ref-candidate-check.py\", \"exec\") && python3 scripts/ref-candidate-check.py self-test && python3 scripts/ref-candidate-check.py qualification --repo /workspace/projects/napplets/napp-uzel/napp --expected-repository jodobear/napp --expected-commit 0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e --record evidence/phase-01/candidate-qualification.md --expected-result stop","status":"pass"},{"command":"python3 scripts/ref-candidate-check.py handoff --repo . --napp-repo /workspace/projects/napplets/napp-uzel/napp --qualification evidence/phase-01/candidate-qualification.md --handoff evidence/phase-01/napp-dependency.md --plan .planning/phases/01-slice-ref-01-poc-replay-accepted-napp-seam/01-01-PLAN.superseded.md && git diff --exit-code -- Cargo.toml Cargo.lock flake.nix flake.lock apps crates contracts fixtures napplets","status":"pass"}],"candidate_base":"42997daaec3e88ae5faf8ff3a0a9dfa024712f7f","candidate_head":"eb3713b46e5dc3061ae4fb31ecaa55560714538f","external_review_status":"pending-separate-authorization","publication_status":"not-authorized","review_evidence_path":"evidence/phase-01/reviews/napp-qualification-eb3713b46e5dc3061ae4fb31ecaa55560714538f.md","schema":"uzel.review-handoff/v1","slice":"napp-qualification"}
```

## Self-Check: PASSED

All declared artifacts exist and all four task/graph commits are present in Git history.
