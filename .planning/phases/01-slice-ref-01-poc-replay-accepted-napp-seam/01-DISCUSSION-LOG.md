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
