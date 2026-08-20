# Phase 2: Canonical Nix release - Research

**Researched:** 2026-08-14
**Domain:** locked Nix packaging for paired Tauri shell and private Rust daemon
**Confidence:** MEDIUM

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Release shape
- Expose `packages.x86_64-linux.uzel` and make it the default package.
- Expose `apps.x86_64-linux.uzel` and make it the default app.
- Keep one closure with a stable `bin/uzel` launcher and package-private shell/daemon
  executables; do not create independent release trains or a public daemon package.
- Keep Tauri/WebKit and `x86_64-linux` as the only release target for this phase.

#### Runtime binding and startup
- The launcher resolves both runtime components from its own immutable package closure,
  never from ambient `PATH`.
- Preserve the current same-user private AF_UNIX socket and XDG state/runtime locations.
- Start and stop the packaged daemon with the shell as one user invocation; preserve
  current relay/runtime configuration rather than inventing a service manager.
- Treat the existing private protocol hello/version rejection as the fail-closed
  compatibility boundary, with a clear launcher or shell diagnostic on failure.

#### Reproducible inputs
- Build frontend and Rust/Tauri outputs from `flake.lock`, `Cargo.lock`, `pnpm-lock.yaml`,
  checked-in fixtures, and the current exact git dependencies without repinning them.
- Use standard nixpkgs Rust, pnpm, wrapper, desktop, and closure primitives; add no second
  package manager, lockfile, vendoring scheme, or custom package framework.
- Package only runtime-required assets and native libraries; development/test tools stay in
  the dev shell.
- Keep `uzel-poc-validated-pack/compatibility.lock` as bounded compatibility evidence, not
  a build-system replacement.

#### Acceptance evidence
- Prove package/default/app evaluation and locked build, then launch by exact store path
  from a temporary non-checkout working directory.
- Assert no ambient `uzel-napd` or checkout path is used and exercise protocol mismatch as
  an explicit nonzero failure.
- Record closure size/contents, exact nampplets and NMP references, desktop assets, and the
  smallest affected real Weston/WebKit package smoke.
- Run focused probes while debugging, one coherent affected candidate validation, then the
  exact-head Codex and CodeRabbit sequence from `WORKFLOW.md`.

### the agent's Discretion
- Exact standard nixpkgs builders and file layout inside the single package closure.
- Whether the package smoke extends the current native script or adds one narrow
  package-specific entrypoint, provided native WebKit evidence remains native and no
  duplicate evidence machinery appears.

### Deferred Ideas (OUT OF SCOPE)

Flatpak, service-manager integration, multi-architecture releases, public daemon APIs,
automatic update/signing, CI expansion, and Social Home are outside Phase 2.
</user_constraints>

## Project Constraints (from AGENTS.md)

- `WORKFLOW.md` is sole active delivery-process authority. Plan one bounded issue/branch/PR, one lean plan, and only three to seven observable acceptance checks. [VERIFIED: AGENTS.md:1; WORKFLOW.md:1-25]
- Run ordinary repository commands through the locked flake: `nix --extra-experimental-features 'nix-command flakes' develop --command pnpm <script>`. Do not add a toolchain or dependency lock. [VERIFIED: WORKFLOW.md:28-40]
- Keep Graphify advisory/disposable; do not build it for this phase and never commit `.planning/graphs/` or `graphify-out/`. [VERIFIED: AGENTS.md:7-10]
- Preserve native WebKit/Weston work for a stable affected candidate, then run one complete affected validation and exact-head reviewer sequence. [VERIFIED: WORKFLOW.md:41-62]
- No upstream mutation occurs in this research. Any future upstream-bound change requires its dedicated `jodobear` fork branch and contribution record. [VERIFIED: AGENTS.md:12-14]

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PKG-01 | Linux user can build `packages.uzel` and the default package/app from locked inputs on `x86_64-linux`, with the development shell and package-dependent checks exposed by the flake. | Explicit package/default/app/check outputs, fixed frontend and Cargo dependency fetches. |
| PKG-02 | Release reviewer can prove Cargo resolution, `Cargo.lock`, flake inputs, `flake.lock`, fixtures, and packaged runtime closure identify the same accepted native-runtime and NMP commits. | One source/lock audit plus closure-reference report; no repinning. |
| PKG-03 | Linux user can start the packaged Uzel artifact from its Nix store path without the checkout, development shell, ambient system packages, or arbitrary daemon discovery from `PATH`. | Closure-private launcher, sanitized non-checkout invocation, hostile ambient-daemon check. |
| PKG-04 | Operator receives a clear failure when packaged Uzel runtime components are incompatible instead of an incompatible daemon starting silently. | Shell startup performs existing versioned `Hello` before ready; mismatch probe must be nonzero. |
| PKG-05 | Release reviewer can inspect package closure contents and size, exact native-runtime/NMP references, desktop assets, and path-relevant package/WebKit smoke evidence. | Narrow package report plus affected real Weston/WebKit store-path smoke. |
</phase_requirements>

## Summary

Current flake is development-shell-only: its sole declared system is `"x86_64-linux"` and it exposes `devShells.${system}.default`; it has no package, app, or check outputs. Quote: `system = "x86_64-linux";` and `devShells.${system}.default = pkgs.mkShell {`. [VERIFIED: flake.nix:8-12] Phase 2 should add one Nix derivation that produces both executable payloads and a `bin/uzel` supervisor/launcher, then expose that exact derivation as `packages.uzel`, package default, `apps.uzel`, and app default. Nix validates package/app/check output schemas through `nix flake check`; package/default and app/default are the standard flake interface. [CITED: https://releases.nixos.org/nix/nix-2.15.3/manual/command-ref/new-cli/nix3-flake-check.html]

Build frontend assets from the existing PNPM lock and compile both Rust binaries from the existing Cargo lock in the same derivation. The frontend lock is `lockfileVersion: '9.0'`; workspace metadata declares `"packageManager": "pnpm@10.8.0"`; Cargo pins native runtime crates at `e2f69f325a6b45213accdacfcc125e80e0687b4c`. [VERIFIED: pnpm-lock.yaml:1; package.json:6; Cargo.toml:20-23] The closure report must cross-check this source value with Cargo's locked NMP and native-runtime sources, not alter any of them. Quote: `source = "git+https://github.com/pablof7z/nmp.git?rev=005dc2a5f12aa414961b313d05ebb021934e385c#005dc2a5f12aa414961b313d05ebb021934e385c"`. [VERIFIED: Cargo.lock:2505-2519]

**Primary recommendation:** Create one `flake.nix` package derivation using fixed PNPM and Cargo dependency inputs, install shell and daemon under package-private `libexec`, and make `bin/uzel` supervise them by absolute closure paths; add handshake enforcement before `UZEL_SHELL_READY` plus one package-specific evidence probe.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Locked source-to-artifact build | Package/build | Storage | Nix owns fixed input realization; lockfiles own dependency identity. [CITED: https://releases.nixos.org/nix/nix-2.15.3/manual/command-ref/new-cli/nix3-flake-check.html] |
| Frontend bundle and Tauri shell | Package/build | Desktop runtime | Build produces Tauri frontend distribution and shell binary; launch remains native desktop work. [VERIFIED: apps/uzel/src-tauri/tauri.conf.json:7-10] |
| Paired daemon lifecycle | Package launcher | API/backend | Launcher owns one invocation's child process lifetime; daemon remains private AF_UNIX runtime authority. [VERIFIED: scripts/dev.sh:4-25] |
| Component compatibility | API/backend | Desktop runtime | Protocol owns version decision; shell must perform it before marking ready. [VERIFIED: crates/napd/src/server.rs:132-145; apps/uzel/src-tauri/src/main.rs:533-541] |
| XDG socket/state | API/backend | OS runtime | Existing daemon and shell agree on same user XDG runtime path. Quote: `path.join("uzel/napd.sock")`. [VERIFIED: apps/uzel-napd/src/main.rs:128-144; apps/uzel/src-tauri/src/main.rs:567-573] |
| Closure/assets/native evidence | Package/build | Desktop runtime | Package report proves closure; Weston/WebKit probe proves native launch only after candidate stabilizes. [VERIFIED: scripts/linux-run-smoke.sh:133-141; WORKFLOW.md:60-62] |

## Standard Stack

### Core

| Component | Version / Pin | Purpose | Prescriptive use |
|-----------|---------------|---------|------------------|
| Existing Nix flake/nixpkgs | `38a4887411571457d700c51c64a6e49ead2ed5ab` | Reproducible Linux inputs | Extend this flake only; retain its lock. Quote: `inputs.nixpkgs.url = "github:NixOS/nixpkgs/38a4887411571457d700c51c64a6e49ead2ed5ab";`. [VERIFIED: flake.nix:2-5] |
| Nix package/app/check outputs | no external package | Canonical build/run/evaluation API | Expose named and default outputs for only `x86_64-linux`. [CITED: https://releases.nixos.org/nix/nix-2.15.3/manual/command-ref/new-cli/nix3-flake-check.html] |
| Nixpkgs Rust + PNPM fixed-dependency helpers | exact locked-nixpkgs attribute names to verify at implementation | Offline/sandboxable build inputs | Use standard `buildRustPackage`/Cargo lock handling and PNPM dependency fetch/config hook; do not introduce a JS package manager or vendoring framework. [ASSUMED] |
| Tauri shell + WebKitGTK 4.1 runtime libraries | Rust `tauri = "=2.11.5"`; pinned `webkitgtk_4_1` nixpkgs input | Native Linux desktop shell | Keep WebKitGTK, GTK, OpenSSL, appindicator, and librsvg runtime inputs inside closure; Tauri documents Linux WebKitGTK 4.1 dependency family. [VERIFIED: Cargo.toml:27-28; flake.nix:18-33] [CITED: https://v2.tauri.app/start/prerequisites/] |

### Supporting

| Component | Purpose | Use |
|-----------|---------|-----|
| `makeWrapper` / shell launcher | Bind immutable runtime paths and needed desktop environment | Use a shell-capable wrapper/launcher because it must create/clean up a daemon child; Nix documents `makeWrapper` for hard-coded runtime dependency paths. [CITED: https://nixos.org/manual/nixpkgs/stable/] |
| Existing `linux-run-smoke.sh` | Native Weston/WebKit acceptance behavior | Refactor minimally only if it can invoke a supplied package launcher; otherwise add one narrow package smoke that reuses its markers and cleanup model. [VERIFIED: scripts/linux-run-smoke.sh:20-141; scripts/linux-run-smoke.sh:245-290] |
| Existing asset digest script | Checked-in fixture/trusted-shell integrity | Run unchanged within package validation; it already prints expected runtime/trusted-shell evidence. [VERIFIED: scripts/check-pinned-assets.sh:16-41] |

### Alternatives Considered

| Instead of | Could Use | Decision |
|------------|-----------|----------|
| One Nix closure | Flatpak or per-component releases | Reject: explicitly deferred and violates locked single-closure scope. [VERIFIED: .planning/phases/02-canonical-nix-release/02-CONTEXT.md:50-51] |
| Closure-private launcher | systemd/user service or ambient `uzel-napd` lookup | Reject: changes lifecycle model and violates private no-`PATH` runtime decision. [VERIFIED: .planning/phases/02-canonical-nix-release/02-CONTEXT.md:18-24] |
| Fixed Nix PNPM/Cargo fetches | Corepack cache, `pnpm install` during runtime, second vendoring tool | Reject: build must be locked and runtime closure must not depend on mutable checkout/cache. [VERIFIED: flake.nix:49-54; WORKFLOW.md:28-40] |

**Installation:** None. Phase adds no external package and must not change `Cargo.lock`, `pnpm-lock.yaml`, `flake.lock`, or dependency manifests solely to package existing source. [VERIFIED: .planning/phases/02-canonical-nix-release/02-CONTEXT.md:27-34]

## Architecture Patterns

### System Architecture Diagram

```text
locked flake + Cargo.lock + pnpm-lock.yaml + fixtures
                    |
                    v
fixed PNPM deps + fixed Cargo/git deps --> frontend dist + shell/daemon release binaries
                    |                                      |
                    +------------------> one /nix/store closure
                                                     |
                                                     v
  non-checkout cwd --> /nix/store/.../bin/uzel launcher
                             | absolute package-private paths, no PATH discovery
                             v
                    uzel-napd --live --> XDG_RUNTIME_DIR/uzel/napd.sock
                             |                      |
                             +----- versioned Hello-+--> Uzel shell -> WebKit
                                      mismatch => nonzero diagnostic
```

### Recommended Project Structure

```text
flake.nix                                # package/default/app/check outputs and closure assembly
apps/uzel/src-tauri/src/main.rs           # fail-closed Hello before shell readiness
scripts/package-smoke.sh                  # narrow store-path/closure/native evidence entrypoint
package.json                              # one script that delegates to package smoke
contracts/package-smoke-script.test.mjs   # static guard for smoke wiring, if new script needs it
```

Likely modifications are `flake.nix`, `apps/uzel/src-tauri/src/main.rs`, `scripts/package-smoke.sh`, and `package.json`; add the focused static test only if the repository's existing shell-script test pattern cannot cover it. Do not edit lockfiles unless the package build proves an existing lock is inconsistent; such a result is an implementation blocker, not permission to repin. [VERIFIED: package.json:21-41; contracts/linux-smoke-script.test.mjs:13-46]

### Pattern 1: Single derivation, private payload, public supervisor

**What:** Build frontend before release Cargo build, install Tauri shell and `uzel-napd` to `libexec`, and emit only `bin/uzel` as public entrypoint. The launcher invokes both payload paths literally from its own output, backgrounds the daemon, waits for the shell, and always terminates/reaps the child.

**When to use:** Every `nix build .#uzel`, default package, and app launch. No service unit and no second daemon artifact.

**Evidence:** Development already uses one shell-owned daemon PID, cleanup trap, `--live`, and Tauri invocation. Quote: `trap cleanup EXIT INT TERM`; `cargo run -p uzel-napd --`; `pnpm --dir apps/uzel tauri dev`. [VERIFIED: scripts/dev.sh:4-25]

### Pattern 2: Fixed dependency materialization, then locked build

**What:** Nix fetches PNPM dependencies into a fixed-output derivation and imports Cargo dependencies from `Cargo.lock` with concrete git-source hashes. Run PNPM in frozen/offline mode and Cargo with `--locked`; final `flake.nix` contains no fake hash.

**When to use:** Package derivation only. Dev shell remains developer tooling, not a release input.

**Why:** Existing workspace has both a PNPM lock and Cargo git dependencies. Quote: `lockfileVersion: '9.0'`; `source = "git+https://github.com/jodobear/nampplets?rev=e2f69f325a6b45213accdacfcc125e80e0687b4c#e2f69f325a6b45213accdacfcc125e80e0687b4c"`. [VERIFIED: pnpm-lock.yaml:1; Cargo.lock:2562-2576]

### Pattern 3: Runtime handshake gates readiness

**What:** In shell setup, construct `UnixClient`, issue `Request::Hello { version: VERSION }`, require matching `Response::Hello`, print a bounded diagnostic and return an error on refusal/unexpected response; only then manage client and print readiness. Add a focused unit test for matching and mismatched response classification.

**When to use:** Every launched package and development shell. This is the only compatibility gate; do not invent artifact version protocols.

**Evidence:** Daemon already returns `"version_mismatch"` for nonmatching hello, but shell currently immediately manages a client and prints readiness without a hello. Quote: `Request::Hello { version } if version == VERSION => Response::Hello { version }`; `"version_mismatch"`; `println!("UZEL_SHELL_READY");`. [VERIFIED: crates/napd/src/server.rs:132-145; apps/uzel/src-tauri/src/main.rs:533-541]

### Anti-Patterns to Avoid

- **Launcher uses `uzel-napd` by name:** ambient `PATH` can select incompatible or malicious daemon. Use output-qualified private path only. [VERIFIED: .planning/phases/02-canonical-nix-release/02-CONTEXT.md:18-20]
- **`exec` shell after backgrounding daemon:** launcher loses cleanup trap and can orphan daemon. Wait for shell and clean daemon on every exit. [ASSUMED]
- **Native smoke starts `pnpm dev`:** proves checkout/dev workflow, not packaged artifact. Existing script presently does exactly this; package proof needs supplied store-path entrypoint. Quote: `setsid pnpm dev >"$SMOKE_TMP/uzel.log" 2>&1 &`. [VERIFIED: scripts/linux-run-smoke.sh:245-246]
- **Put Weston, Chromium, Cargo, PNPM in release runtime closure:** violates lean closure requirement; they are test/build tooling. [VERIFIED: flake.nix:13-35; .planning/phases/02-canonical-nix-release/02-CONTEXT.md:31-33]

## Don't Hand-Roll

| Problem | Do not build | Use instead | Why |
|---------|--------------|-------------|-----|
| JS dependency cache | bespoke checked-in `node_modules` or runtime installer | Nixpkgs fixed PNPM dependency fetch/config facilities [ASSUMED] | Preserves `pnpm-lock.yaml`-bound, reproducible source materialization. |
| Cargo git vendoring | ad-hoc copied git trees | `buildRustPackage` Cargo lock import with final fixed git hashes [ASSUMED] | Cargo lock remains source of Rust resolution truth. |
| Desktop runtime search | hand-written `LD_LIBRARY_PATH` or user `PATH` discovery | Nix package `buildInputs` plus wrapper support [CITED: https://nixos.org/manual/nixpkgs/stable/] | Closure identifies native libraries deterministically. |
| Protocol compatibility scheme | package-version comparison or public daemon API | Existing protocol `Hello`/`VERSION` | Daemon already owns rejection semantics. [VERIFIED: crates/napd-protocol/src/lib.rs:19-21; crates/napd/src/server.rs:132-145] |
| Process manager | systemd/service state | existing one-invocation shell/daemon supervisor pattern | Scope locks lifecycle to launch and preserves private socket model. [VERIFIED: scripts/dev.sh:4-25] |

**Key insight:** Nix packaging must compose existing locks, private IPC, and native libraries; it must not become a parallel runtime/configuration system.

## Common Pitfalls

### Pitfall 1: Cargo git dependencies make a superficially locked build impure

**What goes wrong:** `Cargo.lock` contains git sources but Nix receives no final fixed output hashes, so build fails in sandbox or admits an unresolved/fake hash.

**How to avoid:** Start package derivation with the existing lock, run once only to obtain expected hash values, replace every fake value, and make final locked build reject any lock update. The source has both NMP and nampplets git inputs. [VERIFIED: Cargo.lock:2505-2519; Cargo.lock:2562-2576]

### Pitfall 2: PNPM version/lock mismatch

**What goes wrong:** build helper PNPM major cannot read lockfile 9 or Corepack downloads a mutable manager.

**How to avoid:** Make first derivation check select a PNPM 10 helper from the flake-pinned nixpkgs and use existing `pnpm@10.8.0`/lockfile 9; fail package evaluation before compiling if unavailable. [VERIFIED: package.json:6; pnpm-lock.yaml:1] [ASSUMED]

### Pitfall 3: Compatibility check exists but is not enforced

**What goes wrong:** server can reject mismatch but shell sends `Status` first and reaches ready against a wrong daemon.

**How to avoid:** perform `Hello` synchronously in setup before readiness; mismatch smoke asserts nonzero plus diagnostic. Current status reader sends `Request::Status`; daemon distinguishes status from hello. Quote: `client .request(&Request::Status)`; `Request::Hello { version } => Response::error(`. [VERIFIED: apps/uzel/src-tauri/src/main.rs:117-130; crates/napd/src/server.rs:132-145]

### Pitfall 4: Correct binary, wrong evidence

**What goes wrong:** test passes because it runs from checkout/dev shell, finds local daemon through `PATH`, or loads host libraries.

**How to avoid:** package smoke creates temp XDG runtime/data and non-checkout CWD, injects a decoy `uzel-napd` ahead of `PATH`, executes the exact Nix output path, and asserts no checkout path in logs/closure. Reuse native smoke's isolated XDG setup. [VERIFIED: scripts/linux-run-smoke.sh:133-141] [ASSUMED]

### Pitfall 5: Native check cost expands every edit

**What goes wrong:** Weston/WebKit is embedded in routine static/flakes checks, causing slow non-packaging feedback.

**How to avoid:** flake check covers evaluation/package-dependent static layout; run real Weston/WebKit package smoke only for affected stable candidate per `WORKFLOW.md`. [VERIFIED: WORKFLOW.md:60-62]

## Code Examples

### Required output and evidence commands

```bash
# Locked package/default/app evaluation and realization.
nix --extra-experimental-features 'nix-command flakes' flake check
nix --extra-experimental-features 'nix-command flakes' build .#uzel
nix --extra-experimental-features 'nix-command flakes' build .
nix --extra-experimental-features 'nix-command flakes' run .#uzel

# Release-review closure evidence; run against output returned by no-link build.
nix --extra-experimental-features 'nix-command flakes' path-info --closure-size "$store_path"
nix-store --query --requisites "$store_path"
```

The `packages`, `apps`, and `checks` output shapes are validated by `nix flake check`. [CITED: https://releases.nixos.org/nix/nix-2.15.3/manual/command-ref/new-cli/nix3-flake-check.html] `"uzel"` is a locked phase output name, not a new package name. [VERIFIED: .planning/phases/02-canonical-nix-release/02-CONTEXT.md:10-11]

### Required compatibility assertion

```text
start packaged daemon -> shell sends existing Hello(VERSION) -> matching Hello(VERSION) -> ready
                                             -> version_mismatch -> diagnostic + nonzero exit
```

The existing discrete protocol contract is: `pub const VERSION: u8 = 0;`, `Hello { version: u8 }`, and `Response::error("version_mismatch", ...)`. [VERIFIED: crates/napd-protocol/src/lib.rs:19-21; crates/napd-protocol/src/lib.rs:39-45; crates/napd/src/server.rs:132-137]

## State of the Art

| Old approach | Required Phase 2 approach | Impact |
|--------------|---------------------------|--------|
| `nix develop` plus Corepack cache and `pnpm dev` | fixed package derivation and absolute store-path launcher | Release no longer depends on checkout/dev shell. [VERIFIED: flake.nix:49-54; scripts/linux-run-smoke.sh:245-246] |
| Daemon supports version rejection, shell does not invoke it at startup | shell gates readiness on Hello | Incompatible components fail closed. [VERIFIED: crates/napd/src/server.rs:132-145; apps/uzel/src-tauri/src/main.rs:533-541] |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Locked nixpkgs exposes compatible `pnpm_10` fixed-dependency fetch/config helpers. | Standard Stack; Pitfall 2 | Package evaluation fails; select exact standard helper before implementation. |
| A2 | `buildRustPackage` Cargo lock import can represent all current git source hashes without introducing a vendor framework. | Standard Stack; Pitfall 1 | Need a minimal standard-Nix adjustment; do not repin dependencies. |
| A3 | A sanitized package smoke can run the app with explicit Weston test tooling while proving the app itself does not inherit dev-shell/ambient runtime dependencies. | Pitfall 4 | Refine probe environment, not product architecture. |
| A4 | Waiting for shell instead of `exec` is necessary for launcher-owned daemon cleanup. | Anti-Patterns | Validate with exit/interrupt probe. |

## Open Questions

1. **Which exact fixed hashes will the locked PNPM and Cargo fetch derivations report?**
   - What we know: existing locks identify all source revisions and integrity metadata. [VERIFIED: pnpm-lock.yaml:1-120; Cargo.lock:2505-2774]
   - What's unclear: Nix fixed-output hashes have not yet been calculated in this worktree.
   - Recommendation: implementation's first focused build captures expected values, then commits only final hashes and reruns locked build; no lockfile updates.
2. **Can the existing Linux smoke accept a package-launch command without diluting its dev proof?**
   - What we know: it hard-codes `pnpm dev`. [VERIFIED: scripts/linux-run-smoke.sh:245-246]
   - Recommendation: prefer one narrow `scripts/package-smoke.sh` that reuses marker/cleanup behavior, unless a small explicit command override keeps the existing script readable.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Nix | flake evaluation/build | ✓ | 2.34.1 | none |
| Rust/Cargo | current source checks | ✓ | 1.89.0 | locked flake toolchain |
| Node | current frontend source | ✓ | 22.22.0 | locked flake toolchain |
| PNPM | frontend packaging | ✗ host | — | flake development/package input |
| Weston | native package smoke | ✗ host | — | flake development/test tooling only |

**Missing dependencies with no fallback:** None for planning; Nix provides missing PNPM/Weston in pinned environment.

**Missing dependencies with fallback:** Host PNPM and Weston are intentionally absent; do not install them globally. [VERIFIED: environment probes, 2026-08-14; WORKFLOW.md:28-40]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard control |
|---------------|---------|------------------|
| V2 Authentication | no | no user authentication change |
| V3 Session Management | no | no session model change |
| V4 Access Control | yes | preserve same-user private AF_UNIX endpoint and socket permission checks. Quote: `fs::Permissions::from_mode(0o600)`. [VERIFIED: crates/napd/src/server.rs:345-362] |
| V5 Input Validation | yes | retain daemon option validation and only package-owned launcher configuration. [VERIFIED: apps/uzel-napd/src/main.rs:55-125] |
| V6 Cryptography | no | no cryptographic implementation or key handling change |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard mitigation |
|---------|--------|---------------------|
| Ambient daemon substitution | Spoofing/Tampering | Launcher invokes closure-private daemon by absolute path; decoy-`PATH` probe proves it. [ASSUMED] |
| Incompatible shell/daemon | Tampering/Denial of service | Enforce existing `Hello` mismatch rejection before shell ready. [VERIFIED: crates/napd/src/server.rs:132-145] |
| Socket-parent or stale-socket attack | Elevation/Tampering | Preserve daemon's socket-parent type/mode and stale socket identity checks. [VERIFIED: crates/napd/src/server.rs:479-534] |
| Checkout/dev dependency escape | Tampering | run exact output from non-checkout, sanitized environment and audit closure references. [ASSUMED] |

## Sources

### Primary (MEDIUM confidence)

- [Nix flake check reference](https://releases.nixos.org/nix/nix-2.15.3/manual/command-ref/new-cli/nix3-flake-check.html) — output schema and check semantics.
- [Nixpkgs reference manual](https://nixos.org/manual/nixpkgs/stable/) — wrapper behavior and package build mechanics.
- [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/) — Linux WebKitGTK 4.1 native dependency family.

### Local source (HIGH confidence)

- `flake.nix`, `Cargo.toml`, `Cargo.lock`, `pnpm-lock.yaml`, `scripts/dev.sh`, `scripts/linux-run-smoke.sh`, and private IPC source cited inline.

## Metadata

**Confidence breakdown:**
- Standard stack: MEDIUM — Nix/Tauri docs are official, but exact locked-nixpkgs helper attribute/hashes require first evaluation.
- Architecture: HIGH — source directly shows current dev lifecycle, socket model, and missing startup hello.
- Pitfalls: HIGH — driven by current shell/dev-smoke behavior and locked git dependency inventory.

**Research date:** 2026-08-14
**Valid until:** 2026-09-13
