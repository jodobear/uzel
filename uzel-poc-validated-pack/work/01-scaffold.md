# Work 01 — scaffold

## Goal

Create the smallest reproducible single-repository workspace.

## Depends on

Accepted Work 00 pins and commands.

## Tasks

- Create Nix, Cargo, pnpm, Tauri, and Svelte workspace.
- Add only:
  - `apps/uzel`;
  - `apps/uzel-napd`;
  - `crates/napd`;
  - `crates/napd-protocol`;
  - `napplets/` and `fixtures/` directories.
- Make daemon and empty shell start from one documented command.
- Establish Rust/frontend/Fallow/document commands.
- Add dependency-boundary checks.

## Acceptance

- clean Nix shell;
- daemon and shell processes start;
- Fedora run and Debian build smoke;
- no feature implementation or speculative crate split.

## Non-goals

No runtime session, NMP query, napplet, persistence schema, or UI polish.
