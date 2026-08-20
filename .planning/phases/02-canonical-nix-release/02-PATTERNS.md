# Phase 2: Canonical Nix release - Pattern Map

**Mapped:** 2026-08-14
**Files analyzed:** 5 proposed files
**Analogs found:** 4 / 5

Phase requirements: PKG-01, PKG-02, PKG-03, PKG-04, PKG-05. Lockfiles are evidence inputs, not planned edits.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|---|---|---|---|---|
| `flake.nix` | config/package | batch, transform | existing `flake.nix` dev-shell output | role-match |
| `apps/uzel/src-tauri/src/main.rs` | desktop controller | request-response | `crates/napd/src/server.rs` hello branch | data-flow-match |
| `scripts/package-smoke.sh` | utility/test harness | process, file-I/O | `scripts/linux-run-smoke.sh` | role-and-flow-match |
| `package.json` | config | request-response | existing smoke script entries in `package.json` | exact extension |
| `contracts/package-smoke-script.test.mjs` | test | transform/process | `contracts/linux-smoke-script.test.mjs` | role-and-flow-match |

## Pattern Assignments

### `flake.nix` (config/package, batch transform)

**Analog:** `flake.nix` — current single-system flake and dev shell. No existing package/app/check output exists, so this is a role-match extension, not a copyable package derivation.

**System and output scope** (lines 6-12):

```nix
outputs = { nixpkgs, ... }:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in
  {
    devShells.${system}.default = pkgs.mkShell {
```

Keep this `system` binding. Add `packages.${system}.uzel`, `packages.${system}.default`, `apps.${system}.uzel`, `apps.${system}.default`, and package-dependent `checks` beside—not instead of—the existing `devShells` output.

**Native desktop runtime inputs** (lines 37-45):

```nix
LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
  gtk3
  libayatana-appindicator
  librsvg
  webkitgtk_4_1
]);
LIBGL_ALWAYS_SOFTWARE = "1";
LIBGL_DRIVERS_PATH = "${pkgs.mesa}/lib/dri";
__EGL_VENDOR_LIBRARY_FILENAMES = "${pkgs.mesa}/share/glvnd/egl_vendor.d/50_mesa.json";
```

Treat this list as current desktop-runtime evidence. Package runtime closure must carry needed native libraries; do not copy development-only tools from `packages` lines 13-35 (Weston, Chromium, Deno, testing tools) into release closure.

**Dev-only PATH mutation** (lines 49-54):

```nix
shellHook = ''
  export UZEL_COREPACK_BIN="$PWD/.cache/corepack-bin"
  mkdir -p "$UZEL_COREPACK_BIN"
  corepack enable --install-directory "$UZEL_COREPACK_BIN" >/dev/null
  export PATH="$UZEL_COREPACK_BIN:$PATH"
'';
```

Do not inherit this into packaged launch. PKG-03 prohibits checkout/PATH discovery; package launcher must embed literal closure paths.

**Locked-source evidence to preserve:** `Cargo.toml` lines 20-23 pin nampplets `e2f69f325a6b45213accdacfcc125e80e0687b4c`; `Cargo.lock` line 2508 pins NMP `005dc2a5f12aa414961b313d05ebb021934e385c`; `flake.nix` line 4 pins nixpkgs. Package report/check should read these, not repin them.

---

### `apps/uzel/src-tauri/src/main.rs` (desktop controller, request-response)

**Analog:** `crates/napd/src/server.rs` lines 132-137, backed by `crates/napd-protocol/src/lib.rs` `UnixClient::request` and hello frame tests. Same private IPC request/response compatibility flow; shell must consume it before readiness.

**Existing protocol imports** (`apps/uzel/src-tauri/src/main.rs`, lines 9-12):

```rust
use napd_protocol::{
    ClientError, Diagnostics, FetchedSurface, NappletReview, Request, Response, RoutedEnvelope,
    UnixClient,
};
```

Extend this existing import group with protocol `VERSION` only if needed; retain one `UnixClient`/`Request`/`Response` contract, no new launcher protocol.

**Fail-closed server authority** (`crates/napd/src/server.rs`, lines 132-137):

```rust
let response = match request {
    Request::Hello { version } if version == VERSION => Response::Hello { version },
    Request::Hello { version } => Response::error(
        "version_mismatch",
        format!("daemon protocol is {VERSION}, client requested {version}"),
    ),
```

Shell setup must synchronously send `Request::Hello { version: VERSION }`, accept only matching `Response::Hello`, and return a clear startup error for `Response::Error { code: "version_mismatch", .. }` or any unexpected/error response. Do this before the established readiness marker at `apps/uzel/src-tauri/src/main.rs` lines 533-541.

**Socket ownership** (`apps/uzel/src-tauri/src/main.rs`, lines 567-573):

```rust
fn default_socket_path() -> Result<PathBuf, &'static str> {
    env::var_os("XDG_RUNTIME_DIR")
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .map(|path| path.join("uzel/napd.sock"))
        .ok_or("XDG_RUNTIME_DIR is required for the private daemon socket")
}
```

Reuse same XDG path. Do not pass ambient daemon paths or invent a global service boundary.

**Protocol test idiom** (`crates/napd/src/server.rs`, lines 557-564):

```rust
assert_eq!(
    exchange(&socket, &Request::Hello { version: VERSION }),
    Response::Hello { version: VERSION }
);
```

Closest test extension is an exact hello/mismatch assertion in current Rust test modules; no new test framework.

---

### `scripts/package-smoke.sh` (utility/test harness, process + file-I/O)

**Analog:** `scripts/linux-run-smoke.sh`. Reuse isolation, Weston/WebKit setup, marker reporting, timeouts, process-group cleanup, and evidence preservation. New script must invoke exact store-path launcher, not `pnpm dev`.

**Shell contract and cleanup primitives** (lines 1-25, 48-88):

```bash
#!/usr/bin/env bash
set -euo pipefail

SMOKE_NAME=${UZEL_SMOKE_NAME:-linux}
SUCCESS_MARKER=${UZEL_SMOKE_SUCCESS_MARKER:-LINUX_RUN_SMOKE_OK}
STARTUP_TIMEOUT_SECONDS=${UZEL_SMOKE_STARTUP_TIMEOUT_SECONDS:-600}
RUNTIME_TIMEOUT_SECONDS=${UZEL_SMOKE_RUNTIME_TIMEOUT_SECONDS:-120}
SHUTDOWN_GRACE_SECONDS=${UZEL_SMOKE_SHUTDOWN_GRACE_SECONDS:-5}
...
stop_child() {
  local pid=$1
  local scope=$2
  ...
  kill -TERM -- "$signal_target" 2>/dev/null || true
  ...
  wait "$pid" 2>/dev/null || true
```

Use `set -euo pipefail`, validated env-configurable timeouts, PID bookkeeping, `stop_child`, and EXIT/INT/TERM trap style. Launcher may own daemon child; smoke must own/kill its launcher process group.

**Isolated native runtime** (lines 129-141):

```bash
export XDG_RUNTIME_DIR="$SMOKE_TMP/runtime"
mkdir -m 700 "$XDG_RUNTIME_DIR"
export XDG_DATA_HOME="$SMOKE_TMP/data"
mkdir -m 700 "$XDG_DATA_HOME"
export WAYLAND_DISPLAY=wayland-uzel
export GDK_BACKEND=wayland
export NO_AT_BRIDGE=1
export UZEL_RUN_HOSTILE_PROBE=1
export UZEL_RUN_WEBKIT_RECOVERY_PROBE=1
```

Package smoke must retain these native Weston/WebKit variables and invoke from non-checkout cwd. Add hostile `PATH` decoy as package-specific evidence; never resolve daemon through it.

**Readiness and failure reporting** (lines 152-171, 250-287):

```bash
runtime_markers_ready() {
  rg -q "$NAPD_READY_PATTERN" "$SMOKE_TMP/uzel.log" \
    && rg -q '^UZEL_SHELL_READY$' "$SMOKE_TMP/uzel.log" \
    && ...
}
...
if ! kill -0 "$DEV_PID" 2>/dev/null; then
  echo 'Linux runtime exited before readiness markers' >&2
  report_marker_state
  exit 1
fi
```

Retain anchored readiness markers, marker-state diagnostics, and fail-on-early-exit. Add package closure/content/size, source revision, desktop asset, no-checkout/no-ambient-daemon, and mismatch evidence as explicit checks/markers. Existing dev-only launch is `setsid pnpm dev` at lines 245-246; package smoke must deliberately replace only that command with supplied exact `/nix/store/.../bin/uzel` entrypoint.

---

### `package.json` (config, request-response)

**Analog:** existing root script delegation (`package.json` lines 21-41). Exact extension.

```json
"scripts": {
  "dev": "bash scripts/dev.sh",
  "check:boundaries": "bash scripts/check-boundaries.sh",
  "smoke": "bash scripts/smoke.sh",
  "smoke:linux": "bash scripts/linux-run-smoke.sh",
  "smoke:fedora": "bash scripts/fedora-run-smoke.sh"
}
```

Add one narrow package-smoke script delegating to `bash scripts/package-smoke.sh`. Do not inline Nix commands, supply ambient package paths, or duplicate lifecycle logic in JSON. Normal commands remain `nix ... develop --command pnpm <script>` per `WORKFLOW.md` lines 28-40.

---

### `contracts/package-smoke-script.test.mjs` (test, transform/process)

**Analog:** `contracts/linux-smoke-script.test.mjs` — static script-contract checks with Node built-ins and `rg` subprocess.

**Imports and temporary fixture contract** (lines 1-31):

```javascript
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import {
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
...
return spawnSync('rg', ['-q', napdReadyPattern(), log]).status === 0;
```

**Test declaration pattern** (lines 35-46):

```javascript
test('napd readiness survives shared-log output before the marker', () => {
  assert.equal(
    matchesLog(
      '\\u001b[1mconcurrent cargo outputUZEL_NAPD_READY role=runtime-authority\\n',
    ),
    true,
  );
});
```

If new package script has static wiring/anti-regression guarantees not covered by this test, copy this Node test shape. Assert literal package entrypoint, non-checkout setup, hostile PATH guard, `UZEL_SHELL_READY`/mismatch behavior, cleanup traps, and no `pnpm dev` in package script. Avoid mock WebKit: real native evidence stays in the package smoke.

## Shared Patterns

### Locked toolchain and script entrypoint

**Source:** `WORKFLOW.md` lines 28-40; `package.json` lines 21-41.
**Apply to:** Nix validation wiring and package smoke scripts.

Normal repository command shape is:

```sh
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm <script>
```

Do not add a toolchain, package manager, or lock. Release proof is exception only in that it deliberately invokes exact store-path artifact from non-checkout cwd.

### Private daemon lifecycle

**Source:** `scripts/dev.sh` lines 4-25.
**Apply to:** package launcher.

```bash
UZEL_NAPD_PID=

cleanup() {
  if [[ -n "$UZEL_NAPD_PID" ]]; then
    kill "$UZEL_NAPD_PID" 2>/dev/null || true
    wait "$UZEL_NAPD_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM
...
cargo run -p uzel-napd -- \
  --live \
  "${NAPD_RELAY_ARGS[@]}" &
UZEL_NAPD_PID=$!
```

Copy one-invocation ownership and reap semantics, replacing development `cargo run` with absolute closure-private daemon path. Preserve relay arguments/runtime config; no service manager.

### Protocol compatibility

**Source:** `crates/napd/src/server.rs` lines 132-137.
**Apply to:** shell startup and package mismatch probe.

```rust
Request::Hello { version } if version == VERSION => Response::Hello { version },
Request::Hello { version } => Response::error(
    "version_mismatch",
    format!("daemon protocol is {VERSION}, client requested {version}"),
),
```

One established versioned hello is fail-closed authority. Do not create parallel launcher-version logic.

### Native evidence and cleanup

**Source:** `scripts/linux-run-smoke.sh` lines 92-131, 133-141, 220-290.
**Apply to:** `scripts/package-smoke.sh`.

Existing script owns Weston, XDG isolation, process cleanup, readiness timeout, failure log preservation, native hostile/recovery markers. Reuse this mechanism; change startup target only to packaged launcher.

## No Analog Found

| File | Role | Data Flow | Reason |
|---|---|---|---|
| `flake.nix` package/app/check derivation body | config/package | batch, transform | Current flake has only a dev shell; use research's standard Nix primitives after validating current pinned nixpkgs attributes. |

## Metadata

**Analog search scope:** `flake.nix`, root `package.json`, `scripts/`, `contracts/`, shell/daemon/protocol Rust sources, Cargo locks.
**Files scanned:** 11 direct analog/evidence files.
**Pattern extraction date:** 2026-08-14
