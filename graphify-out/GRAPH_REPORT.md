# Graph Report - uzel-lean-reset  (2026-08-12)

## Corpus Check
- 141 files · ~106,137 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1425 nodes · 2072 edges · 191 communities (88 shown, 103 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 26 edges (avg confidence: 0.72)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `47dc4cf6`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- UnixClient
- trusted-shell.js
- check-napplet-imports.mjs
- LinuxRunner
- Delivery, quality, review and packaging discipline
- runner.rs
- Uzel product and incubation architecture
- EventBuffer
- .forward_from_surface
- Material findings and corrections
- A5 — mandatory post-M5 whole-system audit
- M0 / GSD Phase 1 — truthful baseline and replay contract
- Production engineering dimensions
- main.js
- fixtures.rs
- compilerOptions
- package.json
- scripts
- Fast-moving ecosystem, compatibility and upstream stewardship
- Decision, nuance and educational-knowledge system
- M5 / GSD delivery phases 7, 7.1–7.9 — production-candidate hardening and audit freeze
- default.json
- acceptance.test.mjs
- Uzel agent instructions
- linux-run-smoke.sh
- GSD ingest — reorient the existing Uzel project in place
- Phase 01: SLICE-REF-01 — POC Replay & Accepted Napp Seam - Research
- tauri.conf.json
- Bounded slices
- debian13-live-test.sh
- projection-failure.js
- M4.5 / GSD delivery phases 6, 6.1–6.2 — scheduling, recovery and cross-domain composition
- Upstream contribution ledger
- parse_options
- App.svelte
- POC status
- POC scope and acceptance
- Assumption validation and decision gates
- Debian 13 live test
- ref-candidate-check.py
- package.json
- trusted-shell-policy.js
- createSurfaceHost
- 04-execution.md
- Tests, quality gates, and demo
- package.json
- package.json
- profile-open-v1.schema.json
- main.rs
- Slice 03 preflight
- probes.test.mjs
- linux-smoke-script.test.mjs
- Uzel POC agent instructions
- Provisional component design
- hostile_probe.rs
- Slice 02 preflight
- Slice 05 preflight — integrated composed demo
- AcceptSettings
- RelayDiagnosticsSink
- debian13-setup.sh
- resource.rs
- POC documentation audit
- Post-POC extraction
- nampplets Linux reuse map
- Slice 01 preflight
- Slice 04 preflight — daemon, NMP, and persistence
- Slice 06 preflight — hardening and clean demo acceptance
- WebKit/Tauri trust spike
- mock-native.js
- Work 00 — validate assumptions
- Work 01 — scaffold
- Work 07 — issue-driven stabilization
- NMP API and ownership map
- Work 02 — Linux exact-build runner
- Work 04 — daemon, NMP, and persistence
- Arc
- AsRef
- BTreeMap
- Drop
- Error
- Mutex
- Option
- Path
- PathBuf
- Result
- Self
- String
- Vec
- VecDeque
- Execution slices
- Uzel single-repository POC
- Work 03 — portable napplets
- Work 05 — composed demo
- audit_docs.py
- Work 06 — hardening and demo acceptance
- fedora-run-smoke.sh
- check_sha256
- debian-build-smoke.sh
- dev.sh
- README.md
- Exact signed Work 02 fixture
- Fixtures
- README.md
- build-signed-napplet-fixtures.sh
- check-boundaries.sh
- smoke.sh
- FACT-001 — Kehto PR 204 and the 0.29 package line
- FACT-002 — NAP and NIP revision status
- FACT-003 — Linux-neutral nampplets reuse
- FACT-004 — NMP facade and ownership
- FACT-005 — Tauri/WebKit child-frame authority
- FACT-006 — strict CSP and direct egress
- FACT-007 — local daemon IPC
- FACT-008 — executable Linux toolchain
- FACT-009 — Linux exact-build runner
- FACT-010 — portable napplets and queryless INC
- FACT-011 — daemon-owned runtime and canonical NMP data
- FACT-012 — bounded multi-surface composition
- FACT-013 — hostile Linux child boundary
- FACT-014 — public identity, resource, and naddr catalog flow
- files
- Box
- Error
- PathBuf
- Result
- String
- Vec
- Arc
- Drop
- Mutex
- Option
- Result
- String
- Vec
- BTreeMap
- Option
- PathBuf
- Result
- Self
- String
- Vec
- Arc
- AsRef
- Box
- BTreeMap
- Error
- Mutex
- Option
- Path
- PathBuf
- Result
- Self
- String
- T
- UnixStream
- Vec
- Option
- Arc
- Error
- Result
- Self
- String
- T
- Vec
- AsRef
- BTreeMap
- Drop
- Error
- Option
- Path
- PathBuf
- Result
- Self
- String
- UnixStream
- Vec
- VecDeque
- Path
- Uzel
- Project State
- POC architecture
- Verified facts
- Gate 0 preflight and Slice 01 decision

## God Nodes (most connected - your core abstractions)
1. `LinuxRunner` - 38 edges
2. `UnixClient` - 33 edges
3. `RunnerError` - 24 edges
4. `ClientError` - 22 edges
5. `validate_record()` - 20 edges
6. `Response` - 19 edges
7. `scripts` - 18 edges
8. `CheckError` - 18 edges
9. `candidate_record()` - 16 edges
10. `HostileProbeState` - 15 edges

## Surprising Connections (you probably didn't know these)
- `clearObjectUrls()` --indirect_call--> `row()`  [INFERRED]
  napplets/follow-list/src/main.js → contracts/kind0-profile.test.mjs
- `createAvatarObserver()` --indirect_call--> `row()`  [INFERRED]
  napplets/follow-list/src/main.js → contracts/kind0-profile.test.mjs
- `ConfirmNappletError` --references--> `ClientError`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `ReviewNappletError` --references--> `ClientError`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs
- `read_runtime_status()` --references--> `UnixClient`  [EXTRACTED]
  apps/uzel/src-tauri/src/main.rs → crates/napd-protocol/src/lib.rs

## Import Cycles
- None detected.

## Communities (191 total, 103 thin omitted)

### Community 0 - "UnixClient"
Cohesion: 0.06
Nodes (60): DecodeError, Into, Read, SurfaceLaunch, UnixListener, authoritative_reconciliation_retires_ambiguous_operation_ids(), CatalogCapability, chunked_routed_envelope_reassembles_on_one_connection() (+52 more)

### Community 1 - "trusted-shell.js"
Cohesion: 0.25
Nodes (18): boundedJSON(), compatibilityPreludeSource(), decodedBase64Length(), decodeResourceBlob(), exactFields(), isBoundedEnvelope(), isPlainObject(), isVerifiedArtifactBaseURL() (+10 more)

### Community 2 - "check-napplet-imports.mjs"
Cohesion: 0.06
Nodes (37): allowedDependencyTargets, allowedGuardedGlobalAccesses, { compile: compileSvelte, parse: parseSvelte }, contractsRoot, declarativeNetworkViolations(), dependencyGroups, dependencyViolations(), directNetworkIdentifiers (+29 more)

### Community 3 - "LinuxRunner"
Cohesion: 0.10
Nodes (19): BTreeSet, Debug, Formatter, FromUtf8Error, RuntimeAccountHandle, RuntimeController, RuntimeObservation, RuntimeRelayDiagnosticsObservation (+11 more)

### Community 4 - "Delivery, quality, review and packaging discipline"
Cohesion: 0.11
Nodes (18): Anti-Patterns, Architectural Constraints, Architecture, Catalog Review and Exact-Build Install, Component Responsibilities, Cross-Cutting Concerns, Cross-Napplet Profile Flow, Data Flow (+10 more)

### Community 5 - "runner.rs"
Cohesion: 0.13
Nodes (11): RuntimeRelayLane, absent_surface_cleanup_is_idempotent(), catalog_cancellation_is_terminal(), catalog_cancellation_keeps_retryable_reviews_and_discards_terminal_stale_tokens(), hostile_probe_commits_exact_session_config_before_returning(), pending_review_tokens_are_sorted_bounded_and_reconcilable(), ProductState, relay_lane_name() (+3 more)

### Community 6 - "Uzel product and incubation architecture"
Cohesion: 0.13
Nodes (14): Blossom and Authoring, Canonical Nix Package, Definition of Done, Later Platforms and Capabilities, Lean Delivery and Review, Local Files, Out of Scope, POC Reference and Napp Seam (+6 more)

### Community 7 - "EventBuffer"
Cohesion: 0.18
Nodes (12): Condvar, Fn, RuntimeEvent, RuntimeObservationFrame, RuntimeObserver, buffered_responses_are_byte_bounded_and_consumed_once(), BufferedEvents, event_bytes() (+4 more)

### Community 8 - ".forward_from_surface"
Cohesion: 0.21
Nodes (11): eventually_identity_query(), identity_query(), inc_emit_waits_for_an_inc_event_not_an_unrelated_push(), launch_identity_surface(), payload_identity_cannot_select_surface_or_session(), profile_open_crosses_inc_with_runtime_owned_sender(), public_identity_profile_follows_and_picture_cross_only_native_providers(), ResponseExpectation (+3 more)

### Community 9 - "Material findings and corrections"
Cohesion: 0.22
Nodes (3): Decision, Source baseline, FACT-XXX — title

### Community 10 - "A5 — mandatory post-M5 whole-system audit"
Cohesion: 0.18
Nodes (10): Codebase Concerns, Dependencies at Risk, Fragile Areas, Known Bugs, Missing Critical Features, Performance Bottlenecks, Scaling Limits, Security Considerations (+2 more)

### Community 11 - "M0 / GSD Phase 1 — truthful baseline and replay contract"
Cohesion: 0.20
Nodes (9): Code Style, Coding Conventions, Comments, Error Handling, Function Design, Import Organization, Logging, Module Design (+1 more)

### Community 12 - "Production engineering dimensions"
Cohesion: 0.20
Nodes (9): Common Patterns, Coverage, Fixtures and Factories, Mocking, Test File Organization, Test Framework, Test Structure, Test Types (+1 more)

### Community 13 - "main.js"
Cohesion: 0.06
Nodes (69): canonicalProfile(), canonicalProfiles(), optionalText(), profileQueryBatches(), profileQueryRequest(), retryProfileQueryRequests(), splitProfileQueryRequest(), A (+61 more)

### Community 14 - "fixtures.rs"
Cohesion: 0.22
Nodes (6): ArtifactFetchRequest, ArtifactFetchResponse, ArtifactSource, ExactFixtureSource, fixture_by_name(), FixtureDefinition

### Community 15 - "compilerOptions"
Cohesion: 0.08
Nodes (23): DOM, DOM.Iterable, ES2023, src/**/*.svelte, src/**/*.ts, vite/client, vite.config.ts, compilerOptions (+15 more)

### Community 16 - "package.json"
Cohesion: 0.07
Nodes (27): svelte, vite, playwright, svelte-check, @sveltejs/vite-plugin-svelte, @tauri-apps/api, typescript, dependencies (+19 more)

### Community 17 - "scripts"
Cohesion: 0.06
Nodes (35): fallow, @napplet/cli, devDependencies, fallow, @napplet/cli, engines, node, name (+27 more)

### Community 18 - "Fast-moving ecosystem, compatibility and upstream stewardship"
Cohesion: 0.22
Nodes (8): Acceptance checks, Appetite, Boundaries, Dependencies, Lean process reset, No-gos, Outcome, Risks

### Community 19 - "Decision, nuance and educational-knowledge system"
Cohesion: 0.22
Nodes (8): APIs & External Services, Authentication & Identity, CI/CD & Deployment, Data Storage, Environment Configuration, External Integrations, Monitoring & Observability, Webhooks & Callbacks

### Community 20 - "M5 / GSD delivery phases 7, 7.1–7.9 — production-candidate hardening and audit freeze"
Cohesion: 0.25
Nodes (7): Configuration, Frameworks, Key Dependencies, Languages, Platform Requirements, Runtime, Technology Stack

### Community 21 - "default.json"
Cohesion: 0.25
Nodes (7): core:default, main, description, identifier, permissions, $schema, windows

### Community 22 - "acceptance.test.mjs"
Cohesion: 0.07
Nodes (40): ACTION_IDS, bindingFromEvent(), bindingMatches(), DEFAULT_KEYBINDINGS, defaultPreferences(), KEYBINDING_ACTIONS, parsePreferences(), validateKeybindings() (+32 more)

### Community 23 - "Uzel agent instructions"
Cohesion: 0.25
Nodes (7): Codebase Structure, Directory Layout, Directory Purposes, Key File Locations, Naming Conventions, Special Directories, Where to Add New Code

### Community 24 - "linux-run-smoke.sh"
Cohesion: 0.16
Nodes (17): cleanup(), GDK_BACKEND, hostile_markers_are_ordered(), NO_AT_BRIDGE, preserve_failure(), preserve_logs(), report_marker(), report_marker_state() (+9 more)

### Community 27 - "tauri.conf.json"
Cohesion: 0.12
Nodes (16): app, security, windows, build, beforeBuildCommand, beforeDevCommand, devUrl, frontendDist (+8 more)

### Community 29 - "debian13-live-test.sh"
Cohesion: 0.20
Nodes (13): CARGO_INCREMENTAL, CARGO_PROFILE_DEV_DEBUG, fail(), record_prebuild(), reexec_with_nix_group(), run_cache_probe(), run_startup_step(), debian13-live-test.sh script (+5 more)

### Community 34 - "Upstream contribution ledger"
Cohesion: 0.25
Nodes (7): Active contributions, Authority and ownership, Entry template, Slice 02 upstream result, Slice 03 upstream result, Slice 06 upstream result, Upstream contribution ledger

### Community 37 - "parse_options"
Cohesion: 0.33
Nodes (11): IntoIterator, Item, Iterator, default_runtime_root(), default_socket_path(), live_configuration_requires_explicit_live_mode(), main(), next_path() (+3 more)

### Community 38 - "App.svelte"
Cohesion: 0.25
Nodes (3): @tauri-apps/api/core, ./preferences.js, ./projection-failure.js

### Community 39 - "POC status"
Cohesion: 0.33
Nodes (5): Accepted provisional risks, Gate 0 — validated baseline, Implementation, Latest integrated evidence, POC status

### Community 40 - "POC scope and acceptance"
Cohesion: 0.17
Nodes (11): Accepted post-foundation extension, Architectural invariants, Demo-complete, Foundation-complete, Non-goals, Objective, POC scope and acceptance, Required napplets (+3 more)

### Community 41 - "Assumption validation and decision gates"
Cohesion: 0.22
Nodes (9): Assumption validation and decision gates, Current observed baseline, Gate 0 decision, Gate matrix, Hard stops, Per-slice validation, Plan correction rule, Required outputs (+1 more)

### Community 45 - "Debian 13 live test"
Cohesion: 0.40
Nodes (4): 1. Clone and install the launcher, 2. Run automated real-WebKit acceptance, 3. Run visible desktop demo, Debian 13 live test

### Community 47 - "ref-candidate-check.py"
Cohesion: 0.13
Nodes (50): Any, CompletedProcess, Namespace, RuntimeError, TemporaryDirectory, approved_reachability(), candidate_argv_ok(), candidate_record() (+42 more)

### Community 49 - "package.json"
Cohesion: 0.10
Nodes (20): @napplet/conformance-cli, @napplet/nap, @napplet/shim, @napplet/vite-plugin, vite, dependencies, @napplet/nap, @napplet/shim (+12 more)

### Community 50 - "trusted-shell-policy.js"
Cohesion: 0.83
Nodes (3): directive(), innerPolicyContent(), outerPolicyContent()

### Community 51 - "createSurfaceHost"
Cohesion: 0.24
Nodes (5): createSurfaceHost(), mount(), receive(), unmount(), Window

### Community 55 - "Tests, quality gates, and demo"
Cohesion: 0.20
Nodes (9): Deterministic demo, Final acceptance, Hostile frame, Live demo, Napplet/web, Quality commands, Required test layers, Runtime/Rust (+1 more)

### Community 58 - "package.json"
Cohesion: 0.10
Nodes (20): @napplet/conformance-cli, @napplet/nap, @napplet/shim, @napplet/vite-plugin, vite, dependencies, @napplet/nap, @napplet/shim (+12 more)

### Community 59 - "package.json"
Cohesion: 0.11
Nodes (17): @napplet/nap, @napplet/shim, @napplet/vite-plugin, vite, dependencies, @napplet/nap, @napplet/shim, devDependencies (+9 more)

### Community 60 - "profile-open-v1.schema.json"
Cohesion: 0.13
Nodes (14): pubkey, version, additionalProperties, $id, properties, pubkey, version, pattern (+6 more)

### Community 62 - "main.rs"
Cohesion: 0.07
Nodes (45): AtomicU64, FnMut, From, State, TauriPlugin, Url, HostileProbeState, allowed_navigation() (+37 more)

### Community 63 - "Slice 03 preflight"
Cohesion: 0.20
Nodes (10): API and ownership evidence, Automated review corrections, Commands and observed results, Exact pins and fixtures, Hostile fixture scope, Manifest correction, Next step, Slice 03 preflight (+2 more)

### Community 64 - "probes.test.mjs"
Cohesion: 0.13
Nodes (12): denied(), results, target, attemptRawWebKitInvoke(), boundedAttempt(), nativeSurface(), PROBE_NAMES, sentinelTargets() (+4 more)

### Community 66 - "linux-smoke-script.test.mjs"
Cohesion: 0.67
Nodes (3): matchesLog(), napdReadyPattern(), script

### Community 68 - "Uzel POC agent instructions"
Cohesion: 0.22
Nodes (9): Engineering rules, Mission, POC exclusions, Quality gate, Repository boundaries, Required method, Trust rules, Upstream contribution policy (+1 more)

### Community 69 - "Provisional component design"
Cohesion: 0.22
Nodes (9): Developer mode, Exact-build fixtures, Identity and Nostr reads, Local daemon protocol, Napplet convention, Provisional component design, Runtime state, Shell UI (+1 more)

### Community 70 - "hostile_probe.rs"
Cohesion: 0.16
Nodes (15): AtomicBool, AtomicUsize, JoinHandle, accepted_report(), BeaconAttempt, control_accept_is_not_counted_as_a_probe_connection(), exact_surface_cancellation_retires_the_attached_probe(), HostileProbeReport (+7 more)

### Community 71 - "Slice 02 preflight"
Cohesion: 0.22
Nodes (9): Commands and observed results, Exact dependency and asset record, Next step, Preserved failed Fedora probe, Required design correction, Runtime evidence, Slice 02 preflight, Upstream result (+1 more)

### Community 72 - "Slice 05 preflight — integrated composed demo"
Cohesion: 0.22
Nodes (8): Commands and results, Exact dependency and upstream evidence, Exact next step, Honest boundary, Linux shell evidence, Outcome, Runtime composition evidence, Slice 05 preflight — integrated composed demo

### Community 81 - "AcceptSettings"
Cohesion: 0.38
Nodes (5): NativeSettingsExecutor, NativeSettingsOpenResult, NativeSettingsRequest, AcceptSettings, UnavailableSettings

### Community 82 - "RelayDiagnosticsSink"
Cohesion: 0.50
Nodes (3): RuntimeRelayDiagnosticsObserver, RuntimeRelayDiagnosticsSnapshot, RelayDiagnosticsSink

### Community 83 - "debian13-setup.sh"
Cohesion: 0.46
Nodes (7): fail(), print_nix_builder_state(), print_nix_daemon_state(), refresh_nix_builder_state(), refresh_nix_daemon_state(), debian13-setup.sh script, wait_for_nix_daemon_socket()

### Community 84 - "resource.rs"
Cohesion: 0.09
Nodes (27): Cancellation, Client, Duration, Future, Instant, IpAddr, Output, PinnedHttpsRequest (+19 more)

### Community 85 - "POC documentation audit"
Cohesion: 0.25
Nodes (7): Confidence, Corrections, Diagram review, Gate 0 resolution, POC documentation audit, Problems found in the previous pack, Verdict

### Community 86 - "Post-POC extraction"
Cohesion: 0.25
Nodes (7): First hardening follow-ups, Likely `kehto/napd`, Moves to `jodobear/napplets`, POC shortcuts that must not silently become platform contracts, Post-POC extraction, Remains in Uzel, Rewrite criteria

### Community 87 - "nampplets Linux reuse map"
Cohesion: 0.25
Nodes (7): Apple-only edges, Build evidence, Compatibility candidate, Crate map, nampplets Linux reuse map, Result, Runtime entry points

### Community 88 - "Slice 01 preflight"
Cohesion: 0.25
Nodes (8): Boundary result, Commands and observed results, Debian probe correction, Fedora probe correction, Locked workspace, Next step, Slice 01 preflight, Verdict

### Community 89 - "Slice 04 preflight — daemon, NMP, and persistence"
Cohesion: 0.25
Nodes (7): Commands and results, Exact next step, NMP and persistence evidence, Outcome, Private protocol evidence, Remaining boundaries, Slice 04 preflight — daemon, NMP, and persistence

### Community 90 - "Slice 06 preflight — hardening and clean demo acceptance"
Cohesion: 0.22
Nodes (8): Bubblewrap decision, Commands and results, Exact hostile evidence, Failed evidence and toolchain limit, Go/no-go and exact next steps, Outcome, Slice 06 preflight — hardening and clean demo acceptance, Upstream result

### Community 91 - "WebKit/Tauri trust spike"
Cohesion: 0.25
Nodes (7): Accepted child CSP, Boundary of proof, Executable probe, Low-level handler nuance, Source verification, Verdict, WebKit/Tauri trust spike

### Community 92 - "mock-native.js"
Cohesion: 0.24
Nodes (9): diagnostics(), invoke(), isRoutedProfileQuery(), nativeEnvelope(), profileEvent(), profileEventsForQuery(), profileFor(), review() (+1 more)

### Community 93 - "Work 00 — validate assumptions"
Cohesion: 0.25
Nodes (7): Done when, Goal, Outputs, Read, Stop conditions, Tasks, Work 00 — validate assumptions

### Community 94 - "Work 01 — scaffold"
Cohesion: 0.25
Nodes (8): Acceptance, Depends on, Entry status, Goal, Non-goals, Status, Tasks, Work 01 — scaffold

### Community 95 - "Work 07 — issue-driven stabilization"
Cohesion: 0.25
Nodes (7): Active issue — #19, Completed issue — #10, Entry evidence, Exit rule, Goal, Work 07 — issue-driven stabilization, Work graph

### Community 98 - "NMP API and ownership map"
Cohesion: 0.29
Nodes (6): Accepted pin, Executable probe, nampplets adapter seam, NMP API and ownership map, Ownership boundary, Public facade

### Community 99 - "Work 02 — Linux exact-build runner"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 02 — Linux exact-build runner

### Community 100 - "Work 04 — daemon, NMP, and persistence"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 04 — daemon, NMP, and persistence

### Community 118 - "Execution slices"
Cohesion: 0.33
Nodes (6): Dependency graph, Execution slices, Handoff, Parallel work, Slice entry gate, Slices

### Community 120 - "Uzel single-repository POC"
Cohesion: 0.33
Nodes (6): Demo result, Document map, Mandatory first step, Scope rule, Start order, Uzel single-repository POC

### Community 121 - "Work 03 — portable napplets"
Cohesion: 0.33
Nodes (6): Acceptance, Entry status and pins, Goal, Read, Tasks, Work 03 — portable napplets

### Community 122 - "Work 05 — composed demo"
Cohesion: 0.33
Nodes (5): Acceptance, Goal, Non-goals, Tasks, Work 05 — composed demo

### Community 124 - "audit_docs.py"
Cohesion: 0.70
Nodes (4): main(), manifest_paths(), strip_code(), write_manifest()

### Community 125 - "Work 06 — hardening and demo acceptance"
Cohesion: 0.40
Nodes (4): Acceptance, Goal, Tasks, Work 06 — hardening and demo acceptance

### Community 126 - "fedora-run-smoke.sh"
Cohesion: 0.50
Nodes (3): fedora-run-smoke.sh script, UZEL_SMOKE_NAME, UZEL_SMOKE_SUCCESS_MARKER

### Community 230 - "Uzel"
Cohesion: 0.08
Nodes (20): Uzel agent instructions, Active, Constraints, Context, Core Value, Key Decisions, Out of Scope, Requirements (+12 more)

### Community 239 - "POC architecture"
Cohesion: 0.22
Nodes (9): Accepted upstream seam, Composition flow, POC architecture, Repository zones, Runtime topology, Session start, Shared Nostr flow, Trust domains (+1 more)

### Community 240 - "Verified facts"
Cohesion: 0.29
Nodes (7): Kehto #204, `nampplets`, NAP registry, Napplet packages, NIP-5A and NIP-5D, NMP, Verified facts

### Community 241 - "Gate 0 preflight and Slice 01 decision"
Cohesion: 0.22
Nodes (9): Accepted provisional risks, Confirmed assumptions, Decision, Exact next steps, Gate 0 preflight and Slice 01 decision, Gate results, Rejected assumptions, Required design changes (+1 more)

## Knowledge Gaps
- **529 isolated node(s):** `name`, `private`, `version`, `type`, `dev:web` (+524 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **103 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `LinuxRunner` connect `LinuxRunner` to `.forward_from_surface`, `UnixClient`, `runner.rs`, `EventBuffer`?**
  _High betweenness centrality (0.016) - this node is a cross-community bridge._
- **Why does `linux_resource_provider()` connect `resource.rs` to `LinuxRunner`?**
  _High betweenness centrality (0.013) - this node is a cross-community bridge._
- **What connects `name`, `private`, `version` to the rest of the system?**
  _529 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `UnixClient` be split into smaller, more focused modules?**
  _Cohesion score 0.061419753086419754 - nodes in this community are weakly interconnected._
- **Should `check-napplet-imports.mjs` be split into smaller, more focused modules?**
  _Cohesion score 0.06155632984901278 - nodes in this community are weakly interconnected._
- **Should `LinuxRunner` be split into smaller, more focused modules?**
  _Cohesion score 0.1039136302294197 - nodes in this community are weakly interconnected._
- **Should `Delivery, quality, review and packaging discipline` be split into smaller, more focused modules?**
  _Cohesion score 0.10526315789473684 - nodes in this community are weakly interconnected._