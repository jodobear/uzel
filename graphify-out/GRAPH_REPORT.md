# Graph Report - .  (2026-08-13)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 1462 nodes · 2450 edges · 129 communities (95 shown, 34 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 26 edges (avg confidence: 0.72)
- Token cost: 7,923 input · 15,928 output

## Graph Freshness
- Built from commit: `d98d3404`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Protocol Error Handling
- trusted-shell.js
- Dependency and Network Identifiers
- LinuxRunner
- Hostile Probe Network Handling
- Identity and Session State
- Daemon Server Socket Management
- EventBuffer
- Protocol Asset Transfer Handling
- Asset Fetch and Delivery
- Runtime Failure Handling
- Request
- Architecture
- Profile Query Management
- fixtures.rs
- TypeScript Module Configuration
- Svelte TypeScript Development Setup
- Napplet Tooling and Core
- Project Planning and Risks
- tauri.conf.json
- Requirements: Uzel
- Core Configuration Defaults
- Application Preferences and Bindings
- debian13-live-test.sh
- Response
- String
- POC scope and acceptance
- 04-execution.md
- Codebase Concerns
- uzel-poc-validated-pack/README.md
- Projection Failure Handling
- Provisional component design
- Identity and State Management
- Process Summary and Validation
- Runtime Testing and Demos
- Verification and Gates
- Upstream Evidence and Verdict
- parse_options
- Application Code and Logic
- Buffered Event Handling
- External Integrations and APIs
- Engineering Rules and Policies
- Decision Gates and Validation
- Trust Domains and Flow
- Coding Style and Conventions
- Risk and Decision Management
- Runtime Error Handling
- Testing Patterns and Fixtures
- Napplet Build Dependencies
- Trusted Shell Policy
- Trusted Shell Surface Host
- Runtime Evidence and Results
- Slice Preflight and Evidence
- Slice Hardening and Go-No-Go
- Codebase Structure Layout
- Napplet Profile Build
- hostile-egress/package.json
- profile-open-v1.schema.json
- Technology Stack and Config
- Client Interaction and Cancellation
- Nix Setup Scripting
- Hostile Egress Probing
- Test Maintenance Process
- Linux Smoke Testing
- Documentation Audit Results
- Extraction and Hardening
- Linux Compatibility Map
- Probe Results and Boundaries
- Slice Preflight Outcomes
- Trust and Source Verification
- Task Status and Goals
- Napplet Package Registry
- Vite Follow List
- Hostile Egress Setup
- Profile Card Setup
- Upstream Contribution Ledger
- NMP API Mapping
- Documentation Auditing Paths
- Native Settings Execution
- Runtime Diagnostics Sink
- Validation and Stop Conditions
- Resource Cancellation and Client
- Linux Build Runner
- Daemon Persistence Goals
- Issue Stabilization Work
- Execution Slices and Handoff
- Order Start Process
- Implementation Status
- Task and Goal Tracking
- mock-native.js
- Integration and Demo Work
- Hardening and Acceptance
- Stream and Response Writing
- Graph Portability Check
- Fedora Smoke Testing
- Pinned Asset Checking
- Debian Build Smoke
- Development Cleanup Script
- Graphify Cache Refresh
- Trusted Shell Documentation
- Candidate Qualification Process
- Napp Dependency Handoff
- Signed Work Fixtures
- Project Fixtures
- Napplets Implementation
- Build Signed Napplet Fixtures
- Boundary Checks
- Smoke Testing
- Package Line Revisions
- Specification Revisions
- Linux Napplet Reuse
- NMP Facade Ownership
- WebKit Trust Authority
- CSP Egress Policy
- Local Daemon IPC
- Linux Toolchain
- Linux Build Runner
- Portable Napplets
- Daemon NMP Runtime
- Multi-Surface Composition
- Hostile Linux Boundary
- Identity Catalog Flow
- File Manifest Data
- Agent Testing and Setup
- Current Execution State

## God Nodes (most connected - your core abstractions)
1. `LinuxRunner` - 44 edges
2. `follow-list/src/main.js` - 40 edges
3. `src-tauri/src/main.rs` - 38 edges
4. `UnixClient` - 38 edges
5. `napd-protocol/src/lib.rs` - 32 edges
6. `RunnerError` - 29 edges
7. `ClientError` - 24 edges
8. `profile-card/src/main.js` - 24 edges
9. `Response` - 22 edges
10. `scripts` - 22 edges

## Surprising Connections (you probably didn't know these)
- `ConfirmNappletError` --references--> `ClientError`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `ReviewNappletError` --references--> `ClientError`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `runtime_diagnostics()` --references--> `Diagnostics`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `review_napplet()` --references--> `NappletReview`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `project_surface()` --references--> `FetchedSurface`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs

## Import Cycles
- None detected.

## Communities (129 total, 34 thin omitted)

### Community 0 - "Protocol Error Handling"
Cohesion: 0.38
Nodes (7): ProtocolError, read_exact_or_truncated(), read_frame(), Error, T, DecodeError, Read

### Community 1 - "trusted-shell.js"
Cohesion: 0.25
Nodes (18): boundedJSON(), compatibilityPreludeSource(), decodedBase64Length(), decodeResourceBlob(), exactFields(), isBoundedEnvelope(), isPlainObject(), isVerifiedArtifactBaseURL() (+10 more)

### Community 2 - "Dependency and Network Identifiers"
Cohesion: 0.06
Nodes (37): allowedDependencyTargets, allowedGuardedGlobalAccesses, { compile: compileSvelte, parse: parseSvelte }, contractsRoot, declarativeNetworkViolations(), dependencyGroups, dependencyViolations(), directNetworkIdentifiers (+29 more)

### Community 3 - "LinuxRunner"
Cohesion: 0.18
Nodes (9): LinuxRunner, BTreeMap, Drop, PathBuf, RuntimeMode, Debug, Formatter, RuntimeObservation (+1 more)

### Community 4 - "Hostile Probe Network Handling"
Cohesion: 0.12
Nodes (24): accepted_report(), BeaconAttempt, control_accept_is_not_counted_as_a_probe_connection(), exact_surface_cancellation_retires_the_attached_probe(), HostileProbeReport, HostileProbeState, HostileProbeVerdict, later_loopback_connection_is_counted_separately() (+16 more)

### Community 5 - "Identity and Session State"
Cohesion: 0.11
Nodes (21): absent_surface_cleanup_is_idempotent(), catalog_cancellation_is_terminal(), catalog_cancellation_keeps_retryable_reviews_and_discards_terminal_stale_tokens(), eventually_identity_query(), hostile_probe_commits_exact_session_config_before_returning(), identity_query(), inc_emit_waits_for_an_inc_event_not_an_unrelated_push(), launch_identity_surface() (+13 more)

### Community 6 - "Daemon Server Socket Management"
Cohesion: 0.15
Nodes (25): napd/src/lib.rs, active_daemon_socket_is_not_unlinked(), daemon_routes_inc_delivery_to_the_other_exact_surface(), daemon_serves_ordered_verified_asset_and_shuts_down(), DaemonServer, exchange(), existing_shared_socket_parent_is_not_chmodded(), incomplete_client_times_out_without_blocking_the_next_request() (+17 more)

### Community 7 - "EventBuffer"
Cohesion: 0.20
Nodes (11): Condvar, bounded_diagnostic(), EventBuffer, EventSink, ProductState, Arc, Mutex, Option (+3 more)

### Community 8 - "Protocol Asset Transfer Handling"
Cohesion: 0.15
Nodes (20): napd-protocol/src/lib.rs, authoritative_reconciliation_retires_ambiguous_operation_ids(), chunked_routed_envelope_reassembles_on_one_connection(), confirm_responses_lost_replay_surface_without_a_second_operation(), deterministic_presend_failures_do_not_retain_catalog_operations(), encode_asset_chunk(), failed_asset_transfer_stops_the_launched_surface(), frames_round_trip_with_big_endian_length() (+12 more)

### Community 9 - "Asset Fetch and Delivery"
Cohesion: 0.25
Nodes (7): ClientError, decode_asset_chunk(), DeliveryError, FetchedSurface, Box, Result, UnixStream

### Community 10 - "Runtime Failure Handling"
Cohesion: 0.16
Nodes (17): cleanup(), GDK_BACKEND, hostile_markers_are_ordered(), NO_AT_BRIDGE, preserve_failure(), preserve_logs(), report_marker(), report_marker_state() (+9 more)

### Community 11 - "Request"
Cohesion: 0.19
Nodes (15): Request, AssetTransfer, bounded_detail(), DaemonState, InvalidOperationId, replay_key(), ReplayCache, ReplayEntry (+7 more)

### Community 12 - "Architecture"
Cohesion: 0.11
Nodes (18): Anti-Patterns, Architectural Constraints, Architecture, Catalog Review and Exact-Build Install, Component Responsibilities, Cross-Cutting Concerns, Cross-Napplet Profile Flow, Data Flow (+10 more)

### Community 13 - "Profile Query Management"
Cohesion: 0.06
Nodes (75): canonicalProfile(), canonicalProfiles(), optionalText(), profileQueryBatches(), profileQueryRequest(), retryProfileQueryRequests(), splitProfileQueryRequest(), A (+67 more)

### Community 14 - "fixtures.rs"
Cohesion: 0.20
Nodes (7): ArtifactFetchRequest, ArtifactFetchResponse, ArtifactSource, ExactFixtureSource, fixture_by_name(), FixtureDefinition, Option

### Community 15 - "TypeScript Module Configuration"
Cohesion: 0.08
Nodes (23): compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, isolatedModules, lib, module (+15 more)

### Community 16 - "Svelte TypeScript Development Setup"
Cohesion: 0.07
Nodes (28): uzel/package.json, dependencies, svelte, @tauri-apps/api, devDependencies, playwright, svelte-check, @sveltejs/vite-plugin-svelte (+20 more)

### Community 17 - "Napplet Tooling and Core"
Cohesion: 0.05
Nodes (39): fallow, @napplet/cli, devDependencies, fallow, @napplet/cli, engines, node, name (+31 more)

### Community 18 - "Project Planning and Risks"
Cohesion: 0.22
Nodes (8): Acceptance checks, Appetite, Boundaries, Dependencies, Lean process reset, No-gos, Outcome, Risks

### Community 19 - "tauri.conf.json"
Cohesion: 0.12
Nodes (16): app, security, windows, build, beforeBuildCommand, beforeDevCommand, devUrl, frontendDist (+8 more)

### Community 20 - "Requirements: Uzel"
Cohesion: 0.13
Nodes (14): Blossom and Authoring, Canonical Nix Package, Definition of Done, Later Platforms and Capabilities, Lean Delivery and Review, Local Files, Out of Scope, POC Reference and Napp Seam (+6 more)

### Community 21 - "Core Configuration Defaults"
Cohesion: 0.25
Nodes (7): description, identifier, permissions, $schema, windows, core:default, main

### Community 22 - "Application Preferences and Bindings"
Cohesion: 0.07
Nodes (40): ACTION_IDS, bindingFromEvent(), bindingMatches(), DEFAULT_KEYBINDINGS, defaultPreferences(), KEYBINDING_ACTIONS, parsePreferences(), validateKeybindings() (+32 more)

### Community 23 - "debian13-live-test.sh"
Cohesion: 0.20
Nodes (13): CARGO_INCREMENTAL, CARGO_PROFILE_DEV_DEBUG, fail(), record_prebuild(), reexec_with_nix_group(), run_cache_probe(), run_startup_step(), debian13-live-test.sh script (+5 more)

### Community 24 - "Response"
Cohesion: 0.32
Nodes (11): CatalogCapability, Diagnostics, NappletReview, RelayDiagnostic, Response, Option, Self, String (+3 more)

### Community 25 - "String"
Cohesion: 0.33
Nodes (6): public_naddr_reviews_confirms_and_launches_exact_single_file(), AsRef, Path, Self, String, Vec

### Community 26 - "POC scope and acceptance"
Cohesion: 0.17
Nodes (11): Accepted post-foundation extension, Architectural invariants, Demo-complete, Foundation-complete, Non-goals, Objective, POC scope and acceptance, Required napplets (+3 more)

### Community 28 - "Codebase Concerns"
Cohesion: 0.18
Nodes (10): Codebase Concerns, Dependencies at Risk, Fragile Areas, Known Bugs, Missing Critical Features, Performance Bottlenecks, Scaling Limits, Security Considerations (+2 more)

### Community 29 - "uzel-poc-validated-pack/README.md"
Cohesion: 0.25
Nodes (5): uzel-poc-validated-pack/AGENTS.md, Decision, Source baseline, uzel-poc-validated-pack/README.md, FACT-XXX — title

### Community 31 - "Provisional component design"
Cohesion: 0.20
Nodes (9): Developer mode, Exact-build fixtures, Identity and Nostr reads, Local daemon protocol, Napplet convention, Provisional component design, Runtime state, Shell UI (+1 more)

### Community 32 - "Identity and State Management"
Cohesion: 0.18
Nodes (13): BTreeSet, artifact_base_url(), read_launched_document(), reconcile_launched_session(), Error, Result, RunnerError, StatePersistFailure (+5 more)

### Community 33 - "Process Summary and Validation"
Cohesion: 0.33
Nodes (5): Changes, Commits, Lean process reset summary, Outcome, Validation

### Community 34 - "Runtime Testing and Demos"
Cohesion: 0.20
Nodes (9): Deterministic demo, Final acceptance, Hostile frame, Live demo, Napplet/web, Quality commands, Required test layers, Runtime/Rust (+1 more)

### Community 35 - "Verification and Gates"
Cohesion: 0.50
Nodes (3): External merge gate, Lean process reset verification, Local gate

### Community 36 - "Upstream Evidence and Verdict"
Cohesion: 0.20
Nodes (10): API and ownership evidence, Automated review corrections, Commands and observed results, Exact pins and fixtures, Hostile fixture scope, Manifest correction, Next step, Slice 03 preflight (+2 more)

### Community 37 - "parse_options"
Cohesion: 0.27
Nodes (18): uzel-napd/src/main.rs, default_runtime_root(), default_socket_path(), live_configuration_requires_explicit_live_mode(), main(), next_path(), next_value(), Options (+10 more)

### Community 38 - "Application Code and Logic"
Cohesion: 0.25
Nodes (3): @tauri-apps/api/core, ./preferences.js, ./projection-failure.js

### Community 39 - "Buffered Event Handling"
Cohesion: 0.31
Nodes (7): buffered_responses_are_byte_bounded_and_consumed_once(), BufferedEvents, event_bytes(), response_event(), VecDeque, RuntimeEvent, RuntimeObservationFrame

### Community 40 - "External Integrations and APIs"
Cohesion: 0.22
Nodes (8): APIs & External Services, Authentication & Identity, CI/CD & Deployment, Data Storage, Environment Configuration, External Integrations, Monitoring & Observability, Webhooks & Callbacks

### Community 41 - "Engineering Rules and Policies"
Cohesion: 0.22
Nodes (9): Engineering rules, Mission, POC exclusions, Quality gate, Repository boundaries, Required method, Trust rules, Upstream contribution policy (+1 more)

### Community 42 - "Decision Gates and Validation"
Cohesion: 0.22
Nodes (9): Assumption validation and decision gates, Current observed baseline, Gate 0 decision, Gate matrix, Hard stops, Per-slice validation, Plan correction rule, Required outputs (+1 more)

### Community 43 - "Trust Domains and Flow"
Cohesion: 0.22
Nodes (9): Accepted upstream seam, Composition flow, POC architecture, Repository zones, Runtime topology, Session start, Shared Nostr flow, Trust domains (+1 more)

### Community 44 - "Coding Style and Conventions"
Cohesion: 0.20
Nodes (9): Code Style, Coding Conventions, Comments, Error Handling, Function Design, Import Organization, Logging, Module Design (+1 more)

### Community 45 - "Risk and Decision Management"
Cohesion: 0.22
Nodes (9): Accepted provisional risks, Confirmed assumptions, Decision, Exact next steps, Gate 0 preflight and Slice 01 decision, Gate results, Rejected assumptions, Required design changes (+1 more)

### Community 47 - "Runtime Error Handling"
Cohesion: 0.15
Nodes (51): Any, Namespace, RuntimeError, approved_reachability(), candidate_argv_ok(), candidate_record(), canonical_record(), CheckError (+43 more)

### Community 48 - "Testing Patterns and Fixtures"
Cohesion: 0.20
Nodes (9): Common Patterns, Coverage, Fixtures and Factories, Mocking, Test File Organization, Test Framework, Test Structure, Test Types (+1 more)

### Community 49 - "Napplet Build Dependencies"
Cohesion: 0.10
Nodes (21): follow-list/package.json, dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/conformance-cli, @napplet/vite-plugin, vite (+13 more)

### Community 50 - "Trusted Shell Policy"
Cohesion: 0.83
Nodes (3): directive(), innerPolicyContent(), outerPolicyContent()

### Community 51 - "Trusted Shell Surface Host"
Cohesion: 0.24
Nodes (5): createSurfaceHost(), mount(), receive(), unmount(), Window

### Community 52 - "Runtime Evidence and Results"
Cohesion: 0.22
Nodes (9): Commands and observed results, Exact dependency and asset record, Next step, Preserved failed Fedora probe, Required design correction, Runtime evidence, Slice 02 preflight, Upstream result (+1 more)

### Community 53 - "Slice Preflight and Evidence"
Cohesion: 0.22
Nodes (8): Commands and results, Exact dependency and upstream evidence, Exact next step, Honest boundary, Linux shell evidence, Outcome, Runtime composition evidence, Slice 05 preflight — integrated composed demo

### Community 54 - "Slice Hardening and Go-No-Go"
Cohesion: 0.22
Nodes (8): Bubblewrap decision, Commands and results, Exact hostile evidence, Failed evidence and toolchain limit, Go/no-go and exact next steps, Outcome, Slice 06 preflight — hardening and clean demo acceptance, Upstream result

### Community 55 - "Codebase Structure Layout"
Cohesion: 0.25
Nodes (7): Codebase Structure, Directory Layout, Directory Purposes, Key File Locations, Naming Conventions, Special Directories, Where to Add New Code

### Community 58 - "Napplet Profile Build"
Cohesion: 0.10
Nodes (21): profile-card/package.json, dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/conformance-cli, @napplet/vite-plugin, vite (+13 more)

### Community 59 - "hostile-egress/package.json"
Cohesion: 0.11
Nodes (18): hostile-egress/package.json, dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/vite-plugin, vite, @napplet/nap (+10 more)

### Community 60 - "profile-open-v1.schema.json"
Cohesion: 0.13
Nodes (14): additionalProperties, $id, properties, pubkey, version, pattern, type, required (+6 more)

### Community 61 - "Technology Stack and Config"
Cohesion: 0.25
Nodes (7): Configuration, Frameworks, Key Dependencies, Languages, Platform Requirements, Runtime, Technology Stack

### Community 62 - "Client Interaction and Cancellation"
Cohesion: 0.10
Nodes (54): src-tauri/src/main.rs, allowed_navigation(), ambiguous_confirmation_crosses_as_a_typed_retry_state(), ambiguous_review_crosses_as_a_typed_retry_state(), cancel_napplet_review(), clean_token_snapshot(), confirm_napplet(), ConfirmNappletError (+46 more)

### Community 63 - "Nix Setup Scripting"
Cohesion: 0.46
Nodes (7): fail(), print_nix_builder_state(), print_nix_daemon_state(), refresh_nix_builder_state(), refresh_nix_daemon_state(), debian13-setup.sh script, wait_for_nix_daemon_socket()

### Community 64 - "Hostile Egress Probing"
Cohesion: 0.13
Nodes (13): hostile-egress/src/main.js, denied(), results, target, attemptRawWebKitInvoke(), boundedAttempt(), nativeSurface(), PROBE_NAMES (+5 more)

### Community 65 - "Test Maintenance Process"
Cohesion: 0.54
Nodes (5): init_repository(), MaintenanceTests, CompletedProcess, Path, run()

### Community 66 - "Linux Smoke Testing"
Cohesion: 0.67
Nodes (3): matchesLog(), napdReadyPattern(), script

### Community 67 - "Documentation Audit Results"
Cohesion: 0.25
Nodes (7): Confidence, Corrections, Diagram review, Gate 0 resolution, POC documentation audit, Problems found in the previous pack, Verdict

### Community 68 - "Extraction and Hardening"
Cohesion: 0.25
Nodes (7): First hardening follow-ups, Likely `kehto/napd`, Moves to `jodobear/napplets`, POC shortcuts that must not silently become platform contracts, Post-POC extraction, Remains in Uzel, Rewrite criteria

### Community 69 - "Linux Compatibility Map"
Cohesion: 0.25
Nodes (7): Apple-only edges, Build evidence, Compatibility candidate, Crate map, nampplets Linux reuse map, Result, Runtime entry points

### Community 70 - "Probe Results and Boundaries"
Cohesion: 0.25
Nodes (8): Boundary result, Commands and observed results, Debian probe correction, Fedora probe correction, Locked workspace, Next step, Slice 01 preflight, Verdict

### Community 71 - "Slice Preflight Outcomes"
Cohesion: 0.25
Nodes (7): Commands and results, Exact next step, NMP and persistence evidence, Outcome, Private protocol evidence, Remaining boundaries, Slice 04 preflight — daemon, NMP, and persistence

### Community 72 - "Trust and Source Verification"
Cohesion: 0.25
Nodes (7): Accepted child CSP, Boundary of proof, Executable probe, Low-level handler nuance, Source verification, Verdict, WebKit/Tauri trust spike

### Community 73 - "Task Status and Goals"
Cohesion: 0.25
Nodes (8): Acceptance, Depends on, Entry status, Goal, Non-goals, Status, Tasks, Work 01 — scaffold

### Community 74 - "Napplet Package Registry"
Cohesion: 0.29
Nodes (7): Kehto #204, `nampplets`, NAP registry, Napplet packages, NIP-5A and NIP-5D, NMP, Verified facts

### Community 78 - "Upstream Contribution Ledger"
Cohesion: 0.29
Nodes (7): Active contributions, Authority and ownership, Entry template, Slice 02 upstream result, Slice 03 upstream result, Slice 06 upstream result, Upstream contribution ledger

### Community 79 - "NMP API Mapping"
Cohesion: 0.29
Nodes (6): Accepted pin, Executable probe, nampplets adapter seam, NMP API and ownership map, Ownership boundary, Public facade

### Community 80 - "Documentation Auditing Paths"
Cohesion: 0.57
Nodes (6): audit(), main(), manifest_paths(), Path, strip_code(), write_manifest()

### Community 81 - "Native Settings Execution"
Cohesion: 0.38
Nodes (5): AcceptSettings, UnavailableSettings, NativeSettingsExecutor, NativeSettingsOpenResult, NativeSettingsRequest

### Community 82 - "Runtime Diagnostics Sink"
Cohesion: 0.50
Nodes (3): RelayDiagnosticsSink, RuntimeRelayDiagnosticsObserver, RuntimeRelayDiagnosticsSnapshot

### Community 83 - "Validation and Stop Conditions"
Cohesion: 0.29
Nodes (7): Done when, Goal, Outputs, Read, Stop conditions, Tasks, Work 00 — validate assumptions

### Community 84 - "Resource Cancellation and Client"
Cohesion: 0.09
Nodes (34): Cancellation, Client, cancellation_signal(), cancelled_resolution_returns_without_network_work(), linux_resource_provider(), LinuxResourceNetwork, map_reqwest_error(), MonotonicResourceClock (+26 more)

### Community 85 - "Linux Build Runner"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 02 — Linux exact-build runner

### Community 86 - "Daemon Persistence Goals"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 04 — daemon, NMP, and persistence

### Community 87 - "Issue Stabilization Work"
Cohesion: 0.29
Nodes (7): Active issue — #19, Completed issue — #10, Entry evidence, Exit rule, Goal, Work 07 — issue-driven stabilization, Work graph

### Community 88 - "Execution Slices and Handoff"
Cohesion: 0.33
Nodes (6): Dependency graph, Execution slices, Handoff, Parallel work, Slice entry gate, Slices

### Community 89 - "Order Start Process"
Cohesion: 0.33
Nodes (6): Demo result, Document map, Mandatory first step, Scope rule, Start order, Uzel single-repository POC

### Community 90 - "Implementation Status"
Cohesion: 0.33
Nodes (5): Accepted provisional risks, Gate 0 — validated baseline, Implementation, Latest integrated evidence, POC status

### Community 91 - "Task and Goal Tracking"
Cohesion: 0.33
Nodes (6): Acceptance, Entry status and pins, Goal, Read, Tasks, Work 03 — portable napplets

### Community 92 - "mock-native.js"
Cohesion: 0.24
Nodes (9): diagnostics(), invoke(), isRoutedProfileQuery(), nativeEnvelope(), profileEvent(), profileEventsForQuery(), profileFor(), review() (+1 more)

### Community 93 - "Integration and Demo Work"
Cohesion: 0.33
Nodes (5): Acceptance, Goal, Non-goals, Tasks, Work 05 — composed demo

### Community 94 - "Hardening and Acceptance"
Cohesion: 0.40
Nodes (4): Acceptance, Goal, Tasks, Work 06 — hardening and demo acceptance

### Community 95 - "Stream and Response Writing"
Cohesion: 0.83
Nodes (4): handle_stream(), UnixStream, write_chunked_envelope(), write_response()

### Community 96 - "Graph Portability Check"
Cohesion: 0.67
Nodes (3): main(), Path, tracked_graph_files()

### Community 97 - "Fedora Smoke Testing"
Cohesion: 0.50
Nodes (3): fedora-run-smoke.sh script, UZEL_SMOKE_NAME, UZEL_SMOKE_SUCCESS_MARKER

### Community 230 - "Agent Testing and Setup"
Cohesion: 0.05
Nodes (34): Uzel agent instructions, ui/README.md, Evidence boundary, Renderer acceptance harness, 1. Clone and install the launcher, 2. Run automated real-WebKit acceptance, 3. Run visible desktop demo, Debian 13 live test (+26 more)

## Knowledge Gaps
- **549 isolated node(s):** `name`, `private`, `version`, `type`, `dev:web` (+544 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **34 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `uzel-poc-validated-pack/README.md` connect `uzel-poc-validated-pack/README.md` to `Runtime Testing and Demos`, `Documentation Audit Results`, `Extraction and Hardening`, `Implementation Status`, `Agent Testing and Setup`, `Slice Hardening and Go-No-Go`, `Order Start Process`, `POC scope and acceptance`, `04-execution.md`, `Provisional component design`?**
  _High betweenness centrality (0.028) - this node is a cross-community bridge._
- **Why does `LinuxRunner` connect `LinuxRunner` to `Identity and State Management`, `Identity and Session State`, `Daemon Server Socket Management`, `EventBuffer`, `Request`, `Response`, `String`?**
  _High betweenness centrality (0.024) - this node is a cross-community bridge._
- **Why does `src-tauri/src/main.rs` connect `Client Interaction and Cancellation` to `Hostile Probe Network Handling`?**
  _High betweenness centrality (0.017) - this node is a cross-community bridge._
- **What connects `name`, `private`, `version` to the rest of the system?**
  _549 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Dependency and Network Identifiers` be split into smaller, more focused modules?**
  _Cohesion score 0.06155632984901278 - nodes in this community are weakly interconnected._
- **Should `Hostile Probe Network Handling` be split into smaller, more focused modules?**
  _Cohesion score 0.11746031746031746 - nodes in this community are weakly interconnected._
- **Should `Identity and Session State` be split into smaller, more focused modules?**
  _Cohesion score 0.10952380952380952 - nodes in this community are weakly interconnected._