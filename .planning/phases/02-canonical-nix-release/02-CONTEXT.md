# Phase 2: Canonical Nix release - Context

**Gathered:** 2026-08-14
**Status:** Ready for planning

<domain>
## Phase Boundary

Produce one reproducible `x86_64-linux` Uzel package and app from the existing locked
source. The store-path release contains and launches the exact compatible Uzel shell and
private daemon without relying on the checkout, development shell, ambient packages, or
arbitrary `PATH` discovery.

</domain>

<decisions>
## Implementation Decisions

### Release shape
- Expose `packages.x86_64-linux.uzel` and make it the default package.
- Expose `apps.x86_64-linux.uzel` and make it the default app.
- Keep one closure with a stable `bin/uzel` launcher and package-private shell/daemon
  executables; do not create independent release trains or a public daemon package.
- Keep Tauri/WebKit and `x86_64-linux` as the only release target for this phase.

### Runtime binding and startup
- The launcher resolves both runtime components from its own immutable package closure,
  never from ambient `PATH`.
- Preserve the current same-user private AF_UNIX socket and XDG state/runtime locations.
- Start and stop the packaged daemon with the shell as one user invocation; preserve
  current relay/runtime configuration rather than inventing a service manager.
- Treat the existing private protocol hello/version rejection as the fail-closed
  compatibility boundary, with a clear launcher or shell diagnostic on failure.

### Reproducible inputs
- Build frontend and Rust/Tauri outputs from `flake.lock`, `Cargo.lock`, `pnpm-lock.yaml`,
  checked-in fixtures, and the current exact git dependencies without repinning them.
- Use standard nixpkgs Rust, pnpm, wrapper, desktop, and closure primitives; add no second
  package manager, lockfile, vendoring scheme, or custom package framework.
- Package only runtime-required assets and native libraries; development/test tools stay in
  the dev shell.
- Keep `uzel-poc-validated-pack/compatibility.lock` as bounded compatibility evidence, not
  a build-system replacement.

### Acceptance evidence
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

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `flake.nix` already pins nixpkgs and defines the complete native development dependency set.
- `apps/uzel/src-tauri/` builds the current Tauri shell; `apps/uzel-napd/` builds the private daemon.
- `scripts/dev.sh` already models paired daemon/shell lifetime for development.
- `scripts/linux-run-smoke.sh` supplies the accepted real Weston/WebKit probe.

### Established Patterns
- Normal commands enter through locked Nix plus pnpm scripts.
- Rust owns runtime truth; Svelte owns presentation; the daemon protocol already performs a
  versioned hello and fails on mismatch.
- Exact source/fixture pins and native success markers are already recorded without secrets.

### Integration Points
- Extend `flake.nix` from dev-shell-only to package/check/app outputs.
- Bind the package launcher to the shell and daemon produced by the same derivation.
- Add only the package-aware probe and package metadata needed by PKG-01 through PKG-05.

</code_context>

<specifics>
## Specific Ideas

Issue: https://github.com/jodobear/uzel/issues/46

</specifics>

<deferred>
## Deferred Ideas

Flatpak, service-manager integration, multi-architecture releases, public daemon APIs,
automatic update/signing, CI expansion, and Social Home are outside Phase 2.

</deferred>
