# Coding Conventions

**Analysis Date:** 2026-08-09

## Naming Patterns

**Files:**
- Use kebab-case for multiword JavaScript and shell files: `apps/uzel/src/projection-failure.js`, `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js`, and `scripts/check-pinned-assets.sh`.
- Name JavaScript unit tests `<module>.test.mjs`: `apps/uzel/src/preferences.test.mjs`, `contracts/kind0-profile.test.mjs`, and `napplets/follow-list/tests/model.test.mjs`.
- Use snake_case for Rust modules: `crates/napd/src/resource.rs` and `apps/uzel/src-tauri/src/hostile_probe.rs`.
- Use PascalCase for Svelte components: `apps/uzel/src/App.svelte`. Keep the Vite entry lowercase as `apps/uzel/src/main.ts`.
- Give domain owners specific names. Put cross-napplet schema/projection code in `contracts/kind0-profile.js` or `contracts/profile-open.js`; do not create a generic `utils` module, per `uzel-poc-validated-pack/AGENTS.md`.

**Functions:**
- Use camelCase for JavaScript/TypeScript functions: `profileQueryBatches` in `contracts/kind0-profile.js` and `restartActiveSurfaces` in `apps/uzel/src/App.svelte`.
- Use `create...` for factories that close over bounded mutable state: `createBoundedTaskQueue` and `createProfileRetryBudget` in `napplets/follow-list/src/model.js`.
- Use predicate names beginning with `is`, `has`, or `validate` when returning booleans: `isCanonicalPubkey` in `contracts/profile-open.js` and `validateKeybindings` in `apps/uzel/src/preferences.js`.
- Use snake_case for Rust functions and methods: `start_named_fixture` in `crates/napd/src/runner.rs` and `read_frame` in `crates/napd-protocol/src/lib.rs`.
- Name tests as behavior statements in snake_case for Rust (`daemon_serves_ordered_verified_asset_and_shuts_down` in `crates/napd/src/server.rs`) and sentence fragments for Node (`'malformed profile-open payloads fail closed'` in `contracts/profile-open.test.mjs`).

**Variables:**
- Use camelCase for JavaScript/TypeScript locals and state: `requestGeneration` in `napplets/profile-card/src/main.js` and `cleanupRequired` in `apps/uzel/src/App.svelte`.
- Use snake_case for Rust locals and fields: `surface_token` in `crates/napd-protocol/src/lib.rs` and `active_read_identity` in `crates/napd/src/runner.rs`.
- Use `SCREAMING_SNAKE_CASE` for module constants in every language: `MAXIMUM_ENVELOPE_BYTES` in `crates/napd-protocol/src/lib.rs`, `MAXIMUM_AVATAR_REQUESTS` in `napplets/follow-list/src/model.js`, and `TEST_NADDR` in `apps/uzel/tests/ui/playwright.config.mjs`.
- Name bounded values explicitly with `MAXIMUM_...`, `..._LIMIT`, or `..._TIMEOUT`: `MAXIMUM_PENDING_OPERATIONS` in `crates/napd-protocol/src/lib.rs` and `RESPONSE_TIMEOUT` in `crates/napd/src/runner.rs`.

**Types:**
- Use PascalCase for TypeScript aliases and Rust structs/enums: `SurfaceLaunch` in `apps/uzel/src/App.svelte`, `ProtocolError` in `crates/napd-protocol/src/lib.rs`, and `RuntimeMode` in `crates/napd/src/runner.rs`.
- Use PascalCase Rust enum variants and let Serde own wire renaming: `Request::StartFixture` serializes through `#[serde(tag = "operation", rename_all = "snake_case")]` in `crates/napd-protocol/src/lib.rs`.
- Keep protocol-facing field casing explicit. Rust structs use snake_case fields plus `#[serde(rename_all = "camelCase")]` in `crates/napd-protocol/src/lib.rs`; TypeScript mirrors the resulting camelCase contract in `apps/uzel/src/App.svelte`.

## Code Style

**Formatting:**
- Format Rust with the Rust 1.89 `rustfmt` component pinned in `rust-toolchain.toml`. Run `pnpm format:check`, which delegates to `cargo fmt --all -- --check` in `package.json`.
- Follow the checked-in JavaScript/TypeScript/Svelte style in `apps/uzel/vite.config.ts` and `apps/uzel/src/preferences.js`: two-space indentation, single-quoted strings, semicolons, trailing commas in multiline literals/calls, and parentheses around multiline expressions.
- No Prettier, Biome, or JavaScript formatter configuration exists in the tracked root. Preserve local formatting when editing `apps/uzel/src/App.svelte`, `contracts/*.js`, and `napplets/*/src/*.js`; `package.json` only enforces Rust formatting.
- Use numeric separators for size/count constants: `64 * 1_024` in `crates/napd-protocol/src/lib.rs`, `4_096` in `apps/uzel/src/preferences.js`, and `91_001` in `napplets/hostile-egress/src/probes.js`.
- Keep reusable Rust crates free of unsafe code where declared: `crates/napd-protocol/src/lib.rs` uses `#![forbid(unsafe_code)]`.

**Linting:**
- Run `pnpm lint` from `package.json`. It runs `cargo clippy --workspace --all-targets -- -D warnings` and then `pnpm check:boundaries`.
- Treat every Clippy warning as an error. Keep all Rust targets clean under the workspace configuration in `Cargo.toml`.
- Run `pnpm fallow` with `config/fallow.jsonc`. It rejects unresolved imports, unlisted dependencies, unused files, and unused dependencies while excluding generated/preserved paths such as `graphify-out/**` and `uzel-poc-validated-pack/**`.
- Keep reusable runtime crates independent of UI/platform code. `scripts/check-boundaries.sh` rejects `tauri` or `svelte` references in `crates/**/*.rs` and `crates/**/Cargo.toml`.
- Keep product napplets on the allowlisted dependency set and without direct browser network authority. `scripts/check-napplet-imports.mjs` parses JS/TS/Svelte/HTML, rejects Uzel/napd/Tauri imports, dynamic imports, network-capable DOM sinks, dynamic code execution, and unapproved dependencies.
- Access remote content in product napplets through NAP SDKs, as in `napplets/follow-list/src/main.js` (`outboxQuery`, `resourceBytes`) and `napplets/profile-card/src/main.js`; never add `fetch`, `WebSocket`, remote `src`, or comparable browser authority that `scripts/check-napplet-imports.mjs` forbids.
- Run `pnpm check` from `package.json` for napplet builds, strict Svelte checking through `apps/uzel/tsconfig.json`, the shell build, and `cargo check --workspace`.

## Import Organization

**Order:**
1. Put Node built-ins first in test/tool files, grouped together: `node:assert/strict`, `node:fs`, and `node:path` lead `apps/uzel/tests/ui/acceptance.test.mjs` and `scripts/check-napplet-imports.mjs`.
2. Add a blank line, then external packages: `playwright` and `vite` in `apps/uzel/tests/ui/acceptance.test.mjs`; `@tauri-apps/api/core` and `svelte` in `apps/uzel/src/App.svelte`.
3. Add a blank line, then repository-relative imports. Import shared contracts before local module helpers, as in `napplets/follow-list/src/main.js`.
4. For napplet entry points, keep the side-effect shim first, then NAP SDK domains, then shared contracts, then local modules: `napplets/profile-card/src/main.js` and `napplets/follow-list/src/main.js` are canonical examples.
5. In Rust, let rustfmt group `std`, external crates, and `crate::...` imports: `crates/napd/src/runner.rs` and `crates/napd/src/resource.rs` show the pattern.

**Path Aliases:**
- Not detected. `apps/uzel/tsconfig.json` defines no `baseUrl` or `paths`; use explicit relative paths such as `../../../contracts/profile-open.js` from `napplets/follow-list/src/main.js`.
- Do not use barrel aliases to blur ownership. `crates/napd/src/lib.rs` re-exports only the crate's public boundary; JavaScript modules export directly from their owner files such as `contracts/kind0-profile.js`.

## Error Handling

**Patterns:**
- Model recoverable Rust failures as typed `thiserror` enums and propagate them with `Result` and `?`: `ProtocolError` and `ClientError` in `crates/napd-protocol/src/lib.rs`, `RunnerError` in `crates/napd/src/runner.rs`, and `ServerError` in `crates/napd/src/server.rs`.
- Convert low-level errors at ownership boundaries with `#[from]`, `map_err`, or explicit variants. `crates/napd/src/resource.rs` maps `reqwest::Error` into the bounded `ResourceNetworkError` contract instead of exposing transport internals.
- Return structured daemon refusal codes through `Response::Error { code, detail }` and `Response::error(...)` in `crates/napd-protocol/src/lib.rs`; callers match the exact error code in `crates/napd/src/server.rs` tests.
- Validate untrusted JavaScript input before use and fail closed with `null`, `[]`, or a default: `parseProfileOpen` in `contracts/profile-open.js`, `canonicalProfile` in `contracts/kind0-profile.js`, and `parsePreferences` in `apps/uzel/src/preferences.js`.
- Throw `TypeError` or `RangeError` for caller/programmer contract violations: `profileOpen` in `contracts/profile-open.js` and queue/budget factories in `napplets/follow-list/src/model.js`.
- Catch user/runtime failures at the UI owner and expose compact state without silently claiming success. `apps/uzel/src/App.svelte` retains cleanup/retry state and uses `finally` blocks to release busy flags; `napplets/profile-card/src/main.js` writes degraded status text while guarding against stale request generations.
- Use `void promise.catch(...)` only for intentional fire-and-forget work with an explicit response path. `napplets/follow-list/src/main.js` cancels stale queued avatar jobs; `apps/uzel/src/App.svelte` reports failed shell acknowledgements or uses best-effort teardown during component destruction.
- Preserve exact cancellation identity and bounds. `napplets/follow-list/src/model.js` links `AbortSignal`s and rejects queued jobs with explicit reasons; `crates/napd/src/runner.rs` bounds buffers, reviews, and response waits.

## Logging

**Framework:** No logging framework detected.

**Patterns:**
- Use stable, machine-readable stdout/stderr markers for executable smoke evidence: `UZEL_NAPD_READY` in `apps/uzel-napd/src/main.rs` and `UZEL_NAP_SHELL_OK`, `UZEL_HOSTILE_PROBE_FAILED`, and related markers in `apps/uzel/src-tauri/src/main.rs`.
- Use `println!` for positive gate markers and `eprintln!` for failures in `apps/uzel/src-tauri/src/main.rs`; keep marker names stable because `scripts/linux-run-smoke.sh` and `contracts/linux-smoke-script.test.mjs` inspect them.
- Report UI-facing failures through bounded state and status text in `apps/uzel/src/App.svelte`, not `console.log`.
- Keep the developer envelope log bounded to `MAX_LOG_ENTRIES` in `apps/uzel/src/App.svelte`; log only direction, surface, and envelope type rather than payload content.
- Reserve `console.error` for tool diagnostics and deliberate harness faults in `scripts/check-napplet-imports.mjs` and `apps/uzel/tests/ui/fixtures/mock-native.js`.

## Comments

**When to Comment:**
- Comment ownership, trust, ordering, or non-obvious boundedness decisions rather than restating code. Examples include response de-duplication in `crates/napd/src/runner.rs`, NAP-SHELL ordering in `apps/uzel/public/trusted-shell/trusted-shell.js`, and unload/beacon ordering in `apps/uzel/src/App.svelte`.
- Explain why a best-effort catch is intentionally empty, as in `napplets/hostile-egress/src/probes.js` and `apps/uzel/public/trusted-shell/trusted-shell.js`.
- Keep protocol and security rationale adjacent to the constant or branch it constrains: frame-size comments in `crates/napd-protocol/src/lib.rs` are the model.

**JSDoc/TSDoc:**
- Use JSDoc primarily to make checked JavaScript types explicit where inference is insufficient. `apps/uzel/src/preferences.js` annotates unknown inputs, event shapes, and `ReadonlySet<string>` while `apps/uzel/tsconfig.json` enables `allowJs` and `checkJs`.
- Use Rust doc comments for public invariants and wire bounds: `crates/napd-protocol/src/lib.rs` documents frame and envelope constants. Private implementation details use ordinary `//` comments in `crates/napd/src/runner.rs`.
- TypeScript in `apps/uzel/src/App.svelte` uses native type aliases rather than JSDoc.

## Function Design

**Size:** Keep functions cohesive, control flow shallow, and owners small per `uzel-poc-validated-pack/AGENTS.md`. Extract pure validation/projection into `contracts/*.js` or a napplet's `src/model.js`; keep DOM orchestration in `src/main.js` and native orchestration in the owning Rust module.

**Parameters:**
- Pass explicit domain values and bounded options rather than catch-all configuration objects: `profileQueryRequest(pubkey)` in `contracts/kind0-profile.js` and `LinuxRunner::open_live(...)` in `crates/napd/src/runner.rs`.
- Accept `unknown` at trust boundaries and narrow immediately in `apps/uzel/src/App.svelte`, `apps/uzel/src/projection-failure.js`, and `contracts/profile-open.js`.
- Use defaults only for safe bounds or injectable test seams: queue limits in `napplets/follow-list/src/model.js` and `WorkerConstructor` in `napplets/hostile-egress/src/probes.js`.

**Return Values:**
- Return immutable/shared contract data where callers must not mutate it: `Object.freeze` in `contracts/profile-open.js`, `apps/uzel/src/projection-failure.js`, and factory return values in `napplets/follow-list/src/model.js`.
- Use `null` for one absent/invalid projected value, `[]` for no collection work, and typed `Result` for Rust operations that can fail: `contracts/kind0-profile.js` and `crates/napd-protocol/src/lib.rs`.
- Preserve exact data that acts as evidence. `canonicalProfile` in `contracts/kind0-profile.js` keeps the original `event.content` in `contentText` instead of normalizing it.

## Module Design

**Exports:** Use named exports for JavaScript contracts/models (`contracts/profile-open.js`, `napplets/follow-list/src/model.js`) and keep UI entry points side-effect-only (`napplets/profile-card/src/main.js`). In Rust, keep modules private and re-export the intended crate surface from `crates/napd/src/lib.rs`.

**Barrel Files:** `crates/napd/src/lib.rs` is the only deliberate public facade pattern. No JavaScript barrel index is used; import from exact owner files such as `contracts/kind0-profile.js`.

**Ownership constraints:**
- Put shell/presentation state in `apps/uzel/src/App.svelte`, Tauri IPC adaptation in `apps/uzel/src-tauri/src/main.rs`, daemon runtime behavior in `crates/napd/src/*.rs`, and wire contracts in `crates/napd-protocol/src/lib.rs`, following `uzel-poc-validated-pack/AGENTS.md`.
- Put portable, untrusted napplets under `napplets/*`; they must not import `apps/uzel`, `crates/napd`, or Tauri, enforced by `scripts/check-napplet-imports.mjs`.
- Put cross-napplet payload/schema ownership under `contracts/`, as shown by `contracts/profile-open.js` and `contracts/profile-open-v1.schema.json`.

---

*Convention analysis: 2026-08-09*
