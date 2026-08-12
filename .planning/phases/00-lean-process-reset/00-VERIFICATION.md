# Lean process reset verification

**Result:** candidate verified for review; external exact-head merge gates pending.

| Acceptance check | Evidence | Result |
|---|---|---|
| One canonical workflow | `WORKFLOW.md`; root `AGENTS.md` is a short pointer; abandoned v4 authority and old phase loop absent | PASS |
| One locked command entrypoint | `README.md` and `WORKFLOW.md` use locked flake plus `pnpm`; all existing domain lockfiles unchanged | PASS |
| Minimal execution pointer | `.planning/STATE.md` contains phase, issue, branch/worktree, PR, blocker, and next action only | PASS |
| Pre-deletion inventory | Draft PR #38 contained exact-path `KEEP`, `DELETE`, and `PRESERVE-TEMPORARILY` classes before cleanup | PASS |
| Every DELETE removed | Git tree/path assertions cover all six classified paths/subtrees; Git history remains archive | PASS |
| Preservation boundaries hold | Product/evidence/lock diffs are empty; temporary evidence paths and protected forensic material exist | PASS |
| Locked guidance consistent | Executable commands in active codebase guidance use the canonical Nix-plus-pnpm entrypoint | PASS |
| Canonical graph current | Persisted labels from deleted v4 sources removed; final `graphify update .` follows stabilized closeout docs | PASS |

## Local gate

`pnpm docs:check` under the locked Nix environment passed after the review fixes with zero
errors and warnings.
`git diff --check` passed. The change is documentation/process/Graphify-only; product source,
tests, fixtures, dependencies, pins, compatibility lock, and upstream ledger are unchanged.

## External merge gate

Merge requires required CI, fresh GitHub Codex, and a substantive CodeRabbit review green
on one exact SHA, followed by one final affected validation on that SHA. CodeRabbit's
auto-review status is skipped and is not approval. Any semantic change invalidates prior
review.
