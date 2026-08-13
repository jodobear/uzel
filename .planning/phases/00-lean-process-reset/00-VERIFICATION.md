# Lean process reset verification

**Result:** candidate verified for review; external exact-head merge gates pending.

| Acceptance check | Evidence | Result |
|---|---|---|
| One canonical workflow | `WORKFLOW.md`; root `AGENTS.md` is a short pointer; abandoned v4 authority and old phase loop absent | PASS |
| One locked command entrypoint | `README.md` and `WORKFLOW.md` use locked flake plus `pnpm`; all existing domain lockfiles unchanged | PASS |
| Minimal execution pointer | `.planning/STATE.md` contains phase, issue, branch/worktree, PR, blocker, and next action only | PASS |
| GSD machine routing | `init progress` and `roadmap.analyze` detect five phases and select Phase 1 next; `state-snapshot` detects reset phase `0` with status `review` | PASS |
| Pre-deletion inventory | Draft PR #38 contained exact-path `KEEP`, `DELETE`, and `PRESERVE-TEMPORARILY` classes before cleanup | PASS |
| Every DELETE removed | Git tree/path assertions cover all six classified paths/subtrees; Git history remains archive | PASS |
| Preservation boundaries hold | Product/evidence/lock diffs are empty; temporary evidence paths and protected forensic material exist | PASS |
| Locked guidance consistent | Executable commands in active codebase guidance use the canonical Nix-plus-pnpm entrypoint | PASS |
| Canonical graph current | Locked `pnpm graphify:refresh` ran the supported update; 1,462 nodes, 2,450 edges, and 129 fresh labels contain no cache, stale v4, or absolute checkout paths; all seven codebase-document labels map uniquely to their hubs | PASS |
| Graphify defect traceable | Issue [#39](https://github.com/jodobear/uzel/issues/39) records reviewer URL, SHA, P1 severity, impact, ownership, revisit trigger, and acceptance | PASS |
| Graphify cache portable | Machine-local cache is ignored/purged; issue [#40](https://github.com/jodobear/uzel/issues/40) records the upstream absolute-key/pruning defect and full finding contract | PASS |
| GSD agent bake current | Official GSD 1.10.0 global Codex reinstall completed; `init progress` emits no stale-bake warning and model policy was not edited | PASS |
| Review circuit breaker | Candidate count is diagnostic; only repeated material root cause, conflicting requirements, or no safe disposition stops autonomous review work | PASS |
| Fixture generation locked | `fixtures/README.md` uses locked `pnpm fixtures:build`; signed fixture payload bytes were not regenerated | PASS |
| Documentation validation read-only | Locked `pnpm docs:check` emits repository-relative `root: "."`; relocated clean-repository test leaves tracked state unchanged | PASS |

## Local gate

`pnpm docs:check` under the locked Nix environment passed after the review fixes with zero
errors and warnings.
Locked `pnpm test:maintenance` passed both relocation and repeat-refresh cases.
`git diff --check` passed. The change is maintenance/process/Graphify-only; product source,
fixture payloads, dependencies, pins, compatibility lock, and upstream ledger are unchanged.

Focused machine-routing commands:

```sh
node /home/at/.codex/gsd-core/bin/gsd-tools.cjs init progress
node /home/at/.codex/gsd-core/bin/gsd-tools.cjs query roadmap.analyze
node /home/at/.codex/gsd-core/bin/gsd-tools.cjs state-snapshot
```

Required observations: both roadmap projections report `phase_count: 5`; analysis reports
`missing_phase_details: null` and `next_phase: "1"`; state reports `current_phase: "0"`,
`current_phase_name: "Lean process reset"`, and `status: "review"`.

## External merge gate

Merge requires required CI, fresh GitHub Codex, and a substantive CodeRabbit review green
on one exact SHA, followed by one final affected validation on that SHA. CodeRabbit's
auto-review status is skipped and is not approval. Any semantic change invalidates prior
review.
