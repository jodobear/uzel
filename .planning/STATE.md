---
current_phase: 1
current_phase_name: POC replay and runtime recovery
total_phases: 5
current_plan: 01
total_plans_in_phase: 1
status: in_progress
progress: 50
last_activity: 2026-08-14
last_activity_desc: Product runtime authority corrected; REF-02, REF-03, REF-04 and protected disposition remain
---

# Current execution

- Phase: 1 — POC replay and runtime recovery
- Issue: [#42](https://github.com/jodobear/uzel/issues/42)
- Branch/worktree: `phase/01-poc-replay-napp-seam`; `/tmp/uzel-phase-01-replay-seam`
- PR: [#43](https://github.com/jodobear/uzel/pull/43) (draft)
- Plan 01: active — replay and ownership baseline complete; REF-02, REF-03, REF-04, and protected-evidence disposition remain
- Current authority: Uzel owns product runtime/composition and its private daemon; `jodobear/nampplets` supplies pinned native runtime crates; `pablof7z/nmp` owns Nostr semantics
- Human-only gate: protected replay/process worktrees under `/tmp` remain preserved in place and cannot be moved, archived, or deleted without authority
- Next action: complete current source-binding, restart-state, and induced real-WebKit recovery probes; then update evidence once and freeze one review head
