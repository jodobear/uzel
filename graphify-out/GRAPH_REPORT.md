# Graph Report - uzel-issue21  (2026-07-31)

## Corpus Check
- 129 files · ~85,567 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1244 nodes · 1768 edges · 178 communities (79 shown, 99 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 13 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `5c45fa67`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- HostileProbeState
- Assumption validation and decision gates
- check-napplet-imports.mjs
- Provisional component design
- Tests, quality gates, and demo
- Uzel POC agent instructions
- POC architecture
- POC architecture
- POC documentation audit
- Work 01 — scaffold
- Work 00 — validate assumptions
- 04-execution.md
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
- Slice 03 preflight
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
- build.rs
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
- Verified facts
- Execution slices
- Uzel single-repository POC
- Work 03 — portable napplets
- POC status
- build-signed-napplet-fixtures.sh
- ClientError
- Request
- Exact signed Work 02 fixture
- AcceptSettings
- RelayDiagnosticsSink
- FACT-011-daemon-nmp.md
- resource.rs
- FACT-012-composed-demo.md
- README.md
- Verified facts
- debian13-live-test.sh
- FACT-001 — Kehto PR 204 and the 0.29 package line
- debian13-setup.sh
- Work 02 — Linux exact-build runner
- mock-native.js
- FACT-006 — strict CSP and direct egress
- FACT-004 — NMP facade and ownership
- Work 04 — daemon, NMP, and persistence
- FACT-014-live-identity-catalog.md
- Work 05 — composed demo
- POC status
- Upstream contribution ledger
- FACT-009 — Linux exact-build runner
- FACT-010 — portable napplets and queryless INC
- Work 07 — issue-driven stabilization
- FACT-011 — daemon-owned runtime and canonical NMP data
- FACT-012 — bounded multi-surface composition
- FACT-013 — hostile Linux child boundary
- FACT-014 — public identity, resource, and naddr catalog flow
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
- Vec
- Option
- lib.rs
- Arc
- Error
- Result
- Self
- String
- T
- Vec
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
- server.rs
- Tests, quality gates, and demo
- .fmt

## God Nodes (most connected - your core abstractions)
1. `LinuxRunner` - 38 edges
2. `UnixClient` - 33 edges
3. `RunnerError` - 24 edges
4. `ClientError` - 22 edges
5. `Response` - 19 edges
6. `scripts` - 18 edges
7. `HostileProbeState` - 15 edges
8. `compilerOptions` - 15 edges
9. `write_frame()` - 15 edges
10. `readVerifiedCorpus()` - 13 edges

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

## Communities (178 total, 99 thin omitted)

### Community 0 - "HostileProbeState"
Cohesion: 0.13
Nodes (18): AtomicBool, AtomicU64, AtomicUsize, JoinHandle, accepted_report(), BeaconAttempt, control_accept_is_not_counted_as_a_probe_connection(), exact_surface_cancellation_retires_the_attached_probe() (+10 more)

### Community 1 - "Assumption validation and decision gates"
Cohesion: 0.25
Nodes (18): boundedJSON(), compatibilityPreludeSource(), decodedBase64Length(), decodeResourceBlob(), exactFields(), isBoundedEnvelope(), isPlainObject(), isVerifiedArtifactBaseURL() (+10 more)

### Community 2 - "check-napplet-imports.mjs"
Cohesion: 0.06
Nodes (37): allowedDependencyTargets, allowedGuardedGlobalAccesses, { compile: compileSvelte, parse: parseSvelte }, contractsRoot, declarativeNetworkViolations(), dependencyGroups, dependencyViolations(), directNetworkIdentifiers (+29 more)

### Community 3 - "Provisional component design"
Cohesion: 0.13
Nodes (11): RuntimeRelayLane, absent_surface_cleanup_is_idempotent(), catalog_cancellation_is_terminal(), catalog_cancellation_keeps_retryable_reviews_and_discards_terminal_stale_tokens(), hostile_probe_commits_exact_session_config_before_returning(), pending_review_tokens_are_sorted_bounded_and_reconcilable(), ProductState, relay_lane_name() (+3 more)

### Community 4 - "Tests, quality gates, and demo"
Cohesion: 0.15
Nodes (27): canonicalDirectory(), CorpusVerificationError, DEFAULT_CORPUS_LOCK, exactTag(), EXPECTED_DOMAINS, EXPECTED_EVENT_IDS, EXPECTED_SAFE_AUTOMATION, infrastructure() (+19 more)

### Community 5 - "Uzel POC agent instructions"
Cohesion: 0.18
Nodes (8): Evidence boundary, Renderer acceptance harness, 1. Clone and install the launcher, 2. Run automated real-WebKit acceptance, 3. Run visible desktop demo, Debian 13 live test, Development, Uzel

### Community 6 - "POC architecture"
Cohesion: 0.16
Nodes (17): cleanup(), GDK_BACKEND, hostile_markers_are_ordered(), NO_AT_BRIDGE, preserve_failure(), preserve_logs(), report_marker(), report_marker_state() (+9 more)

### Community 7 - "POC architecture"
Cohesion: 0.18
Nodes (12): Condvar, Fn, RuntimeEvent, RuntimeObservationFrame, RuntimeObserver, buffered_responses_are_byte_bounded_and_consumed_once(), BufferedEvents, event_bytes() (+4 more)

### Community 8 - "POC documentation audit"
Cohesion: 0.21
Nodes (11): Value, eventually_identity_query(), identity_query(), inc_emit_waits_for_an_inc_event_not_an_unrelated_push(), launch_identity_surface(), payload_identity_cannot_select_surface_or_session(), profile_open_crosses_inc_with_runtime_owned_sender(), public_identity_profile_follows_and_picture_cross_only_native_providers() (+3 more)

### Community 9 - "Work 01 — scaffold"
Cohesion: 0.12
Nodes (16): app, security, windows, build, beforeBuildCommand, beforeDevCommand, devUrl, frontendDist (+8 more)

### Community 10 - "Work 00 — validate assumptions"
Cohesion: 0.20
Nodes (13): CARGO_INCREMENTAL, CARGO_PROFILE_DEV_DEBUG, fail(), record_prebuild(), reexec_with_nix_group(), run_cache_probe(), run_startup_step(), debian13-live-test.sh script (+5 more)

### Community 11 - "04-execution.md"
Cohesion: 0.16
Nodes (5): Slice handoff, Acceptance, Goal, Tasks, Work 06 — hardening and demo acceptance

### Community 13 - "Work 04 — daemon, NMP, and persistence"
Cohesion: 0.09
Nodes (33): isCanonicalPubkey(), parseProfileOpen(), profileOpen(), PUBKEY, list, loadFollows(), refresh, render() (+25 more)

### Community 14 - ".fetch"
Cohesion: 0.22
Nodes (6): ArtifactFetchRequest, ArtifactFetchResponse, ArtifactSource, ExactFixtureSource, fixture_by_name(), FixtureDefinition

### Community 15 - "Uzel single-repository POC"
Cohesion: 0.08
Nodes (23): DOM, DOM.Iterable, ES2023, src/**/*.svelte, src/**/*.ts, vite/client, vite.config.ts, compilerOptions (+15 more)

### Community 16 - "POC status"
Cohesion: 0.07
Nodes (27): svelte, vite, playwright, svelte-check, @sveltejs/vite-plugin-svelte, @tauri-apps/api, typescript, dependencies (+19 more)

### Community 17 - "Work 05 — composed demo"
Cohesion: 0.06
Nodes (35): fallow, @napplet/cli, devDependencies, fallow, @napplet/cli, engines, node, name (+27 more)

### Community 18 - "Work 06 — hardening and demo acceptance"
Cohesion: 0.17
Nodes (11): Accepted post-foundation extension, Architectural invariants, Demo-complete, Foundation-complete, Non-goals, Objective, POC scope and acceptance, Required napplets (+3 more)

### Community 19 - "audit_docs.py"
Cohesion: 0.20
Nodes (9): Developer mode, Exact-build fixtures, Identity and Nostr reads, Local daemon protocol, Napplet convention, Provisional component design, Runtime state, Shell UI (+1 more)

### Community 21 - "README.md"
Cohesion: 0.25
Nodes (7): core:default, main, description, identifier, permissions, $schema, windows

### Community 22 - "Gate 0 preflight and Slice 01 decision"
Cohesion: 0.07
Nodes (40): ACTION_IDS, bindingFromEvent(), bindingMatches(), DEFAULT_KEYBINDINGS, defaultPreferences(), KEYBINDING_ACTIONS, parsePreferences(), validateKeybindings() (+32 more)

### Community 23 - "Slice 03 preflight"
Cohesion: 0.20
Nodes (10): API and ownership evidence, Automated review corrections, Commands and observed results, Exact pins and fixtures, Hostile fixture scope, Manifest correction, Next step, Slice 03 preflight (+2 more)

### Community 24 - "WebKit/Tauri trust spike"
Cohesion: 0.22
Nodes (9): Engineering rules, Mission, POC exclusions, Quality gate, Repository boundaries, Required method, Trust rules, Upstream contribution policy (+1 more)

### Community 25 - "NMP API and ownership map"
Cohesion: 0.22
Nodes (9): Assumption validation and decision gates, Current observed baseline, Gate 0 decision, Gate matrix, Hard stops, Per-slice validation, Plan correction rule, Required outputs (+1 more)

### Community 26 - "FACT-001-kehto-package-line.md"
Cohesion: 0.22
Nodes (9): Accepted upstream seam, Composition flow, POC architecture, Repository zones, Runtime topology, Session start, Shared Nostr flow, Trust domains (+1 more)

### Community 27 - "FACT-002-spec-revisions.md"
Cohesion: 0.22
Nodes (9): Accepted provisional risks, Confirmed assumptions, Decision, Exact next steps, Gate 0 preflight and Slice 01 decision, Gate results, Rejected assumptions, Required design changes (+1 more)

### Community 28 - "FACT-003-nampplets-linux-reuse.md"
Cohesion: 0.22
Nodes (9): Commands and observed results, Exact dependency and asset record, Next step, Preserved failed Fedora probe, Required design correction, Runtime evidence, Slice 02 preflight, Upstream result (+1 more)

### Community 29 - "FACT-004-nmp-facade.md"
Cohesion: 0.22
Nodes (8): Commands and results, Exact dependency and upstream evidence, Exact next step, Honest boundary, Linux shell evidence, Outcome, Runtime composition evidence, Slice 05 preflight — integrated composed demo

### Community 30 - "FACT-005-webkit-tauri-trust.md"
Cohesion: 0.25
Nodes (8): Bubblewrap decision, Commands and results, Exact hostile evidence, Failed evidence and toolchain limit, Go/no-go and exact next steps, Outcome, Slice 06 preflight — hardening and clean demo acceptance, Upstream result

### Community 31 - "FACT-006-csp-egress.md"
Cohesion: 0.46
Nodes (7): fail(), print_nix_builder_state(), print_nix_daemon_state(), refresh_nix_builder_state(), refresh_nix_daemon_state(), debian13-setup.sh script, wait_for_nix_daemon_socket()

### Community 32 - "FACT-007-local-ipc.md"
Cohesion: 0.25
Nodes (7): Confidence, Corrections, Diagram review, Gate 0 resolution, POC documentation audit, Problems found in the previous pack, Verdict

### Community 33 - "FACT-008-toolchain.md"
Cohesion: 0.29
Nodes (7): First hardening follow-ups, Likely `kehto/napd`, Moves to `jodobear/napplets`, POC shortcuts that must not silently become platform contracts, Post-POC extraction, Remains in Uzel, Rewrite criteria

### Community 34 - "fedora-run-smoke.sh"
Cohesion: 0.25
Nodes (7): Apple-only edges, Build evidence, Compatibility candidate, Crate map, nampplets Linux reuse map, Result, Runtime entry points

### Community 35 - "debian-build-smoke.sh"
Cohesion: 0.25
Nodes (8): Boundary result, Commands and observed results, Debian probe correction, Fedora probe correction, Locked workspace, Next step, Slice 01 preflight, Verdict

### Community 36 - "dev.sh"
Cohesion: 0.25
Nodes (7): Commands and results, Exact next step, NMP and persistence evidence, Outcome, Private protocol evidence, Remaining boundaries, Slice 04 preflight — daemon, NMP, and persistence

### Community 37 - "main.rs"
Cohesion: 0.33
Nodes (11): IntoIterator, Item, Iterator, default_runtime_root(), default_socket_path(), live_configuration_requires_explicit_live_mode(), main(), next_path() (+3 more)

### Community 39 - "build.rs"
Cohesion: 0.25
Nodes (7): Accepted child CSP, Boundary of proof, Executable probe, Low-level handler nuance, Source verification, Verdict, WebKit/Tauri trust spike

### Community 40 - "Post-POC extraction"
Cohesion: 0.25
Nodes (7): Done when, Goal, Outputs, Read, Stop conditions, Tasks, Work 00 — validate assumptions

### Community 41 - "lib.rs"
Cohesion: 0.10
Nodes (33): DecodeError, Read, authoritative_reconciliation_retires_ambiguous_operation_ids(), CatalogCapability, chunked_routed_envelope_reassembles_on_one_connection(), ClientError, confirm_responses_lost_replay_surface_without_a_second_operation(), decode_asset_chunk() (+25 more)

### Community 42 - "README.md"
Cohesion: 0.25
Nodes (8): Acceptance, Depends on, Entry status, Goal, Non-goals, Status, Tasks, Work 01 — scaffold

### Community 43 - "README.md"
Cohesion: 0.20
Nodes (9): Decision, Kehto #204, `nampplets`, NAP registry, Napplet packages, NIP-5A and NIP-5D, NMP, Source baseline (+1 more)

### Community 44 - "check-boundaries.sh"
Cohesion: 0.29
Nodes (7): Active contributions, Authority and ownership, Entry template, Slice 02 upstream result, Slice 03 upstream result, Slice 06 upstream result, Upstream contribution ledger

### Community 45 - "smoke.sh"
Cohesion: 0.29
Nodes (6): Accepted pin, Executable probe, nampplets adapter seam, NMP API and ownership map, Ownership boundary, Public facade

### Community 47 - "lib.rs"
Cohesion: 0.14
Nodes (12): Debug, FromUtf8Error, RuntimeAccountHandle, RuntimeObservation, RuntimeRelayDiagnosticsObservation, artifact_base_url(), bounded_diagnostic(), LinuxRunner (+4 more)

### Community 48 - "POC documentation audit"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 02 — Linux exact-build runner

### Community 49 - "ExactFixtureSource"
Cohesion: 0.10
Nodes (20): @napplet/conformance-cli, @napplet/nap, @napplet/shim, @napplet/vite-plugin, vite, dependencies, @napplet/nap, @napplet/shim (+12 more)

### Community 50 - "trusted-shell-policy.js"
Cohesion: 0.83
Nodes (3): directive(), innerPolicyContent(), outerPolicyContent()

### Community 51 - "Window"
Cohesion: 0.24
Nodes (5): createSurfaceHost(), mount(), receive(), unmount(), Window

### Community 52 - "check-pinned-assets.sh"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 04 — daemon, NMP, and persistence

### Community 53 - "README.md"
Cohesion: 0.33
Nodes (5): Deliberate refresh, External napplet interoperability corpus, Failure meaning, Offline verification, Provenance

### Community 54 - "README.md"
Cohesion: 0.33
Nodes (6): Dependency graph, Execution slices, Handoff, Parallel work, Slice entry gate, Slices

### Community 55 - "FACT-009-linux-runner.md"
Cohesion: 0.33
Nodes (6): Demo result, Document map, Mandatory first step, Scope rule, Start order, Uzel single-repository POC

### Community 58 - "package.json"
Cohesion: 0.10
Nodes (20): @napplet/conformance-cli, @napplet/nap, @napplet/shim, @napplet/vite-plugin, vite, dependencies, @napplet/nap, @napplet/shim (+12 more)

### Community 59 - "package.json"
Cohesion: 0.11
Nodes (17): @napplet/nap, @napplet/shim, @napplet/vite-plugin, vite, dependencies, @napplet/nap, @napplet/shim, devDependencies (+9 more)

### Community 60 - "profile-open-v1.schema.json"
Cohesion: 0.13
Nodes (14): pubkey, version, additionalProperties, $id, properties, pubkey, version, pattern (+6 more)

### Community 61 - "Assumption validation and decision gates"
Cohesion: 0.33
Nodes (6): Acceptance, Entry status and pins, Goal, Read, Tasks, Work 03 — portable napplets

### Community 62 - "Slice 02 preflight"
Cohesion: 0.09
Nodes (38): FnMut, From, State, TauriPlugin, Url, allowed_navigation(), ambiguous_confirmation_crosses_as_a_typed_retry_state(), ambiguous_review_crosses_as_a_typed_retry_state() (+30 more)

### Community 63 - "Work 00 — validate assumptions"
Cohesion: 0.33
Nodes (5): Acceptance, Goal, Non-goals, Tasks, Work 05 — composed demo

### Community 64 - "main.js"
Cohesion: 0.13
Nodes (12): denied(), results, target, attemptRawWebKitInvoke(), boundedAttempt(), nativeSurface(), PROBE_NAMES, sentinelTargets() (+4 more)

### Community 65 - "04-execution.md"
Cohesion: 0.33
Nodes (6): BTreeSet, RuntimeController, RuntimeSessionSnapshot, RuntimeSnapshot, read_launched_document(), reconcile_launched_session()

### Community 66 - "Work 02 — Linux exact-build runner"
Cohesion: 0.60
Nodes (3): infrastructure_failure(), verify-external-napplet-corpus.sh script, trust_failure()

### Community 67 - "lib.rs"
Cohesion: 0.50
Nodes (3): fedora-run-smoke.sh script, UZEL_SMOKE_NAME, UZEL_SMOKE_SUCCESS_MARKER

### Community 68 - "Verified facts"
Cohesion: 0.83
Nodes (3): main(), strip_code(), write_manifest()

### Community 81 - "AcceptSettings"
Cohesion: 0.38
Nodes (5): NativeSettingsExecutor, NativeSettingsOpenResult, NativeSettingsRequest, AcceptSettings, UnavailableSettings

### Community 82 - "RelayDiagnosticsSink"
Cohesion: 0.50
Nodes (3): RuntimeRelayDiagnosticsObserver, RuntimeRelayDiagnosticsSnapshot, RelayDiagnosticsSink

### Community 84 - "resource.rs"
Cohesion: 0.09
Nodes (27): Cancellation, Client, Duration, Future, Instant, IpAddr, Output, PinnedHttpsRequest (+19 more)

### Community 92 - "mock-native.js"
Cohesion: 0.26
Nodes (7): diagnostics(), invoke(), nativeEnvelope(), profileFor(), review(), routedProfileEvent(), surfaceLaunch()

### Community 98 - "POC status"
Cohesion: 0.29
Nodes (6): Accepted provisional risks, Gate 0 — validated baseline, Implementation, Latest integrated evidence, POC status, Work 07 parallel branch evidence — issue #21

### Community 102 - "Work 07 — issue-driven stabilization"
Cohesion: 0.22
Nodes (8): Active parallel issue — #21 data-only corpus, Completed issue — #10, Completed issue — #19, Entry evidence, Exit rule, Goal, Work 07 — issue-driven stabilization, Work graph

### Community 175 - "server.rs"
Cohesion: 0.12
Nodes (31): Into, SurfaceLaunch, UnixListener, Request, Response, active_daemon_socket_is_not_unlinked(), AssetTransfer, bounded_detail() (+23 more)

### Community 176 - "Tests, quality gates, and demo"
Cohesion: 0.20
Nodes (9): Deterministic demo, Final acceptance, Hostile frame, Live demo, Napplet/web, Quality commands, Required test layers, Runtime/Rust (+1 more)

## Knowledge Gaps
- **433 isolated node(s):** `name`, `private`, `version`, `type`, `dev:web` (+428 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **99 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `LinuxRunner` connect `lib.rs` to `04-execution.md`, `Provisional component design`, `POC architecture`, `POC documentation audit`, `server.rs`, `.fmt`?**
  _High betweenness centrality (0.022) - this node is a cross-community bridge._
- **Why does `linux_resource_provider()` connect `resource.rs` to `lib.rs`?**
  _High betweenness centrality (0.012) - this node is a cross-community bridge._
- **What connects `name`, `private`, `version` to the rest of the system?**
  _433 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `HostileProbeState` be split into smaller, more focused modules?**
  _Cohesion score 0.1310344827586207 - nodes in this community are weakly interconnected._
- **Should `check-napplet-imports.mjs` be split into smaller, more focused modules?**
  _Cohesion score 0.06155632984901278 - nodes in this community are weakly interconnected._
- **Should `Provisional component design` be split into smaller, more focused modules?**
  _Cohesion score 0.13157894736842105 - nodes in this community are weakly interconnected._
- **Should `Work 01 — scaffold` be split into smaller, more focused modules?**
  _Cohesion score 0.11764705882352941 - nodes in this community are weakly interconnected._