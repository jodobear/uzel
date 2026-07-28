# Graph Report - uzel  (2026-07-29)

## Corpus Check
- 96 files · ~41,326 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 659 nodes · 735 edges · 81 communities (55 shown, 26 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `5e6e99b8`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- POC scope and acceptance
- Assumption validation and decision gates
- check-napplet-imports.mjs
- Provisional component design
- Tests, quality gates, and demo
- Uzel POC agent instructions
- POC architecture
- Observed facts
- POC documentation audit
- Work 01 — scaffold
- Work 00 — validate assumptions
- Work 00 — validate assumptions
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
- Post-POC extraction
- README.md
- README.md
- check-boundaries.sh
- smoke.sh
- lib.rs
- POC documentation audit
- ExactFixtureSource
- trusted-shell-policy.js
- Window
- check-pinned-assets.sh
- README.md
- README.md
- FACT-009-linux-runner.md
- package.json
- package.json
- profile-open-v1.schema.json
- Assumption validation and decision gates
- Slice 02 preflight
- Work 00 — validate assumptions
- main.js
- 04-execution.md
- Work 02 — Linux exact-build runner
- Work 04 — daemon, NMP, and persistence
- Upstream contribution ledger
- Execution slices
- Uzel single-repository POC
- Work 03 — portable napplets
- POC status
- build-signed-napplet-fixtures.sh
- FACT-010-portable-napplets.md
- Work 05 — composed demo
- Work 06 — hardening and demo acceptance
- README.md

## God Nodes (most connected - your core abstractions)
1. `LinuxRunner` - 16 edges
2. `compilerOptions` - 15 edges
3. `scripts` - 14 edges
4. `Slice 03 preflight` - 10 edges
5. `EventBuffer` - 9 edges
6. `Uzel POC agent instructions` - 9 edges
7. `Assumption validation and decision gates` - 9 edges
8. `POC architecture` - 9 edges
9. `Provisional component design` - 9 edges
10. `Gate 0 preflight and Slice 01 decision` - 9 edges

## Surprising Connections (you probably didn't know these)
- `start_fixture()` --references--> `LinuxRunner`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd/src/runner.rs
- `start_fixture()` --references--> `SurfaceLaunch`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd/src/runner.rs
- `forward_surface_envelope()` --references--> `LinuxRunner`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd/src/runner.rs
- `directFollows()` --calls--> `isCanonicalPubkey()`  [EXTRACTED]
  napplets/follow-list/src/model.js → contracts/profile-open.js
- `canonicalProfile()` --calls--> `isCanonicalPubkey()`  [EXTRACTED]
  napplets/profile-card/src/model.js → contracts/profile-open.js

## Import Cycles
- None detected.

## Communities (81 total, 26 thin omitted)

### Community 0 - "POC scope and acceptance"
Cohesion: 0.18
Nodes (10): Architectural invariants, Demo-complete, Foundation-complete, Non-goals, Objective, POC scope and acceptance, Required napplets, Stop rule (+2 more)

### Community 1 - "Assumption validation and decision gates"
Cohesion: 0.23
Nodes (18): boundedJSON(), compatibilityPreludeSource(), decodedBase64Length(), decodeResourceBlob(), exactFields(), isBoundedEnvelope(), isPlainObject(), isVerifiedArtifactBaseURL() (+10 more)

### Community 2 - "check-napplet-imports.mjs"
Cohesion: 0.09
Nodes (25): allowedGuardedGlobalAccesses, anyResourceAttribute, { compile: compileSvelte, parse: parseSvelte }, declarativeNetworkViolations(), dependencyGroups, dependencyViolations(), directNetworkIdentifiers, globalReturningProperties (+17 more)

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

### Community 9 - "Work 01 — scaffold"
Cohesion: 0.25
Nodes (8): Acceptance, Depends on, Entry status, Goal, Non-goals, Status, Tasks, Work 01 — scaffold

### Community 10 - "Work 00 — validate assumptions"
Cohesion: 0.20
Nodes (10): API and ownership evidence, Automated review corrections, Commands and observed results, Exact pins and fixtures, Hostile fixture scope, Manifest correction, Next step, Slice 03 preflight (+2 more)

### Community 11 - "Work 00 — validate assumptions"
Cohesion: 0.38
Nodes (8): forward_surface_envelope(), HostileProbe, report_hostile_probe(), Mutex, Result, String, start_fixture(), State

### Community 12 - "Work 02 — Linux exact-build runner"
Cohesion: 0.25
Nodes (8): Boundary result, Commands and observed results, Debian probe correction, Fedora probe correction, Locked workspace, Next step, Slice 01 preflight, Verdict

### Community 13 - "Work 04 — daemon, NMP, and persistence"
Cohesion: 0.12
Nodes (24): isCanonicalPubkey(), parseProfileOpen(), profileOpen(), PUBKEY, list, render(), start(), status (+16 more)

### Community 14 - "Execution slices"
Cohesion: 0.40
Nodes (4): AcceptSettings, NativeSettingsExecutor, NativeSettingsOpenResult, NativeSettingsRequest

### Community 15 - "Uzel single-repository POC"
Cohesion: 0.08
Nodes (23): compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, isolatedModules, lib, module (+15 more)

### Community 16 - "POC status"
Cohesion: 0.08
Nodes (23): dependencies, svelte, @tauri-apps/api, devDependencies, svelte-check, @sveltejs/vite-plugin-svelte, typescript, vite (+15 more)

### Community 17 - "Work 05 — composed demo"
Cohesion: 0.06
Nodes (31): fallow, @napplet/cli, devDependencies, fallow, @napplet/cli, engines, node, name (+23 more)

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

### Community 40 - "Post-POC extraction"
Cohesion: 0.29
Nodes (6): Likely `kehto/napd`, Moves to `jodobear/napplets`, POC shortcuts that must not silently become platform contracts, Post-POC extraction, Remains in Uzel, Rewrite criteria

### Community 47 - "lib.rs"
Cohesion: 0.07
Nodes (36): Arc, ArtifactFetchRequest, ArtifactFetchResponse, ArtifactSource, AsRef, Condvar, EventBuffer, EventSink (+28 more)

### Community 48 - "POC documentation audit"
Cohesion: 0.25
Nodes (7): Confidence, Corrections, Diagram review, Gate 0 resolution, POC documentation audit, Problems found in the previous pack, Verdict

### Community 49 - "ExactFixtureSource"
Cohesion: 0.10
Nodes (20): dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/conformance-cli, @napplet/vite-plugin, vite, @napplet/conformance-cli (+12 more)

### Community 50 - "trusted-shell-policy.js"
Cohesion: 0.83
Nodes (3): directive(), innerPolicyContent(), outerPolicyContent()

### Community 58 - "package.json"
Cohesion: 0.10
Nodes (20): dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/conformance-cli, @napplet/vite-plugin, vite, @napplet/conformance-cli (+12 more)

### Community 59 - "package.json"
Cohesion: 0.11
Nodes (17): dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/vite-plugin, vite, @napplet/nap, @napplet/shim (+9 more)

### Community 60 - "profile-open-v1.schema.json"
Cohesion: 0.13
Nodes (14): additionalProperties, $id, properties, pubkey, version, pattern, type, required (+6 more)

### Community 61 - "Assumption validation and decision gates"
Cohesion: 0.22
Nodes (9): Assumption validation and decision gates, Current observed baseline, Gate 0 decision, Gate matrix, Hard stops, Per-slice validation, Plan correction rule, Required outputs (+1 more)

### Community 62 - "Slice 02 preflight"
Cohesion: 0.22
Nodes (9): Commands and observed results, Exact dependency and asset record, Next step, Preserved failed Fedora probe, Required design correction, Runtime evidence, Slice 02 preflight, Upstream result (+1 more)

### Community 63 - "Work 00 — validate assumptions"
Cohesion: 0.25
Nodes (7): Done when, Goal, Outputs, Read, Stop conditions, Tasks, Work 00 — validate assumptions

### Community 64 - "main.js"
Cohesion: 0.15
Nodes (9): results, target, nativeSurface(), PROBE_NAMES, sentinelTargets(), workerLoad(), LoadedWorker, RejectedWorker (+1 more)

### Community 66 - "Work 02 — Linux exact-build runner"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 02 — Linux exact-build runner

### Community 67 - "Work 04 — daemon, NMP, and persistence"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 04 — daemon, NMP, and persistence

### Community 68 - "Upstream contribution ledger"
Cohesion: 0.33
Nodes (6): Active contributions, Authority and ownership, Entry template, Slice 02 upstream result, Slice 03 upstream result, Upstream contribution ledger

### Community 69 - "Execution slices"
Cohesion: 0.33
Nodes (6): Dependency graph, Execution slices, Handoff, Parallel work, Slice entry gate, Slices

### Community 70 - "Uzel single-repository POC"
Cohesion: 0.33
Nodes (6): Demo result, Document map, Mandatory first step, Scope rule, Start order, Uzel single-repository POC

### Community 71 - "Work 03 — portable napplets"
Cohesion: 0.33
Nodes (6): Acceptance, Entry status and pins, Goal, Read, Tasks, Work 03 — portable napplets

### Community 72 - "POC status"
Cohesion: 0.40
Nodes (5): Accepted provisional risks, Gate 0 — validated baseline, Implementation, Latest integrated evidence, POC status

### Community 78 - "Work 05 — composed demo"
Cohesion: 0.33
Nodes (5): Acceptance, Goal, Non-goals, Tasks, Work 05 — composed demo

### Community 79 - "Work 06 — hardening and demo acceptance"
Cohesion: 0.40
Nodes (4): Acceptance, Goal, Tasks, Work 06 — hardening and demo acceptance

## Knowledge Gaps
- **346 isolated node(s):** `name`, `private`, `version`, `type`, `dev:web` (+341 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **26 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Slice 03 preflight` connect `Work 00 — validate assumptions` to `04-execution.md`?**
  _High betweenness centrality (0.009) - this node is a cross-community bridge._
- **Why does `Uzel POC agent instructions` connect `Uzel POC agent instructions` to `POC documentation audit`?**
  _High betweenness centrality (0.008) - this node is a cross-community bridge._
- **What connects `name`, `private`, `version` to the rest of the system?**
  _346 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `check-napplet-imports.mjs` be split into smaller, more focused modules?**
  _Cohesion score 0.08866995073891626 - nodes in this community are weakly interconnected._
- **Should `Work 04 — daemon, NMP, and persistence` be split into smaller, more focused modules?**
  _Cohesion score 0.11553030303030302 - nodes in this community are weakly interconnected._
- **Should `Uzel single-repository POC` be split into smaller, more focused modules?**
  _Cohesion score 0.08333333333333333 - nodes in this community are weakly interconnected._
- **Should `POC status` be split into smaller, more focused modules?**
  _Cohesion score 0.08333333333333333 - nodes in this community are weakly interconnected._