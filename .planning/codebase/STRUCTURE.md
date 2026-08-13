# Codebase Structure

**Analysis Date:** 2026-08-09

## Directory Layout

```text
uzel/
├── apps/
│   ├── uzel/                    # Svelte renderer, Tauri backend, trusted shell, UI tests
│   └── uzel-napd/               # Linux daemon binary entry point
├── crates/
│   ├── napd/                    # Runtime composition, server, fixtures, resource adapter
│   └── napd-protocol/           # Versioned bounded AF_UNIX wire contract/client
├── napplets/
│   ├── follow-list/             # Portable direct-follow selector napplet
│   ├── profile-card/            # Portable kind-0 profile napplet
│   └── hostile-egress/          # Test-only hostile browser/native probe napplet
├── contracts/                   # Shared cross-napplet schema, validation, query projection
├── fixtures/                    # Signed exact-build manifests and generated single-file HTML
├── scripts/                     # Dev, boundary, build, smoke, and platform automation
├── config/                      # Workspace analysis/tool configuration
├── graphify-out/                # Committed generated code graph and report
├── uzel-poc-validated-pack/     # Scope, evidence, work slices, source pins, status
├── .planning/codebase/          # GSD codebase maps
├── Cargo.toml                   # Rust workspace and shared pinned dependencies
├── package.json                 # JavaScript workspace orchestration and quality commands
├── pnpm-workspace.yaml          # Frontend/napplet workspace membership
├── flake.nix                    # Pinned x86_64 Linux development environment
├── Containerfile.debian         # Pinned Debian build environment
├── README.md                    # Operator/developer entry documentation
└── AGENTS.md                    # Root agent routing to validated-pack instructions
```

This layout describes the git-tracked primary repository. Ignore untracked archival copies and ZIP exports when navigating or adding code; use `git ls-files` as the authoritative primary-tree inventory.

## Directory Purposes

**`apps/uzel/`:**
- Purpose: Own the installed desktop product shell and presentation boundary.
- Contains: Svelte application, Tauri Rust backend, pinned trusted-shell browser assets, shell-specific helpers, styles, and UI acceptance tests.
- Key files: `apps/uzel/src/App.svelte`, `apps/uzel/src/main.ts`, `apps/uzel/src-tauri/src/main.rs`, `apps/uzel/index.html`, `apps/uzel/tests/ui/acceptance.test.mjs`.
- Boundary: Product-specific UI/Tauri code stays here; reusable daemon/runtime behavior belongs under `crates/napd/`.

**`apps/uzel/public/trusted-shell/`:**
- Purpose: Bundle reviewed upstream browser mediation bytes unavailable through Cargo resources.
- Contains: CSP/policy, NAP prelude/domain projection, artifact materialization, and multi-surface host.
- Key files: `apps/uzel/public/trusted-shell/trusted-shell.js`, `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js`, `apps/uzel/public/trusted-shell/README.md`.
- Boundary: Treat the directory as a pinned unit. Do not locally fork one file without updating the full reviewed pin and validation evidence.

**`apps/uzel/src-tauri/`:**
- Purpose: Own the native desktop process and renderer-to-daemon adapter.
- Contains: Tauri commands, navigation policy, hostile probe state, Tauri config, capability manifest, icons, and build script.
- Key files: `apps/uzel/src-tauri/src/main.rs`, `apps/uzel/src-tauri/src/hostile_probe.rs`, `apps/uzel/src-tauri/tauri.conf.json`, `apps/uzel/src-tauri/capabilities/default.json`.
- Boundary: Keep runtime/NMP logic out; use `napd-protocol::UnixClient`.

**`apps/uzel-napd/`:**
- Purpose: Own the executable Linux daemon entry point.
- Contains: Minimal crate manifest and CLI/process bootstrap.
- Key files: `apps/uzel-napd/src/main.rs`, `apps/uzel-napd/Cargo.toml`.
- Boundary: Parse process configuration and compose `napd`; put reusable behavior in `crates/napd/`.

**`crates/napd/`:**
- Purpose: Own reusable daemon-side mechanisms and product policy adaptation around Nampplets.
- Contains: Library facade, exact fixtures, hardened resource provider, runtime runner, AF_UNIX server, and in-module Rust tests.
- Key files: `crates/napd/src/lib.rs`, `crates/napd/src/runner.rs`, `crates/napd/src/server.rs`, `crates/napd/src/resource.rs`, `crates/napd/src/fixtures.rs`.
- Boundary: Must not depend on Tauri, Svelte, styling, or renderer code; `scripts/check-boundaries.sh` enforces this.

**`crates/napd-protocol/`:**
- Purpose: Own the native shell/daemon contract and client.
- Contains: Request/response/DTO enums and structs, wire bounds, length-prefixed JSON frames, base64 chunk helpers, retry tracking, and unit tests.
- Key files: `crates/napd-protocol/src/lib.rs`, `crates/napd-protocol/Cargo.toml`.
- Boundary: Keep this crate transport-focused and usable by both Tauri shell and daemon server.

**`napplets/`:**
- Purpose: Own independent runtime-agnostic untrusted applications.
- Contains: One directory per napplet, each with HTML entry, package manifest, Vite config, `src/`, and `tests/`.
- Key files: `napplets/README.md`, `napplets/follow-list/src/main.js`, `napplets/profile-card/src/main.js`, `napplets/hostile-egress/src/main.js`.
- Boundary: Import only published Napplet packages, same-napplet modules, and shared `contracts/`; `scripts/check-napplet-imports.mjs` rejects product/runtime/peer imports.

**`contracts/`:**
- Purpose: Own schemas and strict projections shared across otherwise independent napplets.
- Contains: Versioned JSON Schema, JavaScript parsers/builders/query projection, and Node tests.
- Key files: `contracts/profile-open-v1.schema.json`, `contracts/profile-open.js`, `contracts/kind0-profile.js`, `contracts/profile-open.test.mjs`, `contracts/kind0-profile.test.mjs`.
- Boundary: Put cross-napplet payload ownership here, not in either producer or consumer napplet.

**`fixtures/`:**
- Purpose: Store committed signed NIP-5D event fixtures and exact single-file artifact bytes consumed by runtime tests/demo.
- Contains: Per-fixture `event.json` and `index.html`, a Nostr live-event fixture, and regeneration guidance.
- Key files: `fixtures/README.md`, `fixtures/follow-list/event.json`, `fixtures/follow-list/index.html`, `fixtures/profile-card/event.json`, `fixtures/profile-card/index.html`, `fixtures/hostile-egress/event.json`, `fixtures/hostile-egress/index.html`.
- Boundary: Repin fixture event, HTML, hashes in `crates/napd/src/fixtures.rs`, and pinned-asset checks together.

**`scripts/`:**
- Purpose: Own reproducible repository orchestration and acceptance commands.
- Contains: Development launcher, signed-fixture builder, import/boundary/pin checks, Linux/Fedora/Debian build and WebKit smokes.
- Key files: `scripts/dev.sh`, `scripts/check-boundaries.sh`, `scripts/check-napplet-imports.mjs`, `scripts/check-pinned-assets.sh`, `scripts/linux-run-smoke.sh`, `scripts/debian13-live-test.sh`.
- Boundary: Prefer a named script for multi-process/platform workflows; keep behavior fail-fast with explicit evidence markers.

**`config/`:**
- Purpose: Hold repository-wide tool configuration not owned by a language ecosystem root file.
- Contains: Fallow dependency/file analysis config.
- Key files: `config/fallow.jsonc`.

**`graphify-out/`:**
- Purpose: Hold the queryable architecture/code graph required before codebase exploration.
- Contains: `graph.json`, generated HTML visualization, graph report, labels, and manifest. Machine-local AST/stat cache is ignored and never committed.
- Key files: `graphify-out/graph.json`, `graphify-out/GRAPH_REPORT.md`, `graphify-out/graph.html`, `graphify-out/manifest.json`.
- Boundary: Query `graphify-out/graph.json` first for codebase questions. After code changes, run the locked `pnpm graphify:refresh` entrypoint and commit refreshed canonical graph output separately.

**`uzel-poc-validated-pack/`:**
- Purpose: Own normative POC scope, evidence, source pins, work slices, contribution ledger, and durable status.
- Contains: Agent instructions, `STATUS.md`, `compatibility.lock`, design/validation docs, fact records, reports, work slices, audit scripts, and templates.
- Key files: `uzel-poc-validated-pack/AGENTS.md`, `uzel-poc-validated-pack/STATUS.md`, `uzel-poc-validated-pack/docs/00-scope.md`, `uzel-poc-validated-pack/work/07-stabilize-product.md`, `uzel-poc-validated-pack/docs/08-upstream-contributions.md`.
- Boundary: Read active slice and only named source documents before implementation; update status/evidence under its workflow rules.

## Key File Locations

**Entry Points:**
- `package.json`: Workspace-level dev/build/check/test/lint/smoke command router.
- `scripts/dev.sh`: Starts live daemon and Tauri development shell.
- `apps/uzel-napd/src/main.rs`: Daemon executable entry.
- `apps/uzel/src-tauri/src/main.rs`: Tauri native executable entry and command registry.
- `apps/uzel/index.html`: Browser document that loads trusted-shell assets before the renderer.
- `apps/uzel/src/main.ts`: Svelte mount entry.
- `napplets/*/src/main.js`: Independent napplet browser entries.

**Configuration:**
- `Cargo.toml`: Rust workspace members, editions, Rust version, local crate links, and exact upstream Nampplets revision.
- `rust-toolchain.toml`: Rust `1.89.0`, Clippy, and rustfmt pin.
- `package.json`: Node engine, pnpm version, package overrides, and workspace scripts.
- `pnpm-workspace.yaml`: Includes `apps/uzel` and every `napplets/*` package.
- `apps/uzel/package.json`: Svelte/Vite/Tauri frontend commands and dependencies.
- `apps/uzel/vite.config.ts`: Local Vite host/port and Svelte plugin.
- `apps/uzel/tsconfig.json`: Strict checked TypeScript/JavaScript renderer configuration.
- `apps/uzel/src-tauri/tauri.conf.json`: Desktop window, build hooks, frontend dist, and shell CSP.
- `apps/uzel/src-tauri/capabilities/default.json`: Minimal Tauri authority for the main window.
- `flake.nix`, `flake.lock`: Pinned x86_64 Linux development environment.
- `Containerfile.debian`: Pinned Debian build toolchain.
- `config/fallow.jsonc`: Unused/unresolved dependency and file checks.

**Core Logic:**
- `apps/uzel/src/App.svelte`: Product/UI composition and renderer lifecycle state machines.
- `apps/uzel/public/trusted-shell/trusted-shell.js`: Trusted materialization and NAP projection primitives.
- `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js`: Multi-surface source binding and iframe lifecycle.
- `apps/uzel/src-tauri/src/main.rs`: Native command adapter and navigation policy.
- `crates/napd-protocol/src/lib.rs`: Private wire contract and client.
- `crates/napd/src/server.rs`: Socket/server dispatch and transfer staging.
- `crates/napd/src/runner.rs`: Runtime/NMP composition, state, exact-build lifecycle, and routing.
- `crates/napd/src/resource.rs`: Hardened Linux NAP-RESOURCE transport adapter.
- `contracts/profile-open.js`, `contracts/kind0-profile.js`: Shared payload/query/projection ownership.

**Testing:**
- `crates/napd/src/*.rs`, `crates/napd-protocol/src/lib.rs`: Rust unit and integration-style module tests under `#[cfg(test)]`.
- `apps/uzel/src/*.test.mjs`: Renderer helper unit tests.
- `apps/uzel/tests/ui/acceptance.test.mjs`: Playwright shell/surface acceptance against a mocked native boundary.
- `napplets/*/tests/*.test.mjs`: Napplet model/probe tests.
- `contracts/*.test.mjs`: Cross-napplet and script contract tests.
- `fixtures/*/`: Signed inputs/artifacts used by Rust, conformance, and smoke tests.
- `scripts/smoke.sh`, `scripts/linux-run-smoke.sh`, `scripts/debian13-live-test.sh`: Composed and platform acceptance.

**Documentation and Evidence:**
- `README.md`: Current development and acceptance entry point.
- `DEBIAN13-LIVE-TEST.md`: Debian 13 live acceptance procedure.
- `uzel-poc-validated-pack/STATUS.md`: Active work slice, integrated evidence, limitations, and next action.
- `uzel-poc-validated-pack/docs/`: Normative scope/design/execution/testing/source/contribution docs.
- `uzel-poc-validated-pack/reports/`: Probe and preflight evidence.

## Naming Conventions

**Files:**
- Rust modules use lowercase snake case where multiple words are needed: `apps/uzel/src-tauri/src/hostile_probe.rs`.
- Rust binary/library entries use ecosystem-standard `src/main.rs` and `src/lib.rs`: `apps/uzel-napd/src/main.rs`, `crates/napd/src/lib.rs`.
- Svelte components use PascalCase: `apps/uzel/src/App.svelte`.
- JavaScript implementation files use lowercase kebab case for multiword concerns: `apps/uzel/src/projection-failure.js`, `contracts/profile-open.js`.
- JavaScript tests use `<subject>.test.mjs`: `apps/uzel/src/preferences.test.mjs`, `contracts/profile-open.test.mjs`, `napplets/follow-list/tests/model.test.mjs`.
- Shell scripts use lowercase kebab case and `.sh`: `scripts/check-pinned-assets.sh`, `scripts/linux-run-smoke.sh`.
- JSON schemas include payload name and version: `contracts/profile-open-v1.schema.json`.
- Signed fixture directories match napplet/build identities: `fixtures/follow-list/`, `fixtures/profile-card/`, `fixtures/hostile-egress/`.
- Planning/evidence documents use numbered kebab-case for ordered docs/work: `uzel-poc-validated-pack/docs/03-provisional-design.md`, `uzel-poc-validated-pack/work/07-stabilize-product.md`.

**Directories:**
- Product applications use lowercase kebab case under `apps/`: `apps/uzel-napd/`.
- Rust library crates use lowercase kebab case matching Cargo package names: `crates/napd-protocol/`.
- Napplet package directories use manifest `d`-tag-style lowercase kebab names: `napplets/follow-list/`, `napplets/profile-card/`.
- Keep one owner per top-level concern: `contracts/`, `fixtures/`, `scripts/`, `config/`.

**Symbols and Data Shapes:**
- Rust functions/modules/fields use `snake_case`; structs/enums use `PascalCase`; constants use `SCREAMING_SNAKE_CASE` (`crates/napd/src/runner.rs`).
- JavaScript/TypeScript functions and variables use `camelCase`; type aliases use `PascalCase`; constants use `SCREAMING_SNAKE_CASE` (`apps/uzel/src/App.svelte`, `contracts/kind0-profile.js`).
- Native protocol enum variants serialize with `snake_case`; UI DTO fields serialize with `camelCase` through explicit serde attributes (`crates/napd-protocol/src/lib.rs`, `apps/uzel/src-tauri/src/main.rs`).
- NAP envelope types use dotted lowercase names such as `shell.ready`, `identity.getFollows.result`, and `inc.event` (`crates/napd/src/runner.rs`, `apps/uzel/src/App.svelte`).
- Runtime surface tokens include product, napplet/surface name, and monotonically persisted generation (`crates/napd/src/runner.rs:945`, `crates/napd/src/runner.rs:1080`).

## Where to Add New Code

**New Product Feature:**
- Primary renderer code: `apps/uzel/src/`; keep `apps/uzel/src/App.svelte` as composition root and place cohesive state/validation helpers in specifically named sibling modules.
- Native-only shell adapter: `apps/uzel/src-tauri/src/`; add a focused Rust module beside `hostile_probe.rs` when lifecycle/state is independent.
- Tests: `apps/uzel/src/<subject>.test.mjs` for pure helpers and `apps/uzel/tests/ui/acceptance.test.mjs` for rendered/surface flows.
- Rule: If feature changes runtime truth, start in `crates/napd/` rather than implementing it in renderer or Tauri.

**New Daemon Operation:**
- Wire types/client method: `crates/napd-protocol/src/lib.rs`.
- Request dispatch/transport behavior: `crates/napd/src/server.rs`.
- Runtime-owned behavior: a focused module under `crates/napd/src/`, exported narrowly through `crates/napd/src/lib.rs`; extend `crates/napd/src/runner.rs` only when operation belongs to the runtime composition facade.
- Renderer projection, only if UI-visible: `apps/uzel/src-tauri/src/main.rs` and `apps/uzel/src/App.svelte`.
- Tests: co-locate Rust tests in the affected module and add UI acceptance only for renderer-visible behavior.
- Rule: Update bounds, typed errors, replay semantics, cleanup behavior, and every layer projection together.

**New Portable Napplet:**
- Implementation: `napplets/<napplet-name>/src/main.js` plus focused `src/<concern>.js` modules.
- Package/build: `napplets/<napplet-name>/package.json`, `index.html`, and `vite.config.js`; `pnpm-workspace.yaml` already includes `napplets/*`.
- Tests: `napplets/<napplet-name>/tests/<concern>.test.mjs`.
- Signed fixture: `fixtures/<napplet-name>/event.json` and `fixtures/<napplet-name>/index.html`.
- Runtime pin: `crates/napd/src/fixtures.rs`; update `scripts/build-signed-napplet-fixtures.sh` and `scripts/check-pinned-assets.sh` when applicable.
- Rule: Keep imports runtime-agnostic and pass `scripts/check-napplet-imports.mjs`.

**New Cross-Napplet Payload:**
- Schema owner: `contracts/<topic>-v<version>.schema.json`.
- Strict parser/builder: `contracts/<topic>.js`.
- Tests: `contracts/<topic>.test.mjs`.
- Producer/consumer imports: only the relevant `napplets/*/src/` entries.
- Rule: Use exact field sets, explicit versioning, bounded values, and immutable parsed results following `contracts/profile-open.js`.

**New Runtime Resource/Provider Adapter:**
- Implementation: focused module under `crates/napd/src/`, using `crates/napd/src/resource.rs` as the placement pattern.
- Composition: register with `RuntimeController` in `crates/napd/src/runner.rs:372`.
- Tests: module-local `#[cfg(test)]` tests plus composed daemon tests if routing changes.
- Rule: Preserve deadline, cancellation, byte, redirect, proxy, address, and scheme policy at the adapter boundary.

**New IPC DTO:**
- Native source of truth: `crates/napd-protocol/src/lib.rs`.
- Tauri UI projection: `apps/uzel/src-tauri/src/main.rs`.
- TypeScript mirror: `apps/uzel/src/App.svelte` or a focused type-owning sibling module if extracted.
- Rule: Use explicit serde rename policy and test round-trip/invalid response behavior.

**Utilities:**
- Shared cross-napplet helpers: `contracts/` only when they encode a real shared contract/domain rule.
- Renderer helpers: `apps/uzel/src/<specific-concern>.js`.
- Napplet helpers: `napplets/<name>/src/<specific-concern>.js`.
- Rust helpers: private functions in the owning module or a focused new module under `crates/napd/src/`.
- Rule: Do not create generic `utils`, catch-all managers, or cross-boundary helper packages; keep ownership explicit per `uzel-poc-validated-pack/AGENTS.md`.

**New Script/Operational Check:**
- Repository-wide check: `scripts/check-<concern>.sh` or `.mjs`, wired into `package.json`.
- Platform smoke: extend the named `scripts/linux-run-smoke.sh`, `scripts/fedora-run-smoke.sh`, or `scripts/debian13-live-test.sh` owner.
- Documentation audit behavior: `uzel-poc-validated-pack/scripts/audit_docs.py`.
- Rule: Emit stable explicit success/failure markers and keep cleanup bounded and fail-safe.

## Special Directories

**`apps/uzel/public/trusted-shell/`:**
- Purpose: Reviewed upstream browser trust-boundary assets.
- Generated: No; copied/pinned vendor source.
- Committed: Yes.
- Modification rule: Update as a reviewed set with provenance and hostile/UI acceptance.

**`fixtures/`:**
- Purpose: Signed public test manifests and exact artifact bytes.
- Generated: Partly; napplet HTML/events are produced by `scripts/build-signed-napplet-fixtures.sh`, while documentation and Nostr input fixtures are curated.
- Committed: Yes.
- Modification rule: Use only disposable test keys; update all runtime hashes and asset checks together.

**`graphify-out/`:**
- Purpose: Queryable generated code graph and visual/report artifacts.
- Generated: Yes, by Graphify.
- Committed: Yes, except dated generated subdirectories ignored by `.gitignore`.
- Modification rule: Run the locked `pnpm graphify:refresh` entrypoint after code changes and commit graph refresh separately; never commit `graphify-out/cache/`.

**`uzel-poc-validated-pack/`:**
- Purpose: Validated scope, evidence, source pins, work workflow, and durable status.
- Generated: No; audit outputs/probe reports are tool-produced but the pack is controlled project documentation.
- Committed: Yes.
- Modification rule: Follow `uzel-poc-validated-pack/AGENTS.md`, active `work/*.md`, and contribution ledger requirements.

**`.planning/codebase/`:**
- Purpose: Current codebase maps consumed by GSD planning and execution.
- Generated: Yes, by GSD codebase mapping.
- Committed: Repository policy not detected; preserve orchestrator ownership of git operations.

**`target/`:**
- Purpose: Rust build outputs, including platform-specific smoke targets.
- Generated: Yes.
- Committed: No; ignored by `.gitignore`.

**`node_modules/` and `**/dist/`:**
- Purpose: Installed JavaScript dependencies and Vite/napplet build output.
- Generated: Yes.
- Committed: No; ignored by `.gitignore`.

**`.artifacts/`:**
- Purpose: Local Debian/live/smoke evidence and build outputs.
- Generated: Yes.
- Committed: No; ignored by `.gitignore`.

---

*Structure analysis: 2026-08-09*
