# Graph Report - uzel  (2026-07-29)

## Corpus Check
- 71 files · ~27,854 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 465 nodes · 497 edges · 58 communities (37 shown, 21 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3ecad56a`
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
- fedora-run-smoke.sh
- debian-build-smoke.sh
- dev.sh
- App.svelte
- main.rs
- README.md
- README.md
- check-boundaries.sh
- smoke.sh
- lib.rs
- Uzel single-repository POC
- ExactFixtureSource
- trusted-shell-policy.js
- Window
- check-pinned-assets.sh
- README.md
- README.md
- FACT-009-linux-runner.md

## God Nodes (most connected - your core abstractions)
1. `LinuxRunner` - 16 edges
2. `compilerOptions` - 15 edges
3. `scripts` - 12 edges
4. `EventBuffer` - 9 edges
5. `Uzel POC agent instructions` - 9 edges
6. `Assumption validation and decision gates` - 9 edges
7. `POC architecture` - 9 edges
8. `Provisional component design` - 9 edges
9. `Gate 0 preflight and Slice 01 decision` - 9 edges
10. `Slice 02 preflight` - 9 edges

## Surprising Connections (you probably didn't know these)
- `start_fixture()` --references--> `LinuxRunner`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd/src/runner.rs
- `start_fixture()` --references--> `SurfaceLaunch`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd/src/runner.rs
- `forward_surface_envelope()` --references--> `LinuxRunner`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd/src/runner.rs

## Import Cycles
- None detected.

## Communities (58 total, 21 thin omitted)

### Community 0 - "POC scope and acceptance"
Cohesion: 0.18
Nodes (10): Architectural invariants, Demo-complete, Foundation-complete, Non-goals, Objective, POC scope and acceptance, Required napplets, Stop rule (+2 more)

### Community 1 - "Assumption validation and decision gates"
Cohesion: 0.23
Nodes (18): boundedJSON(), compatibilityPreludeSource(), decodedBase64Length(), decodeResourceBlob(), exactFields(), isBoundedEnvelope(), isPlainObject(), isVerifiedArtifactBaseURL() (+10 more)

### Community 2 - "README.md"
Cohesion: 0.33
Nodes (6): Acceptance, Entry status and pins, Goal, Read, Tasks, Work 03 — portable napplets

### Community 3 - "Provisional component design"
Cohesion: 0.20
Nodes (9): Developer mode, Exact-build fixtures, Identity and Nostr reads, Local daemon protocol, Napplet convention, Provisional component design, Runtime state, Shell UI (+1 more)

### Community 4 - "Tests, quality gates, and demo"
Cohesion: 0.20
Nodes (9): Deterministic demo, Final acceptance, Hostile frame, Live demo, Napplet/web, Quality commands, Required test layers, Runtime/Rust (+1 more)

### Community 5 - "Uzel POC agent instructions"
Cohesion: 0.22
Nodes (9): Engineering rules, Mission, POC exclusions, Quality gate, Repository boundaries, Required method, Trust rules, Upstream contribution policy (+1 more)

### Community 6 - "POC architecture"
Cohesion: 0.22
Nodes (9): Accepted upstream seam, Composition flow, POC architecture, Repository zones, Runtime topology, Session start, Shared Nostr flow, Trust domains (+1 more)

### Community 7 - "Observed facts"
Cohesion: 0.22
Nodes (9): Decision, Kehto #204, `nampplets`, NAP registry, Napplet packages, NIP-5A and NIP-5D, NMP, Source baseline (+1 more)

### Community 8 - "POC documentation audit"
Cohesion: 0.07
Nodes (23): Uzel agent instructions, Development, Uzel, Assumption validation and decision gates, Current observed baseline, Gate 0 decision, Gate matrix, Hard stops (+15 more)

### Community 9 - "Post-POC extraction"
Cohesion: 0.29
Nodes (6): Likely `kehto/napd`, Moves to `jodobear/napplets`, POC shortcuts that must not silently become platform contracts, Post-POC extraction, Remains in Uzel, Rewrite criteria

### Community 10 - "Work 00 — validate assumptions"
Cohesion: 0.25
Nodes (7): Done when, Goal, Outputs, Read, Stop conditions, Tasks, Work 00 — validate assumptions

### Community 11 - "Work 01 — scaffold"
Cohesion: 0.04
Nodes (38): Dependency graph, Execution slices, Handoff, Parallel work, Slice entry gate, Slices, Slice handoff, Acceptance (+30 more)

### Community 12 - "Work 02 — Linux exact-build runner"
Cohesion: 0.22
Nodes (9): Commands and observed results, Exact dependency and asset record, Next step, Preserved failed Fedora probe, Required design correction, Runtime evidence, Slice 02 preflight, Upstream result (+1 more)

### Community 13 - "Work 04 — daemon, NMP, and persistence"
Cohesion: 0.25
Nodes (7): Confidence, Corrections, Diagram review, Gate 0 resolution, POC documentation audit, Problems found in the previous pack, Verdict

### Community 14 - "Execution slices"
Cohesion: 0.25
Nodes (8): Boundary result, Commands and observed results, Debian probe correction, Fedora probe correction, Locked workspace, Next step, Slice 01 preflight, Verdict

### Community 15 - "Uzel single-repository POC"
Cohesion: 0.08
Nodes (23): compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, isolatedModules, lib, module (+15 more)

### Community 16 - "POC status"
Cohesion: 0.08
Nodes (23): dependencies, svelte, @tauri-apps/api, devDependencies, svelte-check, @sveltejs/vite-plugin-svelte, typescript, vite (+15 more)

### Community 17 - "Work 05 — composed demo"
Cohesion: 0.09
Nodes (21): fallow, devDependencies, fallow, engines, node, name, packageManager, private (+13 more)

### Community 18 - "Work 06 — hardening and demo acceptance"
Cohesion: 0.12
Nodes (16): app, security, windows, build, beforeBuildCommand, beforeDevCommand, devUrl, frontendDist (+8 more)

### Community 19 - "audit_docs.py"
Cohesion: 0.83
Nodes (3): main(), strip_code(), write_manifest()

### Community 21 - "README.md"
Cohesion: 0.25
Nodes (7): description, identifier, permissions, $schema, windows, core:default, main

### Community 22 - "Gate 0 preflight and Slice 01 decision"
Cohesion: 0.22
Nodes (9): Accepted provisional risks, Confirmed assumptions, Decision, Exact next steps, Gate 0 preflight and Slice 01 decision, Gate results, Rejected assumptions, Required design changes (+1 more)

### Community 23 - "nampplets Linux reuse map"
Cohesion: 0.25
Nodes (7): Apple-only edges, Build evidence, Compatibility candidate, Crate map, nampplets Linux reuse map, Result, Runtime entry points

### Community 24 - "WebKit/Tauri trust spike"
Cohesion: 0.25
Nodes (7): Accepted child CSP, Boundary of proof, Executable probe, Low-level handler nuance, Source verification, Verdict, WebKit/Tauri trust spike

### Community 25 - "NMP API and ownership map"
Cohesion: 0.29
Nodes (6): Accepted pin, Executable probe, nampplets adapter seam, NMP API and ownership map, Ownership boundary, Public facade

### Community 34 - "fedora-run-smoke.sh"
Cohesion: 0.29
Nodes (7): cleanup(), GDK_BACKEND, NO_AT_BRIDGE, preserve_failure(), fedora-run-smoke.sh script, WAYLAND_DISPLAY, XDG_RUNTIME_DIR

### Community 40 - "main.rs"
Cohesion: 0.38
Nodes (8): forward_surface_envelope(), HostileProbe, report_hostile_probe(), Mutex, Result, String, start_fixture(), State

### Community 47 - "lib.rs"
Cohesion: 0.08
Nodes (28): Arc, AsRef, Condvar, EventBuffer, EventSink, LinuxRunner, payload_identity_cannot_select_surface_or_session(), Mutex (+20 more)

### Community 48 - "Uzel single-repository POC"
Cohesion: 0.33
Nodes (6): Demo result, Document map, Mandatory first step, Scope rule, Start order, Uzel single-repository POC

### Community 49 - "ExactFixtureSource"
Cohesion: 0.40
Nodes (4): ArtifactFetchRequest, ArtifactFetchResponse, ArtifactSource, ExactFixtureSource

### Community 50 - "trusted-shell-policy.js"
Cohesion: 0.83
Nodes (3): directive(), innerPolicyContent(), outerPolicyContent()

## Knowledge Gaps
- **254 isolated node(s):** `name`, `private`, `version`, `type`, `dev:web` (+249 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **21 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Uzel POC agent instructions` connect `Uzel POC agent instructions` to `POC documentation audit`?**
  _High betweenness centrality (0.015) - this node is a cross-community bridge._
- **What connects `name`, `private`, `version` to the rest of the system?**
  _254 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `POC documentation audit` be split into smaller, more focused modules?**
  _Cohesion score 0.07130124777183601 - nodes in this community are weakly interconnected._
- **Should `Work 01 — scaffold` be split into smaller, more focused modules?**
  _Cohesion score 0.04343971631205674 - nodes in this community are weakly interconnected._
- **Should `Uzel single-repository POC` be split into smaller, more focused modules?**
  _Cohesion score 0.08333333333333333 - nodes in this community are weakly interconnected._
- **Should `POC status` be split into smaller, more focused modules?**
  _Cohesion score 0.08333333333333333 - nodes in this community are weakly interconnected._
- **Should `Work 05 — composed demo` be split into smaller, more focused modules?**
  _Cohesion score 0.09090909090909091 - nodes in this community are weakly interconnected._