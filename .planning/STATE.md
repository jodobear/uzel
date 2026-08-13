---
current_phase: 1
current_phase_name: POC replay and accepted Napp seam
total_phases: 5
current_plan: 01
total_plans_in_phase: 1
status: blocked
progress: 43
last_activity: 2026-08-14
last_activity_desc: Replay passed; protected disposition and REF-02, REF-03, REF-04, REF-07 remain blocked
---

# Current execution

- Phase: 1 — POC replay and accepted Napp seam
- Issue: [#42](https://github.com/jodobear/uzel/issues/42)
- Branch/worktree: `phase/01-poc-replay-napp-seam`; `/tmp/uzel-phase-01-replay-seam`
- PR: [#43](https://github.com/jodobear/uzel/pull/43) (draft)
- Plan 01: active and blocked — replay is complete; protected-evidence disposition and REF-02, REF-03, REF-04, and REF-07 remain open
- Blocker: protected replay/process worktrees under `/tmp` lack durable retention and cannot be moved, archived, or deleted without human authority; REF-02 awaits source-binding proof through an adopted Napp request path; REF-03 still lacks one restart replay that observes recovered follows, an already-installed exact build, and an ambiguous outcome after restart; REF-04 awaits an induced real-WebKit recovery cycle; REF-07 awaits committed qualifying evidence from `jodobear/napp`.
- Next action: with human authority, preserve the protected temporary worktrees in a durable location without deleting their refs/evidence; after Napp owner publishes one qualifying exact successor, rebind the fail-closed checker/evidence without overwriting the STOP baseline, adopt the bounded Uzel seam, and add the smallest affected restart/native recovery probes before final exact-head reviews
