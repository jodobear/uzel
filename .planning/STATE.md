---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: milestone
current_phase: 01
current_phase_name: SLICE-REF-01 — POC Replay & Accepted Napp Seam
status: paused
stopped_at: V4 reorientation and independent Phase 1 plan review required
last_updated: "2026-08-11T12:25:41Z"
last_activity: 2026-08-11
last_activity_desc: Superseded plans disabled; v4 reorientation required
progress:
  total_phases: 5
  completed_phases: 0
  total_plans: 5
  completed_plans: 1
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-08-09)

**Core value:** Uzel makes local-first napplet composition visibly useful without duplicating runtime, Nostr, trust, or persistence truth owned by Napp and NMP.
**Current focus:** Reorient Phase 01 from the committed v4 authority; no implementation may resume before independent plan review passes.

## Current Position

Phase: 01 (SLICE-REF-01 — POC Replay & Accepted Napp Seam) — PAUSED
Plan: superseded plans preserved as evidence; revision not started
Status: Reorientation and independent review required
Last activity: 2026-08-11 — Superseded plans disabled; v4 reorientation required

Progress: [██░░░░░░░░] 20%

## Performance Metrics

**Velocity:**

- Total plans completed: 1 historical evidence-only plan
- Average duration: -
- Total execution time: 0 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| - | - | - | - |

**Recent Trend:**

- Last 5 plans: -
- Trend: -

*Updated after each plan completion*
**Per-Plan Metrics:**

| Plan | Duration | Tasks | Files |
|------|----------|-------|-------|
| Phase 01-slice-ref-01-poc-replay-accepted-napp-seam P04 | 7m 28s | 2 tasks | 9 files |

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table. Current roadmap decisions:

- [Phase 1]: Accepted public Napp client/events/testkit seam is Social's non-negotiable gate; never promote POC-private IPC.
- [Phase 1]: Candidate absence blocks only qualification/adaptation; merged-POC replay, ownership/pressure evidence, and product-document admission may be planned independently.
- [Phase 1]: One instance and one active read profile are the proven baseline; no multi-instance promise or duplicate NMP/Uzel store.
- [Phase 2]: Canonical Linux release is the Nix closure with one matching accepted Napp commit; no ambient `PATH` runtime.
- [Phase 3]: CI starts measured and conservative; its final package/merge-full contract consumes Phase 2.
- [Phases 4–5]: Social starts only after accepted REF, PKG, and CI M0 delivery gates; Uzel retains no duplicate Nostr/cache/runtime truth.
- [Phase ?]: Current Napp candidate is a repository-qualified stop, not an adapter seam.
- [Phase ?]: Plan-01 parity binds to its recorded immutable Uzel commit, not later HEAD changes.

### Pending Todos

- Run `docs/plans/uzel-product-incubation-v4-2026-08-10/prompts/01-reorient-current-gsd.md`.
- Independently review the revised Phase 1 plan with `prompts/02-review-phase-1.md`.

### Blockers/Concerns

- [Phase 1] Adapter implementation is blocked until an exact committed Napp client/events/testkit candidate passes qualification.
- [Phase 1] The v4 authority is committed; GSD reorientation and independent plan review remain incomplete.
- [Phase 1] Legacy Work 07 status is stale about PR #30; merged source is `19519c3`, while Debian 13 visible acceptance remains open.
- [Phases 4–5] New Social execution is blocked until accepted committed Napp candidate plus REF, PKG, and CI M0 gates are accepted.
- [Phases 1–3] REF preservation and Uzel-only package/CI research can progress independently; final package and merge-full evidence require accepted Napp/package outputs.

## Deferred Items

| Category | Item | Status | Deferred At |
|----------|------|--------|-------------|
| Files | Local Files | After Social Home acceptance | 2026-08-09 |
| Authoring | Blossom, signing, wallets, and key custody | After accepted filesystem and external-signer seams | 2026-08-09 |
| Platforms | FIPS, media, ContextVM, Relatr, search, TUI, WASI, Android, native napplets, Flatpak | Beyond first milestone | 2026-08-09 |

## Session Continuity

Last session: 2026-08-11T12:25:41Z
Stopped at: V4 reorientation and independent review required
Resume file: .planning/phases/01-slice-ref-01-poc-replay-accepted-napp-seam/.continue-here.md
