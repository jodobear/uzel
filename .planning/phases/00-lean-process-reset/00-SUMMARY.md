# Lean process reset summary

## Outcome

Uzel now has one active delivery authority in `WORKFLOW.md`, one minimal execution pointer,
one contextual issue/branch/worktree/PR, and concise unplanned roadmap outcomes.

## Changes

- Established locked `nix develop --command pnpm <script>` entrypoint without adding a
  competing lock.
- Replaced root agent workflow prose with a short authority pointer.
- Removed every path classified `DELETE` in PR #38's pre-deletion inventory: abandoned v4
  process pack, superseded Phase 1 planning loop, duplicate handoff/session/research files,
  and duplicate POC ZIP.
- Preserved product source, tests, fixtures, locks, extracted POC evidence, contribution
  ledger, three Phase 1 temporary evidence paths, protected forensic ref/worktree, and
  portable incident evidence.
- Reconciled active codebase guidance with the locked Nix-plus-pnpm entrypoint.
- Removed persisted Graphify labels inherited from deleted v4 sources, then refreshed the
  canonical graph separately after the documentation stabilized.

## Commits

- `28479e9` — establish lean delivery authority.
- `47dc4cf` — remove obsolete planning machinery.
- `b1ca944` — initial post-cleanup Graphify refresh; superseded by the final refresh after
  these closeout artifacts were added.
- `d9570df` — record the lean reset summary and verification.
- `3a7e9db` — refresh the graph after adding closeout artifacts.
- `b230b36` — fix the first exact-head Codex documentation findings.
- `2157719` — refresh graph labels after the first review.
- `c795c86` — fix the remaining exact-head graph-label and locked-entrypoint findings.

## Validation

- `nix --extra-experimental-features 'nix-command flakes' develop --command pnpm docs:check`
  — 47 Markdown documents, 78 links, 9 Mermaid blocks, zero errors/warnings.
- Exact deletion, preservation, source/lock non-change, state-budget, plan-budget, and
  Graphify stale-node assertions are recorded in `00-VERIFICATION.md`.

The initial and second-candidate GitHub Codex findings were fixed in the branch. Fresh
same-head GitHub Codex, substantive CodeRabbit, required CI, and final exact-head
validation remain PR merge gates, not duplicated repository receipts.
