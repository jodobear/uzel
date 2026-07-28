# Slice 01 preflight

## Verdict

**PASS.** The Linux-only single-repository scaffold satisfies every acceptance item in [`../work/01-scaffold.md`](../work/01-scaffold.md). This result does not depend on Kehto PR #218 merging and does not claim any runtime/NMP feature implementation.

## Locked workspace

| Input | Exact value |
|---|---|
| nixpkgs | `38a4887411571457d700c51c64a6e49ead2ed5ab` |
| `flake.lock` SHA-256 | `07a836d15a009b9960eab20f0269fb36efcd300d51a68b6093a123a80673ec25` |
| `Cargo.lock` SHA-256 | `460f541390c8f9d8a14306d2263969925621851e63d9902eb6c128e693fe7c3c` |
| `pnpm-lock.yaml` SHA-256 | `1941e8b66fb762f45e5074014b27979db18c10ed2976c1d7b2c4e1ee4fb496cd` |
| Rust | `1.89.0 (29483883e 2025-08-04)` |
| Node | `22.23.1` |
| pnpm | `10.8.0` |
| Tauri CLI / crate | `2.11.4` / `2.11.5` |
| WebKitGTK on Fedora/Nix | `2.52.5` |
| Fallow | signed `3.9.1` package and platform binary |
| Mermaid CLI | `11.16.0` |

Gate 0 recorded host Node 22.22.0. That observation was not suitable as the reproducible workspace pin: the exact accepted nixpkgs commit supplies Node 22.23.1, which is what all Slice 01 Nix-shell evidence uses.

## Commands and observed results

```sh
nix --extra-experimental-features 'nix-command flakes' flake check
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm install --frozen-lockfile
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm check
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm test
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm lint
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm format:check
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm docs:check
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm fallow
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm smoke
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm smoke:fedora
bash scripts/debian-build-smoke.sh
```

Observed:

- flake evaluates successfully and the direct dev-shell binaries report the locked versions above;
- Svelte reports 0 errors and 0 warnings, Vite produces the shell assets, and the full Cargo workspace checks;
- all Rust unit/doc tests pass;
- Clippy with `-D warnings`, rustfmt, documentation audit, and dependency-boundary checks pass;
- Fallow 3.9.1 reports 0 issues with `--fail-on-issues`;
- local smoke reports `UZEL_NAPD_READY role=runtime-authority` and `SLICE_01_SMOKE_OK`;
- Fedora run reports `FEDORA_RUN_SMOKE_OK daemon=ready shell=ready compositor=weston-headless-gl` after both processes remain alive;
- Debian Bookworm reports Node 22.23.1, pnpm 10.8.0, Rust 1.89.0, builds the locked frontend, compiles the complete Tauri workspace, and reports `DEBIAN_BUILD_SMOKE_OK`.

## Fedora probe correction

The first headless launch failed before Uzel started:

```text
Error: EGL surfaceless platform cannot be used.
fatal: failed to create compositor backend
```

The failed Weston log is preserved at [`probes/slice-01-fedora-failed/weston.txt`](probes/slice-01-fedora-failed/weston.txt). The Nix shell had Mesa installed but did not select its Nix-store EGL vendor or DRI directory. The flake now exports `__EGL_VENDOR_LIBRARY_FILENAMES`, `LIBGL_DRIVERS_PATH`, and `LIBGL_ALWAYS_SOFTWARE=1` from the pinned Mesa derivation. The same GL probe then passed. Pixman was not substituted because Gate 0 had already rejected it for WebKitGTK GPU-process coverage.

## Debian probe correction

The first container command successfully compiled the frontend and complete Rust/Tauri workspace, then failed while committing the resulting multi-gigabyte image:

```text
write /var/tmp/container_images_storage4263621878/1: no space left on device
```

That was a probe packaging failure, not a source-build failure. The corrected smoke builds an immutable-digest Debian toolchain image, mounts the source through Podman's disposable overlay mode, disables Cargo debug data and incremental state, and performs the build in the disposable container. It exits successfully without committing build output. The exact failed dangling Uzel image chain was removed; unrelated images, containers, and volumes were untouched.

Debian inputs:

```text
node:22.23.1-bookworm@sha256:175215a1f306ed5df592434b99cc2019f70624373fe49cb659240a618a846aed
rust:1.89.0-bookworm@sha256:c9ac3fa8945b61dede1e4500d25028aa8fd8a8fe46365fcf9c0422f8d999b9b0
libwebkit2gtk-4.1-dev 2.50.6-1~deb12u2
```

## Boundary result

Workspace contains only the planned app/crate surfaces plus build, config, fixture, and evidence files. `napd` is a thin composition boundary; it does not duplicate NMP or nampplets functionality. No NMP query, runtime session, napplet implementation, persistence schema, cryptography, or product UI exists in Slice 01.

## Next step

Proceed to [`../work/02-linux-runner.md`](../work/02-linux-runner.md). Any upstream-worthy change must first live on a dedicated branch in the corresponding `jodobear` fork and be recorded in [`../docs/08-upstream-contributions.md`](../docs/08-upstream-contributions.md).
