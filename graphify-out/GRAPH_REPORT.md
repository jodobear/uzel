# Graph Report - uzel  (2026-07-28)

## Corpus Check
- 36 files · ~13,078 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 214 nodes · 206 edges · 34 communities (24 shown, 10 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `72f0d136`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- POC scope and acceptance
- Assumption validation and decision gates
- README.md
- Provisional component design
- Tests, quality gates, and demo
- Uzel POC agent instructions
- POC architecture
- Observed facts
- POC documentation audit
- Post-POC extraction
- Work 00 — validate assumptions
- Work 01 — scaffold
- Work 02 — Linux exact-build runner
- Work 04 — daemon, NMP, and persistence
- Execution slices
- Uzel single-repository POC
- POC status
- Work 05 — composed demo
- Work 06 — hardening and demo acceptance
- audit_docs.py
- manifest.json
- README.md
- Gate 0 preflight and Slice 01 decision
- nampplets Linux reuse map
- WebKit/Tauri trust spike
- NMP API and ownership map
- FACT-001-kehto-package-line.md
- FACT-002-spec-revisions.md
- FACT-003-nampplets-linux-reuse.md
- FACT-004-nmp-facade.md
- FACT-005-webkit-tauri-trust.md
- FACT-006-csp-egress.md
- FACT-007-local-ipc.md
- FACT-008-toolchain.md

## God Nodes (most connected - your core abstractions)
1. `Assumption validation and decision gates` - 9 edges
2. `POC architecture` - 9 edges
3. `Provisional component design` - 9 edges
4. `Gate 0 preflight and Slice 01 decision` - 9 edges
5. `Uzel POC agent instructions` - 8 edges
6. `POC scope and acceptance` - 8 edges
7. `POC documentation audit` - 7 edges
8. `Verified facts` - 7 edges
9. `nampplets Linux reuse map` - 7 edges
10. `WebKit/Tauri trust spike` - 7 edges

## Surprising Connections (you probably didn't know these)
- None detected - all connections are within the same source files.

## Import Cycles
- None detected.

## Communities (34 total, 10 thin omitted)

### Community 0 - "POC scope and acceptance"
Cohesion: 0.18
Nodes (10): Architectural invariants, Demo-complete, Foundation-complete, Non-goals, Objective, POC scope and acceptance, Required napplets, Stop rule (+2 more)

### Community 1 - "Assumption validation and decision gates"
Cohesion: 0.22
Nodes (9): Assumption validation and decision gates, Current observed baseline, Gate 0 decision, Gate matrix, Hard stops, Per-slice validation, Plan correction rule, Required outputs (+1 more)

### Community 2 - "README.md"
Cohesion: 0.29
Nodes (6): Acceptance, Entry status and pins, Goal, Read, Tasks, Work 03 — portable napplets

### Community 3 - "Provisional component design"
Cohesion: 0.20
Nodes (9): Developer mode, Exact-build fixtures, Identity and Nostr reads, Local daemon protocol, Napplet convention, Provisional component design, Runtime state, Shell UI (+1 more)

### Community 4 - "Tests, quality gates, and demo"
Cohesion: 0.20
Nodes (9): Deterministic demo, Final acceptance, Hostile frame, Live demo, Napplet/web, Quality commands, Required test layers, Runtime/Rust (+1 more)

### Community 5 - "Uzel POC agent instructions"
Cohesion: 0.22
Nodes (8): Engineering rules, Mission, POC exclusions, Quality gate, Repository boundaries, Required method, Trust rules, Uzel POC agent instructions

### Community 6 - "POC architecture"
Cohesion: 0.22
Nodes (9): Accepted upstream seam, Composition flow, POC architecture, Repository zones, Runtime topology, Session start, Shared Nostr flow, Trust domains (+1 more)

### Community 7 - "Observed facts"
Cohesion: 0.22
Nodes (9): Decision, Kehto #204, `nampplets`, NAP registry, Napplet packages, NIP-5A and NIP-5D, NMP, Source baseline (+1 more)

### Community 8 - "POC documentation audit"
Cohesion: 0.25
Nodes (7): Confidence, Corrections, Diagram review, Gate 0 resolution, POC documentation audit, Problems found in the previous pack, Verdict

### Community 9 - "Post-POC extraction"
Cohesion: 0.29
Nodes (6): Likely `kehto/napd`, Moves to `jodobear/napplets`, POC shortcuts that must not silently become platform contracts, Post-POC extraction, Remains in Uzel, Rewrite criteria

### Community 10 - "Work 00 — validate assumptions"
Cohesion: 0.29
Nodes (7): Done when, Goal, Outputs, Read, Stop conditions, Tasks, Work 00 — validate assumptions

### Community 11 - "Work 01 — scaffold"
Cohesion: 0.25
Nodes (7): Acceptance, Depends on, Entry status, Goal, Non-goals, Tasks, Work 01 — scaffold

### Community 12 - "Work 02 — Linux exact-build runner"
Cohesion: 0.25
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 02 — Linux exact-build runner

### Community 13 - "Work 04 — daemon, NMP, and persistence"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 04 — daemon, NMP, and persistence

### Community 14 - "Execution slices"
Cohesion: 0.33
Nodes (6): Dependency graph, Execution slices, Handoff, Parallel work, Slice entry gate, Slices

### Community 15 - "Uzel single-repository POC"
Cohesion: 0.33
Nodes (6): Demo result, Document map, Mandatory first step, Scope rule, Start order, Uzel single-repository POC

### Community 16 - "POC status"
Cohesion: 0.33
Nodes (5): Current blockers, Gate 0 — validated baseline, Implementation, Latest integrated evidence, POC status

### Community 17 - "Work 05 — composed demo"
Cohesion: 0.33
Nodes (5): Acceptance, Goal, Non-goals, Tasks, Work 05 — composed demo

### Community 18 - "Work 06 — hardening and demo acceptance"
Cohesion: 0.40
Nodes (4): Acceptance, Goal, Tasks, Work 06 — hardening and demo acceptance

### Community 19 - "audit_docs.py"
Cohesion: 0.83
Nodes (3): main(), strip_code(), write_manifest()

### Community 22 - "Gate 0 preflight and Slice 01 decision"
Cohesion: 0.22
Nodes (9): Blockers, Confirmed assumptions, Decision, Exact next steps, Gate 0 preflight and Slice 01 decision, Gate results, Rejected assumptions, Required design changes (+1 more)

### Community 23 - "nampplets Linux reuse map"
Cohesion: 0.25
Nodes (7): Apple-only edges, Build evidence, Compatibility candidate, Crate map, nampplets Linux reuse map, Result, Runtime entry points

### Community 24 - "WebKit/Tauri trust spike"
Cohesion: 0.25
Nodes (7): Accepted child CSP, Boundary of proof, Executable probe, Low-level handler nuance, Source verification, Verdict, WebKit/Tauri trust spike

### Community 25 - "NMP API and ownership map"
Cohesion: 0.29
Nodes (6): Accepted pin, Executable probe, nampplets adapter seam, NMP API and ownership map, Ownership boundary, Public facade

## Knowledge Gaps
- **150 isolated node(s):** `files`, `Mission`, `Required method`, `Engineering rules`, `Repository boundaries` (+145 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **10 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Assumption validation and decision gates` connect `Assumption validation and decision gates` to `README.md`?**
  _High betweenness centrality (0.058) - this node is a cross-community bridge._
- **Why does `POC architecture` connect `POC architecture` to `README.md`?**
  _High betweenness centrality (0.058) - this node is a cross-community bridge._
- **What connects `files`, `Mission`, `Required method` to the rest of the system?**
  _150 weakly-connected nodes found - possible documentation gaps or missing edges._