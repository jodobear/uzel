# Testing Patterns

**Analysis Date:** 2026-08-09

## Test Framework

**Runner:**
- Node.js built-in `node:test`, using the Node `>=22.12 <23` runtime declared in `package.json`. Unit tests live in `apps/uzel/src/*.test.mjs`, `contracts/*.test.mjs`, and `napplets/*/tests/*.test.mjs`.
- Rust built-in test harness from Rust 1.89.0 pinned in `rust-toolchain.toml`. Tests are inline `#[cfg(test)] mod tests` modules in `crates/**/*.rs` and `apps/**/*.rs`.
- Playwright library 1.62.0 driven by `node:test`, not Playwright Test. Configuration/constants live in `apps/uzel/tests/ui/playwright.config.mjs`; the suite lives in `apps/uzel/tests/ui/acceptance.test.mjs`.
- Config: root orchestration in `package.json`; UI constants in `apps/uzel/tests/ui/playwright.config.mjs`; strict frontend checking in `apps/uzel/tsconfig.json`; no Jest or Vitest configuration detected.

**Assertion Library:**
- JavaScript uses `node:assert/strict`, with `assert.equal`, `assert.deepEqual`, `assert.match`, `assert.throws`, `assert.rejects`, and `assert.doesNotReject`, as in `contracts/kind0-profile.test.mjs` and `napplets/hostile-egress/tests/probes.test.mjs`.
- Rust uses standard `assert!`, `assert_eq!`, `assert_ne!`, `matches!`, and explicit `panic!` branches, as in `crates/napd/src/server.rs` and `crates/napd-protocol/src/lib.rs`.
- Browser interaction uses Playwright locators and their waiting APIs, then validates returned state with `node:assert/strict` in `apps/uzel/tests/ui/acceptance.test.mjs`.

**Run Commands:**
```bash
pnpm test                 # Node unit/contract tests, napplet tests, Rust tests, pinned-asset audit
pnpm test:ui              # Deterministic Chromium renderer acceptance
pnpm test:conformance     # Built follow-list/profile-card Napplet conformance
pnpm check                # Builds, strict Svelte check, shell build, cargo check
pnpm lint                 # Clippy -D warnings plus architecture/boundary scanner
pnpm fallow               # Unused/unlisted/unresolved dependency and file audit
pnpm smoke                # Focused live Rust probe selected by scripts/smoke.sh
pnpm smoke:linux          # Real Tauri/WebKit/daemon Linux acceptance
pnpm smoke:fedora         # Fedora wrapper around real Linux smoke
pnpm docs:check           # Documentation/link/Mermaid audit
```

No watch-mode script is configured in `package.json` or `apps/uzel/package.json`. No coverage command is configured.

## Test File Organization

**Location:**
- Co-locate shell pure-module tests beside source: `apps/uzel/src/preferences.js` with `apps/uzel/src/preferences.test.mjs`, and `apps/uzel/src/projection-failure.js` with `apps/uzel/src/projection-failure.test.mjs`.
- Co-locate shared-contract tests at the contract root: `contracts/kind0-profile.js` with `contracts/kind0-profile.test.mjs`, and `contracts/profile-open.js` with `contracts/profile-open.test.mjs`.
- Put each napplet's model/probe tests in its own `tests/` directory: `napplets/follow-list/tests/model.test.mjs`, `napplets/profile-card/tests/model.test.mjs`, and `napplets/hostile-egress/tests/probes.test.mjs`.
- Put renderer acceptance and its native-boundary fixture under `apps/uzel/tests/ui/`: `acceptance.test.mjs`, `fixtures/index.html`, and `fixtures/mock-native.js`.
- Put Rust unit/integration-style tests in the owning source file under `#[cfg(test)]`: `crates/napd/src/runner.rs`, `crates/napd/src/server.rs`, `crates/napd/src/resource.rs`, `crates/napd-protocol/src/lib.rs`, and `apps/uzel/src-tauri/src/main.rs`.
- Keep signed/exact-build test data under `fixtures/`: `fixtures/follow-list/`, `fixtures/profile-card/`, `fixtures/good-morning/`, `fixtures/hostile-egress/`, and `fixtures/nostr/live-events.jsonl`.
- Keep executable system gates under `scripts/`, including `scripts/check-pinned-assets.sh`, `scripts/linux-run-smoke.sh`, and `scripts/check-napplet-imports.mjs`.

**Naming:**
- Use `*.test.mjs` for Node tests and import the `.js` implementation explicitly: `napplets/profile-card/tests/model.test.mjs` imports `../src/model.js`.
- Use behavior-focused Rust test names in snake_case inside an owning `tests` module: `oversized_routed_envelope_is_chunked_without_raising_the_frame_limit` in `crates/napd/src/server.rs`.
- Prefix renderer cases with stable scenario identifiers from `SCENARIOS` in `apps/uzel/tests/ui/playwright.config.mjs`, such as `initialization-identity-failure` and `restart-reconciliation`.

**Structure:**
```text
apps/uzel/src/
├── preferences.js
├── preferences.test.mjs
├── projection-failure.js
└── projection-failure.test.mjs

contracts/
├── kind0-profile.js
├── kind0-profile.test.mjs
├── profile-open.js
└── profile-open.test.mjs

napplets/<name>/
├── src/
└── tests/*.test.mjs

apps/uzel/tests/ui/
├── acceptance.test.mjs
├── playwright.config.mjs
└── fixtures/
    ├── index.html
    └── mock-native.js

crates/<crate>/src/*.rs
└── #[cfg(test)] mod tests { ... }
```

## Test Structure

**Suite Organization:**
```javascript
// Pattern from contracts/profile-open.test.mjs
import assert from 'node:assert/strict';
import test from 'node:test';

import { parseProfileOpen } from './profile-open.js';

test('malformed profile-open payloads fail closed', () => {
  for (const malformed of [null, [], {}, { version: 2 }]) {
    assert.equal(parseProfileOpen(malformed), null);
  }
});
```

```rust
// Pattern from crates/napd/src/resource.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinned_transport_refuses_an_empty_address_set() {
        // Arrange exact request, call owner, assert typed refusal.
        assert!(matches!(result, Err(ResourceNetworkError::Failed)));
    }
}
```

**Patterns:**
- Keep JavaScript tests flat and behavior-specific. Use module-level constants/helpers for compact fixtures, such as `row(...)` in `contracts/kind0-profile.test.mjs` and `result(...)` in `napplets/profile-card/tests/model.test.mjs`.
- Test bounds at their exact edge and one past it: `contracts/kind0-profile.test.mjs`, `napplets/follow-list/tests/model.test.mjs`, and `crates/napd-protocol/src/lib.rs` assert fixed batch, byte, frame, retry, and queue ceilings.
- Test malformed/untrusted inputs as tables and require fail-closed results: `contracts/profile-open.test.mjs` and `contracts/kind0-profile.test.mjs`.
- Test stateful concurrency by retaining explicit release functions rather than timing alone: `napplets/follow-list/tests/model.test.mjs` proves maximum active queue width and queued cancellation.
- Test native IPC with real local primitives where practical. `crates/napd/src/server.rs` binds a real AF_UNIX socket in `tempfile::TempDir`, spawns the daemon thread, exchanges framed requests, shuts down, and joins the thread.
- Keep live-network Rust probes explicit and ignored by default using `#[ignore = "requires ..."]` in `crates/napd/src/runner.rs`; deterministic `pnpm test` remains network-independent.
- Renderer setup/teardown is suite-wide via `before` and `after` from `node:test` in `apps/uzel/tests/ui/acceptance.test.mjs`. Teardown asserts Vite, browser, browser child tree, and deliberate-fault process groups are gone.
- Renderer cases use nested `t.test` subtests over fixed `VIEWPORTS` from `apps/uzel/tests/ui/playwright.config.mjs`, and use accessible roles/names rather than CSS selectors for user controls.
- Preserve UI failure evidence under ignored `.artifacts/ui-acceptance/`, as documented in `apps/uzel/tests/ui/README.md`; do not commit screenshots, traces, DOM dumps, or metadata.

## Mocking

**Framework:** Custom fakes; no general mocking library detected.

**Patterns:**
```javascript
// Simplified pattern from apps/uzel/tests/ui/fixtures/mock-native.js
async function invoke(command, args = {}) {
  calls.push({ command, args: structuredClone(args) });
  switch (command) {
    case 'runtime_diagnostics':
      return diagnostics();
    case 'start_fixture':
      return surfaceLaunch(args.fixture);
    default:
      throw new Error(`mock native boundary has no ${command} command`);
  }
}

global.__TAURI_INTERNALS__ = { invoke };
```

```javascript
// Injectable browser seam from napplets/hostile-egress/src/probes.js
export function workerLoad(url, WorkerConstructor = globalThis.Worker, timeoutMs = 3_000) {
  const worker = new WorkerConstructor(url);
  // Test supplies deterministic LoadedWorker/RejectedWorker classes.
}
```

**What to Mock:**
- Mock only the Tauri/native/NMP reply boundary for deterministic renderer tests through `apps/uzel/tests/ui/fixtures/mock-native.js`. Record exact calls/envelopes and reject unknown commands or routes.
- Inject narrow platform seams directly as parameters, such as `WorkerConstructor` in `napplets/hostile-egress/src/probes.js` and limits in `napplets/follow-list/src/model.js`.
- Use deterministic error counters/scenarios in `apps/uzel/tests/ui/fixtures/mock-native.js` to exercise ambiguous review, cleanup failure, delayed response, projection overflow, and retry behavior.
- For Rust filesystem/socket tests, use `tempfile::TempDir` in `crates/napd/src/runner.rs` and `crates/napd/src/server.rs`; use real files and Unix sockets rather than mocking standard library I/O.

**What NOT to Mock:**
- Do not replace the trusted surface host or signed napplet HTML in renderer acceptance. `apps/uzel/tests/ui/acceptance.test.mjs` loads checked-in bytes from `fixtures/*/index.html`, verifies their signed path hashes, and drives the real `NMPTrustedShellHost` through the instrumented wrapper in `apps/uzel/tests/ui/fixtures/mock-native.js`.
- Do not claim native/daemon/NMP/WebKit coverage from the mocked UI harness. `apps/uzel/tests/ui/README.md` assigns those claims to Rust tests and `scripts/linux-run-smoke.sh`.
- Do not use placeholder-success tests or fake production implementations, per `uzel-poc-validated-pack/AGENTS.md`.
- Do not bypass Napplet conformance with mocks. Build artifacts and run `pnpm test:conformance` from `package.json` against `napplets/follow-list/dist` and `napplets/profile-card/dist`.

## Fixtures and Factories

**Test Data:**
```javascript
// Pattern from contracts/kind0-profile.test.mjs
const A = 'a'.repeat(64);

function row(author, content = '{}', createdAt = 30, id = `event-${author[0]}`) {
  return {
    event: { id, pubkey: author, kind: 0, created_at: createdAt, tags: [], sig: '', content },
  };
}
```

```rust
// Pattern from crates/napd/src/server.rs
const FIXTURE_INDEX: &[u8] =
    include_bytes!("../../../fixtures/good-morning/index.html");

let temp = TempDir::new().unwrap();
let socket = temp.path().join("run/uzel.sock");
```

**Location:**
- Keep small pure-data factories inside the test file that uses them: `row(...)` in `contracts/kind0-profile.test.mjs` and `response_event(...)` in `crates/napd/src/runner.rs`.
- Keep portable signed artifacts and Nostr records under `fixtures/`; document fixture meaning in `fixtures/README.md` and fixture-local READMEs such as `fixtures/good-morning/README.md`.
- Keep renderer-only fake-native state under `apps/uzel/tests/ui/fixtures/mock-native.js`, separate from production code.
- Verify fixture bytes and pins through `scripts/check-pinned-assets.sh`, which runs as the final part of root `pnpm test` in `package.json`.
- Regenerate signed fixture bytes only through the project process in `scripts/build-signed-napplet-fixtures.sh`; update all coupled event/hash/fixture references together rather than hand-editing one file.

## Coverage

**Requirements:** None enforced by a line/branch/function percentage. No `c8`, `nyc`, Istanbul, `cargo-llvm-cov`, or Tarpaulin configuration/script is present in `package.json`, `apps/uzel/package.json`, or `Cargo.toml`.

Coverage is contract/layer based instead:
- `package.json` makes deterministic unit/contract/Rust/pinned-asset tests one root command.
- `apps/uzel/tests/ui/README.md` defines renderer claims and exclusions.
- `uzel-poc-validated-pack/docs/05-test-and-demo.md` defines runtime/Rust, napplet/web, hostile-frame, deterministic-demo, and final-acceptance layers.
- `uzel-poc-validated-pack/AGENTS.md` requires Rust formatting/Clippy/tests, frontend type/test/accessibility checks, Fallow, conformance, hostile-frame/network tests, and documentation audit.

**View Coverage:**
```bash
# Not configured. No repository coverage report command exists.
```

Use explicit behavior gaps to add tests in the owning layer instead of chasing a percentage. Add pure projection tests beside `contracts/*.js` or `src/model.js`; add IPC/runtime behavior inside the owning Rust module; add renderer state transitions to `apps/uzel/tests/ui/acceptance.test.mjs`; add real WebKit/native claims to `scripts/linux-run-smoke.sh` and its supporting Rust tests.

## Test Types

**Unit Tests:**
- JavaScript pure-model tests cover validation, projection, immutable contracts, queue bounds, cancellation, request-generation gates, and probe helpers in `apps/uzel/src/*.test.mjs`, `contracts/*.test.mjs`, and `napplets/*/tests/*.test.mjs`.
- Rust module tests cover protocol serialization/bounds, runner state, resource cancellation/pinning, server socket safety, Tauri command parsing/reconciliation, and daemon CLI parsing in `crates/**/*.rs` and `apps/**/*.rs`.
- Tool self-tests run inside `scripts/check-napplet-imports.mjs` before repository scanning; every forbidden/allowed AST pattern must classify correctly.

**Integration Tests:**
- `crates/napd/src/server.rs` runs real daemon/client exchanges over AF_UNIX sockets, including asset transfer, INC routing, replay behavior, timeouts, and shutdown.
- `crates/napd-protocol/src/lib.rs` runs framed client/server exchanges on temporary Unix sockets and verifies lost-response replay and cleanup.
- Root `pnpm test` in `package.json` composes Node, napplet, Rust, Tauri feature-split, and pinned-asset checks.
- `pnpm test:conformance` builds and validates portable artifacts using commands declared in `napplets/follow-list/package.json` and `napplets/profile-card/package.json`.

**E2E Tests:**
- Deterministic renderer E2E uses Chromium/Playwright through `apps/uzel/tests/ui/acceptance.test.mjs`; native/NMP responses are mocked, while Svelte, checked-in napplet bytes, iframes, trusted host, routing, accessible controls, and process cleanup are real.
- Real Linux E2E uses `scripts/linux-run-smoke.sh`, invoked by `pnpm smoke:linux` in `package.json`, for daemon, Tauri, Weston/WebKit, exact-build, source-binding, hostile-egress, and user-mode evidence.
- Fedora and Debian wrappers live in `scripts/fedora-run-smoke.sh`, `scripts/debian13-live-test.sh`, and `scripts/debian-build-smoke.sh`.
- Napplet protocol E2E uses released conformance tooling through `pnpm test:conformance`; the hostile fixture is separate under `napplets/hostile-egress/` and `fixtures/hostile-egress/`.

## Common Patterns

**Async Testing:**
```javascript
// Pattern from napplets/follow-list/tests/model.test.mjs
test('profile queries never exceed two active batches', async () => {
  const queue = createBoundedTaskQueue(MAXIMUM_PROFILE_REQUESTS);
  const releases = [];
  const jobs = Array.from({ length: 5 }, () => queue.run(() =>
    new Promise((resolve) => releases.push(resolve))));

  await Promise.resolve();
  assert.equal(releases.length, MAXIMUM_PROFILE_REQUESTS);
  while (releases.length > 0) releases.shift()();
  await Promise.all(jobs);
});
```

- Prefer explicit completion controls and bounded polling over arbitrary sleeps. Queue tests use retained resolvers in `napplets/follow-list/tests/model.test.mjs`; UI helpers use bounded attempts in `apps/uzel/tests/ui/acceptance.test.mjs`.
- Guard stale async results with request generations and assert the guard directly in `napplets/profile-card/tests/model.test.mjs`.
- In renderer tests, wait on accessible outcome text/roles or a specific harness envelope; use fixed timeouts only for externally scheduled behavior in `apps/uzel/tests/ui/acceptance.test.mjs`.

**Error Testing:**
```javascript
// Pattern from napplets/follow-list/tests/model.test.mjs
assert.throws(() => createProfileRetryBudget(-1), RangeError);
await assert.rejects(pending, reason);
```

```rust
// Pattern from crates/napd/src/server.rs
assert!(matches!(
    DaemonServer::bind(&socket, second_runner),
    Err(ServerError::ActiveSocket(_))
));
```

- Assert exact typed error variants/codes, not only a generic failure, in Rust tests under `crates/napd/src/server.rs` and `crates/napd-protocol/src/lib.rs`.
- Assert state after failure and after retry. `apps/uzel/tests/ui/acceptance.test.mjs` checks that failed identity activation starts zero surfaces, controls remain locked, then the next retry starts exactly two surfaces.
- Assert cleanup and leak freedom even for deliberate failure. `apps/uzel/tests/ui/acceptance.test.mjs` launches the `fault-proof` case in an isolated child, requires nonzero exit, and proves the process group and Chromium descendants exit.
- Keep environmental/live failures out of deterministic unit runs by using explicit `#[ignore]` tests in `crates/napd/src/runner.rs` and separate smoke commands in `package.json`.

---

*Testing analysis: 2026-08-09*
