# Lean process reset summary

## Outcome

Uzel now has one active delivery authority in `WORKFLOW.md`, one minimal execution pointer,
one contextual issue/branch/worktree/PR, and five concise machine-routable roadmap outcomes.

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
- Restored canonical GSD phase headings and minimal STATE frontmatter: GSD detects five
  phases, the active reset state, and Phase 1 as the next routable phase.
- Refreshed GSD 1.10.0 through its supported global Codex install path so baked agent
  models match the unchanged project policy; the three detected hooks were backed up and
  verified byte-identical after reinstall.
- Filed Graphify numeric-label reclustering instability as backlog issue
  [#39](https://github.com/jodobear/uzel/issues/39).
- Corrected the review circuit breaker: candidate count signals possible non-convergence
  but only a repeated material root cause, conflicting reviewer requirements, or no safe
  disposition requires human judgment.
- Added locked package entrypoints for signed-fixture generation, explicit documentation
  evidence regeneration, and canonical Graphify refresh; fixture payload bytes were not
  regenerated.
- Made `docs:check` read-only and relocation-independent. Its focused test runs the package
  command from a relocated clean repository and proves both a clean tracked tree and
  checkout-independent output.
- Removed Graphify's machine-local cache from canonical output. The refresh wrapper purges
  it around the supported update, and focused tests prove two unchanged refreshes do not
  retain checkout paths or accumulate repositories. Root defect tracking is issue
  [#40](https://github.com/jodobear/uzel/issues/40).

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
- `c5a5d78` — finalize the second-candidate review record.
- `ca08e18` — refresh the graph for the third coherent review candidate.
- `705897b` — restore concise GSD machine routing.
- The trailing graph-only commit rebuilds the community map wholesale after this final
  semantic artifact stabilizes; it is the PR head requested for final reviews.
- Candidate 5 advances the minimal state pointer to the remaining review, validation,
  merge/close, and Phase 1 work; its trailing graph-only commit refreshes canonical output.
- `d98d340` — make fixture, documentation, and Graphify maintenance workflows portable.
- `3aad066` — replace machine-local cache and stale labels with fresh canonical graph output.
- The final closeout commit updates this summary, verification, and resumed worktree pointer
  after those commits stabilized; no product behavior or dependency changed.
- `260817d` — normalize Graphify manifest metadata and generated refresh guidance, with
  repeat-refresh clean-tree coverage.
- The next semantic commit records these review fixes in the preserved POC status and reset
  closeout; its trailing graph-only commit refreshes every final source location.

## Validation

- `nix --extra-experimental-features 'nix-command flakes' develop --command pnpm docs:check`
  — 47 Markdown documents, 78 links, 9 Mermaid blocks, zero errors/warnings.
- Locked `pnpm test:maintenance` — two focused tests pass: relocated read-only docs checking
  and repeat Graphify refresh cache cleanup.
- Canonical graph — 1,462 nodes, 2,450 edges, 129 freshly labeled communities; all seven
  codebase-document labels map to their own hubs, with no stale v4 or absolute checkout paths.
- Determinism regression — two unchanged refreshes produce no tracked diff after stripping
  checkout mtimes; the generated report points only to locked `pnpm graphify:refresh`.
- Exact deletion, preservation, source/lock non-change, state-budget, plan-budget, and
  Graphify stale-node assertions are recorded in `00-VERIFICATION.md`.

Fresh same-head GitHub Codex, substantive CodeRabbit, required CI, and final exact-head
validation remain PR merge gates, not duplicated repository receipts.
