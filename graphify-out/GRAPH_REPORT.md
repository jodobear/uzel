# Graph Report - uzel  (2026-07-29)

## Corpus Check
- 110 files · ~59,175 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 952 nodes · 1398 edges · 94 communities (68 shown, 26 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 10 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7b204947`
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
- Slice 04 preflight — daemon, NMP, and persistence
- Work 04 — daemon, NMP, and persistence
- .fetch
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
- main.rs
- App.svelte
- Post-POC extraction
- lib.rs
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
- lib.rs
- Upstream contribution ledger
- Execution slices
- Uzel single-repository POC
- Work 03 — portable napplets
- POC status
- build-signed-napplet-fixtures.sh
- FACT-010-portable-napplets.md
- Work 05 — composed demo
- Work 06 — hardening and demo acceptance
- Work 06 — hardening and demo acceptance
- AcceptSettings
- RelayDiagnosticsSink
- FACT-011-daemon-nmp.md
- POC architecture
- FACT-012-composed-demo.md
- README.md
- Verified facts
- debian13-live-test.sh
- Gate 0 preflight and Slice 01 decision
- debian13-setup.sh
- Slice 01 preflight
- Work 01 — scaffold
- Work 02 — Linux exact-build runner

## God Nodes (most connected - your core abstractions)
1. `LinuxRunner` - 39 edges
2. `RunnerError` - 23 edges
3. `scripts` - 17 edges
4. `UnixClient` - 16 edges
5. `compilerOptions` - 15 edges
6. `HostileProbeState` - 14 edges
7. `Response` - 14 edges
8. `parse_options()` - 12 edges
9. `LiveProbe` - 12 edges
10. `write_frame()` - 11 edges

## Surprising Connections (you probably didn't know these)
- `runtime_diagnostics()` --references--> `Diagnostics`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `project_surface()` --references--> `FetchedSurface`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `forward_surface_envelope()` --references--> `RoutedEnvelope`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `runtime_status()` --references--> `UnixClient`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `select_read_identity()` --references--> `UnixClient`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs

## Import Cycles
- None detected.

## Communities (94 total, 26 thin omitted)

### Community 0 - "POC scope and acceptance"
Cohesion: 0.18
Nodes (10): Architectural invariants, Demo-complete, Foundation-complete, Non-goals, Objective, POC scope and acceptance, Required napplets, Stop rule (+2 more)

### Community 1 - "Assumption validation and decision gates"
Cohesion: 0.25
Nodes (18): boundedJSON(), compatibilityPreludeSource(), decodedBase64Length(), decodeResourceBlob(), exactFields(), isBoundedEnvelope(), isPlainObject(), isVerifiedArtifactBaseURL() (+10 more)

### Community 2 - "check-napplet-imports.mjs"
Cohesion: 0.07
Nodes (33): allowedDependencyTargets, allowedGuardedGlobalAccesses, { compile: compileSvelte, parse: parseSvelte }, contractsRoot, declarativeNetworkViolations(), dependencyGroups, dependencyViolations(), directNetworkIdentifiers (+25 more)

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
Cohesion: 0.21
Nodes (12): cleanup(), GDK_BACKEND, hostile_markers_are_ordered(), NO_AT_BRIDGE, preserve_failure(), preserve_logs(), linux-run-smoke.sh script, UZEL_FIXTURE_RELAY_PORT (+4 more)

### Community 7 - "Observed facts"
Cohesion: 0.22
Nodes (9): Assumption validation and decision gates, Current observed baseline, Gate 0 decision, Gate matrix, Hard stops, Per-slice validation, Plan correction rule, Required outputs (+1 more)

### Community 8 - "POC documentation audit"
Cohesion: 0.21
Nodes (4): Uzel agent instructions, Decision, Source baseline, FACT-XXX — title

### Community 9 - "Work 01 — scaffold"
Cohesion: 0.22
Nodes (9): Commands and observed results, Exact dependency and asset record, Next step, Preserved failed Fedora probe, Required design correction, Runtime evidence, Slice 02 preflight, Upstream result (+1 more)

### Community 10 - "Work 00 — validate assumptions"
Cohesion: 0.20
Nodes (10): API and ownership evidence, Automated review corrections, Commands and observed results, Exact pins and fixtures, Hostile fixture scope, Manifest correction, Next step, Slice 03 preflight (+2 more)

### Community 11 - "Work 00 — validate assumptions"
Cohesion: 0.11
Nodes (32): active_daemon_socket_is_not_unlinked(), AssetTransfer, bounded_detail(), daemon_routes_inc_delivery_to_the_other_exact_surface(), daemon_serves_ordered_verified_asset_and_shuts_down(), DaemonServer, DaemonState, exchange() (+24 more)

### Community 12 - "Slice 04 preflight — daemon, NMP, and persistence"
Cohesion: 0.25
Nodes (7): Commands and results, Exact next step, NMP and persistence evidence, Outcome, Private protocol evidence, Remaining boundaries, Slice 04 preflight — daemon, NMP, and persistence

### Community 13 - "Work 04 — daemon, NMP, and persistence"
Cohesion: 0.12
Nodes (24): isCanonicalPubkey(), parseProfileOpen(), profileOpen(), PUBKEY, list, render(), start(), status (+16 more)

### Community 14 - ".fetch"
Cohesion: 0.25
Nodes (7): ArtifactFetchRequest, ArtifactFetchResponse, ArtifactSource, ExactFixtureSource, fixture_by_name(), FixtureDefinition, Option

### Community 15 - "Uzel single-repository POC"
Cohesion: 0.08
Nodes (23): compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, isolatedModules, lib, module (+15 more)

### Community 16 - "POC status"
Cohesion: 0.08
Nodes (23): dependencies, svelte, @tauri-apps/api, devDependencies, svelte-check, @sveltejs/vite-plugin-svelte, typescript, vite (+15 more)

### Community 17 - "Work 05 — composed demo"
Cohesion: 0.06
Nodes (34): fallow, @napplet/cli, devDependencies, fallow, @napplet/cli, engines, node, name (+26 more)

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
Cohesion: 0.25
Nodes (6): 1. Clone and install the launcher, 2. Run automated real-WebKit acceptance, 3. Run visible desktop demo, Debian 13 live test, Development, Uzel

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
Cohesion: 0.50
Nodes (3): fedora-run-smoke.sh script, UZEL_SMOKE_NAME, UZEL_SMOKE_SUCCESS_MARKER

### Community 37 - "main.rs"
Cohesion: 0.27
Nodes (17): default_runtime_root(), default_socket_path(), live_relays_require_explicit_live_mode(), main(), next_path(), next_value(), Options, parse_options() (+9 more)

### Community 40 - "Post-POC extraction"
Cohesion: 0.25
Nodes (7): First hardening follow-ups, Likely `kehto/napd`, Moves to `jodobear/napplets`, POC shortcuts that must not silently become platform contracts, Post-POC extraction, Remains in Uzel, Rewrite criteria

### Community 41 - "lib.rs"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 04 — daemon, NMP, and persistence

### Community 47 - "lib.rs"
Cohesion: 0.06
Nodes (53): Child, Condvar, bounded_diagnostic(), EventBuffer, EventSink, eventually_identity_query(), hostile_probe_commits_exact_session_config_before_returning(), identity_query() (+45 more)

### Community 48 - "POC documentation audit"
Cohesion: 0.25
Nodes (7): Confidence, Corrections, Diagram review, Gate 0 resolution, POC documentation audit, Problems found in the previous pack, Verdict

### Community 49 - "ExactFixtureSource"
Cohesion: 0.10
Nodes (20): dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/conformance-cli, @napplet/vite-plugin, vite, @napplet/conformance-cli (+12 more)

### Community 50 - "trusted-shell-policy.js"
Cohesion: 0.83
Nodes (3): directive(), innerPolicyContent(), outerPolicyContent()

### Community 51 - "Window"
Cohesion: 0.24
Nodes (5): createSurfaceHost(), mount(), receive(), unmount(), Window

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
Nodes (8): Commands and results, Exact dependency and upstream evidence, Exact next step, Honest boundary, Linux shell evidence, Outcome, Runtime composition evidence, Slice 05 preflight — integrated composed demo

### Community 62 - "Slice 02 preflight"
Cohesion: 0.12
Nodes (23): accepted_report(), BeaconAttempt, control_accept_is_not_counted_as_a_probe_connection(), HostileProbeReport, HostileProbeState, HostileProbeVerdict, later_loopback_connection_is_counted_separately(), LiveProbe (+15 more)

### Community 63 - "Work 00 — validate assumptions"
Cohesion: 0.25
Nodes (7): Done when, Goal, Outputs, Read, Stop conditions, Tasks, Work 00 — validate assumptions

### Community 64 - "main.js"
Cohesion: 0.13
Nodes (12): denied(), results, target, attemptRawWebKitInvoke(), boundedAttempt(), nativeSurface(), PROBE_NAMES, sentinelTargets() (+4 more)

### Community 66 - "Work 02 — Linux exact-build runner"
Cohesion: 0.22
Nodes (8): Bubblewrap decision, Commands and results, Exact hostile evidence, Failed evidence and toolchain limit, Go/no-go and exact next steps, Outcome, Slice 06 preflight — hardening and clean demo acceptance, Upstream result

### Community 67 - "lib.rs"
Cohesion: 0.12
Nodes (29): ClientError, decode_asset_chunk(), Diagnostics, encode_asset_chunk(), FetchedSurface, frames_round_trip_with_big_endian_length(), maximum_asset_chunk_fits_control_frame(), oversized_frame_is_rejected_before_body_read() (+21 more)

### Community 68 - "Upstream contribution ledger"
Cohesion: 0.29
Nodes (7): Active contributions, Authority and ownership, Entry template, Slice 02 upstream result, Slice 03 upstream result, Slice 06 upstream result, Upstream contribution ledger

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
Cohesion: 0.16
Nodes (29): allowed_navigation(), default_socket_path(), finish_hostile_probe(), forward_surface_envelope(), hostile_native_probe(), main(), navigation_policy(), project_surface() (+21 more)

### Community 80 - "Work 06 — hardening and demo acceptance"
Cohesion: 0.40
Nodes (4): Acceptance, Goal, Tasks, Work 06 — hardening and demo acceptance

### Community 81 - "AcceptSettings"
Cohesion: 0.38
Nodes (5): AcceptSettings, UnavailableSettings, NativeSettingsExecutor, NativeSettingsOpenResult, NativeSettingsRequest

### Community 82 - "RelayDiagnosticsSink"
Cohesion: 0.50
Nodes (3): RelayDiagnosticsSink, RuntimeRelayDiagnosticsObserver, RuntimeRelayDiagnosticsSnapshot

### Community 84 - "POC architecture"
Cohesion: 0.22
Nodes (9): Accepted upstream seam, Composition flow, POC architecture, Repository zones, Runtime topology, Session start, Shared Nostr flow, Trust domains (+1 more)

### Community 87 - "Verified facts"
Cohesion: 0.29
Nodes (7): Kehto #204, `nampplets`, NAP registry, Napplet packages, NIP-5A and NIP-5D, NMP, Verified facts

### Community 88 - "debian13-live-test.sh"
Cohesion: 0.36
Nodes (7): fail(), reexec_with_nix_group(), debian13-live-test.sh script, UZEL_SMOKE_ARTIFACT_DIR, UZEL_SMOKE_EVIDENCE_DIR, UZEL_SMOKE_NAME, UZEL_SMOKE_SUCCESS_MARKER

### Community 89 - "Gate 0 preflight and Slice 01 decision"
Cohesion: 0.22
Nodes (9): Accepted provisional risks, Confirmed assumptions, Decision, Exact next steps, Gate 0 preflight and Slice 01 decision, Gate results, Rejected assumptions, Required design changes (+1 more)

### Community 90 - "debian13-setup.sh"
Cohesion: 0.46
Nodes (7): fail(), print_nix_builder_state(), print_nix_daemon_state(), refresh_nix_builder_state(), refresh_nix_daemon_state(), debian13-setup.sh script, wait_for_nix_daemon_socket()

### Community 91 - "Slice 01 preflight"
Cohesion: 0.25
Nodes (8): Boundary result, Commands and observed results, Debian probe correction, Fedora probe correction, Locked workspace, Next step, Slice 01 preflight, Verdict

### Community 92 - "Work 01 — scaffold"
Cohesion: 0.25
Nodes (8): Acceptance, Depends on, Entry status, Goal, Non-goals, Status, Tasks, Work 01 — scaffold

### Community 93 - "Work 02 — Linux exact-build runner"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 02 — Linux exact-build runner

## Knowledge Gaps
- **393 isolated node(s):** `name`, `private`, `version`, `type`, `dev:web` (+388 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **26 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `LinuxRunner` connect `lib.rs` to `Work 00 — validate assumptions`?**
  _High betweenness centrality (0.018) - this node is a cross-community bridge._
- **Why does `UnixClient` connect `Work 06 — hardening and demo acceptance` to `lib.rs`?**
  _High betweenness centrality (0.009) - this node is a cross-community bridge._
- **What connects `name`, `private`, `version` to the rest of the system?**
  _393 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `check-napplet-imports.mjs` be split into smaller, more focused modules?**
  _Cohesion score 0.06543385490753911 - nodes in this community are weakly interconnected._
- **Should `Work 00 — validate assumptions` be split into smaller, more focused modules?**
  _Cohesion score 0.10685249709639953 - nodes in this community are weakly interconnected._
- **Should `Work 04 — daemon, NMP, and persistence` be split into smaller, more focused modules?**
  _Cohesion score 0.11553030303030302 - nodes in this community are weakly interconnected._
- **Should `Uzel single-repository POC` be split into smaller, more focused modules?**
  _Cohesion score 0.08333333333333333 - nodes in this community are weakly interconnected._