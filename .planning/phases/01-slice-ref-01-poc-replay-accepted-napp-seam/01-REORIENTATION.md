# Phase 1 v4 reorientation record

**Status:** Complete; ready to plan, not ready to execute
**Date:** 2026-08-11
**Worktree:** `/workspace/projects/napplets/napp-uzel/uzel-phase-1-v4`
**Branch/head:** `phase/01-baseline-v4` at `1b58778f8b2f9945ef2ab9427cdfa673c04eb908`
**Authority:** `docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST.md`

## Planning files changed

- `.planning/config.json`
- `.planning/PROJECT.md`
- `.planning/REQUIREMENTS.md`
- `.planning/ROADMAP.md`
- `.planning/STATE.md`
- `.planning/HANDOFF.json`
- this phase's `01-CONTEXT.md`, `.continue-here.md` and `01-REORIENTATION.md`
- phase directories for every roadmap phase through 7.9

No product source, dependency lock, generated output or blocked replay worktree changed.

## Configuration reconciliation

| Setting | Before | After |
|---|---|---|
| runtime | `codex` | `codex` |
| `workflow.use_worktrees` | absent | `false` |
| `workflow.auto_advance` | `true` | `false` |
| `workflow._auto_chain_active` | `false` | `false` |
| speculative model overrides | none | none |

## Phase map

Before: `1, 2, 3, 4, 5` with stale package/CI/Social dependencies.

After:

```text
1,
2, 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7,
3, 3.1, 3.2, 3.3,
4, 4.1, 4.2, 4.3,
5, 5.1, 5.2, 5.3,
6, 6.1, 6.2,
7, 7.1, 7.2, 7.3, 7.4, 7.5, 7.6, 7.7, 7.8, 7.9
```

All 34 phases now have live roadmap entries and phase directories. Integer phases 2–7
remain the first increments; decimal phases are inserted without renumbering. Each
post-M0 phase is one contextual issue, manual worktree/branch and primary PR. Phase 7.9
freezes one exact candidate; A5 is a mandatory twelve-lane non-implementation stop.

## Toolchain and installed help

| Tool | Observed version | Executable SHA-256 / note |
|---|---|---|
| managed Codex | `0.147.0` | `134063e133f0b4244fa3b251acf973d4fe4b4aeeacbdc135211bf480f59f1477` |
| bare `codex` on PATH | `0.92.0` | `57015466038e5af95bc1d3df371fddf12ac441bd336edd9871c3613599ba2b92` |
| GSD core | `1.8.0` | VERSION file `6754fcea32b88564b0879ceb065063ff5c69d08e65a6046a9f07edb5b324d3e9` |
| GSD full help | installed | `d2bc66716d6adb43da9b4402b23500851ab64fe439f7e2c44766130d958d5290` |
| `gsd-tools.cjs` | installed | `d44831d1e2f9c1c1edae70d519b8cd803b5c526bf468c40fd8630e9b735a5923` |
| CodeRabbit | `0.7.2` | `f9f61ecdb385d3c8d5001ee652dee1d95b3282fd740bd023dc89b18a191d3e97` |
| Node / npm | `v22.22.0` / `10.9.4` | observed live |
| Rust / Cargo | `1.89.0` / `1.89.0` | observed live |
| Nix | `2.34.1` | observed live |

Installed help confirms `plan-phase --ingest --ingest-format --reviews`,
`review --phase --coderabbit`, `phase --insert --edit`, default plan verification and
post-execution `verify-work`. It does not document `plan-phase --validate`; the runbook
command therefore omits that stale assumption.

## Preserved incident evidence

- WIP `b185ad1b8d9d034d151406b12aa189f5a6be970f`, parent
  `431e37af5ca86196dbaf08a534a0a7626c4ae32c`
- clean blocked worktree `/tmp/uzel-01-01-3qGzwY`
- safety ref `wip/phase-1-replay-b185ad1`
- portable bundle `/workspace/projects/napplets/napp-uzel/uzel-phase-1-b185ad1/`
- bundle SHA-256 `2690ff85ed2d561af3592833d6741b92e7cedfb0afdf343db1d140bfde0cba37`
- `sha256sum -c` and `git bundle verify` passed during reorientation

Final WIP disposition remains an execution decision based on separate historical replay
and current package verdicts.

## Contradictions resolved

- Replaced impossible no-provisioning replay with exact-source/exact-lock hermetic
  replay plus a separate current Nix/native baseline.
- Removed package-first, filesystem-first and automatic post-M5 assumptions.
- Replaced the stale five-phase map with the full product sequence through M5.
- Replaced automatic Codex worktrees/advancement with manual bounded worktrees and human
  gates.
- Preserved the architecture invariants and added machine registry, RCP, terminology,
  SIR, negotiation, capability-ledger, fairness, compatibility, patch-lifecycle,
  visibility and learning baselines to M0.

## Unresolved evidence gaps

- Historical replay and final `b185ad1` disposition require execution evidence.
- Current Nix/native installable outputs require live inventory; absent outputs must be
  reported `not_yet_packaged`, with full package acceptance assigned to Phase 2.
- Napp candidate `0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e` remains a `stop`.
- Bare and managed Codex versions disagree; use the managed executable for the phase.
- GSD warns baked agent definitions predate current config; no mid-phase update or model
  override was introduced.
- Graphify is disabled, so the committed codebase map was used without a graph update.
- Preserved manual incident worktrees produce expected non-repairable health warnings.

## Resolved environment constraint

`/workspace/tmp` and `/tmp` resolve to the same 98 GiB `/dev/loop0` filesystem. After
the initial sandbox failure, three exact ignored and unused Rust `target/` directories
were removed from old Upay worktrees. Available capacity increased to 89 GiB (6% used).
No source, registered worktree, active Upay P5.1 work, `upay-p22`, blocked Uzel WIP or
portable evidence was removed.

## Exact next command

```text
$gsd-plan-phase 1 --ingest docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST.md --ingest-format narrative
```

Then run `prompts/02-review-phase-1.md`, resolve every material independent-review and
CodeRabbit finding, and stop for human go/no-go. Do not run `$gsd-resume-work` or execute
Phase 1 during reorientation/review.
