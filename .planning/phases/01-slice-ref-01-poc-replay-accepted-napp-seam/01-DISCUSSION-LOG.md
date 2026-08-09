# Phase 1: SLICE-REF-01 — POC Replay & Accepted Napp Seam - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-09
**Phase:** 1-SLICE-REF-01 — POC Replay & Accepted Napp Seam
**Areas discussed:** Accepted Napp candidate gate, Replay and ownership evidence, Baseline measurement, Recovery and upstream stops

---

## Accepted Napp Candidate Gate

| Option | Description | Selected |
|--------|-------------|----------|
| Exact accepted candidate | Require one exact committed Napp client/events/testkit candidate, accepted by source and executable evidence | ✓ |
| Legacy protocol | Continue on the current private daemon protocol | |
| Uzel facade | Design a temporary product-only compatibility seam | |

**User's choice:** `[--auto]` Exact accepted candidate — recommended default.
**Notes:** `[auto] Accepted Napp candidate gate — Q: "What qualifies as the Phase 1 Napp seam?" → Selected: "Require one exact accepted committed candidate" (recommended default).`

---

## Replay and Ownership Evidence

| Option | Description | Selected |
|--------|-------------|----------|
| Replay then classify | Replay exact POC evidence, then map every seam to source, tests, owner, repository, and commit | ✓ |
| Extract first | Move reusable-looking code before replaying the baseline | |
| Docs only | Treat existing documentation as sufficient proof | |

**User's choice:** `[--auto]` Replay then classify — recommended default.
**Notes:** `[auto] Replay and ownership evidence — Q: "How should the POC be preserved while ownership moves toward Napp?" → Selected: "Replay first and classify every seam with exact evidence" (recommended default).`

---

## Baseline Measurement

| Option | Description | Selected |
|--------|-------------|----------|
| Reproducible measurements | Record exact commands, environment, values, and explicit unavailable fields | ✓ |
| Qualitative only | Record observations without numeric baselines | |
| Aspirational targets | Adopt performance targets before measuring the current path | |

**User's choice:** `[--auto]` Reproducible measurements — recommended default.
**Notes:** `[auto] Baseline measurement — Q: "How should Phase 1 handle lifecycle and resource metrics?" → Selected: "Record reproducible measurements and explicit unavailable fields" (recommended default).`

---

## Recovery and Upstream Stops

| Option | Description | Selected |
|--------|-------------|----------|
| Fail closed | Preserve the POC, record a repository-qualified missing contract, and hand off resumably | ✓ |
| Local shim | Add a private Uzel compatibility layer | |
| Mocked success | Continue implementation with a placeholder Napp result | |

**User's choice:** `[--auto]` Fail closed — recommended default.
**Notes:** `[auto] Recovery and upstream stops — Q: "What happens when the accepted Napp contract or recovery behavior is missing?" → Selected: "Fail closed with a repository-qualified dependency and resumable handoff" (recommended default).`

## the agent's Discretion

- Exact artifact filenames, metric harness organization, and probe grouping within the recorded phase boundary.

## Deferred Ideas

- Nix packaging, delivery CI, Social Home, Files, Blossom, signing, and later platforms/capabilities remain in their assigned later phases.

---

## Source-backed re-audit update — 2026-08-09

User requested all-lane re-audit against mapped source and product-first documentation.
The following decisions supersede narrower automatic defaults where they conflict:

| Question | Selected decision | Rejected expansion |
|---|---|---|
| Does missing Napp block all Phase 1 work? | No. It blocks candidate qualification/adaptation only; POC replay, ownership/pressure evidence, and document admission proceed. | Private facade or idle whole phase |
| What is fastest visible value? | Replay/preserve merged rich profile/follow POC at exact pins. | Rebuild profile/follows or start graph/feed early |
| What architecture is assumed? | Brownfield evidence is retained and classified; no component is automatically final or disposable. | Big-bang rewrite or automatic extraction |
| What instance/profile scope is accepted? | One instance, one active read profile; record collision behavior. | Full multi-instance implementation in REF-01 |
| How does migration roll back? | One narrow Rust/Tauri adapter, parity replay, then selective retirement; revert adapter/pin if needed. | Dual state/write or protocol-wide cutover |
| What documents are authoritative? | Committed `.planning/` plus exact source now; installed root pack becomes authority only after audit/commit. | Treat untracked pack as committed fact |
| What review process applies? | One bounded issue/PR; local CodeRabbit, Codex, final GitHub CodeRabbit once per semantic candidate. | Review reruns on unchanged inputs or Phase 1 planning ceremony |

Full classifications and readiness verdict: `01-REAUDIT.md`.
