# External Integrations

**Analysis Date:** 2026-08-09

## APIs & External Services

**Nostr relay network:**
- NMP-managed WebSocket relays - identity/outbox queries, canonical kind-0 and kind-3/contact evidence, NIP-65 relay discovery, signed NIP-5D catalog lookup, subscriptions, reconnects, and cache replay.
  - SDK/Client: `RuntimeController` through exact-SHA `nmp-native-runtime-ffi`/nampplets composition in `crates/napd/src/runner.rs`; transitive `nmp` and `nostr` crates are pinned in `Cargo.lock`.
  - Endpoints: development and live smoke pass `wss://purplepag.es` as indexer/app relay and `wss://nos.lol` as a second app relay in `scripts/dev.sh`; operators can replace lanes with `uzel-napd` CLI flags from `apps/uzel-napd/src/main.rs`.
  - Auth: selected public key is read-only context, not login; NIP-42 access can be reported in diagnostics but Uzel supplies no signer or credential flow in `crates/napd/src/runner.rs` and `apps/uzel/src/App.svelte`.
  - Ownership: send relay demand only through NAP identity/outbox APIs in `napplets/follow-list/src/main.js` and `napplets/profile-card/src/main.js`; do not add `fetch`, `WebSocket`, or a second Nostr cache to napplets.

**NIP-5D napplet catalog and artifacts:**
- Signed kind-35129 manifests and immutable single-file artifacts - users submit `naddr`/`nostr:naddr` coordinates, NMP resolves and verifies the manifest, freezes review evidence, confirms permissions, and launches the exact aggregate.
  - SDK/Client: `RuntimeController::catalog_review_manual` and `catalog_confirm_install` in `crates/napd/src/runner.rs`, surfaced through `crates/napd-protocol/src/lib.rs`, `apps/uzel/src-tauri/src/main.rs`, and `apps/uzel/src/App.svelte`.
  - Auth: Nostr event signatures and content hashes verified by the pinned nampplets/NMP stack in `Cargo.lock`; Uzel compares author, d-tag, aggregate hash, execution mode, and frozen review in `crates/napd/src/runner.rs`.
  - Artifact source: catalog-declared remote blob sources are owned by the upstream runtime; no fixed production Blossom endpoint is configured by Uzel. Deterministic fixtures are embedded from `fixtures/*/event.json` and `fixtures/*/index.html` through `crates/napd/src/fixtures.rs`.

**Public HTTPS resources:**
- NAP-RESOURCE image delivery - profile/follow pictures are requested by napplets through the resource capability and fetched by the native daemon.
  - SDK/Client: upstream `nmp-native-provider-resource` plus local `reqwest` 0.12.28 adapter in `crates/napd/src/resource.rs`.
  - Transport policy: HTTPS-only, rustls, no proxy, no redirects at transport level, DNS resolution followed by approved-address pinning, bounded deadlines/body sizes, and cancellation in `crates/napd/src/resource.rs`.
  - Auth: none; ordinary public HTTPS GETs carry only `uzel-linux-poc/0.0.0` user agent in `crates/napd/src/resource.rs`.
  - Blossom: NAP-RESOURCE receives a non-routable `https://blossom.invalid/` constructor fallback because Uzel excludes Blossom from this POC; do not treat that value as an external service endpoint in `crates/napd/src/resource.rs`.

**Desktop/native boundary:**
- Tauri IPC and private AF_UNIX daemon protocol - Svelte invokes Tauri commands, Tauri translates them to typed framed requests, and `uzel-napd` owns all NMP/runtime work.
  - SDK/Client: `@tauri-apps/api` `invoke` calls in `apps/uzel/src/App.svelte`, Tauri commands in `apps/uzel/src-tauri/src/main.rs`, `UnixClient` in `crates/napd-protocol/src/lib.rs`, and `DaemonServer` in `crates/napd/src/server.rs`.
  - Auth: same-user filesystem isolation rather than tokens; socket parent validation and mode 0600 binding are enforced in `crates/napd/src/server.rs`.
  - Endpoint: `${XDG_RUNTIME_DIR}/uzel/napd.sock` by default in `apps/uzel-napd/src/main.rs` and `apps/uzel/src-tauri/src/main.rs`.

**Test-only browser isolation probe:**
- Loopback hostile sentinel - verifies that untrusted napplet frames cannot use browser networking or the Tauri native bridge.
  - SDK/Client: ephemeral `TcpListener` and report state in `apps/uzel/src-tauri/src/hostile_probe.rs`; hostile `fetch`, `WebSocket`, `EventSource`, Worker, media, and DOM attempts live in `napplets/hostile-egress/src/`.
  - Auth: exact surface token/source binding in `apps/uzel/src-tauri/src/main.rs`; enabled only by `UZEL_RUN_HOSTILE_PROBE=1`.

## Data Storage

**Databases:**
- SQLite file `runtime.sqlite3` - upstream nampplets runtime metadata/session store.
  - Connection: local path under configured runtime root; no DSN or network database.
  - Client: transitive rusqlite 0.32.1 through `nmp-native-runtime-store`, pinned in `Cargo.lock` and configured in `crates/napd/src/runner.rs`.
- redb file `nmp.redb` - NMP event/query/cache store and cache-first restart source.
  - Connection: local path under configured runtime root; no server connection.
  - Client: transitive redb 4.1.0 through NMP, pinned in `Cargo.lock` and configured in `crates/napd/src/runner.rs`.

**File Storage:**
- Local filesystem only - verified artifacts are cached under `<runtime-root>/artifacts`, and Uzel product state is atomically persisted as `<runtime-root>/uzel-state.json` by `crates/napd/src/runner.rs`.
- Runtime root defaults to `${XDG_DATA_HOME}/uzel` or `${HOME}/.local/share/uzel`; override with `--runtime-root` in `apps/uzel-napd/src/main.rs`.
- Checked-in signed fixtures live in `fixtures/` and are compiled into the daemon by `include_bytes!` in `crates/napd/src/fixtures.rs`; WebKit receives verified bytes over the bounded daemon protocol, not a host path.
- Browser-local preferences use `localStorage` keys owned by `apps/uzel/src/App.svelte`; runtime identity, relay truth, events, and artifacts must not be stored there.

**Caching:**
- NMP/redb provides persistent event and query evidence caching at `nmp.redb`; ownership stays in upstream NMP via `RuntimeController` in `crates/napd/src/runner.rs`.
- Verified artifact cache is the local `artifacts` directory configured in `crates/napd/src/runner.rs`.
- In-memory caches are bounded: runtime events in `crates/napd/src/runner.rs`, replayed operation replies in `crates/napd/src/server.rs`, and renderer object URLs/query scheduling in `napplets/follow-list/src/main.js`.
- External cache service: None detected; no Redis/Memcached dependency or endpoint appears in `Cargo.toml`, `package.json`, or `crates/napd/src/`.

## Authentication & Identity

**Auth Provider:**
- No authentication provider - Uzel accepts an `npub` or 64-character hex public key as selected read context and registers a read-only NMP account in `crates/napd/src/runner.rs`, initiated from `apps/uzel/src/App.svelte` through `apps/uzel/src-tauri/src/main.rs`.
  - Implementation: `RuntimeController::register_read_only_account` plus `activate_local_account`; only the selected public key and product mode are persisted in `uzel-state.json` by `crates/napd/src/runner.rs`.
  - No password, OAuth, wallet, signer, private-key import, or session-token implementation exists in `apps/`, `crates/`, or `napplets/`.
  - Signed napplet authenticity is content verification, not user authentication: exact author/d-tag/aggregate checks live in `crates/napd/src/runner.rs`.
  - NIP-42 relay access is represented in relay diagnostics in `crates/napd/src/runner.rs`, but no Uzel credential or signing flow configures it.

## Monitoring & Observability

**Error Tracking:**
- None detected - no Sentry, OpenTelemetry, hosted error service, or telemetry dependency appears in `Cargo.toml`, `package.json`, `apps/`, or `crates/`.

**Logs:**
- Daemon and Tauri processes write human-readable errors and stable `UZEL_*` acceptance markers to stdout/stderr in `apps/uzel-napd/src/main.rs`, `apps/uzel/src-tauri/src/main.rs`, and `apps/uzel/src-tauri/src/hostile_probe.rs`.
- Live runtime diagnostics expose snapshot revision, sessions, active identity, per-relay lanes/subscriptions/event counts, NIP-11 freshness, rejected private relays, and store/transport degradation through `crates/napd/src/runner.rs`, `crates/napd-protocol/src/lib.rs`, and `apps/uzel/src/App.svelte`.
- Smoke scripts capture daemon/shell and Weston output, preserve failures under configured artifact/evidence directories, and assert ordered markers in `scripts/linux-run-smoke.sh` and `scripts/debian13-live-test.sh`.
- Renderer acceptance writes screenshots and failure state to `.artifacts/ui-acceptance` or `UZEL_UI_ARTIFACT_ROOT` through `apps/uzel/tests/ui/playwright.config.mjs` and `apps/uzel/tests/ui/acceptance.test.mjs`; `.artifacts/` is ignored by `.gitignore`.

## CI/CD & Deployment

**Hosting:**
- Local Linux desktop only - Tauri/WebKit shell plus same-user daemon entry points are `apps/uzel/src-tauri/src/main.rs` and `apps/uzel-napd/src/main.rs`.
- No cloud hosting, remote daemon, deployment platform, or public HTTP server is configured in `apps/`, `crates/`, `flake.nix`, or `Containerfile.debian`.
- Tauri bundling is disabled in `apps/uzel/src-tauri/tauri.conf.json`; `Containerfile.debian` and `scripts/debian-build-smoke.sh` validate builds rather than publish an image.

**CI Pipeline:**
- None detected in the tracked primary repository - no `.github/workflows`, GitLab CI, or other hosted pipeline config is committed.
- Local reproducible gates are orchestrated by `package.json`, `scripts/debian-build-smoke.sh`, `scripts/debian13-live-test.sh`, `scripts/linux-run-smoke.sh`, `flake.nix`, and `Containerfile.debian`.
- Use locked/frozen dependency modes from `README.md` and `scripts/debian13-live-test.sh`; do not convert a green local smoke into a publication claim.

## Environment Configuration

**Required env vars:**
- `XDG_RUNTIME_DIR` - required by both daemon and Tauri shell for the default private socket path in `apps/uzel-napd/src/main.rs` and `apps/uzel/src-tauri/src/main.rs`; an explicit daemon `--socket` replaces the daemon-side requirement but the shell still reads the XDG path.
- `XDG_DATA_HOME` or `HOME` - supplies the default persistent runtime root in `apps/uzel-napd/src/main.rs`; explicit `--runtime-root` removes this daemon requirement.
- No relay endpoint env var exists; pass repeated `--indexer-relay`, `--app-relay`, `--fallback-relay`, and `--allow-local-relay-host` flags defined in `apps/uzel-napd/src/main.rs`.
- Optional test configuration: `UZEL_RUN_HOSTILE_PROBE`, `UZEL_PLAYWRIGHT_CHROMIUM`, `UZEL_UI_ARTIFACT_ROOT`, `UZEL_UI_SCENARIOS`, and `UZEL_UI_FAULT_PROBE_CHILD` in `apps/uzel/src-tauri/src/main.rs` and `apps/uzel/tests/ui/`.
- Optional automation configuration: `UZEL_SMOKE_NAME`, `UZEL_SMOKE_SUCCESS_MARKER`, `UZEL_SMOKE_ARTIFACT_DIR`, `UZEL_SMOKE_EVIDENCE_DIR`, `UZEL_SMOKE_STARTUP_TIMEOUT_SECONDS`, `UZEL_SMOKE_RUNTIME_TIMEOUT_SECONDS`, `UZEL_SMOKE_SHUTDOWN_GRACE_SECONDS`, `UZEL_DEBIAN_SMOKE_IMAGE`, and `UZEL_NAPPLET_FIXTURE` in `scripts/*.sh`.

**Secrets location:**
- Not applicable - no tracked `.env`, credential store, secret manager, or service token config exists in the primary repository.
- Public identities and signed public events are not secrets; only the chosen public read key is persisted by `crates/napd/src/runner.rs`.
- Do not add signer/private-key material: the runtime scope and dependency boundary in `uzel-poc-validated-pack/AGENTS.md` exclude wallets and signers.

## Webhooks & Callbacks

**Incoming:**
- None - no public HTTP listener or webhook route exists in `apps/` or `crates/`.
- Internal callbacks are Tauri commands, framed AF_UNIX daemon requests, runtime observation sinks, browser `MessageEvent` routing, and the test-only loopback hostile sentinel in `apps/uzel/src-tauri/src/main.rs`, `crates/napd/src/server.rs`, `crates/napd/src/runner.rs`, and `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js`; none is an Internet webhook.

**Outgoing:**
- None - no webhook delivery implementation exists in `apps/`, `crates/`, `napplets/`, or `scripts/`.
- Outbound traffic is limited to NMP-managed Nostr relay sessions and native policy-bounded HTTPS resource/artifact retrieval described above; napplet-originated direct browser egress is forbidden and tested by `napplets/hostile-egress/src/` and `scripts/check-napplet-imports.mjs`.

---

*Integration audit: 2026-08-09*
