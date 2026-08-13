# Graph Report - uzel-phase-01-replay-seam  (2026-08-13)

## Corpus Check
- 151 files · ~113,371 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1523 nodes · 2504 edges · 126 communities (98 shown, 28 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 26 edges (avg confidence: 0.72)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `238c8ca9`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- main.rs
- main.js
- UnixClient
- server.rs
- ref-candidate-check.py
- acceptance.test.mjs
- resource.rs
- check-napplet-imports.mjs
- Uzel
- scripts
- package.json
- runner.rs
- compilerOptions
- probes.test.mjs
- package.json
- package.json
- Pattern Assignments
- linux-run-smoke.sh
- trusted-shell.js
- Architecture
- parse_options
- .forward_from_surface
- package.json
- tauri.conf.json
- RunnerError
- EventBuffer
- profile-open-v1.schema.json
- LinuxRunner
- Requirements: Uzel
- debian13-live-test.sh
- mock-native.js
- String
- Phase 1: POC replay and accepted Napp seam - Context
- POC scope and acceptance
- 04-execution.md
- fixtures.rs
- Codebase Concerns
- README.md
- createSurfaceHost
- Coding Conventions
- Testing Patterns
- Provisional component design
- Tests, quality gates, and demo
- Slice 03 preflight
- External Integrations
- Lean process reset
- Uzel POC agent instructions
- Assumption validation and decision gates
- POC architecture
- Gate 0 preflight and Slice 01 decision
- Slice 02 preflight
- Slice 05 preflight — integrated composed demo
- Slice 06 preflight — hardening and clean demo acceptance
- App.svelte
- default.json
- Technology Stack
- Codebase Structure
- debian13-setup.sh
- POC documentation audit
- Post-POC extraction
- nampplets Linux reuse map
- Slice 01 preflight
- Slice 04 preflight — daemon, NMP, and persistence
- WebKit/Tauri trust spike
- Work 01 — scaffold
- AcceptSettings
- run
- Verified facts
- Upstream contribution ledger
- NMP API and ownership map
- audit_docs.py
- POC status
- Work 00 — validate assumptions
- Work 02 — Linux exact-build runner
- Work 04 — daemon, NMP, and persistence
- Work 07 — issue-driven stabilization
- Lean process reset summary
- 01-01-PLAN.md
- Execution slices
- Uzel single-repository POC
- Work 03 — portable napplets
- Work 05 — composed demo
- Work 06 — hardening and demo acceptance
- trusted-shell-policy.js
- projection-failure.js
- linux-smoke-script.test.mjs
- RelayDiagnosticsSink
- Lean process reset verification
- fedora-run-smoke.sh
- check-pinned-assets.sh
- debian-build-smoke.sh
- dev.sh
- README.md
- README.md
- README.md
- README.md
- STATE.md
- build-signed-napplet-fixtures.sh
- check-boundaries.sh
- smoke.sh
- FACT-001-kehto-package-line.md
- FACT-002-spec-revisions.md
- FACT-003-nampplets-linux-reuse.md
- FACT-004-nmp-facade.md
- FACT-005-webkit-tauri-trust.md
- FACT-006-csp-egress.md
- FACT-007-local-ipc.md
- FACT-008-toolchain.md
- FACT-009-linux-runner.md
- FACT-010-portable-napplets.md
- FACT-011-daemon-nmp.md
- FACT-012-composed-demo.md
- FACT-013-hostile-linux-boundary.md
- FACT-014-live-identity-catalog.md
- manifest.json
- Phase 01 Plan 01: POC replay and accepted Napp seam Summary
- Napp external prerequisite
- Phase 01 POC replay
- Phase 01 ownership map

## God Nodes (most connected - your core abstractions)
1. `LinuxRunner` - 44 edges
2. `UnixClient` - 38 edges
3. `RunnerError` - 29 edges
4. `ClientError` - 24 edges
5. `Response` - 22 edges
6. `scripts` - 21 edges
7. `validate_record()` - 21 edges
8. `CheckError` - 18 edges
9. `HostileProbeState` - 17 edges
10. `write_frame()` - 17 edges

## Surprising Connections (you probably didn't know these)
- `ConfirmNappletError` --references--> `ClientError`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `ReviewNappletError` --references--> `ClientError`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `read_runtime_status()` --references--> `UnixClient`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `runtime_status()` --references--> `UnixClient`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `reconcile_runtime()` --references--> `UnixClient`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs

## Import Cycles
- None detected.

## Communities (126 total, 28 thin omitted)

### Community 0 - "main.rs"
Cohesion: 0.06
Nodes (72): accepted_report(), BeaconAttempt, control_accept_is_not_counted_as_a_probe_connection(), exact_surface_cancellation_retires_the_attached_probe(), HostileProbeReport, HostileProbeState, HostileProbeVerdict, later_loopback_connection_is_counted_separately() (+64 more)

### Community 1 - "main.js"
Cohesion: 0.06
Nodes (69): canonicalProfile(), canonicalProfiles(), optionalText(), profileQueryBatches(), profileQueryRequest(), retryProfileQueryRequests(), splitProfileQueryRequest(), A (+61 more)

### Community 2 - "UnixClient"
Cohesion: 0.08
Nodes (49): authoritative_reconciliation_retires_ambiguous_operation_ids(), CatalogCapability, chunked_routed_envelope_reassembles_on_one_connection(), ClientError, confirm_responses_lost_replay_surface_without_a_second_operation(), decode_asset_chunk(), DeliveryError, deterministic_presend_failures_do_not_retain_catalog_operations() (+41 more)

### Community 3 - "server.rs"
Cohesion: 0.09
Nodes (43): Request, active_daemon_socket_is_not_unlinked(), AssetTransfer, bounded_detail(), daemon_routes_inc_delivery_to_the_other_exact_surface(), daemon_serves_ordered_verified_asset_and_shuts_down(), DaemonServer, DaemonState (+35 more)

### Community 4 - "ref-candidate-check.py"
Cohesion: 0.15
Nodes (51): Any, Namespace, RuntimeError, approved_reachability(), candidate_argv_ok(), candidate_record(), canonical_record(), CheckError (+43 more)

### Community 5 - "acceptance.test.mjs"
Cohesion: 0.07
Nodes (40): ACTION_IDS, bindingFromEvent(), bindingMatches(), DEFAULT_KEYBINDINGS, defaultPreferences(), KEYBINDING_ACTIONS, parsePreferences(), validateKeybindings() (+32 more)

### Community 6 - "resource.rs"
Cohesion: 0.09
Nodes (34): Cancellation, Client, cancellation_signal(), cancelled_resolution_returns_without_network_work(), linux_resource_provider(), LinuxResourceNetwork, map_reqwest_error(), MonotonicResourceClock (+26 more)

### Community 7 - "check-napplet-imports.mjs"
Cohesion: 0.06
Nodes (37): allowedDependencyTargets, allowedGuardedGlobalAccesses, { compile: compileSvelte, parse: parseSvelte }, contractsRoot, declarativeNetworkViolations(), dependencyGroups, dependencyViolations(), directNetworkIdentifiers (+29 more)

### Community 8 - "Uzel"
Cohesion: 0.05
Nodes (33): Uzel agent instructions, Evidence boundary, Renderer acceptance harness, 1. Clone and install the launcher, 2. Run automated real-WebKit acceptance, 3. Run visible desktop demo, Debian 13 live test, Active (+25 more)

### Community 9 - "scripts"
Cohesion: 0.05
Nodes (38): fallow, @napplet/cli, devDependencies, fallow, @napplet/cli, engines, node, name (+30 more)

### Community 10 - "package.json"
Cohesion: 0.07
Nodes (27): dependencies, svelte, @tauri-apps/api, devDependencies, playwright, svelte-check, @sveltejs/vite-plugin-svelte, typescript (+19 more)

### Community 11 - "runner.rs"
Cohesion: 0.31
Nodes (7): buffered_responses_are_byte_bounded_and_consumed_once(), BufferedEvents, event_bytes(), response_event(), VecDeque, RuntimeEvent, RuntimeObservationFrame

### Community 12 - "compilerOptions"
Cohesion: 0.08
Nodes (23): compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, isolatedModules, lib, module (+15 more)

### Community 13 - "probes.test.mjs"
Cohesion: 0.13
Nodes (12): denied(), results, target, attemptRawWebKitInvoke(), boundedAttempt(), nativeSurface(), PROBE_NAMES, sentinelTargets() (+4 more)

### Community 14 - "package.json"
Cohesion: 0.10
Nodes (20): dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/conformance-cli, @napplet/vite-plugin, vite, @napplet/conformance-cli (+12 more)

### Community 15 - "package.json"
Cohesion: 0.10
Nodes (20): dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/conformance-cli, @napplet/vite-plugin, vite, @napplet/conformance-cli (+12 more)

### Community 16 - "Pattern Assignments"
Cohesion: 0.10
Nodes (17): Napp Candidate Qualification, Napp Dependency Handoff, Evidence isolation and retention, `evidence/phase-01/candidate-qualification.md` and `evidence/phase-01/napp-dependency.md` (evidence records / transform), `evidence/phase-01/ownership-map.md` (ownership report / transform), `evidence/phase-01/poc-replay.md` (evidence report / batch), Exact source binding, Fail closed + narrow errors (+9 more)

### Community 17 - "linux-run-smoke.sh"
Cohesion: 0.16
Nodes (17): cleanup(), GDK_BACKEND, hostile_markers_are_ordered(), NO_AT_BRIDGE, preserve_failure(), preserve_logs(), report_marker(), report_marker_state() (+9 more)

### Community 18 - "trusted-shell.js"
Cohesion: 0.25
Nodes (18): boundedJSON(), compatibilityPreludeSource(), decodedBase64Length(), decodeResourceBlob(), exactFields(), isBoundedEnvelope(), isPlainObject(), isVerifiedArtifactBaseURL() (+10 more)

### Community 19 - "Architecture"
Cohesion: 0.11
Nodes (18): Anti-Patterns, Architectural Constraints, Architecture, Catalog Review and Exact-Build Install, Component Responsibilities, Cross-Cutting Concerns, Cross-Napplet Profile Flow, Data Flow (+10 more)

### Community 20 - "parse_options"
Cohesion: 0.27
Nodes (17): default_runtime_root(), default_socket_path(), live_configuration_requires_explicit_live_mode(), main(), next_path(), next_value(), Options, parse_options() (+9 more)

### Community 21 - ".forward_from_surface"
Cohesion: 0.11
Nodes (21): absent_surface_cleanup_is_idempotent(), catalog_cancellation_is_terminal(), catalog_cancellation_keeps_retryable_reviews_and_discards_terminal_stale_tokens(), eventually_identity_query(), hostile_probe_commits_exact_session_config_before_returning(), identity_query(), inc_emit_waits_for_an_inc_event_not_an_unrelated_push(), launch_identity_surface() (+13 more)

### Community 22 - "package.json"
Cohesion: 0.11
Nodes (17): dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/vite-plugin, vite, @napplet/nap, @napplet/shim (+9 more)

### Community 23 - "tauri.conf.json"
Cohesion: 0.12
Nodes (16): app, security, windows, build, beforeBuildCommand, beforeDevCommand, devUrl, frontendDist (+8 more)

### Community 24 - "RunnerError"
Cohesion: 0.18
Nodes (13): BTreeSet, artifact_base_url(), read_launched_document(), reconcile_launched_session(), Error, Result, RunnerError, StatePersistFailure (+5 more)

### Community 25 - "EventBuffer"
Cohesion: 0.20
Nodes (11): Condvar, bounded_diagnostic(), EventBuffer, EventSink, ProductState, Arc, Mutex, Option (+3 more)

### Community 26 - "profile-open-v1.schema.json"
Cohesion: 0.13
Nodes (14): additionalProperties, $id, properties, pubkey, version, pattern, type, required (+6 more)

### Community 27 - "LinuxRunner"
Cohesion: 0.18
Nodes (9): LinuxRunner, BTreeMap, Drop, PathBuf, RuntimeMode, Debug, Formatter, RuntimeObservation (+1 more)

### Community 28 - "Requirements: Uzel"
Cohesion: 0.13
Nodes (14): Blossom and Authoring, Canonical Nix Package, Definition of Done, Later Platforms and Capabilities, Lean Delivery and Review, Local Files, Out of Scope, POC Reference and Napp Seam (+6 more)

### Community 29 - "debian13-live-test.sh"
Cohesion: 0.20
Nodes (13): CARGO_INCREMENTAL, CARGO_PROFILE_DEV_DEBUG, fail(), record_prebuild(), reexec_with_nix_group(), run_cache_probe(), run_startup_step(), debian13-live-test.sh script (+5 more)

### Community 30 - "mock-native.js"
Cohesion: 0.24
Nodes (9): diagnostics(), invoke(), isRoutedProfileQuery(), nativeEnvelope(), profileEvent(), profileEventsForQuery(), profileFor(), review() (+1 more)

### Community 31 - "String"
Cohesion: 0.33
Nodes (6): public_naddr_reviews_confirms_and_launches_exact_single_file(), AsRef, Path, Self, String, Vec

### Community 32 - "Phase 1: POC replay and accepted Napp seam - Context"
Cohesion: 0.14
Nodes (13): Accepted behavior and evidence, Active authority, Bounded replay evidence, not active workflow authority, Canonical References, Deferred Ideas, Delivery, Existing Phase 1 evidence, Implementation Decisions (+5 more)

### Community 33 - "POC scope and acceptance"
Cohesion: 0.17
Nodes (11): Accepted post-foundation extension, Architectural invariants, Demo-complete, Foundation-complete, Non-goals, Objective, POC scope and acceptance, Required napplets (+3 more)

### Community 35 - "fixtures.rs"
Cohesion: 0.20
Nodes (7): ArtifactFetchRequest, ArtifactFetchResponse, ArtifactSource, ExactFixtureSource, fixture_by_name(), FixtureDefinition, Option

### Community 36 - "Codebase Concerns"
Cohesion: 0.18
Nodes (10): Codebase Concerns, Dependencies at Risk, Fragile Areas, Known Bugs, Missing Critical Features, Performance Bottlenecks, Scaling Limits, Security Considerations (+2 more)

### Community 37 - "README.md"
Cohesion: 0.25
Nodes (3): Decision, Source baseline, FACT-XXX — title

### Community 38 - "createSurfaceHost"
Cohesion: 0.24
Nodes (5): createSurfaceHost(), mount(), receive(), unmount(), Window

### Community 39 - "Coding Conventions"
Cohesion: 0.20
Nodes (9): Code Style, Coding Conventions, Comments, Error Handling, Function Design, Import Organization, Logging, Module Design (+1 more)

### Community 40 - "Testing Patterns"
Cohesion: 0.20
Nodes (9): Common Patterns, Coverage, Fixtures and Factories, Mocking, Test File Organization, Test Framework, Test Structure, Test Types (+1 more)

### Community 41 - "Provisional component design"
Cohesion: 0.20
Nodes (9): Developer mode, Exact-build fixtures, Identity and Nostr reads, Local daemon protocol, Napplet convention, Provisional component design, Runtime state, Shell UI (+1 more)

### Community 42 - "Tests, quality gates, and demo"
Cohesion: 0.20
Nodes (9): Deterministic demo, Final acceptance, Hostile frame, Live demo, Napplet/web, Quality commands, Required test layers, Runtime/Rust (+1 more)

### Community 43 - "Slice 03 preflight"
Cohesion: 0.20
Nodes (10): API and ownership evidence, Automated review corrections, Commands and observed results, Exact pins and fixtures, Hostile fixture scope, Manifest correction, Next step, Slice 03 preflight (+2 more)

### Community 44 - "External Integrations"
Cohesion: 0.22
Nodes (8): APIs & External Services, Authentication & Identity, CI/CD & Deployment, Data Storage, Environment Configuration, External Integrations, Monitoring & Observability, Webhooks & Callbacks

### Community 45 - "Lean process reset"
Cohesion: 0.22
Nodes (8): Acceptance checks, Appetite, Boundaries, Dependencies, Lean process reset, No-gos, Outcome, Risks

### Community 46 - "Uzel POC agent instructions"
Cohesion: 0.22
Nodes (9): Engineering rules, Mission, POC exclusions, Quality gate, Repository boundaries, Required method, Trust rules, Upstream contribution policy (+1 more)

### Community 47 - "Assumption validation and decision gates"
Cohesion: 0.22
Nodes (9): Assumption validation and decision gates, Current observed baseline, Gate 0 decision, Gate matrix, Hard stops, Per-slice validation, Plan correction rule, Required outputs (+1 more)

### Community 48 - "POC architecture"
Cohesion: 0.22
Nodes (9): Accepted upstream seam, Composition flow, POC architecture, Repository zones, Runtime topology, Session start, Shared Nostr flow, Trust domains (+1 more)

### Community 49 - "Gate 0 preflight and Slice 01 decision"
Cohesion: 0.22
Nodes (9): Accepted provisional risks, Confirmed assumptions, Decision, Exact next steps, Gate 0 preflight and Slice 01 decision, Gate results, Rejected assumptions, Required design changes (+1 more)

### Community 50 - "Slice 02 preflight"
Cohesion: 0.22
Nodes (9): Commands and observed results, Exact dependency and asset record, Next step, Preserved failed Fedora probe, Required design correction, Runtime evidence, Slice 02 preflight, Upstream result (+1 more)

### Community 51 - "Slice 05 preflight — integrated composed demo"
Cohesion: 0.22
Nodes (8): Commands and results, Exact dependency and upstream evidence, Exact next step, Honest boundary, Linux shell evidence, Outcome, Runtime composition evidence, Slice 05 preflight — integrated composed demo

### Community 52 - "Slice 06 preflight — hardening and clean demo acceptance"
Cohesion: 0.22
Nodes (8): Bubblewrap decision, Commands and results, Exact hostile evidence, Failed evidence and toolchain limit, Go/no-go and exact next steps, Outcome, Slice 06 preflight — hardening and clean demo acceptance, Upstream result

### Community 53 - "App.svelte"
Cohesion: 0.25
Nodes (3): @tauri-apps/api/core, ./preferences.js, ./projection-failure.js

### Community 54 - "default.json"
Cohesion: 0.25
Nodes (7): description, identifier, permissions, $schema, windows, core:default, main

### Community 55 - "Technology Stack"
Cohesion: 0.25
Nodes (7): Configuration, Frameworks, Key Dependencies, Languages, Platform Requirements, Runtime, Technology Stack

### Community 56 - "Codebase Structure"
Cohesion: 0.25
Nodes (7): Codebase Structure, Directory Layout, Directory Purposes, Key File Locations, Naming Conventions, Special Directories, Where to Add New Code

### Community 57 - "debian13-setup.sh"
Cohesion: 0.46
Nodes (7): fail(), print_nix_builder_state(), print_nix_daemon_state(), refresh_nix_builder_state(), refresh_nix_daemon_state(), debian13-setup.sh script, wait_for_nix_daemon_socket()

### Community 58 - "POC documentation audit"
Cohesion: 0.25
Nodes (7): Confidence, Corrections, Diagram review, Gate 0 resolution, POC documentation audit, Problems found in the previous pack, Verdict

### Community 59 - "Post-POC extraction"
Cohesion: 0.25
Nodes (7): First hardening follow-ups, Likely `kehto/napd`, Moves to `jodobear/napplets`, POC shortcuts that must not silently become platform contracts, Post-POC extraction, Remains in Uzel, Rewrite criteria

### Community 60 - "nampplets Linux reuse map"
Cohesion: 0.25
Nodes (7): Apple-only edges, Build evidence, Compatibility candidate, Crate map, nampplets Linux reuse map, Result, Runtime entry points

### Community 61 - "Slice 01 preflight"
Cohesion: 0.25
Nodes (8): Boundary result, Commands and observed results, Debian probe correction, Fedora probe correction, Locked workspace, Next step, Slice 01 preflight, Verdict

### Community 62 - "Slice 04 preflight — daemon, NMP, and persistence"
Cohesion: 0.25
Nodes (7): Commands and results, Exact next step, NMP and persistence evidence, Outcome, Private protocol evidence, Remaining boundaries, Slice 04 preflight — daemon, NMP, and persistence

### Community 63 - "WebKit/Tauri trust spike"
Cohesion: 0.25
Nodes (7): Accepted child CSP, Boundary of proof, Executable probe, Low-level handler nuance, Source verification, Verdict, WebKit/Tauri trust spike

### Community 64 - "Work 01 — scaffold"
Cohesion: 0.25
Nodes (8): Acceptance, Depends on, Entry status, Goal, Non-goals, Status, Tasks, Work 01 — scaffold

### Community 65 - "AcceptSettings"
Cohesion: 0.38
Nodes (5): AcceptSettings, UnavailableSettings, NativeSettingsExecutor, NativeSettingsOpenResult, NativeSettingsRequest

### Community 66 - "run"
Cohesion: 0.52
Nodes (5): init_repository(), MaintenanceTests, CompletedProcess, Path, run()

### Community 67 - "Verified facts"
Cohesion: 0.29
Nodes (7): Kehto #204, `nampplets`, NAP registry, Napplet packages, NIP-5A and NIP-5D, NMP, Verified facts

### Community 68 - "Upstream contribution ledger"
Cohesion: 0.29
Nodes (7): Active contributions, Authority and ownership, Entry template, Slice 02 upstream result, Slice 03 upstream result, Slice 06 upstream result, Upstream contribution ledger

### Community 69 - "NMP API and ownership map"
Cohesion: 0.29
Nodes (6): Accepted pin, Executable probe, nampplets adapter seam, NMP API and ownership map, Ownership boundary, Public facade

### Community 70 - "audit_docs.py"
Cohesion: 0.57
Nodes (6): audit(), main(), manifest_paths(), Path, strip_code(), write_manifest()

### Community 71 - "POC status"
Cohesion: 0.29
Nodes (6): Accepted provisional risks, Gate 0 — validated baseline, Implementation, Latest integrated evidence, POC status, Preserved evidence maintenance — 2026-08-13

### Community 72 - "Work 00 — validate assumptions"
Cohesion: 0.29
Nodes (7): Done when, Goal, Outputs, Read, Stop conditions, Tasks, Work 00 — validate assumptions

### Community 73 - "Work 02 — Linux exact-build runner"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 02 — Linux exact-build runner

### Community 74 - "Work 04 — daemon, NMP, and persistence"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 04 — daemon, NMP, and persistence

### Community 75 - "Work 07 — issue-driven stabilization"
Cohesion: 0.29
Nodes (7): Active issue — #19, Completed issue — #10, Entry evidence, Exit rule, Goal, Work 07 — issue-driven stabilization, Work graph

### Community 77 - "Lean process reset summary"
Cohesion: 0.33
Nodes (5): Changes, Commits, Lean process reset summary, Outcome, Validation

### Community 78 - "01-01-PLAN.md"
Cohesion: 0.20
Nodes (9): Appetite, Boundaries, Dependencies, Execution outputs, No-gos, Observable acceptance checks, Outcome, Phase 1 Plan 01: POC replay and accepted Napp seam (+1 more)

### Community 79 - "Execution slices"
Cohesion: 0.33
Nodes (6): Dependency graph, Execution slices, Handoff, Parallel work, Slice entry gate, Slices

### Community 80 - "Uzel single-repository POC"
Cohesion: 0.33
Nodes (6): Demo result, Document map, Mandatory first step, Scope rule, Start order, Uzel single-repository POC

### Community 81 - "Work 03 — portable napplets"
Cohesion: 0.33
Nodes (6): Acceptance, Entry status and pins, Goal, Read, Tasks, Work 03 — portable napplets

### Community 82 - "Work 05 — composed demo"
Cohesion: 0.33
Nodes (5): Acceptance, Goal, Non-goals, Tasks, Work 05 — composed demo

### Community 83 - "Work 06 — hardening and demo acceptance"
Cohesion: 0.40
Nodes (4): Acceptance, Goal, Tasks, Work 06 — hardening and demo acceptance

### Community 84 - "trusted-shell-policy.js"
Cohesion: 0.83
Nodes (3): directive(), innerPolicyContent(), outerPolicyContent()

### Community 86 - "linux-smoke-script.test.mjs"
Cohesion: 0.67
Nodes (3): matchesLog(), napdReadyPattern(), script

### Community 87 - "RelayDiagnosticsSink"
Cohesion: 0.50
Nodes (3): RelayDiagnosticsSink, RuntimeRelayDiagnosticsObserver, RuntimeRelayDiagnosticsSnapshot

### Community 88 - "Lean process reset verification"
Cohesion: 0.50
Nodes (3): External merge gate, Lean process reset verification, Local gate

### Community 89 - "fedora-run-smoke.sh"
Cohesion: 0.50
Nodes (3): fedora-run-smoke.sh script, UZEL_SMOKE_NAME, UZEL_SMOKE_SUCCESS_MARKER

### Community 126 - "Phase 01 Plan 01: POC replay and accepted Napp seam Summary"
Cohesion: 0.17
Nodes (11): Accomplishments, Decisions Made, Deviations from Plan, Files Created, Issues Encountered, Known Stubs, Next Phase Readiness, Performance (+3 more)

### Community 128 - "Napp external prerequisite"
Cohesion: 0.29
Nodes (6): External sibling observation, Fixed qualification, Handoff and inventory disposition, Missing admission categories, Napp external prerequisite, Required next authority

### Community 129 - "Phase 01 POC replay"
Cohesion: 0.33
Nodes (5): Exact source and pins, Phase 01 POC replay, REF-01 through REF-04 replay, REF-06 baseline, Unavailable or failed

### Community 131 - "Phase 01 ownership map"
Cohesion: 0.50
Nodes (3): Ownership and disposition, Phase 01 ownership map, Protected evidence disposition

## Knowledge Gaps
- **593 isolated node(s):** `name`, `private`, `version`, `type`, `dev:web` (+588 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **28 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `LinuxRunner` connect `LinuxRunner` to `UnixClient`, `server.rs`, `.forward_from_surface`, `RunnerError`, `EventBuffer`, `String`?**
  _High betweenness centrality (0.020) - this node is a cross-community bridge._
- **Why does `linux_resource_provider()` connect `resource.rs` to `String`?**
  _High betweenness centrality (0.014) - this node is a cross-community bridge._
- **Why does `NappletReview` connect `UnixClient` to `main.rs`, `LinuxRunner`, `String`?**
  _High betweenness centrality (0.013) - this node is a cross-community bridge._
- **What connects `name`, `private`, `version` to the rest of the system?**
  _593 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `main.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.057967313585291114 - nodes in this community are weakly interconnected._
- **Should `main.js` be split into smaller, more focused modules?**
  _Cohesion score 0.05759493670886076 - nodes in this community are weakly interconnected._
- **Should `UnixClient` be split into smaller, more focused modules?**
  _Cohesion score 0.0837245696400626 - nodes in this community are weakly interconnected._