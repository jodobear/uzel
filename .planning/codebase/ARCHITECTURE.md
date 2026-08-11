<!-- refreshed: 2026-08-09 -->
# Architecture

**Analysis Date:** 2026-08-09

## System Overview

```text
┌───────────────────────────────────────────────────────────────────────────┐
│                    Trusted Linux desktop product shell                    │
├───────────────────────┬───────────────────────┬───────────────────────────┤
│ Svelte UI/orchestrator │ Tauri native backend  │ Pinned trusted-shell host │
│ `apps/uzel/src/`       │ `apps/uzel/src-tauri` │ `apps/uzel/public/`        │
└───────────┬───────────┴────────────┬──────────┴──────────────┬────────────┘
            │ Tauri commands         │ `UnixClient`             │ source-bound
            │                        ▼                          │ `postMessage`
┌───────────┴──────────────────────────────────────────────────┴────────────┐
│                Bounded private AF_UNIX protocol and daemon               │
│ `crates/napd-protocol/src/lib.rs` · `apps/uzel-napd/src/main.rs`          │
│ `crates/napd/src/server.rs`                                               │
└──────────────────────────────────┬────────────────────────────────────────┘
                                   │ typed runtime composition calls
                                   ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                 Linux runtime composition / policy adapter                │
│ `crates/napd/src/runner.rs` · `crates/napd/src/resource.rs`               │
└──────────────────────────────────┬────────────────────────────────────────┘
                                   │ pinned `nmp-native-*` facades
                                   ▼
┌───────────────────────────────────────────────────────────────────────────┐
│ Nampplets RuntimeController + NMP + runtime/artifact/NMP stores           │
│ configured at `crates/napd/src/runner.rs:372`                             │
└──────────────────────────────────┬────────────────────────────────────────┘
                                   │ verified single-file HTML + NAP envelopes
                                   ▼
┌───────────────────────────────────────────────────────────────────────────┐
│ Untrusted opaque-origin napplet frames                                    │
│ `napplets/*/src/main.js` · signed copies under `fixtures/*/`              │
└───────────────────────────────────────────────────────────────────────────┘
```

Uzel uses a ports-and-adapters composition. Product UI, native IPC, daemon policy, upstream runtime authority, and untrusted napplet code remain separate. `crates/napd/src/runner.rs` is the product adapter around the pinned Nampplets `RuntimeController`; it does not reimplement Nostr query, relay, cache, provenance, or diagnostics behavior.

## Component Responsibilities

| Component | Responsibility | File |
|-----------|----------------|------|
| Svelte shell | Own product state, two-pane composition, catalog UI, recovery state machines, identity selection, and diagnostics display | `apps/uzel/src/App.svelte` |
| Browser bootstrap | Load trusted-shell assets before mounting the Svelte application | `apps/uzel/index.html`, `apps/uzel/src/main.ts` |
| Trusted surface host | Materialize verified HTML, create opaque sandboxed iframes, bind `MessageEvent.source` to a runtime surface, and project bounded NAP envelopes | `apps/uzel/public/trusted-shell/trusted-shell.js`, `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js` |
| Tauri backend | Expose the narrow renderer command API, translate protocol results to camel-case UI values, enforce navigation policy, and hold native client/probe state | `apps/uzel/src-tauri/src/main.rs` |
| Hostile boundary probe | Own loopback sentinel lifecycle and validate browser/native denial evidence | `apps/uzel/src-tauri/src/hostile_probe.rs` |
| Daemon binary | Parse fixture/live configuration, open one `LinuxRunner`, bind the private socket, and serve until shutdown | `apps/uzel-napd/src/main.rs` |
| Private protocol | Define bounded request/response types, length-prefixed JSON framing, asset/envelope chunking, retry operation IDs, and `UnixClient` | `crates/napd-protocol/src/lib.rs` |
| Daemon server | Validate socket ownership/permissions, serialize request handling, cache replayable catalog results, and stage verified asset transfers | `crates/napd/src/server.rs` |
| Runtime adapter | Own one upstream `RuntimeController`, identity/product state, exact-build review/confirmation, permissions, surface/session mapping, and response routing | `crates/napd/src/runner.rs` |
| Resource adapter | Supply hardened HTTPS-only NAP-RESOURCE network access with DNS pinning, no redirects/proxy, deadlines, cancellation, and body limits | `crates/napd/src/resource.rs` |
| Exact fixture catalog | Pin signed events, artifact hashes, embedded HTML bytes, artifact URLs, and requested domains | `crates/napd/src/fixtures.rs`, `fixtures/*/event.json`, `fixtures/*/index.html` |
| Cross-napplet contracts | Own profile-open payload validation and canonical kind-0 projection/query batching | `contracts/profile-open.js`, `contracts/profile-open-v1.schema.json`, `contracts/kind0-profile.js` |
| Portable napplets | Implement runtime-agnostic NAP clients without importing Uzel, Tauri, daemon, or peer napplet code | `napplets/follow-list/src/main.js`, `napplets/profile-card/src/main.js`, `napplets/hostile-egress/src/main.js` |
| Boundary gates | Enforce crate/UI and napplet import boundaries, pinned artifacts, and runnable end-to-end evidence | `scripts/check-boundaries.sh`, `scripts/check-napplet-imports.mjs`, `scripts/check-pinned-assets.sh`, `scripts/linux-run-smoke.sh` |

## Pattern Overview

**Overall:** Layered ports-and-adapters desktop application with a private daemon and capability-projected sandboxed surfaces.

**Key Characteristics:**
- Keep UI orchestration in `apps/uzel/`, reusable runtime mechanisms in `crates/napd`, and wire contracts in `crates/napd-protocol`.
- Treat the daemon-owned `RuntimeController` in `crates/napd/src/runner.rs` as the sole runtime/NMP authority; renderer and Tauri backend are clients.
- Move verified artifact bytes across native IPC as bounded chunks, then inject them as `iframe.srcdoc`; never expose an artifact filesystem path to WebKit (`crates/napd/src/server.rs:269`, `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js:53`).
- Bind an untrusted child to a surface through `MessageEvent.source`, not caller-supplied identity (`apps/uzel/public/trusted-shell/trusted-shell.js:309`, `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js:43`).
- Use exact signed build coordinates and frozen permission review before launch (`crates/napd/src/runner.rs:674`, `crates/napd/src/runner.rs:748`).
- Bound queues, frames, pending operations, surfaces, reviews, resource bodies, and response waits at their owning layers (`crates/napd-protocol/src/lib.rs:19`, `crates/napd/src/runner.rs:34`, `crates/napd/src/server.rs:21`).

## Layers

**Product Presentation:**
- Purpose: Render product state and coordinate user-visible workflows.
- Location: `apps/uzel/src/`
- Contains: `App.svelte`, shell preferences, projection-failure adaptation, styles, and browser declarations.
- Depends on: Tauri `invoke`, Svelte, and the global trusted-shell host loaded by `apps/uzel/index.html`.
- Used by: The Tauri WebView configured in `apps/uzel/src-tauri/tauri.conf.json`.
- Rule: Keep runtime truth and protocol ownership out of TypeScript; project only data required by the UI.

**Trusted Browser Boundary:**
- Purpose: Isolate untrusted artifact code and safely project approved NAP domains.
- Location: `apps/uzel/public/trusted-shell/`
- Contains: Reviewed CSP/materialization primitives, NAP prelude, policy, and multi-surface host.
- Depends on: Browser `iframe`, `MessageChannel`, `postMessage`, Blob, and DOM APIs.
- Used by: `apps/uzel/src/App.svelte` through `window.NMPTrustedShellHost`.
- Rule: Preserve vendored bytes as a unit; mount with `sandbox="allow-scripts"`, `srcdoc`, opaque origin, no `allow-same-origin`, and source-bound message mapping.

**Native Shell Adapter:**
- Purpose: Translate renderer commands into private daemon protocol operations without embedding runtime logic.
- Location: `apps/uzel/src-tauri/`
- Contains: Tauri command handlers, navigation allowlist, `UnixClient` managed state, and hostile probe state.
- Depends on: `crates/napd-protocol`, Tauri, and serde.
- Used by: `apps/uzel/src/App.svelte` via `invoke`.
- Rule: Add renderer-visible native operations here only after defining daemon-owned behavior in `crates/napd` and the wire contract in `crates/napd-protocol`.

**Private IPC Contract:**
- Purpose: Decouple desktop shell process from daemon process with bounded, versioned messages.
- Location: `crates/napd-protocol/`
- Contains: `Request`, `Response`, projected DTOs, client errors, frame encoding, transfer reassembly, replayable catalog calls, and `UnixClient`.
- Depends on: Unix streams, serde JSON, base64, and standard synchronization primitives.
- Used by: `apps/uzel/src-tauri/src/main.rs` and `crates/napd/src/server.rs`.
- Rule: Update request, response, client projection, bounds, and protocol tests together.

**Daemon Composition:**
- Purpose: Own process-local runtime authority and enforce product policy before calling upstream facades.
- Location: `crates/napd/`, entered through `apps/uzel-napd/src/main.rs`.
- Contains: Socket server, runtime adapter, exact fixture source, resource provider, product state, catalog state, and surface/session map.
- Depends on: `crates/napd-protocol` and pinned `nmp-native-*` crates declared in `Cargo.toml`.
- Used by: The Tauri native backend through `UnixClient`.
- Rule: Put reusable runtime mechanisms here; do not depend on Tauri, Svelte, styling, or demo-specific UI behavior (`scripts/check-boundaries.sh`).

**Portable Napplet Applications:**
- Purpose: Exercise NAP APIs as independent, untrusted single-file applications.
- Location: `napplets/`
- Contains: One package per napplet with `src/main.js`, focused model/probe modules, tests, HTML, and Vite config.
- Depends on: Published `@napplet/*` packages and shared files under `contracts/`.
- Used by: Build/sign scripts produce exact fixtures under `fixtures/`; the runtime launches those verified bytes.
- Rule: Do not import `apps/`, `crates/`, Tauri, Uzel internals, or another napplet (`scripts/check-napplet-imports.mjs`).

**Shared Payload Contracts:**
- Purpose: Give cross-napplet payloads one schema and validation owner.
- Location: `contracts/`
- Contains: JavaScript validators/projectors, JSON Schema, and contract tests.
- Depends on: Standard JavaScript only.
- Used by: `napplets/follow-list/src/main.js`, `napplets/profile-card/src/main.js`, and repository tests.
- Rule: Add versioned schemas and strict parsers here before adding a new cross-napplet topic.

## Data Flow

### Primary Startup and Surface Handshake

1. `pnpm dev` runs `scripts/dev.sh`, which starts `apps/uzel-napd/src/main.rs` in live mode and then starts the Tauri shell (`scripts/dev.sh:20`).
2. Daemon entry opens one `LinuxRunner`, binds `$XDG_RUNTIME_DIR/uzel/napd.sock`, prints readiness, and enters the serial accept loop (`apps/uzel-napd/src/main.rs:24`, `crates/napd/src/server.rs:345`).
3. Tauri setup registers one `UnixClient` and one `HostileProbeState`, then exposes the narrow invoke handler set (`apps/uzel/src-tauri/src/main.rs:504`).
4. Svelte `onMount` reconciles daemon leftovers, restores or selects the public read identity, requests `profile-card` and `follow-list`, and mounts verified HTML through `NMPTrustedShellHost` (`apps/uzel/src/App.svelte:418`, `apps/uzel/src/App.svelte:1029`).
5. `LinuxRunner::start_named_fixture` verifies the signed manifest, applies upstream permission decisions, launches the exact artifact, reads `/index.html` through `VerifiedRead`, and maps runtime session ID to a generation-bound surface token (`crates/napd/src/runner.rs:965`).
6. `DaemonState::stage_surface` retains HTML in process memory and sends it through ordered 2 KiB asset chunks; `UnixClient` reassembles and validates the transfer (`crates/napd/src/server.rs:269`, `crates/napd-protocol/src/lib.rs:448`).
7. Trusted shell materializes CSP/prelude into `iframe.srcdoc`, creates an opaque-origin `sandbox="allow-scripts"` frame, and maps its window to the surface (`apps/uzel/public/trusted-shell/trusted-shell-surface-host.js:53`).
8. Napplet `shell.ready` travels through the source-bound host, Tauri, AF_UNIX daemon, and upstream runtime; returned `shell.init` is delivered with a `MessageChannel` acknowledgement (`apps/uzel/src/App.svelte:276`, `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js:106`).

### Napplet Request/Response Path

1. An untrusted napplet uses an injected NAP facade, which posts a bounded envelope to its parent (`napplets/follow-list/src/main.js:1`, `apps/uzel/public/trusted-shell/trusted-shell.js`).
2. Surface host accepts only a message whose `event.source` equals a mounted frame's `contentWindow`, then emits a trusted DOM event containing the mapped session (`apps/uzel/public/trusted-shell/trusted-shell-surface-host.js:31`).
3. `App.svelte` invokes `forward_surface_envelope`; Tauri serializes `Request::ForwardEnvelope` through `UnixClient` (`apps/uzel/src/App.svelte:276`, `apps/uzel/src-tauri/src/main.rs:375`).
4. `DaemonState` resolves the surface token to a daemon-owned session and calls `LinuxRunner::forward_from_surface` (`crates/napd/src/server.rs:200`, `crates/napd/src/runner.rs:1126`).
5. `LinuxRunner` records an event cursor, calls upstream `RuntimeController::mapped_envelope`, waits at most 15 seconds for a correlated observation, and maps the response session back to a surface token (`crates/napd/src/runner.rs:1126`).
6. Large NAP-RESOURCE terminals use bounded private-IPC envelope chunks; Tauri returns the projected envelope to Svelte, which delivers it only to the mapped trusted surface (`crates/napd/src/server.rs:414`, `apps/uzel/src/App.svelte:282`).

### Cross-Napplet Profile Flow

1. `follow-list` obtains direct follows through NAP-IDENTITY, enriches them through bounded NAP-OUTBOX batches, and emits `napplet:profile/open` with the strict v1 contract (`napplets/follow-list/src/main.js`, `contracts/profile-open.js`).
2. `LinuxRunner::forward_from_surface` recognizes `inc.emit`, permits the response to originate from another active session, and selects the matching `inc.event` by topic (`crates/napd/src/runner.rs:1270`).
3. The daemon returns the target profile surface token; `App.svelte` sends the event only to that frame (`apps/uzel/src/App.svelte:276`).
4. `profile-card` validates the payload and queries the latest canonical kind-0 profile through NAP-OUTBOX using `contracts/kind0-profile.js` (`napplets/profile-card/src/main.js`).

### Catalog Review and Exact-Build Install

1. Renderer requests a signed `naddr` review; `UnixClient` assigns and retains a replayable operation ID (`apps/uzel/src/App.svelte:543`, `crates/napd-protocol/src/lib.rs:464`).
2. Daemon caches replayable catalog results by validated operation ID; `LinuxRunner` freezes the review token, coordinate, identity, aggregate hash, capabilities, sources, and provenance (`crates/napd/src/server.rs:113`, `crates/napd/src/runner.rs:674`).
3. Renderer sends the same frozen identity and selected domains to confirmation; `LinuxRunner` rejects mismatch, missing required domains, duplicate/unrequested domains, non-single-file artifacts, or non-affirmative permission decisions (`apps/uzel/src/App.svelte:605`, `crates/napd/src/runner.rs:748`).
4. Runtime launches exactly one new verified session; daemon reads only `/index.html`, assigns a fresh generation token, and returns bytes through the existing surface transfer (`crates/napd/src/runner.rs:902`).
5. Ambiguous transport outcomes remain explicit in the UI and use the same operation ID or require authoritative cleanup before later identity/catalog actions (`apps/uzel/src/App.svelte:369`, `apps/uzel/src/App.svelte:605`).

**State Management:**
- Svelte holds ephemeral UI, pane, loader, diagnostics, cleanup, and handshake state inside `apps/uzel/src/App.svelte`; local display preferences use `localStorage` through `apps/uzel/src/preferences.js`.
- Tauri holds process-local `UnixClient` retry state and `HostileProbeState` through managed state in `apps/uzel/src-tauri/src/main.rs:504`.
- Daemon holds active surface/session mappings, pending reviews, bounded replay cache, transfer offsets, and runtime observation buffers in `crates/napd/src/runner.rs` and `crates/napd/src/server.rs`.
- Durable product-owned state is only mode, selected public read identity, and next surface generation in `uzel-state.json`; upstream owns `runtime.sqlite3`, `nmp.redb`, and artifact cache (`crates/napd/src/runner.rs:372`, `crates/napd/src/runner.rs:543`).

## Key Abstractions

**`UnixClient`:**
- Purpose: Synchronous bounded client for one-request-per-connection AF_UNIX IPC, including retry reconciliation and chunk reassembly.
- Examples: `crates/napd-protocol/src/lib.rs:308`, `apps/uzel/src-tauri/src/main.rs:504`.
- Pattern: Protocol client/port with typed errors and replayable command IDs.

**`DaemonState` / `DaemonServer`:**
- Purpose: Separate pure request dispatch state from socket ownership and serving lifecycle.
- Examples: `crates/napd/src/server.rs:98`, `crates/napd/src/server.rs:331`.
- Pattern: Application service plus Unix transport adapter.

**`LinuxRunner`:**
- Purpose: Product policy adapter around one upstream runtime/NMP authority.
- Examples: `crates/napd/src/runner.rs:322`, `crates/napd/src/lib.rs:12`.
- Pattern: Facade/composition root with explicit bounded state.

**`SurfaceLaunch` / `SurfaceMetadata`:**
- Purpose: Carry verified identity, granted domains, unavailable domains, generation token, artifact base URL, and HTML across layers.
- Examples: `crates/napd/src/runner.rs:68`, `crates/napd-protocol/src/lib.rs:86`, `apps/uzel/src-tauri/src/main.rs:20`.
- Pattern: Layer-specific projections; Rust protocol owns native wire shape and Tauri projects camel-case UI data.

**Trusted Surface Host:**
- Purpose: Maintain the authoritative map from browser window source to daemon surface session and mediate all child traffic.
- Examples: `apps/uzel/public/trusted-shell/trusted-shell.js:309`, `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js:16`.
- Pattern: Capability membrane with opaque-origin sandboxed frames.

**`RuntimeController`:**
- Purpose: Upstream facade owning runtime sessions, verification, NMP accounts/query/store, permissions, catalog, providers, and observations.
- Examples: constructed in `crates/napd/src/runner.rs:383`; called throughout `crates/napd/src/runner.rs`.
- Pattern: Pinned upstream authority; adapt it instead of duplicating its domain behavior.

**Contract Functions:**
- Purpose: Strictly validate/version cross-napplet payloads and canonical profile projections.
- Examples: `contracts/profile-open.js`, `contracts/kind0-profile.js`.
- Pattern: Single schema owner shared by otherwise independent napplets.

## Entry Points

**Workspace development:**
- Location: `package.json`, `scripts/dev.sh`
- Triggers: `pnpm dev`
- Responsibilities: Start live daemon with configured relay lanes, then start Tauri/Vite shell; clean daemon child on exit.

**Daemon process:**
- Location: `apps/uzel-napd/src/main.rs`
- Triggers: `cargo run -p uzel-napd`, `scripts/dev.sh`, and smoke scripts.
- Responsibilities: Parse CLI mode/paths/relays, open `LinuxRunner`, bind private socket, serve requests.

**Desktop native process:**
- Location: `apps/uzel/src-tauri/src/main.rs`
- Triggers: Tauri application launch.
- Responsibilities: Register navigation policy, native managed state, and renderer command handlers.

**Web renderer:**
- Location: `apps/uzel/index.html`, `apps/uzel/src/main.ts`
- Triggers: Vite/Tauri WebView load.
- Responsibilities: Load trusted-shell globals first, mount `App.svelte` into `#app`.

**Portable napplets:**
- Location: `napplets/follow-list/src/main.js`, `napplets/profile-card/src/main.js`, `napplets/hostile-egress/src/main.js`
- Triggers: Verified single-file artifact execution inside trusted-shell iframe.
- Responsibilities: Use only projected NAP APIs and their own DOM/model code.

**Validation/runtime probes:**
- Location: `scripts/smoke.sh`, `scripts/linux-run-smoke.sh`, `scripts/debian13-live-test.sh`
- Triggers: Workspace scripts in `package.json`.
- Responsibilities: Exercise daemon readiness, composed shell, exact builds, NAP routing, hostile isolation, and platform-specific WebKit behavior.

## Architectural Constraints

- **Platform:** Linux-only POC. Native IPC uses `std::os::unix::net`; Tauri targets WebKitGTK (`apps/uzel-napd/src/main.rs`, `crates/napd-protocol/src/lib.rs`, `flake.nix`).
- **Threading:** `DaemonServer::serve` handles one accepted Unix connection at a time (`crates/napd/src/server.rs:365`). Upstream runtime observations may arrive asynchronously into a mutex/condvar buffer (`crates/napd/src/runner.rs:216`). NAP-RESOURCE owns a two-worker Tokio runtime (`crates/napd/src/resource.rs:47`). Hostile sentinel owns one bounded native thread (`apps/uzel/src-tauri/src/hostile_probe.rs:100`).
- **Global state:** Protocol operation IDs use one process-wide `AtomicU64` (`crates/napd-protocol/src/lib.rs:37`). Trusted shell installs reviewed globals on `window` (`apps/uzel/public/trusted-shell/trusted-shell.js`, `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js`). Tauri states are registered once in `apps/uzel/src-tauri/src/main.rs:507`.
- **Circular imports:** No local Rust or JavaScript circular dependency chain is detected. Preserve the dependency direction `apps → crates/napd-protocol`, `apps/uzel-napd → crates/napd`, `crates/napd → crates/napd-protocol + upstream`, and `napplets → contracts + published packages`; `scripts/check-boundaries.sh` and `scripts/check-napplet-imports.mjs` enforce critical edges.
- **Runtime ownership:** One daemon-owned `RuntimeController` owns NMP/runtime truth (`crates/napd/src/runner.rs:322`). Do not add a renderer, Tauri, or product-side Nostr cache/query engine.
- **Trust boundary:** Napplet frames receive no Tauri bridge, raw network permission, host path, secret, or caller-selected principal. Preserve CSP materialization and source binding in `apps/uzel/public/trusted-shell/`.
- **Protocol bounds:** Keep envelope, control-frame, asset, chunk, pending-operation, surface, review, and response-time limits aligned across `crates/napd-protocol/src/lib.rs`, `crates/napd/src/server.rs`, `crates/napd/src/runner.rs`, and trusted-shell constants.
- **Verified artifacts:** Runtime launch requires signed exact-build coordinates; catalog launch accepts single-file mode only (`crates/napd/src/runner.rs:748`). Artifact bytes cross memory/IPC, not filesystem URLs.
- **Cross-napplet ownership:** Put shared payload versions and validation under `contracts/`; napplets remain independent (`napplets/README.md`).
- **Vendored shell:** Files under `apps/uzel/public/trusted-shell/` are pinned reviewed upstream bytes. Update and validate them together, and update provenance in `apps/uzel/public/trusted-shell/README.md`.

## Anti-Patterns

### Renderer Coordination Monolith

**What happens:** `apps/uzel/src/App.svelte` owns product rendering plus startup reconciliation, catalog review/install, surface lifecycle, hostile-probe delivery, identity switching, recovery state machines, diagnostics polling, keybindings, and preference persistence.
**Why it's wrong:** New workflows can couple unrelated UI state, duplicate cleanup paths, and make lifecycle invariants difficult to test independently.
**Do this instead:** Keep `App.svelte` as composition root, but put new cohesive renderer state machines beside `apps/uzel/src/preferences.js` and `apps/uzel/src/projection-failure.js`; use focused modules named for the owned workflow, never a generic `utils` module.

### Hand-Mirrored UI DTOs

**What happens:** `SurfaceLaunch`, `NappletReview`, `RuntimeStatus`, diagnostics, and cleanup types are declared separately in `apps/uzel/src/App.svelte`, projected from Rust structs in `apps/uzel/src-tauri/src/main.rs`, and rooted in native types from `crates/napd-protocol/src/lib.rs`.
**Why it's wrong:** Field or serialization changes can drift across Rust and TypeScript without a compile-time cross-language contract.
**Do this instead:** When changing a DTO, update protocol struct, Tauri projection, TypeScript type, invocation use, and UI/native acceptance tests as one change; keep serde `camelCase` annotations explicit in `apps/uzel/src-tauri/src/main.rs` and `crates/napd-protocol/src/lib.rs`.

## Error Handling

**Strategy:** Fail closed at every trust boundary, preserve typed/native error context, bound external detail, and make ambiguous or incomplete cleanup visible instead of silently continuing.

**Patterns:**
- Use typed Rust error enums (`ClientError`, `ProtocolError`, `RunnerError`, `ServerError`) with `thiserror` in `crates/napd-protocol/src/lib.rs`, `crates/napd/src/runner.rs`, and `crates/napd/src/server.rs`.
- Convert runtime errors to bounded protocol `Response::Error` values in `crates/napd/src/server.rs:315`; Tauri projects ambiguous/cleanup-required cases as structured renderer errors in `apps/uzel/src-tauri/src/main.rs:34`.
- Reconcile replayable catalog calls with stable operation IDs and bounded caches (`crates/napd-protocol/src/lib.rs:464`, `crates/napd/src/server.rs:64`).
- Stop a runtime session if verified-document read or post-launch reconciliation fails (`crates/napd/src/runner.rs:1178`, `crates/napd/src/runner.rs:1209`).
- Preserve unresolved cleanup as explicit blocking UI state in `apps/uzel/src/App.svelte:369`, `apps/uzel/src/App.svelte:418`, and `apps/uzel/src/App.svelte:751`.
- Return `false`/`null` for invalid browser projections and refuse delivery in `apps/uzel/public/trusted-shell/trusted-shell.js` and `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js`.

## Cross-Cutting Concerns

**Logging:** Use stable `UZEL_*` process markers in `apps/uzel-napd/src/main.rs`, `apps/uzel/src-tauri/src/main.rs`, and `scripts/linux-run-smoke.sh`; use renderer status/envelope logs in `apps/uzel/src/App.svelte`. Keep sensitive transport material out of logs.
**Validation:** Validate at each owner: frame and transfer bounds in `crates/napd-protocol`, socket/surface/replay state in `crates/napd`, native navigation and hostile reports in `apps/uzel/src-tauri`, source/envelope/CSP projection in trusted shell, and payload schemas in `contracts/`.
**Authentication:** No user authentication exists. Selected Nostr pubkey is a public read context registered through NMP (`crates/napd/src/runner.rs:458`), not proof of identity. AF_UNIX access relies on private parent directory and `0600` socket permissions (`crates/napd/src/server.rs:345`).
**Persistence:** Keep product-owned JSON minimal and atomic in `crates/napd/src/runner.rs:543`; leave runtime, artifact, and NMP stores under upstream ownership.
**Capability policy:** Grant only reviewed, requested, available domains and preserve required-domain checks in `crates/napd/src/runner.rs:748`; project only negotiated domains in `apps/uzel/public/trusted-shell/trusted-shell.js`.

---

*Architecture analysis: 2026-08-09*
