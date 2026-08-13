# Graph Report - .  (2026-08-13)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 1465 nodes · 2452 edges · 130 communities (99 shown, 31 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 26 edges (avg confidence: 0.72)
- Token cost: 6,143 input · 10,539 output

## Graph Freshness
- Built from commit: `b81a2de9`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run locked `pnpm graphify:refresh` after code changes.

## Community Hubs (Navigation)
- Protocol error handling
- trusted-shell.js
- Dependency and Import Management
- LinuxRunner
- Hostile Probe State
- Runner State and Identity
- Server and Socket Handling
- EventBuffer
- Asset Transfer and Frames
- Client Delivery Errors
- Linux Execution Handling
- Request
- Architecture
- Profile Query Management
- fixtures.rs
- TypeScript Configuration
- Frontend Project Dependencies
- Scripting and Package Management
- Project Planning and Risks
- tauri.conf.json
- Requirements: Uzel
- Core Configuration Defaults
- User Preferences and Bindings
- debian13-live-test.sh
- Response
- String
- POC scope and acceptance
- 04-execution.md
- Codebase Concerns
- README.md
- Projection Failure Handling
- Provisional component design
- Identity State Rollback
- Process summary and validation
- Runtime Testing and Demos
- Verification Gates Process
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
- Record Validation and Checking
- Testing Patterns and Fixtures
- NPM Package Management
- Trusted Shell Policy
- Trusted Shell Surface Host
- Runtime Evidence and Results
- Slice Preflight and Evidence
- Slice Hardening and Go-No-Go
- Codebase Structure Layout
- NPM Package Management
- package.json
- profile-open-v1.schema.json
- Technology Stack and Config
- Client Probing and Cancellation
- Nix Setup Scripting
- Execution Results and Workers
- Testing and maintenance
- Linux Smoke Testing
- Audit and verification
- Extraction and hardening
- Nampplets Linux mapping
- Probe results and boundaries
- Preflight checks and evidence
- WebKit trust verification
- Task and goal setting
- Nampplets registry facts
- Contribution ownership ledger
- NMP API mapping
- Documentation auditing
- Native settings execution
- Relay Diagnostics Snapshot
- Validation stop conditions
- Client Resource Cancellation
- Acceptance criteria goals
- NMP persistence goals
- Issue stabilization work
- Execution Slices Handoff
- Demo Scope Start Order
- POC status and risks
- Acceptance Tasks Goals
- mock-native.js
- Integration Acceptance Goals
- Hardening Demo Acceptance
- Stream Write Response
- Graph Portability Check
- Fedora Smoke Markers
- Pinned Asset Check
- Debian Build Cleanup
- Development Cleanup Script
- Graph Cache Refresh
- Trusted Shell README
- Candidate Qualification
- Napp Dependency Handoff
- Signed Work Fixtures
- Project Fixtures
- Napplets Management
- Build Signed Napplet Fixtures
- Boundary Checks
- Smoke Testing
- Package Line Revisions
- NAP NIP Status
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
- FACT-014-live-identity-catalog.md
- manifest.json
- Agent Instructions and Testing
- STATE.md

## God Nodes (most connected - your core abstractions)
1. `LinuxRunner` - 44 edges
2. `UnixClient` - 38 edges
3. `RunnerError` - 29 edges
4. `ClientError` - 24 edges
5. `Response` - 22 edges
6. `scripts` - 22 edges
7. `validate_record()` - 21 edges
8. `CheckError` - 18 edges
9. `HostileProbeState` - 17 edges
10. `write_frame()` - 17 edges

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

## Communities (130 total, 31 thin omitted)

### Community 0 - "Protocol error handling"
Cohesion: 0.38
Nodes (7): ProtocolError, read_exact_or_truncated(), read_frame(), Error, T, DecodeError, Read

### Community 1 - "trusted-shell.js"
Cohesion: 0.25
Nodes (18): boundedJSON(), compatibilityPreludeSource(), decodedBase64Length(), decodeResourceBlob(), exactFields(), isBoundedEnvelope(), isPlainObject(), isVerifiedArtifactBaseURL() (+10 more)

### Community 2 - "Dependency and Import Management"
Cohesion: 0.06
Nodes (37): allowedDependencyTargets, allowedGuardedGlobalAccesses, { compile: compileSvelte, parse: parseSvelte }, contractsRoot, declarativeNetworkViolations(), dependencyGroups, dependencyViolations(), directNetworkIdentifiers (+29 more)

### Community 3 - "LinuxRunner"
Cohesion: 0.18
Nodes (9): LinuxRunner, BTreeMap, Drop, PathBuf, RuntimeMode, Debug, Formatter, RuntimeObservation (+1 more)

### Community 4 - "Hostile Probe State"
Cohesion: 0.12
Nodes (24): accepted_report(), BeaconAttempt, control_accept_is_not_counted_as_a_probe_connection(), exact_surface_cancellation_retires_the_attached_probe(), HostileProbeReport, HostileProbeState, HostileProbeVerdict, later_loopback_connection_is_counted_separately() (+16 more)

### Community 5 - "Runner State and Identity"
Cohesion: 0.11
Nodes (21): absent_surface_cleanup_is_idempotent(), catalog_cancellation_is_terminal(), catalog_cancellation_keeps_retryable_reviews_and_discards_terminal_stale_tokens(), eventually_identity_query(), hostile_probe_commits_exact_session_config_before_returning(), identity_query(), inc_emit_waits_for_an_inc_event_not_an_unrelated_push(), launch_identity_surface() (+13 more)

### Community 6 - "Server and Socket Handling"
Cohesion: 0.15
Nodes (24): active_daemon_socket_is_not_unlinked(), daemon_routes_inc_delivery_to_the_other_exact_surface(), daemon_serves_ordered_verified_asset_and_shuts_down(), DaemonServer, exchange(), existing_shared_socket_parent_is_not_chmodded(), incomplete_client_times_out_without_blocking_the_next_request(), oversized_routed_envelope_is_chunked_without_raising_the_frame_limit() (+16 more)

### Community 7 - "EventBuffer"
Cohesion: 0.20
Nodes (11): Condvar, bounded_diagnostic(), EventBuffer, EventSink, ProductState, Arc, Mutex, Option (+3 more)

### Community 8 - "Asset Transfer and Frames"
Cohesion: 0.15
Nodes (19): authoritative_reconciliation_retires_ambiguous_operation_ids(), chunked_routed_envelope_reassembles_on_one_connection(), confirm_responses_lost_replay_surface_without_a_second_operation(), deterministic_presend_failures_do_not_retain_catalog_operations(), encode_asset_chunk(), failed_asset_transfer_stops_the_launched_surface(), frames_round_trip_with_big_endian_length(), maximum_asset_chunk_fits_control_frame() (+11 more)

### Community 9 - "Client Delivery Errors"
Cohesion: 0.25
Nodes (7): ClientError, decode_asset_chunk(), DeliveryError, FetchedSurface, Box, Result, UnixStream

### Community 10 - "Linux Execution Handling"
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
Nodes (69): canonicalProfile(), canonicalProfiles(), optionalText(), profileQueryBatches(), profileQueryRequest(), retryProfileQueryRequests(), splitProfileQueryRequest(), A (+61 more)

### Community 14 - "fixtures.rs"
Cohesion: 0.20
Nodes (7): ArtifactFetchRequest, ArtifactFetchResponse, ArtifactSource, ExactFixtureSource, fixture_by_name(), FixtureDefinition, Option

### Community 15 - "TypeScript Configuration"
Cohesion: 0.08
Nodes (23): compilerOptions, allowJs, checkJs, esModuleInterop, forceConsistentCasingInFileNames, isolatedModules, lib, module (+15 more)

### Community 16 - "Frontend Project Dependencies"
Cohesion: 0.07
Nodes (27): dependencies, svelte, @tauri-apps/api, devDependencies, playwright, svelte-check, @sveltejs/vite-plugin-svelte, typescript (+19 more)

### Community 17 - "Scripting and Package Management"
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

### Community 22 - "User Preferences and Bindings"
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

### Community 29 - "README.md"
Cohesion: 0.25
Nodes (3): Decision, Source baseline, FACT-XXX — title

### Community 31 - "Provisional component design"
Cohesion: 0.20
Nodes (9): Developer mode, Exact-build fixtures, Identity and Nostr reads, Local daemon protocol, Napplet convention, Provisional component design, Runtime state, Shell UI (+1 more)

### Community 32 - "Identity State Rollback"
Cohesion: 0.18
Nodes (13): BTreeSet, artifact_base_url(), read_launched_document(), reconcile_launched_session(), Error, Result, RunnerError, StatePersistFailure (+5 more)

### Community 33 - "Process summary and validation"
Cohesion: 0.33
Nodes (5): Changes, Commits, Lean process reset summary, Outcome, Validation

### Community 34 - "Runtime Testing and Demos"
Cohesion: 0.20
Nodes (9): Deterministic demo, Final acceptance, Hostile frame, Live demo, Napplet/web, Quality commands, Required test layers, Runtime/Rust (+1 more)

### Community 35 - "Verification Gates Process"
Cohesion: 0.50
Nodes (3): External merge gate, Lean process reset verification, Local gate

### Community 36 - "Upstream Evidence and Verdict"
Cohesion: 0.20
Nodes (10): API and ownership evidence, Automated review corrections, Commands and observed results, Exact pins and fixtures, Hostile fixture scope, Manifest correction, Next step, Slice 03 preflight (+2 more)

### Community 37 - "parse_options"
Cohesion: 0.27
Nodes (17): default_runtime_root(), default_socket_path(), live_configuration_requires_explicit_live_mode(), main(), next_path(), next_value(), Options, parse_options() (+9 more)

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

### Community 47 - "Record Validation and Checking"
Cohesion: 0.15
Nodes (51): Any, Namespace, RuntimeError, approved_reachability(), candidate_argv_ok(), candidate_record(), canonical_record(), CheckError (+43 more)

### Community 48 - "Testing Patterns and Fixtures"
Cohesion: 0.20
Nodes (9): Common Patterns, Coverage, Fixtures and Factories, Mocking, Test File Organization, Test Framework, Test Structure, Test Types (+1 more)

### Community 49 - "NPM Package Management"
Cohesion: 0.10
Nodes (20): dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/conformance-cli, @napplet/vite-plugin, vite, @napplet/conformance-cli (+12 more)

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

### Community 58 - "NPM Package Management"
Cohesion: 0.10
Nodes (20): dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/conformance-cli, @napplet/vite-plugin, vite, @napplet/conformance-cli (+12 more)

### Community 59 - "package.json"
Cohesion: 0.11
Nodes (17): dependencies, @napplet/nap, @napplet/shim, devDependencies, @napplet/vite-plugin, vite, @napplet/nap, @napplet/shim (+9 more)

### Community 60 - "profile-open-v1.schema.json"
Cohesion: 0.13
Nodes (14): additionalProperties, $id, properties, pubkey, version, pattern, type, required (+6 more)

### Community 61 - "Technology Stack and Config"
Cohesion: 0.25
Nodes (7): Configuration, Frameworks, Key Dependencies, Languages, Platform Requirements, Runtime, Technology Stack

### Community 62 - "Client Probing and Cancellation"
Cohesion: 0.10
Nodes (53): allowed_navigation(), ambiguous_confirmation_crosses_as_a_typed_retry_state(), ambiguous_review_crosses_as_a_typed_retry_state(), cancel_napplet_review(), clean_token_snapshot(), confirm_napplet(), ConfirmNappletError, default_socket_path() (+45 more)

### Community 63 - "Nix Setup Scripting"
Cohesion: 0.46
Nodes (7): fail(), print_nix_builder_state(), print_nix_daemon_state(), refresh_nix_builder_state(), refresh_nix_daemon_state(), debian13-setup.sh script, wait_for_nix_daemon_socket()

### Community 64 - "Execution Results and Workers"
Cohesion: 0.13
Nodes (12): denied(), results, target, attemptRawWebKitInvoke(), boundedAttempt(), nativeSurface(), PROBE_NAMES, sentinelTargets() (+4 more)

### Community 65 - "Testing and maintenance"
Cohesion: 0.54
Nodes (5): init_repository(), MaintenanceTests, CompletedProcess, Path, run()

### Community 66 - "Linux Smoke Testing"
Cohesion: 0.67
Nodes (3): matchesLog(), napdReadyPattern(), script

### Community 67 - "Audit and verification"
Cohesion: 0.25
Nodes (7): Confidence, Corrections, Diagram review, Gate 0 resolution, POC documentation audit, Problems found in the previous pack, Verdict

### Community 68 - "Extraction and hardening"
Cohesion: 0.25
Nodes (7): First hardening follow-ups, Likely `kehto/napd`, Moves to `jodobear/napplets`, POC shortcuts that must not silently become platform contracts, Post-POC extraction, Remains in Uzel, Rewrite criteria

### Community 69 - "Nampplets Linux mapping"
Cohesion: 0.25
Nodes (7): Apple-only edges, Build evidence, Compatibility candidate, Crate map, nampplets Linux reuse map, Result, Runtime entry points

### Community 70 - "Probe results and boundaries"
Cohesion: 0.25
Nodes (8): Boundary result, Commands and observed results, Debian probe correction, Fedora probe correction, Locked workspace, Next step, Slice 01 preflight, Verdict

### Community 71 - "Preflight checks and evidence"
Cohesion: 0.25
Nodes (7): Commands and results, Exact next step, NMP and persistence evidence, Outcome, Private protocol evidence, Remaining boundaries, Slice 04 preflight — daemon, NMP, and persistence

### Community 72 - "WebKit trust verification"
Cohesion: 0.25
Nodes (7): Accepted child CSP, Boundary of proof, Executable probe, Low-level handler nuance, Source verification, Verdict, WebKit/Tauri trust spike

### Community 73 - "Task and goal setting"
Cohesion: 0.25
Nodes (8): Acceptance, Depends on, Entry status, Goal, Non-goals, Status, Tasks, Work 01 — scaffold

### Community 74 - "Nampplets registry facts"
Cohesion: 0.29
Nodes (7): Kehto #204, `nampplets`, NAP registry, Napplet packages, NIP-5A and NIP-5D, NMP, Verified facts

### Community 78 - "Contribution ownership ledger"
Cohesion: 0.29
Nodes (7): Active contributions, Authority and ownership, Entry template, Slice 02 upstream result, Slice 03 upstream result, Slice 06 upstream result, Upstream contribution ledger

### Community 79 - "NMP API mapping"
Cohesion: 0.29
Nodes (6): Accepted pin, Executable probe, nampplets adapter seam, NMP API and ownership map, Ownership boundary, Public facade

### Community 80 - "Documentation auditing"
Cohesion: 0.57
Nodes (6): audit(), main(), manifest_paths(), Path, strip_code(), write_manifest()

### Community 81 - "Native settings execution"
Cohesion: 0.38
Nodes (5): AcceptSettings, UnavailableSettings, NativeSettingsExecutor, NativeSettingsOpenResult, NativeSettingsRequest

### Community 82 - "Relay Diagnostics Snapshot"
Cohesion: 0.50
Nodes (3): RelayDiagnosticsSink, RuntimeRelayDiagnosticsObserver, RuntimeRelayDiagnosticsSnapshot

### Community 83 - "Validation stop conditions"
Cohesion: 0.29
Nodes (7): Done when, Goal, Outputs, Read, Stop conditions, Tasks, Work 00 — validate assumptions

### Community 84 - "Client Resource Cancellation"
Cohesion: 0.09
Nodes (34): Cancellation, Client, cancellation_signal(), cancelled_resolution_returns_without_network_work(), linux_resource_provider(), LinuxResourceNetwork, map_reqwest_error(), MonotonicResourceClock (+26 more)

### Community 85 - "Acceptance criteria goals"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 02 — Linux exact-build runner

### Community 86 - "NMP persistence goals"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 04 — daemon, NMP, and persistence

### Community 87 - "Issue stabilization work"
Cohesion: 0.29
Nodes (7): Active issue — #19, Completed issue — #10, Entry evidence, Exit rule, Goal, Work 07 — issue-driven stabilization, Work graph

### Community 88 - "Execution Slices Handoff"
Cohesion: 0.33
Nodes (6): Dependency graph, Execution slices, Handoff, Parallel work, Slice entry gate, Slices

### Community 89 - "Demo Scope Start Order"
Cohesion: 0.33
Nodes (6): Demo result, Document map, Mandatory first step, Scope rule, Start order, Uzel single-repository POC

### Community 90 - "POC status and risks"
Cohesion: 0.29
Nodes (6): Accepted provisional risks, Gate 0 — validated baseline, Implementation, Latest integrated evidence, POC status, Preserved evidence maintenance — 2026-08-13

### Community 91 - "Acceptance Tasks Goals"
Cohesion: 0.33
Nodes (6): Acceptance, Entry status and pins, Goal, Read, Tasks, Work 03 — portable napplets

### Community 92 - "mock-native.js"
Cohesion: 0.24
Nodes (9): diagnostics(), invoke(), isRoutedProfileQuery(), nativeEnvelope(), profileEvent(), profileEventsForQuery(), profileFor(), review() (+1 more)

### Community 93 - "Integration Acceptance Goals"
Cohesion: 0.33
Nodes (5): Acceptance, Goal, Non-goals, Tasks, Work 05 — composed demo

### Community 94 - "Hardening Demo Acceptance"
Cohesion: 0.40
Nodes (4): Acceptance, Goal, Tasks, Work 06 — hardening and demo acceptance

### Community 95 - "Stream Write Response"
Cohesion: 0.83
Nodes (4): handle_stream(), UnixStream, write_chunked_envelope(), write_response()

### Community 96 - "Graph Portability Check"
Cohesion: 0.67
Nodes (3): main(), Path, tracked_graph_files()

### Community 97 - "Fedora Smoke Markers"
Cohesion: 0.50
Nodes (3): fedora-run-smoke.sh script, UZEL_SMOKE_NAME, UZEL_SMOKE_SUCCESS_MARKER

### Community 230 - "Agent Instructions and Testing"
Cohesion: 0.05
Nodes (33): Uzel agent instructions, Evidence boundary, Renderer acceptance harness, 1. Clone and install the launcher, 2. Run automated real-WebKit acceptance, 3. Run visible desktop demo, Debian 13 live test, Active (+25 more)

## Knowledge Gaps
- **543 isolated node(s):** `name`, `private`, `version`, `type`, `dev:web` (+538 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **31 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `LinuxRunner` connect `LinuxRunner` to `Identity State Rollback`, `Runner State and Identity`, `Server and Socket Handling`, `EventBuffer`, `Request`, `Response`, `String`?**
  _High betweenness centrality (0.024) - this node is a cross-community bridge._
- **Why does `linux_resource_provider()` connect `Client Resource Cancellation` to `String`?**
  _High betweenness centrality (0.017) - this node is a cross-community bridge._
- **Why does `NappletReview` connect `Response` to `LinuxRunner`, `Asset Transfer and Frames`, `Client Delivery Errors`, `String`, `Client Probing and Cancellation`?**
  _High betweenness centrality (0.015) - this node is a cross-community bridge._
- **What connects `name`, `private`, `version` to the rest of the system?**
  _543 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Dependency and Import Management` be split into smaller, more focused modules?**
  _Cohesion score 0.06155632984901278 - nodes in this community are weakly interconnected._
- **Should `Hostile Probe State` be split into smaller, more focused modules?**
  _Cohesion score 0.11746031746031746 - nodes in this community are weakly interconnected._
- **Should `Runner State and Identity` be split into smaller, more focused modules?**
  _Cohesion score 0.10952380952380952 - nodes in this community are weakly interconnected._