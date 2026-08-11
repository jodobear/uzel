# Stack Research

**Domain:** Linux local-first Tauri napplet client — Napp consumer, canonical Nix delivery, lean CI
**Researched:** 2026-08-09
**Confidence:** MEDIUM — primary upstream docs verified; exact accepted Napp revision and current lane timings remain Gate 0 evidence.

## Recommended Stack

Keep Uzel's proven Tauri 2 + Svelte + Rust shell. Do not replace its NMP data plane, private daemon boundary, WebKit delivery path, or locked build tooling. This milestone adds only: one accepted Napp client/testkit commit consumed identically by Cargo and Nix; a package-producing flake; and measured, fail-closed CI.

### Locked Baseline — Preserve

| Technology | Locked version/input | Purpose | Decision |
|---|---:|---|---|
| Rust | `1.89.0`, edition 2024 | Tauri backend, canonical surface/layout state, Napp lifecycle | Preserve `rust-toolchain.toml`; Rust remains product/runtime owner. |
| Tauri | Rust `2.11.5`; API `2.11.1` | Existing Linux desktop shell | Preserve. Tauri 2 remains compatible with Linux WebKitGTK 4.1 development/runtime evidence. |
| Svelte | `5.56.8` | Product presentation only | Preserve; do not move runtime/Napp truth into browser state. |
| Node + pnpm | Node `>=22.12 <23`; `pnpm@10.8.0` | Svelte/Vite build and napplet tooling | Preserve lockfile + Corepack use. Use `pnpm install --frozen-lockfile`. |
| Nix flakes | locked `nixpkgs` commit `38a4887411571457d700c51c64a6e49ead2ed5ab` | Linux dependency closure and canonical release | Extend existing flake; never replace it with ambient Fedora/Debian dependencies. |
| WebKitGTK + Weston | `webkitgtk_4_1`, `weston` from locked nixpkgs | Real Linux trust-boundary evidence | Preserve existing package-output smoke route. Tauri docs still require WebKitGTK 4.1 on supported Linux builds. |
| UI quality gates | TypeScript `6.0.3`, Vite `8.1.5`, Playwright `1.62.0`, Fallow `3.9.1` | Strict UI validation, deterministic Chromium, dependency/boundary check | Preserve existing scripts; no alternate frontend test stack. |

### Gate 0 Additions — Validate Before Social Work

| Technology | Version / identity | Purpose | Why Recommended |
|---|---:|---|---|
| `napp-client` + Napp testkit | **one accepted full commit SHA, not a branch/tag** | Uzel's only Napp API and compatibility vectors | Cargo officially supports git `rev` pins and records resolved commits in `Cargo.lock`. Pair a manifest `rev` with lockfile verification; accept no consumer-local contract substitute. |
| Napp Nix flake input/package output | **same full commit SHA as Cargo** | Exact `nappd` / `nappctl` runtime closure | Nix `flake.lock` pins the complete input graph. One `pins/check` must extract and compare Cargo `rev`, `Cargo.lock` resolved commit, flake lock commit, and packaged Napp closure. |
| Nix flake outputs | `packages.uzel`, `packages.default`, `apps.uzel`, `apps.default`, `devShells.default`, `checks.*` for `x86_64-linux` | Store-path launchable release artifact | `nix flake check` should evaluate checks that depend on the actual Uzel package, not a second CI-only build. Package must launch without checkout, devShell, or ambient `PATH`. |
| cargo-nextest | pin current accepted `0.9.140` release after Gate 0 | Fast Rust test runner | Nextest supplies CI profiles and selection, but not doctests; retain `cargo test --doc` separately. Add only if measured warm p95 beats current `cargo test` enough to justify another pinned tool. |
| GitHub Actions | native `pull_request` + `merge_group` events | Changed-scope PR-fast and merge-full | GitHub requires `merge_group` trigger for checks used by merge queues. Use one stable required aggregator that fails on missing/skipped/cancelled expected lanes. |

## Exact Napp Pin Pattern

1. Accept one Napp commit only after its source, exported `napp-client` API, committed testkit vectors, runtime package outputs, and client/runtime mismatch behavior are replayed against Uzel POC evidence.
2. Declare every direct Rust Napp dependency from that repository with the same full `rev`; commit `Cargo.lock`.
3. Add Napp as a flake input at the same revision; commit `flake.lock`; make `packages.uzel` reference the Napp package output directly.
4. Implement a small deterministic `pins/check` script. It fails closed for a missing full SHA, differing Cargo/flakes/locks, a non-commit ref, missing exact runtime binary, or a package closure outside locked inputs.
5. Launch package smoke with an intentionally scrubbed `PATH`, from the store result, and prove compatible runtime selection plus clear mismatch failure. Never search for `nappd`/`nappctl` on `PATH`.

Do **not** treat existing `jodobear/nampplets@e2f69f...` pins as the future Napp acceptance. They are POC baseline evidence, not an approved Napp consumer identity.

## CI Tooling and Scope

| Change class | PR-head lane | Merge-group / scheduled lane |
|---|---|---|
| Docs only | link/diagram/doc audit | none |
| Svelte/UI only | format, `svelte-check`, Node tests, Fallow, targeted Chromium | canonical package once on `merge_group` |
| Rust/product | `cargo fmt --check`, Clippy `-D warnings`, current full workspace or proven nextest scope | canonical package + full suite |
| Napp binding/contracts | Rust + frontend contract/testkit vectors | package/client-runtime compatibility smoke |
| Tauri/WebKit/source-binding/packaging/locks | normal fast lane plus package preflight | package-output Weston/WebKit smoke |

Start Rust scope with one full-workspace compile job so Clippy and test artifacts are reused. Gate 0 records warm p50/p95 for full workspace and a prototype affected reverse-dependency graph; add changed-crate selection only if it materially lowers p95 and promotes unknown/shared manifest/lock/build/CI changes to full scope. Never make a path-filtered job independently required: the required aggregator decides expected jobs from the classifier and fails closed.

Set per-PR workflow concurrency with `cancel-in-progress: true`. Cache pnpm store and Cargo registry/target layers only after measurements, keyed by lockfiles, Rust version, target and OS. Keep Nix store caching separate. GitHub documents that cache contents are readable by untrusted workflows: no secrets, signing material, trusted mutable runner state, or writable trusted cache on fork PRs.

## Development Commands

```bash
# Existing locked baseline — run inside the Nix development shell
pnpm install --frozen-lockfile
pnpm check && pnpm lint && pnpm test

# Canonical package Gate 0 target
nix flake check
nix build .#uzel
./result/bin/uzel --version
```

No `npm install`, global `cargo install`, host WebKit package installation, Flatpak, sccache, or third-party CI platform is required. The repository still needs committed GitHub Actions for `pull_request` and `merge_group`, plus one stable required aggregator; no workflow currently supplies those gates. Add nextest only through its pinned installation/release process once Gate 0 approves it.

## Alternatives Considered

| Recommended | Alternative | When Alternative Fits |
|---|---|---|
| Store-path Nix package | Tauri-generated AppImage/RPM/Deb/Flatpak | Tauri supports these Linux formats, but Uzel's product contract explicitly selects Nix; do not add parallel release artifacts now. |
| Cargo `rev` + Nix locked input + `pins/check` | Cargo lockfile alone | Never: Cargo lock cannot prove Nix runtime closure consumes same Napp commit. |
| Full-workspace Rust fast lane first | Immediate affected-crate resolver | Only after measured p95 benefit and fail-safe classifier fixtures. |
| One aggregator required check | Branch-required path-filter jobs | Never: a required path-filtered check can be skipped without proving scope was correct. |
| Existing Chromium + path-gated Weston/WebKit | Chromium-only native acceptance | Never for host/surface/security/package changes; browser engine coverage is not interchangeable. |

## What NOT to Use

| Avoid | Why | Use Instead |
|---|---|---|
| Napp branch/tag pins or arbitrary `PATH` runtime discovery | Permits client/runtime drift and cannot reproduce an exact trust boundary. | Full accepted commit in Cargo, flake lock, closure and `pins/check`. |
| Uzel-owned Nostr/runtime/grants/cache/signing layer | Breaks NMP and Napp ownership, producing competing local truth. | NMP sole data plane; Napp sole runtime contract/policy; Uzel presentation/product state only. |
| Flatpak or ambient host-package workaround | Contradicts canonical Nix closure and masks missing dependency declarations. | Locked nixpkgs inputs and package-output smoke. |
| Unmeasured sccache + target cache + Nix cache stack | Extra invalidation and restore cost can worsen p95. | One measured cache layer at a time. |
| Automatic retry for required tests | Converts flaky evidence into apparent success. | Fail required lane; diagnose and fix flake. |
| Generic frontend/native rebuild | Discards proven POC acceptance while expanding scope. | Minimal seam extraction behind Tauri/Svelte/Rust baseline. |

## Version Compatibility

| Package A | Compatible With | Notes |
|---|---|---|
| Tauri `2.11.5` | WebKitGTK 4.1 / locked Nix `webkitgtk_4_1` | Existing POC evidence. Re-run real Weston/WebKit against package output after host or package change. |
| Svelte `5.56.8` | Vite `8.1.5`, plugin `7.2.0`, TypeScript `6.0.3` | Existing `pnpm-lock.yaml` resolves this set; update together only in a separately evidenced change. |
| Rust `1.89.0` | Cargo workspace, Clippy/rustfmt components | Exact toolchain file remains source of truth. |
| Accepted Napp commit | Cargo `rev`, `Cargo.lock`, Nix flake input/lock, packaged runtime | Gate 0 must prove equality and replay testkit/client-runtime compatibility before recording a version. |

## Sources

- [Cargo: specifying git dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) — `rev` pin and `Cargo.lock` behavior; MEDIUM confidence (official source, Context7 unavailable in this runtime).
- [Nix flakes reference](https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-flake.html#lock-files) and [Nix `flake check`](https://releases.nixos.org/nix/nix-2.32.3/manual/command-ref/new-cli/nix3-flake-check.html) — lock graph and canonical checks; MEDIUM.
- [GitHub merge-group event](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#merge_group), [concurrency](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/control-workflow-concurrency), and [cache security](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching) — merge queue, cancellation and cache boundary; MEDIUM.
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) and [distribution guide](https://v2.tauri.app/distribute/) — Linux/WebKitGTK delivery context; MEDIUM.
- [cargo-nextest](https://nexte.st/) and [configuration reference](https://nexte.st/docs/configuration/reference/) — CI runner/profile capabilities; MEDIUM.

---
*Stack research for: Uzel Linux Napp-consumer milestone*
*Researched: 2026-08-09*
