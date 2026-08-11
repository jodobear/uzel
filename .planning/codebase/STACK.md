# Technology Stack

**Analysis Date:** 2026-08-09

## Languages

**Primary:**
- Rust 1.89.0, edition 2024 - Linux daemon, private protocol, Tauri backend, NMP composition, persistence, and native resource transport in `apps/uzel-napd/src/main.rs`, `apps/uzel/src-tauri/src/main.rs`, `crates/napd/src/`, and `crates/napd-protocol/src/lib.rs`; toolchain and minimum compiler are pinned in `rust-toolchain.toml` and `Cargo.toml`.
- TypeScript 6.0.3 - Svelte shell entry/type declarations and Vite configuration in `apps/uzel/src/main.ts`, `apps/uzel/src/app.d.ts`, and `apps/uzel/vite.config.ts`; strict ES2023 settings live in `apps/uzel/tsconfig.json`.
- JavaScript (ES modules, ES2023 build target) - portable napplets, trusted surface host, contracts, source checks, and Node tests in `napplets/*/src/*.js`, `apps/uzel/public/trusted-shell/*.js`, `contracts/*.js`, `scripts/check-napplet-imports.mjs`, and `*.test.mjs` files.
- Svelte 5.56.8 - product shell component and presentation in `apps/uzel/src/App.svelte`; dependency pin is in `apps/uzel/package.json`.

**Secondary:**
- Bash - development, build, smoke, Debian, Fedora, fixture-signing, boundary, and asset-pin automation in `scripts/*.sh`.
- HTML and CSS - Tauri/Vite entry document, shell styling, portable napplet entries, and checked-in exact fixtures in `apps/uzel/index.html`, `apps/uzel/src/style.css`, `napplets/*/index.html`, and `fixtures/*/index.html`.
- JSON and JSON Schema - Tauri config/capabilities, napplet events, runtime fixtures, Fallow config, and cross-napplet payload contract in `apps/uzel/src-tauri/tauri.conf.json`, `apps/uzel/src-tauri/capabilities/default.json`, `fixtures/*/event.json`, `config/fallow.jsonc`, and `contracts/profile-open-v1.schema.json`.
- Python 3 - documentation manifest/link/Mermaid audit in `uzel-poc-validated-pack/scripts/audit_docs.py`, invoked by `package.json`.
- Nix - reproducible x86_64 Linux development environment in `flake.nix`, pinned by `flake.lock`.

## Runtime

**Environment:**
- Node.js `>=22.12 <23` runs workspace scripts, built-in tests, Vite, Playwright, and napplet tooling; constraint is in `package.json`, while the container smoke image pins Node 22.23.1 in `Containerfile.debian`.
- Rust 1.89.0 runs the `uzel-napd` daemon and Tauri backend; `rust-toolchain.toml`, `Cargo.toml`, and `Containerfile.debian` agree on the 1.89 toolchain.
- Tauri 2.11.5 with WebKitGTK 4.1 is the Linux desktop runtime; Rust pin is in `Cargo.toml`, frontend API pin is in `apps/uzel/package.json`, and native packages are declared in `flake.nix` and `Containerfile.debian`.
- Tokio 1.53.1 supplies multi-thread async networking for bounded HTTPS resource fetches in `crates/napd/src/resource.rs`; the exact pin is in `Cargo.toml`.
- Deno is available in the Nix development shell and `deno.lock` is committed, but no tracked application or test entry point invokes Deno; do not make Deno a required runtime without adding an explicit script and source owner in `package.json`.

**Package Manager:**
- pnpm 10.8.0 - JavaScript workspace manager pinned by `package.json`, activated in `Containerfile.debian`, and mediated by Corepack in `flake.nix`.
- Cargo/Rustup 1.89.0 toolchain - Rust workspace dependency/build manager configured by `Cargo.toml` and `rust-toolchain.toml`.
- Nix flakes - host development dependency pinning in `flake.nix` and `flake.lock`.
- Lockfiles: `pnpm-lock.yaml`, `Cargo.lock`, `flake.lock`, and `deno.lock` are present and committed; use frozen/locked installs as shown in `README.md`, `scripts/debian13-live-test.sh`, and `scripts/debian-build-smoke.sh`.

## Frameworks

**Core:**
- Tauri 2.11.5 / `@tauri-apps/api` 2.11.1 - Linux desktop shell, Rust command bridge, navigation policy, and minimum desktop capability set in `apps/uzel/src-tauri/`, `apps/uzel/src/App.svelte`, and `apps/uzel/src-tauri/capabilities/default.json`.
- Svelte 5.56.8 - single product-shell UI in `apps/uzel/src/App.svelte`; keep runtime truth and daemon protocol types out of Svelte per `uzel-poc-validated-pack/AGENTS.md`.
- NMP/nampplets native runtime 0.1.0 - runtime controller, Nostr data plane, NAP providers, catalog verification, surface sessions, and stores composed in `crates/napd/src/runner.rs`; four direct crates are pinned to `jodobear/nampplets@e2f69f325a6b45213accdacfcc125e80e0687b4c` in `Cargo.toml` and `Cargo.lock`.
- Napplet NAP packages - `@napplet/nap` 0.29.0 and `@napplet/shim` 0.27.0 define portable runtime-facing APIs in `napplets/*/package.json`; keep napplets independent from Uzel/Tauri code.

**Testing:**
- Rust built-in test harness through `cargo test` - unit, protocol, daemon, resource, persistence, and explicit ignored live-network tests colocated under `#[cfg(test)]` in `apps/uzel-napd/src/main.rs`, `apps/uzel/src-tauri/src/*.rs`, `crates/napd/src/*.rs`, and `crates/napd-protocol/src/lib.rs`.
- Node.js built-in `node:test` - contract, napplet model/probe, shell helper, and renderer acceptance tests in `contracts/*.test.mjs`, `napplets/*/tests/*.test.mjs`, `apps/uzel/src/*.test.mjs`, and `apps/uzel/tests/ui/acceptance.test.mjs`.
- Playwright 1.62.0 - deterministic Chromium renderer acceptance controlled by `apps/uzel/tests/ui/playwright.config.mjs` and `apps/uzel/tests/ui/acceptance.test.mjs`.
- `@napplet/conformance-cli` 0.2.16 with root overrides for `@napplet/conformance` 0.14.0 and `@napplet/core` 0.29.0 - built artifact conformance in `napplets/follow-list/package.json`, `napplets/profile-card/package.json`, and `package.json`.

**Build/Dev:**
- Vite 8.1.5 - builds the Svelte shell and all three single-file napplets; configs are `apps/uzel/vite.config.ts` and `napplets/*/vite.config.js`.
- `@sveltejs/vite-plugin-svelte` 7.2.0 - Svelte compilation in `apps/uzel/vite.config.ts`.
- `@napplet/vite-plugin` 0.12.0 - generates NIP-5A manifests and single-file artifacts with explicit capability declarations in `napplets/*/vite.config.js`.
- `tauri-build` 2.6.3 and Cargo Tauri from the Nix environment - native app generation through `apps/uzel/src-tauri/build.rs`, `apps/uzel/src-tauri/tauri.conf.json`, and `flake.nix`.
- Fallow 3.9.1 - unresolved import, unlisted dependency, unused file, and unused dependency gate configured in `config/fallow.jsonc` and invoked by `package.json`.
- Cargo Clippy and rustfmt - warning-denying lint and formatting gates pinned as toolchain components in `rust-toolchain.toml` and invoked by `package.json`.
- Nix + Podman - reproducible host shell and immutable-digest Debian build smoke through `flake.nix`, `Containerfile.debian`, and `scripts/debian-build-smoke.sh`.

## Key Dependencies

**Critical:**
- `nmp-native-runtime-ffi`, `nmp-native-runtime-core`, `nmp-native-nap-bridge`, and `nmp-native-provider-resource` 0.1.0 at exact nampplets SHA - sole runtime/NMP/provider composition boundary in `Cargo.toml` and `crates/napd/src/runner.rs`; update all pins together and revalidate exact artifacts.
- Transitive `nmp` 0.1.0 at `pablof7z/nmp@005dc2a5f12aa414961b313d05ebb021934e385c` - Nostr engine, transport, resolver, signer types, and redb store recorded in `Cargo.lock`; access it through the pinned nampplets runtime rather than adding a parallel data plane.
- Tauri 2.11.5 and `@tauri-apps/api` 2.11.1 - native shell and renderer IPC in `Cargo.toml`, `apps/uzel/package.json`, `apps/uzel/src-tauri/src/main.rs`, and `apps/uzel/src/App.svelte`.
- Svelte 5.56.8 - product presentation in `apps/uzel/src/App.svelte` and `apps/uzel/package.json`.
- `@napplet/nap` 0.29.0 and `@napplet/shim` 0.27.0 - capability-scoped identity, INC, outbox, resource, shell, and config APIs used by `napplets/*/src/main.js` and declared in `napplets/*/package.json`.
- `serde` 1.0.217, `serde_json` 1.0.138, and `thiserror` 2.0.11 - typed protocol serialization and error surfaces throughout `crates/napd-protocol/src/lib.rs`, `crates/napd/src/`, and `apps/uzel/src-tauri/src/main.rs`; pins are in `Cargo.toml`.

**Infrastructure:**
- `reqwest` 0.12.28 with default features disabled and `rustls-tls` enabled - HTTPS-only, no-proxy, DNS-pinned NAP-RESOURCE transport in `crates/napd/src/resource.rs` and `Cargo.toml`.
- Tokio 1.53.1 - bounded DNS/HTTPS execution in `crates/napd/src/resource.rs`; network/runtime features are declared in `Cargo.toml`.
- `url` 2.5.8 - URL validation and trusted artifact-base generation in `crates/napd/src/runner.rs` and `Cargo.toml`.
- `base64` 0.22.1 - chunked binary protocol payloads in `crates/napd-protocol/src/lib.rs` and `Cargo.toml`.
- redb 4.1.0 and rusqlite 0.32.1 - transitive NMP event storage and native runtime metadata storage recorded in `Cargo.lock`; configured paths are `nmp.redb` and `runtime.sqlite3` in `crates/napd/src/runner.rs`.
- `tempfile` 3.23.0 - isolated filesystem/socket tests for `crates/napd` as declared in `crates/napd/Cargo.toml`.

## Configuration

**Environment:**
- Runtime configuration is CLI-first: `uzel-napd` accepts `--live`, `--socket`, `--runtime-root`, repeated `--indexer-relay`, `--app-relay`, `--fallback-relay`, and `--allow-local-relay-host` options in `apps/uzel-napd/src/main.rs`.
- `XDG_RUNTIME_DIR` determines the private `uzel/napd.sock` path for both daemon and Tauri client; `XDG_DATA_HOME` or `HOME` determines the persistent `uzel` runtime root unless explicit CLI paths are supplied in `apps/uzel-napd/src/main.rs` and `apps/uzel/src-tauri/src/main.rs`.
- `UZEL_RUN_HOSTILE_PROBE=1` enables the smoke-only hostile egress path in `apps/uzel/src-tauri/src/main.rs`; `UZEL_PLAYWRIGHT_CHROMIUM`, `UZEL_UI_ARTIFACT_ROOT`, `UZEL_UI_SCENARIOS`, and `UZEL_UI_FAULT_PROBE_CHILD` configure renderer tests in `apps/uzel/tests/ui/`.
- Smoke/build overrides are optional and bounded: `UZEL_SMOKE_*`, `UZEL_DEBIAN_SMOKE_IMAGE`, and `UZEL_NAPPLET_FIXTURE` are consumed only by `scripts/*.sh`.
- No tracked `.env` file exists in the primary repository; do not introduce secret-bearing environment files into source control.

**Build:**
- Root orchestration and version pins: `package.json`, `pnpm-workspace.yaml`, `pnpm-lock.yaml`, `Cargo.toml`, `Cargo.lock`, and `rust-toolchain.toml`.
- Frontend/native configs: `apps/uzel/vite.config.ts`, `apps/uzel/tsconfig.json`, `apps/uzel/src-tauri/tauri.conf.json`, and `apps/uzel/src-tauri/capabilities/default.json`.
- Napplet build configs: `napplets/follow-list/vite.config.js`, `napplets/profile-card/vite.config.js`, and `napplets/hostile-egress/vite.config.js`.
- Reproducible Linux configs: `flake.nix`, `flake.lock`, `Containerfile.debian`, and `.dockerignore`.
- Static boundary and dependency checks: `scripts/check-boundaries.sh`, `scripts/check-napplet-imports.mjs`, `scripts/check-pinned-assets.sh`, and `config/fallow.jsonc`.

## Platform Requirements

**Development:**
- Use x86_64 Linux with the pinned Nix shell from `flake.nix`; it supplies Rust, Node/Corepack, cargo-tauri, WebKitGTK 4.1, GTK3, Chromium, Weston, Deno, Python, and native build libraries.
- Install frontend dependencies with `pnpm install --frozen-lockfile`, then use root commands from `package.json`; setup and run sequence is documented in `README.md`.
- Real renderer/native smoke requires Wayland/Weston, WebKitGTK, and an XDG runtime directory as prepared by `scripts/linux-run-smoke.sh`.
- Debian 13 live acceptance requirements and host packages are encoded in `scripts/debian13-setup.sh`, `scripts/debian13-live-test.sh`, and `DEBIAN13-LIVE-TEST.md`.

**Production:**
- Deployment target is a local Linux desktop application plus a same-user `uzel-napd` process over a mode-0600 AF_UNIX socket; entry points are `apps/uzel/src-tauri/src/main.rs`, `apps/uzel-napd/src/main.rs`, and `crates/napd/src/server.rs`.
- Tauri bundle generation is disabled (`"active": false`) in `apps/uzel/src-tauri/tauri.conf.json`; the repository defines a POC/runtime and build-smoke path, not a distributable package pipeline.
- No server hosting target is defined; network access is outbound Nostr WebSocket and policy-bounded HTTPS from the local daemon as implemented in `crates/napd/src/runner.rs` and `crates/napd/src/resource.rs`.

---

*Stack analysis: 2026-08-09*
