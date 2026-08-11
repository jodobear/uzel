# Graph Report - uzel-plan-v4-clean  (2026-08-11)

## Corpus Check
- 155 files · ~139,310 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1782 nodes · 2346 edges · 211 communities (107 shown, 104 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 20 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `8995a509`
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
- nampplets Linux reuse map
- WebKit/Tauri trust spike
- NMP API and ownership map
- FACT-002-spec-revisions.md
- FACT-003-nampplets-linux-reuse.md
- FACT-004-nmp-facade.md
- Gate 0 preflight and Slice 01 decision
- Slice 02 preflight
- FACT-007-local-ipc.md
- FACT-008-toolchain.md
- fedora-run-smoke.sh
- debian-build-smoke.sh
- main.rs
- App.svelte
- build.rs
- Post-POC extraction
- Post-POC extraction
- README.md
- WebKit/Tauri trust spike
- check-boundaries.sh
- smoke.sh
- Work 04 — daemon, NMP, and persistence
- ExactFixtureSource
- trusted-shell-policy.js
- Window
- check-pinned-assets.sh
- Work 04 — daemon, NMP, and persistence
- README.md
- ClientError
- package.json
- package.json
- profile-open-v1.schema.json
- Slice 02 preflight
- Work 00 — validate assumptions
- main.js
- 04-execution.md
- linux-smoke-script.test.mjs
- lib.rs
- Verified facts
- Execution slices
- check_sha256
- POC status
- build-signed-napplet-fixtures.sh
- FACT-010-portable-napplets.md
- ClientError
- Request
- build-signed-napplet-fixtures.sh
- AcceptSettings
- RelayDiagnosticsSink
- FACT-011-daemon-nmp.md
- resource.rs
- FACT-012-composed-demo.md
- README.md
- Work 06 — hardening and demo acceptance
- debian13-live-test.sh
- FACT-004 — NMP facade and ownership
- debian13-setup.sh
- Work 02 — Linux exact-build runner
- mock-native.js
- FACT-004-nmp-facade.md
- FACT-007 — local daemon IPC
- Work 04 — daemon, NMP, and persistence
- FACT-014-live-identity-catalog.md
- POC status
- Upstream contribution ledger
- FACT-013 — hostile Linux child boundary
- FACT-012-composed-demo.md
- FACT-013-hostile-linux-boundary.md
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
- manifest.json
- ProtocolError
- handle_stream
- Work 05 — composed demo
- MANIFEST.json
- audit_docs.py
- fedora-run-smoke.sh
- check_sha256
- debian-build-smoke.sh
- dev.sh
- 01-reorient-current-gsd.md
- 02-review-phase-1.md
- 03-execute-phase-1.md
- 04-post-m5-audit.md
- 05-phase-closeout.md
- 06-upstream-contribution.md
- 07-milestone-learning.md
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

## God Nodes (most connected - your core abstractions)
1. `LinuxRunner` - 38 edges
2. `UnixClient` - 33 edges
3. `Uzel product and incubation architecture` - 31 edges
4. `Material findings and corrections` - 28 edges
5. `RunnerError` - 24 edges
6. `A5 — mandatory post-M5 whole-system audit` - 24 edges
7. `ClientError` - 22 edges
8. `Delivery, quality, review and packaging discipline` - 21 edges
9. `Decision, nuance and educational-knowledge system` - 20 edges
10. `Response` - 19 edges

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

## Communities (211 total, 104 thin omitted)

### Community 0 - "POC scope and acceptance"
Cohesion: 0.06
Nodes (64): DecodeError, Into, Read, SurfaceLaunch, UnixListener, authoritative_reconciliation_retires_ambiguous_operation_ids(), CatalogCapability, chunked_routed_envelope_reassembles_on_one_connection() (+56 more)

### Community 1 - "Assumption validation and decision gates"
Cohesion: 0.25
Nodes (18): boundedJSON(), compatibilityPreludeSource(), decodedBase64Length(), decodeResourceBlob(), exactFields(), isBoundedEnvelope(), isPlainObject(), isVerifiedArtifactBaseURL() (+10 more)

### Community 2 - "check-napplet-imports.mjs"
Cohesion: 0.06
Nodes (37): allowedDependencyTargets, allowedGuardedGlobalAccesses, { compile: compileSvelte, parse: parseSvelte }, contractsRoot, declarativeNetworkViolations(), dependencyGroups, dependencyViolations(), directNetworkIdentifiers (+29 more)

### Community 3 - "Provisional component design"
Cohesion: 0.13
Nodes (13): Debug, Formatter, FromUtf8Error, RuntimeAccountHandle, RuntimeObservation, RuntimeRelayDiagnosticsObservation, artifact_base_url(), bounded_diagnostic() (+5 more)

### Community 4 - "Tests, quality gates, and demo"
Cohesion: 0.04
Nodes (49): 10. Interoperability and version-skew tests, 11. Supply-chain and release-evidence tests, 1. Unit/model tests, 2. Contract and vector tests, 3. Component/integration tests, 4. Adversarial/security tests, 5. Fault injection and recovery, 6. Browser/native tests (+41 more)

### Community 5 - "Uzel POC agent instructions"
Cohesion: 0.13
Nodes (11): RuntimeRelayLane, absent_surface_cleanup_is_idempotent(), catalog_cancellation_is_terminal(), catalog_cancellation_keeps_retryable_reviews_and_discards_terminal_stale_tokens(), hostile_probe_commits_exact_session_config_before_returning(), pending_review_tokens_are_sorted_bounded_and_reconcilable(), ProductState, relay_lane_name() (+3 more)

### Community 6 - "POC architecture"
Cohesion: 0.04
Nodes (47): Admission, fairness and abuse resistance, Authority model, Blossom and attachment ownership, Canonical Nostr engine owns Nostr semantics, Capability maturity and production evidence, Capability negotiation and launch transcript, Character, Cross-napplet composition (+39 more)

### Community 7 - "POC architecture"
Cohesion: 0.18
Nodes (12): Condvar, Fn, RuntimeEvent, RuntimeObservationFrame, RuntimeObserver, buffered_responses_are_byte_bounded_and_consumed_once(), BufferedEvents, event_bytes() (+4 more)

### Community 8 - "POC documentation audit"
Cohesion: 0.21
Nodes (11): eventually_identity_query(), identity_query(), inc_emit_waits_for_an_inc_event_not_an_unrelated_push(), launch_identity_surface(), payload_identity_cannot_select_surface_or_session(), profile_open_crosses_inc_with_runtime_owned_sender(), public_identity_profile_follows_and_picture_cross_only_native_providers(), ResponseExpectation (+3 more)

### Community 9 - "Work 01 — scaffold"
Cohesion: 0.05
Nodes (36): Artifact audit expectations, Audit method, Cross-document coherence result, Executive verdict, Final acceptance of the plan pack, H-01 — “Exact pin” was underspecified, H-02 — a human-readable compatibility statement could drift from shipped behavior, H-03 — compatibility was discoverable only after guest execution (+28 more)

### Community 10 - "Work 00 — validate assumptions"
Cohesion: 0.06
Nodes (36): A5 — mandatory post-M5 whole-system audit, Attack the assumptions, Audit independence, Audit synthesis, Blocking findings, Blocking findings, Blocking findings, Candidate evidence freeze (+28 more)

### Community 11 - "04-execution.md"
Cohesion: 0.07
Nodes (30): Acceptance environment, Authority and schema baseline, Current Nix package/native acceptance, Ecosystem, compatibility and maturity baseline, Exact closure policy, Exit gate, `failed_behavior`, Incident preservation (+22 more)

### Community 12 - "Slice 04 preflight — daemon, NMP, and persistence"
Cohesion: 0.08
Nodes (25): 10. Data safety, 11. UX and accessibility as correctness, 12. Operations and observability, 1. Semantic ownership and dependency direction, 2. Compatibility and launch integrity, 3. State-machine correctness, 4. Memory, CPU, queues and wakeups, 5. Fuzzing and property tests (+17 more)

### Community 13 - "Work 04 — daemon, NMP, and persistence"
Cohesion: 0.06
Nodes (69): canonicalProfile(), canonicalProfiles(), optionalText(), profileQueryBatches(), profileQueryRequest(), retryProfileQueryRequests(), splitProfileQueryRequest(), A (+61 more)

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
Cohesion: 0.08
Nodes (24): Before external contact, Candidate-next shadow probe, Canonical representation, Channel selection, Claim-specific authority, Compatibility campaign, Current ecosystem posture, Externally consumable compatibility and conformance kit (+16 more)

### Community 19 - "audit_docs.py"
Cohesion: 0.09
Nodes (23): A5 knowledge handoff, Agent-facing reference, Architecture Decision Records, Canonical terminology and concept registry, Capability ledgers, Claim-specific authority and contradiction handling, Decision, nuance and educational-knowledge system, Education pipeline (+15 more)

### Community 20 - "manifest.json"
Cohesion: 0.40
Nodes (5): Entry conditions, Full M5 acceptance journey, M5 exit gate, M5 / GSD delivery phases 7, 7.1–7.9 — production-candidate hardening and audit freeze, Visible outcome

### Community 21 - "README.md"
Cohesion: 0.25
Nodes (7): core:default, main, description, identifier, permissions, $schema, windows

### Community 22 - "Gate 0 preflight and Slice 01 decision"
Cohesion: 0.07
Nodes (40): ACTION_IDS, bindingFromEvent(), bindingMatches(), DEFAULT_KEYBINDINGS, defaultPreferences(), KEYBINDING_ACTIONS, parsePreferences(), validateKeybindings() (+32 more)

### Community 23 - "nampplets Linux reuse map"
Cohesion: 0.50
Nodes (3): Active programme gate, Brownfield POC evidence, Uzel agent instructions

### Community 24 - "WebKit/Tauri trust spike"
Cohesion: 0.16
Nodes (17): cleanup(), GDK_BACKEND, hostile_markers_are_ordered(), NO_AT_BRIDGE, preserve_failure(), preserve_logs(), report_marker(), report_marker_state() (+9 more)

### Community 25 - "NMP API and ownership map"
Cohesion: 0.12
Nodes (16): `01-01` — reconcile the incident and produce replay evidence, `01-02` — current Nix package and native baseline, `01-03` — authority, schema and threat baseline, `01-04` — measured CI, test and review baseline, `01-05` — ecosystem, compatibility, maturity and knowledge baseline, Current Phase 1 incident, Dependencies and execution order, GSD ingest — reorient the existing Uzel project in place (+8 more)

### Community 27 - "FACT-002-spec-revisions.md"
Cohesion: 0.12
Nodes (16): app, security, windows, build, beforeBuildCommand, beforeDevCommand, devUrl, frontendDist (+8 more)

### Community 28 - "FACT-003-nampplets-linux-reuse.md"
Cohesion: 0.18
Nodes (11): Bounded slices, Phase 7.1 — two simultaneous packaged instances, Phase 7.2 — exact-build registry, launch, revoke and removal, Phase 7.3 — exact-build update, quarantine and previous-build rollback, Phase 7.4 — bounded operations and diagnostics, Phase 7.5 — schema migration, integrity and corruption recovery, Phase 7.6 — backup, restore, profile deletion and package rollback truth, Phase 7.7 — surface and supported-Linux closure (+3 more)

### Community 29 - "FACT-004-nmp-facade.md"
Cohesion: 0.20
Nodes (13): CARGO_INCREMENTAL, CARGO_PROFILE_DEV_DEBUG, fail(), record_prebuild(), reexec_with_nix_group(), run_cache_probe(), run_startup_step(), debian13-live-test.sh script (+5 more)

### Community 31 - "Slice 02 preflight"
Cohesion: 0.14
Nodes (14): Bounded notification within phase 6.2, Bounded slices, Entry conditions, Exclusions, M4.5 exit gate, M4.5 / GSD delivery phases 6, 6.1–6.2 — scheduling, recovery and cross-domain composition, Phase 6.1 — due-time signer behavior and restart reconciliation, Phase 6.2 — integrated composition, notification and resource closure (+6 more)

### Community 32 - "FACT-007-local-ipc.md"
Cohesion: 0.30
Nodes (15): Any, audit_fences(), audit_hashes(), audit_links(), audit_pack_symlinks(), audit_policy(), audit_toml(), clean_target() (+7 more)

### Community 33 - "FACT-008-toolchain.md"
Cohesion: 0.15
Nodes (13): Bounded slices, Entry conditions, Exclusions, Final text review within phase 4.1, M3 exit gate, M3 / GSD delivery phases 4, 4.1–4.3 — external signer and deliberate text publication, Outcome model, Phase 4.1 — publication grant and trusted payload review (+5 more)

### Community 34 - "fedora-run-smoke.sh"
Cohesion: 0.25
Nodes (7): Active contributions, Authority and ownership, Entry template, Slice 02 upstream result, Slice 03 upstream result, Slice 06 upstream result, Upstream contribution ledger

### Community 35 - "debian-build-smoke.sh"
Cohesion: 0.15
Nodes (12): Destination, Installation and Phase 1 restart runbook, Step 0 — pause and preserve the blocked execution, Step 1 — verify and install the planning pack, Step 2 — create a clean manual Phase 1 worktree, Step 3 — inspect and reorient the existing GSD project, Step 4 — replan Phase 1 in place, Step 5 — independently review and converge Phase 1 plans (+4 more)

### Community 37 - "main.rs"
Cohesion: 0.33
Nodes (11): IntoIterator, Item, Iterator, default_runtime_root(), default_socket_path(), live_configuration_requires_explicit_live_mode(), main(), next_path() (+3 more)

### Community 38 - "App.svelte"
Cohesion: 0.25
Nodes (3): @tauri-apps/api/core, ./preferences.js, ./projection-failure.js

### Community 39 - "build.rs"
Cohesion: 0.33
Nodes (5): Accepted provisional risks, Gate 0 — validated baseline, Implementation, Latest integrated evidence, POC status

### Community 40 - "Post-POC extraction"
Cohesion: 0.17
Nodes (11): Accepted post-foundation extension, Architectural invariants, Demo-complete, Foundation-complete, Non-goals, Objective, POC scope and acceptance, Required napplets (+3 more)

### Community 41 - "Post-POC extraction"
Cohesion: 0.05
Nodes (37): Assumption validation and decision gates, Current observed baseline, Gate 0 decision, Gate matrix, Hard stops, Per-slice validation, Plan correction rule, Required outputs (+29 more)

### Community 42 - "README.md"
Cohesion: 0.18
Nodes (11): Bounded slices, Draft model, Entry conditions, Exclusions, M2 exit gate, M2 / GSD delivery phases 3, 3.1–3.3 — local-first offline authoring, Phase 3.1 — minimal Composer create/edit/save/reopen, Phase 3.2 — draft library and conflict semantics (+3 more)

### Community 43 - "WebKit/Tauri trust spike"
Cohesion: 0.18
Nodes (11): Bounded slices, Entry conditions, M4 exit gate, M4 / GSD delivery phases 5, 5.1–5.3 — verified static-image attachment round trip, Outcome separation, Phase 5.1 — Blossom authorization and verified upload, Phase 5.2 — verified fetch, cache and offline reopen, Phase 5.3 — Composer attachment, export and integrated round trip (+3 more)

### Community 44 - "check-boundaries.sh"
Cohesion: 0.18
Nodes (11): Capability negotiation is not a safe implicit-presence check, Ecosystem source scan — 10 August 2026, Immediate M0 outputs, Manifest and exact-build identity are moving, NAP status and implementation-led movement, Napplet packages and conformance, NMP maturity and ownership, Nostr standards posture (+3 more)

### Community 45 - "smoke.sh"
Cohesion: 0.18
Nodes (8): Evidence boundary, Renderer acceptance harness, 1. Clone and install the launcher, 2. Run automated real-WebKit acceptance, 3. Run visible desktop demo, Debian 13 live test, Development, Uzel

### Community 48 - "Work 04 — daemon, NMP, and persistence"
Cohesion: 0.20
Nodes (10): Decision, Document routing, Fast-moving ecosystem decision, Immediate Phase 1 decision, Immediate sequence, Mandatory stop after M5, Non-negotiable architecture, Production-candidate definition (+2 more)

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
Cohesion: 0.20
Nodes (9): Agent reference delta, Contradiction and disclosure check, Decisions that survived contact with the product, Ecosystem movement, Educational backlog seeds, Human case-study outline, Milestone <M> learning digest, Nuances and negative results (+1 more)

### Community 53 - "Work 04 — daemon, NMP, and persistence"
Cohesion: 0.16
Nodes (5): Slice handoff, Acceptance, Goal, Tasks, Work 06 — hardening and demo acceptance

### Community 54 - "README.md"
Cohesion: 0.33
Nodes (6): BTreeSet, RuntimeController, RuntimeSessionSnapshot, RuntimeSnapshot, read_launched_document(), reconcile_launched_session()

### Community 55 - "ClientError"
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

### Community 62 - "Slice 02 preflight"
Cohesion: 0.05
Nodes (56): AtomicBool, AtomicU64, AtomicUsize, FnMut, From, JoinHandle, State, TauriPlugin (+48 more)

### Community 63 - "Work 00 — validate assumptions"
Cohesion: 0.20
Nodes (10): API and ownership evidence, Automated review corrections, Commands and observed results, Exact pins and fixtures, Hostile fixture scope, Manifest correction, Next step, Slice 03 preflight (+2 more)

### Community 64 - "main.js"
Cohesion: 0.13
Nodes (12): denied(), results, target, attemptRawWebKitInvoke(), boundedAttempt(), nativeSurface(), PROBE_NAMES, sentinelTargets() (+4 more)

### Community 65 - "04-execution.md"
Cohesion: 0.22
Nodes (9): Bounded slices, Phase 2.1 — local profile and cached/live text Home, Phase 2.2 — trusted destination policy and bounded resource fetch, Phase 2.3 — isolated raster normalization and resource cache, Phase 2.4 — People and Profile guest surface, Phase 2.5 — mediated cross-surface intents, Phase 2.6 — M1 diagnostics and integrated closure, Phase 2.7 — external clean-room compatibility and composition capstone (+1 more)

### Community 66 - "linux-smoke-script.test.mjs"
Cohesion: 0.67
Nodes (3): matchesLog(), napdReadyPattern(), script

### Community 67 - "lib.rs"
Cohesion: 0.22
Nodes (8): Agent trap, Consequence for Uzel, Educational seed, Evidence, Executable witness, Learning, LRN-#### — <reusable lesson>, Question or failed assumption

### Community 68 - "Verified facts"
Cohesion: 0.22
Nodes (9): Engineering rules, Mission, POC exclusions, Quality gate, Repository boundaries, Required method, Trust rules, Upstream contribution policy (+1 more)

### Community 69 - "Execution slices"
Cohesion: 0.22
Nodes (9): Developer mode, Exact-build fixtures, Identity and Nostr reads, Local daemon protocol, Napplet convention, Provisional component design, Runtime state, Shell UI (+1 more)

### Community 71 - "check_sha256"
Cohesion: 0.22
Nodes (9): Commands and observed results, Exact dependency and asset record, Next step, Preserved failed Fedora probe, Required design correction, Runtime evidence, Slice 02 preflight, Upstream result (+1 more)

### Community 72 - "POC status"
Cohesion: 0.22
Nodes (8): Commands and results, Exact dependency and upstream evidence, Exact next step, Honest boundary, Linux shell evidence, Outcome, Runtime composition evidence, Slice 05 preflight — integrated composed demo

### Community 73 - "build-signed-napplet-fixtures.sh"
Cohesion: 0.25
Nodes (7): ADR-#### — <decision>, Alternatives considered, Consequences, Context, Decision, Evidence, Revisit trigger

### Community 74 - "FACT-010-portable-napplets.md"
Cohesion: 0.25
Nodes (8): Bounds, admission, fairness and operations, Capability ledger — <capability-id>, Claim and journeys, Contract, negotiation and state, Evidence matrix, Persistence and recovery, Trust and identity, Upstream and known gaps

### Community 78 - "ClientError"
Cohesion: 0.22
Nodes (8): Capability negotiation, Evidence, Exact source identities, Exclusions and deviations, Identity and trust semantics, Migration and rollback, Supported behavior, Uzel Runtime Compatibility Profile — <profile-id>

### Community 79 - "Request"
Cohesion: 0.25
Nodes (8): Alternatives and consequences, Chosen interpretation, Exact sources, Executable evidence, Question or contradiction, Recheck triggers, SIR-#### — <interpreted seam or ambiguity>, Upstream route

### Community 80 - "build-signed-napplet-fixtures.sh"
Cohesion: 0.25
Nodes (8): Adoption and patch removal, Channel decision, Lifecycle events, Problem and impact, Reproduction, UPR-#### — <upstream interaction>, Upstream response, Uzel workaround or patch

### Community 81 - "AcceptSettings"
Cohesion: 0.38
Nodes (5): NativeSettingsExecutor, NativeSettingsOpenResult, NativeSettingsRequest, AcceptSettings, UnavailableSettings

### Community 82 - "RelayDiagnosticsSink"
Cohesion: 0.50
Nodes (3): RuntimeRelayDiagnosticsObserver, RuntimeRelayDiagnosticsSnapshot, RelayDiagnosticsSink

### Community 83 - "FACT-011-daemon-nmp.md"
Cohesion: 0.46
Nodes (7): fail(), print_nix_builder_state(), print_nix_daemon_state(), refresh_nix_builder_state(), refresh_nix_daemon_state(), debian13-setup.sh script, wait_for_nix_daemon_socket()

### Community 84 - "resource.rs"
Cohesion: 0.09
Nodes (27): Cancellation, Client, Duration, Future, Instant, IpAddr, Output, PinnedHttpsRequest (+19 more)

### Community 85 - "FACT-012-composed-demo.md"
Cohesion: 0.25
Nodes (7): Confidence, Corrections, Diagram review, Gate 0 resolution, POC documentation audit, Problems found in the previous pack, Verdict

### Community 86 - "README.md"
Cohesion: 0.25
Nodes (7): First hardening follow-ups, Likely `kehto/napd`, Moves to `jodobear/napplets`, POC shortcuts that must not silently become platform contracts, Post-POC extraction, Remains in Uzel, Rewrite criteria

### Community 87 - "Work 06 — hardening and demo acceptance"
Cohesion: 0.25
Nodes (7): Apple-only edges, Build evidence, Compatibility candidate, Crate map, nampplets Linux reuse map, Result, Runtime entry points

### Community 88 - "debian13-live-test.sh"
Cohesion: 0.25
Nodes (8): Boundary result, Commands and observed results, Debian probe correction, Fedora probe correction, Locked workspace, Next step, Slice 01 preflight, Verdict

### Community 89 - "FACT-004 — NMP facade and ownership"
Cohesion: 0.25
Nodes (7): Commands and results, Exact next step, NMP and persistence evidence, Outcome, Private protocol evidence, Remaining boundaries, Slice 04 preflight — daemon, NMP, and persistence

### Community 90 - "debian13-setup.sh"
Cohesion: 0.25
Nodes (8): Bubblewrap decision, Commands and results, Exact hostile evidence, Failed evidence and toolchain limit, Go/no-go and exact next steps, Outcome, Slice 06 preflight — hardening and clean demo acceptance, Upstream result

### Community 91 - "Work 02 — Linux exact-build runner"
Cohesion: 0.25
Nodes (7): Accepted child CSP, Boundary of proof, Executable probe, Low-level handler nuance, Source verification, Verdict, WebKit/Tauri trust spike

### Community 92 - "mock-native.js"
Cohesion: 0.24
Nodes (9): diagnostics(), invoke(), isRoutedProfileQuery(), nativeEnvelope(), profileEvent(), profileEventsForQuery(), profileFor(), review() (+1 more)

### Community 93 - "FACT-004-nmp-facade.md"
Cohesion: 0.25
Nodes (7): Done when, Goal, Outputs, Read, Stop conditions, Tasks, Work 00 — validate assumptions

### Community 94 - "FACT-007 — local daemon IPC"
Cohesion: 0.25
Nodes (8): Acceptance, Depends on, Entry status, Goal, Non-goals, Status, Tasks, Work 01 — scaffold

### Community 95 - "Work 04 — daemon, NMP, and persistence"
Cohesion: 0.25
Nodes (7): Active issue — #19, Completed issue — #10, Entry evidence, Exit rule, Goal, Work 07 — issue-driven stabilization, Work graph

### Community 96 - "FACT-014-live-identity-catalog.md"
Cohesion: 0.29
Nodes (6): A5 — mandatory post-M5 stop, Continuous lanes, Programme map, Roadmap rule, Slice and issue size, Uzel product-first roadmap

### Community 98 - "POC status"
Cohesion: 0.29
Nodes (6): Accepted pin, Executable probe, nampplets adapter seam, NMP API and ownership map, Ownership boundary, Public facade

### Community 99 - "Upstream contribution ledger"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 02 — Linux exact-build runner

### Community 100 - "FACT-013 — hostile Linux child boundary"
Cohesion: 0.29
Nodes (7): Acceptance, Entry status and pins, Goal, Non-goals, Read, Tasks, Work 04 — daemon, NMP, and persistence

### Community 101 - "FACT-012-composed-demo.md"
Cohesion: 0.33
Nodes (6): Entry conditions, Exclusions, M1 exit gate, M1 / GSD delivery phases 2, 2.1–2.7 — coherent composable Social Home, Surface and persistence rules, Visible outcome

### Community 102 - "FACT-013-hostile-linux-boundary.md"
Cohesion: 0.33
Nodes (6): Exclusions, M0 exit gate, M0 / GSD Phase 1 — truthful baseline and execution reset, Required outputs, Required plans, Visible outcome

### Community 103 - "Box"
Cohesion: 0.33
Nodes (5): Capability promotion, Contradiction and disclosure check, Durable deltas, Phase <N> closeout, Residual findings and triggers

### Community 118 - "manifest.json"
Cohesion: 0.33
Nodes (6): Dependency graph, Execution slices, Handoff, Parallel work, Slice entry gate, Slices

### Community 120 - "ProtocolError"
Cohesion: 0.20
Nodes (6): Demo result, Document map, Mandatory first step, Scope rule, Start order, Uzel single-repository POC

### Community 121 - "handle_stream"
Cohesion: 0.33
Nodes (6): Acceptance, Entry status and pins, Goal, Read, Tasks, Work 03 — portable napplets

### Community 122 - "Work 05 — composed demo"
Cohesion: 0.33
Nodes (5): Acceptance, Goal, Non-goals, Tasks, Work 05 — composed demo

### Community 123 - "MANIFEST.json"
Cohesion: 0.22
Nodes (7): date, excluded_from_payload_hashes, files, name, revision, reports/audit.json, SHA256SUMS

### Community 124 - "audit_docs.py"
Cohesion: 0.70
Nodes (4): main(), manifest_paths(), strip_code(), write_manifest()

### Community 126 - "fedora-run-smoke.sh"
Cohesion: 0.50
Nodes (3): fedora-run-smoke.sh script, UZEL_SMOKE_NAME, UZEL_SMOKE_SUCCESS_MARKER

## Knowledge Gaps
- **852 isolated node(s):** `name`, `private`, `version`, `type`, `dev:web` (+847 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **104 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Delivery, quality, review and packaging discipline` connect `Tests, quality gates, and demo` to `dev.sh`?**
  _High betweenness centrality (0.014) - this node is a cross-community bridge._
- **Why does `A5 — mandatory post-M5 whole-system audit` connect `Work 00 — validate assumptions` to `dev.sh`?**
  _High betweenness centrality (0.013) - this node is a cross-community bridge._
- **Why does `Decision, nuance and educational-knowledge system` connect `audit_docs.py` to `dev.sh`?**
  _High betweenness centrality (0.011) - this node is a cross-community bridge._
- **What connects `name`, `private`, `version` to the rest of the system?**
  _852 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `POC scope and acceptance` be split into smaller, more focused modules?**
  _Cohesion score 0.05798969072164949 - nodes in this community are weakly interconnected._
- **Should `check-napplet-imports.mjs` be split into smaller, more focused modules?**
  _Cohesion score 0.06155632984901278 - nodes in this community are weakly interconnected._
- **Should `Provisional component design` be split into smaller, more focused modules?**
  _Cohesion score 0.1268939393939394 - nodes in this community are weakly interconnected._