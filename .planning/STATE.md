---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: milestone
current_phase: 1
current_phase_name: SLICE-REF-01 — POC Replay & Accepted Napp Seam
status: planning
stopped_at: "Phase 1 planning blocked: accepted Napp client/events/testkit candidate absent"
last_updated: "2026-08-09T15:39:55.889Z"
last_activity: 2026-08-09
last_activity_desc: Initial five-slice MVP roadmap created; all v1 requirements mapped.
progress:
  total_phases: 1
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-08-09)

**Core value:** Uzel makes local-first napplet composition visibly useful without duplicating runtime, Nostr, trust, or persistence truth owned by Napp and NMP.
**Current focus:** Phase 1 — SLICE-REF-01: POC Replay & Accepted Napp Seam

## Current Position

Phase: 1 of 5 (SLICE-REF-01 — POC Replay & Accepted Napp Seam)
Plan: 0 of TBD
Status: Ready to plan
Last activity: 2026-08-09 — Initial five-slice MVP roadmap created; all v1 requirements mapped.

Progress: [░░░░░░░░░░] 0%

## Performance Metrics

**Velocity:**

- Total plans completed: 0
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

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table. Current roadmap decisions:

- [Phase 1]: Accepted public Napp client/events/testkit seam is Social's non-negotiable gate; never promote POC-private IPC.
- [Phase 2]: Canonical Linux release is the Nix closure with one matching accepted Napp commit; no ambient `PATH` runtime.
- [Phase 3]: CI starts measured and conservative; its final package/merge-full contract consumes Phase 2.
- [Phases 4–5]: Social starts only after accepted REF, PKG, and CI M0 delivery gates; Uzel retains no duplicate Nostr/cache/runtime truth.

### Pending Todos

None yet.

### Blockers/Concerns

- [Phases 4–5] Social execution is blocked until accepted committed Napp candidate plus REF, PKG, and CI M0 gates are accepted.
- [Phases 1–3] REF and PKG can progress independently; CI can measure early but cannot finalize package/merge-full evidence before PKG.

## Deferred Items

| Category | Item | Status | Deferred At |
|----------|------|--------|-------------|
| Files | Local Files | After Social Home acceptance | 2026-08-09 |
| Authoring | Blossom, signing, wallets, and key custody | After accepted filesystem and external-signer seams | 2026-08-09 |
| Platforms | FIPS, media, ContextVM, Relatr, search, TUI, WASI, Android, native napplets, Flatpak | Beyond first milestone | 2026-08-09 |

## Session Continuity

Last session: 2026-08-09T15:39:55.885Z
Stopped at: Phase 1 planning blocked: accepted Napp client/events/testkit candidate absent
Resume file: .planning/phases/01-slice-ref-01-poc-replay-accepted-napp-seam/01-RESEARCH.md
