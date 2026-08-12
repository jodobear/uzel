# Phase 1 plan-review record

**Status:** Corrected convergence candidate; exact-head final verdict remains external on the PR and controls the gate.
**PR:** `jodobear/uzel#36`
**Review base:** `21d3a0cbe5306cf6bf1943aba18cef77ed34ba37`
**First reviewed head:** `1cba508ec90f598ff15b37bb68974e3cb0319ce4`
**Prompt:** `docs/plans/uzel-product-incubation-v4-2026-08-10/prompts/02-review-phase-1.md`

## Approved review lanes

Only local CodeRabbit and GitHub Codex on one exact pushed PR SHA may satisfy the
plan-review gate. They may run concurrently after the candidate is frozen and pushed;
acceptance waits for both dispositions. A `rate_limit` from local CodeRabbit's required
immutable-candidate review mode, recorded before findings, permits green GitHub Codex to
satisfy the fallback. Later or alternate-mode rate limits do not. Claude, OpenCode, remote
CodeRabbit and local Codex self-review are prohibited. Any finding-driven commit requires
a new exact-head attempt and GitHub Codex review.

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

## Cycle 3

- Local CodeRabbit CLI `0.7.2` reviewed the immutable committed diff from the same base
  through `79cfa9e8a0be8e4e53649ada24bd92f3d5c2b67a` and completed with twenty-one
  findings: one critical and twenty major.
- Twenty findings were accepted for manual correction. Validators now cover safe archive
  manifest parsing, exact replay-claim coverage, honest absent package/workflow operations,
  measured-or-explicitly-unmeasured resources, real Git citation/blob/range resolution,
  parsed architecture cross-links, local-versus-remote review identities, alias-aware
  command inventory, pre-finding rate-limit semantics, finalized ledger identity, positive
  and negative negotiation taxonomy, independent installed-RCP reads, internal-to-public
  provenance/redaction, exact evidence-set source/exclusion fields, allowlisted transition
  packet eligibility, immutable review receipts, and mechanically checked 210-second polls.
- The remaining finding requested a `candidate_head` or `final_review_head` naming this
  review record's complete containing commit. It is rejected as a Git self-reference: any
  commit that inserts its own prospective SHA changes that SHA. Exact candidate/final review
  heads remain external PR review evidence; this record may name only already-existing
  historical reviewed heads.
- The managed Codex preflight now resolves `/home/at/.bun/bin/codex` by absolute path and
  verifies `codex-cli 0.147.0` plus executable SHA-256 before freezing the tool profile.
- This was the third review cycle. On `2026-08-12` the owner explicitly superseded the
  former three-cycle ceiling: continue fixing legitimate findings and stop only when all
  material findings are resolved and every remainder is recorded as a low-benefit P2
  backlog item with stable ID, source, severity, owner, rationale, bounded Phase-1
  non-impact, and revisit trigger.

## Current gate

Commit/push the complete corrected plan set and continue approved local-CodeRabbit and
GitHub-Codex exact-head cycles in parallel where safe. The final exact-head result stays
external on GitHub so recording it cannot invalidate the reviewed commit. Phase 1 remains
unapproved until the approved exact-head review path is satisfied, all material findings are
fixed or low-benefit P2 backlog items, and the human execution gate is explicit.

## Cycle 4

- Local CodeRabbit CLI `0.7.2` reviewed immutable candidate
  `40103fac08bf463c2bd111f402c39d5de41b74ac` and completed with twenty material findings.
- GitHub Codex was requested concurrently on the same exact SHA. It acknowledged the request
  but had not produced a verdict before accepted CodeRabbit fixes invalidated that candidate;
  no later `40103fa` result can satisfy the next exact-head gate.
- Nineteen findings were accepted directly: atomic archive verification; complete package
  provenance/native evidence; source tree and structure-registry binding; artifact-manifest
  bijection; capability-versus-review schema; installed RCP and transcript digest checks;
  source-record digest checks; recursively fail-closed packet, eligibility, receipt and UTC
  poll validation; pre-finding rate-limit wording; historical research labeling; runbook
  command order; and prompt/checklist fallback specificity.
- The proposed serial-only reviewer ordering was rejected as stale against explicit owner
  direction on `2026-08-12`. Resolution permits local CodeRabbit and GitHub Codex to run
  concurrently only after one candidate is committed and pushed, requires both same-SHA
  dispositions before acceptance, and limits fallback to CodeRabbit's required candidate-
  review mode returning `rate_limit` before any finding.
- No Cycle-4 item was deferred to P2.

## Cycle 5

- Local CodeRabbit CLI `0.7.2` reviewed immutable pushed candidate
  `732a668df6f775ccce6fb99f04cf15926484f9d8` and completed with twenty findings:
  seventeen major and three minor.
- GitHub Codex review `4912791656`, submitted `2026-08-12T03:31:24Z`, reviewed the same
  exact candidate and reported nine P1 findings. Both results invalidate `732a668`.
- Review prompt SHA-256 was
  `11486fc7180702644f3ff94792c744c85e72ed7aeb39f7f88435d32e230337d3`.
  Local command was `coderabbit review --agent --committed --base-commit
  21d3a0cbe5306cf6bf1943aba18cef77ed34ba37`. GitHub request evidence is
  `https://github.com/jodobear/uzel/pull/36#issuecomment-5261901479`; review evidence is
  `https://github.com/jodobear/uzel/pull/36#pullrequestreview-4912791656`.
- Accepted fixes cover live incident/tool/claim/disposition validation; safe retained-
  artifact containment; complete Nix discovery; nonvacuous current-source architecture,
  manifest, review-stage, package/RCP, negotiation, disclosure, closeout/teaching, and
  transition-row evidence; plus strict packet/decision/receipt parsing.
- Historical `01-REAUDIT.md` Napp gating was explicitly superseded, GSD tool evidence was
  aligned to live `1.10.0`, the pack review diagram/prompt were corrected, and the upstream
  contribution log became owned evidence.
- Two review claims were rejected with live evidence: the incident-restoration claim used
  an unrelated checkout while the actual execution worktree contains the exact commit,
  ref, clean registered blocked worktree, and archive; serial-only review ordering conflicts
  with the owner's explicit concurrent same-SHA direction and is unnecessary because
  acceptance waits for both dispositions. Previously corrected structure-registry inclusion
  was also retained rather than duplicated.
- No Cycle-5 item was deferred to P2.

Cycle-5 finding ledger (exactly twenty CR plus nine GH findings; all accepted):

| Lane / ID | Full actionable finding | Disposition |
|---|---|---|
| CR-01 | Plan 01 lacked executable live incident validation. | Added no-network fail-closed validator contract. |
| CR-02 | Claim evidence/blocker schema was not byte-bound. | Added exact manifest/digest and explicit critical-blocker validation. |
| CR-03 | Historical tool inventory could be empty/incomplete. | Required nonempty categorized six-origin inventory and closure links. |
| CR-04 | Final `b185ad1` disposition was not mechanically derived. | Required exact claim-set derivation, evidence and limitations. |
| CR-05 | Retained Plan-02 artifacts lacked safe containment. | Added regular-file, no-symlink-component, canonical-root validation. |
| CR-06 | Native artifact paths/digests were not exact cross-references. | Added exact retained-artifact manifest coverage and byte checks. |
| CR-07 | Plan-03 claims could omit citations without a strict absence reason. | Required resolved citations or typed, evidenced absence. |
| CR-08 | Plan-03 structural registries/cross-links could be empty. | Required nonempty exact registry sets and parsed links. |
| CR-09 | Plan-04 artifact manifest was not a full bijection. | Required unique command/reviewer/claim ownership and exact byte coverage. |
| CR-10 | Plan-04 review stages/order were weakly checked. | Required exactly two canonical stages and pushed-SHA-before-review order. |
| CR-11 | Installed RCP realization/provenance was not exact. | Bound independent installed bytes, package provenance and source profile. |
| CR-12 | Python integer checks accepted booleans. | Replaced with exact `type(value) is int` checks. |
| CR-13 | Plan-06 could execute after reviewed-plan drift. | Added external exact reviewed-head/receipt gate. |
| CR-14 | Plan-07 rows were not structurally sourced. | Required E-derived exact typed transition rows. |
| CR-15 | Plan-07 JSON accepted duplicate keys. | Added duplicate-key rejection and canonical JSON bytes. |
| CR-16 | Decision fields accepted duplicates/unknown names. | Added exact field allowlist and duplicate rejection. |
| CR-17 | Hold could retain invalid attempted-review receipts. | Required every present receipt to validate on hold and approve. |
| CR-18 | Re-audit described serial review despite owner-approved overlap. | Corrected to concurrent same-pushed-SHA attempts with joined disposition. |
| CR-19 | Re-audit retained an active historical Napp entry gate. | Marked historical/superseded; made absence capability-scoped. |
| CR-20 | Prompt contained malformed fallback sentence. | Rewrote exact pre-finding `rate_limit` fallback condition. |
| GH-01 | `not_yet_packaged` did not prove complete successful Nix discovery. | Required successful complete `flake show` and `eval`; otherwise blocking unavailable verdict. |
| GH-02 | Negotiation transcript set/vector binding could be empty or detached. | Required nonempty unique transcripts and exact vector-to-transcript links. |
| GH-03 | Plan-07 mandatory transition rows were not derived from E. | Required E-bound mandatory row derivation and exact set matching. |
| GH-04 | Review ordering did not push candidate before both reviewers. | Corrected both plan and pack lifecycle to push before concurrent attempts. |
| GH-05 | Plan-05 did not own canonical upstream contribution ledger update. | Added validated-pack ledger to task ownership and checks. |
| GH-06 | Plan-01 tool inventory could omit required origins/categories. | Added exact origin enum and required category coverage. |
| GH-07 | Plan-01 did not validate final preserved-WIP disposition. | Added one evidence-derived WIP disposition contract. |
| GH-08 | Plan-03 citations were not bound to Plan-02 measured source identity. | Bound citations to exact current Plan-02 source head/tree. |
| GH-09 | Plan-03 registry families could be empty. | Required nonempty interaction/format/boundary/storage/threat registries. |

Separate review claims outside those twenty-nine reported findings:

| Claim ID | Claim | Disposition |
|---|---|---|
| REJ-01 | Restore incident evidence because the review inspected an unrelated checkout. | Rejected: live execution worktree proved exact WIP/parent/ref/clean registered worktree/archive identities; Plan 01 now revalidates them fail-closed at execution. |
| REJ-02 | Force reviewers to run serially. | Rejected: owner authorized overlap after one pushed immutable SHA; acceptance still waits for both dispositions and any later commit invalidates both. |
| DUP-01 | Add structure-registry inclusion already present in corrected Plan 03. | Duplicate/no new change: retained existing exact nonempty registry-set contract. |

## Cycle 6

- Local CodeRabbit CLI `0.7.2` (executable SHA-256
  `f9f61ecdb385d3c8d5001ee652dee1d95b3282fd740bd023dc89b18a191d3e97`)
  reviewed immutable pushed candidate
  `dd22d21c291d986771fda0ed132208d7cb328e09` from review base
  `21d3a0cbe5306cf6bf1943aba18cef77ed34ba37` and completed with seventeen
  material findings. Command: `coderabbit review --agent --committed --base-commit
  21d3a0cbe5306cf6bf1943aba18cef77ed34ba37`.
- GitHub Codex review `4913339393`, submitted `2026-08-12T05:38:26Z`, reviewed
  the same exact candidate and reported seven P1 findings. Request evidence:
  `https://github.com/jodobear/uzel/pull/36#issuecomment-5262753976`; review
  evidence: `https://github.com/jodobear/uzel/pull/36#pullrequestreview-4913339393`.
- Review-prompt SHA-256 was
  `2f479c7ef2fa8a1955ea87002c647364e8e3f3e74301e69a7ecede7de6391860`.
  GitHub Codex was polled only after the required 210-second wait windows.
  Every material finding was accepted; no Cycle-6 finding was waived or deferred.
- Corrections cover exact runbook archive parity; executable no-network and Graphify
  evidence; honest package/measurement/status records; complete source/citation/CI
  registries; authority-derived capability, blocker, replacement and review inventories;
  exact plan/E/P/decision/closeout ancestry; complete PR-diff and immutable reviewer-output
  identity; remote receipt reachability; passed teaching witnesses; and a stock-GSD
  transition confirmation before post-verification closeout. Required tracking is part of
  closeout commit `C`, then final candidate freezes as `F=C`; no later tracked commit is
  permitted.

Cycle-6 finding ledger (exactly seventeen CodeRabbit plus seven GitHub Codex findings;
all accepted):

| Lane / ID | Full actionable finding | Corrective disposition |
|---|---|---|
| CR-01 | Review-prerequisite membership could be selected by packet rows. | Derive exact prerequisite inventory from authoritative E-bound sources. |
| CR-02 | Candidate review refs could change between reads. | Fetch and validate immutable refs atomically; recheck remote reachability. |
| CR-03 | Reviewer status/findings could come from a synthesized summary. | Retain actual immutable output and derive all lane fields from those bytes. |
| CR-04 | Unknown derived categories defaulted to informational. | Reject every category outside the exact authority-derived enum. |
| CR-05 | Hold review statuses and receipts were under-specified. | Require exact status enum and a valid receipt for every requested lane. |
| CR-06 | Retired source facts could bypass structural source-object checks. | Validate exact source object, pointer, ID, status and digest for every row. |
| CR-07 | Packet records were not equal to authoritative manifest fields. | Require exact field equality and manifest bijection. |
| CR-08 | Applicable architecture links could be empty. | Require nonempty exact applicable-link sets or evidenced typed inapplicability. |
| CR-09 | Network evidence did not bind exact probes and local fixtures. | Bind admitted commands, endpoints and expected results to the closure plan. |
| CR-10 | Citation test IDs did not resolve a source-test registry. | Require exact registry resolution and current source identity. |
| CR-11 | Capability matrix could omit identity/capability declarations. | Derive both universes from authorities, require set equality, then exact product. |
| CR-12 | Package status and exit evidence admitted inconsistent shapes. | Enforce fail-closed verdict/exit/output matrices with integer-only statuses. |
| CR-13 | Evidence-set receipt lacked executor handoff identity. | Bind plan, task, executor handoff and exact normal-commit sequence to E. |
| CR-14 | CI gates were not exact manifest subjects. | Require unique gate IDs, full fields, subjects and post-mutation byte parity. |
| CR-15 | Project review-policy documents could drift from plan validators. | Align PROJECT, ROADMAP, STATE and HANDOFF with exact two-lane policy. |
| CR-16 | Retained raw evidence root lacked complete safe-containment ownership. | Own ignored raw root and reject symlink/root escapes. |
| CR-17 | Evidence descriptor entries lacked tracked-file and current-byte checks. | Require regular tracked files, no symlink components and worktree/Git equality. |
| GH-01 | Incident archive validation contradicted the authoritative 8/5 runbook shape. | Require exact eight outer entries and five inner snapshot entries with FD-safe reads. |
| GH-02 | Closeout evidence was frozen before decision and phase verification. | Move closeout after summaries, canonical verification, completed UAT and learning curation. |
| GH-03 | Reviewer receipts did not bind the complete PR diff. | Bind repository, PR, live base, merge base, head and canonical diff digest. |
| GH-04 | Custom receipt refs were not proven remotely reachable. | Push content-addressed refs and validate them from authenticated remote state. |
| GH-05 | Plan-created validation code omitted mandatory Graphify refresh. | Add executable `graphify update .` ownership and a separate graph-only commit gate. |
| GH-06 | Measurements were not bound to retained RSS/size artifacts. | Require exact artifact IDs, paths, digests, units and byte parity. |
| GH-07 | Network-probe evidence was incomplete. | Require exact egress, DNS and declared local-fixture probes with retained output. |
