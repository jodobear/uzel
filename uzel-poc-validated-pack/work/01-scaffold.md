# Work 01 — scaffold

## Status

**Complete.** The accepted Linux-only scaffold is implemented on `feat/slice-01-scaffold`; evidence is in [`../reports/slice-01-preflight.md`](../reports/slice-01-preflight.md).

## Goal

Create the smallest reproducible single-repository workspace.

## Depends on

Accepted Work 00 pins and commands.

## Entry status

**Authorized by Gate 0.** [`../compatibility.lock`](../compatibility.lock) records a Linux-scoped go against exact provisional pins. Upstream nampplets ratification and Apple evidence are not POC blockers.

The committed environment uses nixpkgs `38a4887411571457d700c51c64a6e49ead2ed5ab`, Nix 2.34.1, Rust 1.89.0, Node 22.23.1 from that nixpkgs commit, pnpm 10.8.0, Tauri CLI 2.11.4/crate 2.11.5, WebKitGTK 2.52.5, Fallow 3.9.1, and Mermaid CLI 11.16.0. Gate 0 observed host Node 22.22.0; it is not the dev-shell pin. The flake and Cargo/pnpm lockfiles do not depend on the mutable `nixpkgs` registry alias.

## Tasks

- Create a pinned Nix flake dev shell, Cargo, pnpm, Tauri, and Svelte workspace.
- Add only:
  - `apps/uzel`;
  - `apps/uzel-napd`;
  - `crates/napd`;
  - `crates/napd-protocol`;
  - `napplets/` and `fixtures/` directories.
- Make daemon and empty shell start from one documented command.
- Enable the Corepack pnpm shim on `PATH` before Turbo scripts; `corepack pnpm` alone does not satisfy nested `pnpm` discovery.
- Establish Rust/frontend/Fallow/document commands from the validated ledger in `reports/preflight.md`.
- Add dependency-boundary checks.

## Acceptance

- [x] clean Nix shell;
- [x] daemon and shell processes start;
- [x] Fedora run and Debian build smoke;
- [x] no feature implementation or speculative crate split.

## Non-goals

No runtime session, NMP query, napplet, persistence schema, or UI polish.
