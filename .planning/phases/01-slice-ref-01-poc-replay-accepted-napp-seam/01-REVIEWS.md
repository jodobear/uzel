# Phase 1 plan-review record

**Status:** Corrected convergence candidate; exact-head final verdict remains external on the PR and controls the gate.
**PR:** `jodobear/uzel#36`
**Review base:** `21d3a0cbe5306cf6bf1943aba18cef77ed34ba37`
**First reviewed head:** `1cba508ec90f598ff15b37bb68974e3cb0319ce4`
**Prompt:** `docs/plans/uzel-product-incubation-v4-2026-08-10/prompts/02-review-phase-1.md`

## Approved review lanes

Only local CodeRabbit followed by GitHub Codex on an exact pushed PR SHA may satisfy
the plan-review gate. A local CodeRabbit `rate_limit` recorded before findings permits
green GitHub Codex to satisfy the fallback. Claude, OpenCode, remote CodeRabbit and local
Codex self-review are prohibited. Any finding-driven commit requires a new exact-head
attempt and GitHub Codex review.

## Cycle 1

- Local CodeRabbit CLI `0.7.2` reviewed the immutable diff ending at
  `1cba508ec90f598ff15b37bb68974e3cb0319ce4` and returned `rate_limit` before emitting
  findings. No other failure was treated as fallback authorization.
- GitHub Codex review `4912138554`, submitted `2026-08-12T01:13:24Z`, reviewed exact
  commit `1cba508ec90f598ff15b37bb68974e3cb0319ce4` and reported eleven P1 findings.
- All eleven findings were accepted for correction; none was waived.

| Finding | Corrective disposition |
|---|---|
| Restore preserved incident objects | Revalidated commit, parent, safety ref, clean registered worktree and checksummed bundle; plans now use portable identities plus a fail-closed restored-archive input. |
| Bind packages to the RCP hash | Plan 05 now owns explicit package/profile binding and diagnostics evidence, with fail-closed package mismatch vectors. |
| Reject downgrade and mid-session replacement | Plan 05 now requires required/optional capability semantics plus downgrade, stale-session and mid-session replacement vectors before guest execution. |
| Complete upstream lifecycle/accountability | Plan 05 now records distinct patch/PR/merge/release/adoption/removal states, named human authority, AI/authorship/signoff/DCO/CLA/public-text policy and independent-peer acquisition status. |
| Require global admission and anti-starvation | Plan 05 now requires global and per-principal bounds, named anti-starvation policy/owner and noisy/quiet, many-principal, adversarial, cancellation and overload vectors. |
| Remove superseded Napp blocker | Handoff/reorientation/current plans now follow v4 Uzel shell/product-service/runtime/engine ownership; no external Napp candidate gates Phase 1. |
| Point authority tracer at real owners | Plan 03 now uses Graphify-located `apps/uzel*`, `crates/napd*`, `contracts`, `napplets` and current checker paths. |
| Use one CodeRabbit rate-limit enum | Review contracts and verifier use `rate_limit` consistently. |
| Execute plans sequentially | Plans now form seven ordered waves: `01-01` through `01-07`; Plans 06-07 isolate executor-real E then P/review/decision closeout from Plan 05 compatibility closure. |
| Reject Phase 2 while blockers remain | Plan 07 generates complete fail-closed packet; approve is unavailable with missing, stale or unresolved blocker/replacement evidence, while hold is immediate. |
| Review final execution evidence | Plan 06 ends with normal Task-2 commit E; Plan 07 ends Task 1 with normal commit P, then separates human decision and later decision/SUMMARY exact-PR-SHA review without self-reference. |

During convergence, an additional internal audit found and corrected three review-path
defects: GitHub Codex plan review now precedes execution; all review stage SHAs must equal
the current pushed candidate; and prohibited reviewer semantics are positively asserted
rather than rejected by a contradictory text search. A final independent plan-checker
pass then found and corrected three execution blockers: Task 3's evidence work was split
from the blocking human checkpoint and decision writer; Plan 04's measurement record was
made capability-only to remove commit-SHA self-reference; and live `STATE.md` was aligned
with five sequential waves and v4 Uzel ownership. The stale historical
`01-04-SUMMARY.md` was archived so GSD cannot mistake revised Plan 04 for completed work.
A follow-up checker pass caught three more execution defects: runbook review timing was
aligned to the complete E/P candidate; oversized five-task Plan 05 was split into bounded
Plan 05 compatibility closure, Plan 06 evidence freeze, and Plan 07 transition closeout; and stale roadmap progress
text plus the historical summary path were corrected.

## Cycle 2

- Local CodeRabbit CLI `0.7.2` reviewed the immutable committed diff from
  `21d3a0cbe5306cf6bf1943aba18cef77ed34ba37` through
  `6333179c436a69e5b2397489afab04a0bce4c0b7` and completed with twenty findings:
  nineteen major and one minor. All were accepted; none was waived.
- The batch correction makes every evidence validator non-vacuous and schema-bound:
  preserved incident paths/blob/content/diff identities and replay-attempt count; package
  verdict-specific outputs; command/measurement/workflow bindings; source citation and
  architecture/threat/storage cross-links; tool/command/reviewer evidence; per-upstream
  lifecycle and per-queue pressure controls; distinct source versus packaged RCP hashes;
  public redaction schema; exact transition-manifest coverage; and separate packet-candidate
  versus final-PR SHAs.
- Resume/history text now names seven plans and seven waves, marks the earlier summary and Napp
  research model non-authoritative, and states the current Uzel/provider ownership rule.
  Pack review language now names only local CodeRabbit plus GitHub Codex and commits the
  complete closeout candidate before either exact-SHA review.
- Pack manifest, checksums and canonical audit report were regenerated. The original
  `00-GSD-INGEST.md` remains byte-identical at its recorded SHA-256.

## Current gate

Run independent plan checking, commit and push the complete corrected plan set, retry
local CodeRabbit on that exact head, then request GitHub Codex review of the same SHA.
The final review result remains
on GitHub so recording it cannot invalidate the reviewed commit. Phase 1 remains
unapproved until that review is green and the human execution gate is explicit.
