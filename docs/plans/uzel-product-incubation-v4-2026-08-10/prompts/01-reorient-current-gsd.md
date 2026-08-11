# Prompt — reorient the existing Uzel GSD project

Work in the clean manual Phase 1 worktree. This is a **planning/state reconciliation
only** task. Do not modify product source, dependency locks, generated build output or
the blocked replay worktree.

Read:

- `docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST.md`
- `docs/plans/uzel-product-incubation-v4-2026-08-10/01-BASELINE-REPLAY.md`
- `docs/plans/uzel-product-incubation-v4-2026-08-10/03-ROADMAP.md`
- `docs/plans/uzel-product-incubation-v4-2026-08-10/04-DELIVERY-QUALITY.md`
- current `.planning/PROJECT.md`, `REQUIREMENTS.md`, `ROADMAP.md`, `STATE.md`, config,
  Phase 1 context/plans, pause report and codebase map;
- Git graph/worktree/evidence state around `b185ad1` and its parent.

Goals:

1. Preserve the brownfield project, accepted history, mapping and blocked-incident
   evidence.
2. Ensure runtime is Codex and `workflow.use_worktrees=false`; do not add speculative
   model overrides.
3. Record that Uzel is one product-shaped repository through M5; no package-first
   dependency or automatic post-M5 programme exists.
4. Replace the impossible no-provisioning replay rule with exact-source/exact-lock
   hermetic replay plus a separate current Nix/native baseline; record
   `not_yet_packaged` and assign complete package acceptance to Phase 2 when outputs do
   not yet exist.
5. Preserve `b185ad1`, its worktree, safety ref and portable checksummed archive. Planning
   may assign only a provisional disposition.
6. Reconcile the complete future roadmap **now**, before Phase 1 execution. Preserve
   integer phases 2–7 as first increments; insert decimals without deleting or
   renumbering integers. Required sequence:

   ```text
   2, 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7,
   3, 3.1, 3.2, 3.3,
   4, 4.1, 4.2, 4.3,
   5, 5.1, 5.2, 5.3,
   6, 6.1, 6.2,
   7, 7.1, 7.2, 7.3, 7.4, 7.5, 7.6, 7.7, 7.8, 7.9
   ```

   Use current supported GSD phase-management commands and keep roadmap/state/phase
   directories coherent. Each listed phase is one issue/worktree/primary PR.
7. Make 7.9 the exact production-candidate freeze and A5 the mandatory twelve-lane
   non-implementation stop.
8. Remove stale package-first dependencies, broad filesystem-first work, stale or unverified GSD
   command assumptions, automatic Codex worktree assumptions and automatic post-M5 advancement.
9. Preserve the architecture invariants: local profile/actor/subject separation,
   first-party guest parity, singular Nostr owner, daemon product-service ownership,
   mediated destination policy, canonical event-template/final-event binding with final
   signer output validated before any relay write, Blossom authorization validated before
   any upload body, narrow future-action grants, a separate low-authority media worker
   and no shell database.
10. Add the M0 ecosystem/profile/maturity/knowledge baseline: canonical machine-readable
    upstream registry; immutable-source map; canonical machine RCP with generated human
    rendering and package hash binding; canonical terminology registry with stable term
    IDs/supersession; global/per-principal admission, fairness and anti-starvation baseline;
    required/optional fail-before-guest negotiation
    and transcript vectors; manifest/exact-build SIR; capability ledgers; independent
    interop/version-skew plan; upstream/local-patch lifecycle; visibility/embargo;
    phase-closeout and milestone-learning process.
11. Ensure no Phase 2 execution can begin before the manifest/build identity and launch-
    negotiation profile receives a human go/no-go decision.
12. Record installed Codex/GSD versions and command help; require supported `--ingest`,
    `--ingest-format`, `--reviews`, `--coderabbit`, `--insert` and `--edit` forms plus default plan verification and post-execution `verify-work` rather than
    guessing from stale documentation.

Output:

- exact planning/config/roadmap files changed;
- before/after phase map;
- unresolved contradictions/evidence gaps;
- confirmation that no product/dependency source changed;
- exact next command from the runbook for Phase 1 planning.

Do not execute Phase 1 or run `$gsd-resume-work` in this session.
