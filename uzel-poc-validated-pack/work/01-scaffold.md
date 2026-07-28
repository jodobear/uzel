# Work 01 — scaffold

## Goal

Create the smallest reproducible single-repository workspace.

## Depends on

Accepted Work 00 pins and commands.

## Entry status

**Blocked by Gate 0.** Do not execute this slice until [`../compatibility.lock`](../compatibility.lock) changes to a go verdict after the 0.29 nampplets/conformance reconciliation.

When unblocked, start from the locked environment record: nixpkgs `38a4887411571457d700c51c64a6e49ead2ed5ab`, Nix 2.34.1, Rust 1.89.0 for nampplets, Node 22.22.0, pnpm 10.8.0, Tauri CLI 2.11.4/crate 2.11.5, WebKitGTK 2.52.5, Fallow 3.9.1, and Mermaid CLI 11.16.0. Put these in a committed flake/Cargo/package lock; do not depend on the mutable `nixpkgs` registry alias.

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

- clean Nix shell;
- daemon and shell processes start;
- Fedora run and Debian build smoke;
- no feature implementation or speculative crate split.

## Non-goals

No runtime session, NMP query, napplet, persistence schema, or UI polish.
