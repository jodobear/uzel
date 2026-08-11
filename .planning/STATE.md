---
gsd_state_version: 1.0
milestone: v4
milestone_name: Uzel product incubation through M5
current_phase: 01
current_phase_name: Truthful baseline and execution reset
status: ready_to_plan
stopped_at: V4 reorientation complete; revised Phase 1 planning required
last_updated: "2026-08-11T17:43:06Z"
last_activity: 2026-08-11
last_activity_desc: Reconciled GSD planning state to committed v4 programme authority
progress:
  total_phases: 34
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
---

# Project State

## Project Reference

See `.planning/PROJECT.md` and the authoritative programme ingest at
`docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST.md`.

**Core value:** Uzel delivers visibly useful local-first napplet composition while Napp
owns runtime authority and one canonical Nostr engine owns Nostr truth.

**Current focus:** Plan Phase 1 from the committed v4 authority. Do not execute until
Prompt 02 independent review resolves all material findings and the human gate passes.

## Current Position

- Phase: 1 of 34 — Truthful baseline and execution reset
- Plans: zero executable plans; five superseded plans retained as incident evidence
- Status: ready to plan
- Progress: `[..........]` 0%

## Accumulated Context

### Decisions

- Uzel remains one product-shaped repository through Phase 7.9/M5.
- Historical exact-source replay and the current Nix/native baseline produce separate
  verdicts; neither substitutes for the other.
- `b185ad1b8d9d034d151406b12aa189f5a6be970f`, its worktree, safety ref and portable
  archive remain preserved. Planning assigns only provisional disposition.
- Runtime is Codex. Automatic GSD worktrees and automatic phase advancement are off.
- Phase 2 cannot execute before the manifest/exact-build and launch-negotiation profile
  receives an explicit human go/no-go.
- Phase 7.9 freezes one exact candidate. A5 is a mandatory twelve-lane
  non-implementation stop, not automatic release or programme continuation.
- Installed help supports `--ingest`, `--ingest-format`, `--reviews`, `--coderabbit`,
  `--insert`, `--edit`, default plan verification and post-execution `verify-work`.
  Installed help does not document `--validate`; do not add it to the planning command.

### Pending Todos

- Run the exact Phase 1 planning command recorded below.
- Independently review the revised plans with `prompts/02-review-phase-1.md` and local
  CodeRabbit; resolve material findings.
- Stop for human go/no-go before any Phase 1 implementation.

### Blockers and Evidence Gaps

- Historical replay remains unresolved until execution realizes the exact source/lock
  closure and records an evidence-based final disposition.
- Current installable package outputs have not yet been proved; record
  `not_yet_packaged` when absent and assign full package acceptance to Phase 2.
- Napp candidate `0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e` remains a qualified stop,
  not an accepted product client/events/testkit seam.
- Managed Codex is `0.147.0`; bare `codex` resolves stale Nix `0.92.0` on `PATH`.
- GSD `1.8.0` warns that baked agent definitions predate current config; no speculative
  model override was added and no toolchain update belongs in this phase.
- Graphify is disabled in project config, so committed `.planning/codebase/` maps were
  used without mutating the graph.
- Preserved manual/incident worktrees yield expected health warnings; do not prune them.

### Resolved Environment Constraint

- `/workspace/tmp` and `/tmp` are the same 98 GiB `/dev/loop0` filesystem. Three
  verified ignored and unused Rust `target/` trees were removed, increasing available
  space to 89 GiB (6% used). Source, registered worktrees, active Upay P5.1 work,
  `upay-p22`, blocked Uzel WIP and portable evidence were preserved.

## Exact Next Action

```text
$gsd-plan-phase 1 --ingest docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST.md --ingest-format narrative
```

Do not run `$gsd-resume-work` and do not execute Phase 1 in this state.

## Session Continuity

Resume file: `.planning/phases/01-slice-ref-01-poc-replay-accepted-napp-seam/.continue-here.md`
