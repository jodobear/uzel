# Phase 2: Canonical Nix release summary

## Outcome

Phase 2 is complete. Uzel now ships one exact-pinned `x86_64-linux` Nix store closure
whose public launcher owns its package-private Tauri/WebKit shell and daemon, checks their
private protocol compatibility before readiness, and fails closed across socket, process,
source, and package-identity boundaries.

## Delivery

- Issue [#46](https://github.com/jodobear/uzel/issues/46) is closed.
- PR [#48](https://github.com/jodobear/uzel/pull/48) merged reviewed head
  `2c272773a109ed79d836da704049e5543d1a7943` as merge commit
  `43e983fce04ad88b8046e90c7be2536274d193b4`.
- GitHub Codex and substantive CodeRabbit review were clean on the exact merge head.

## Acceptance

- PKG-01: named/default package, app, checks, and development-shell outputs resolve from
  locked inputs.
- PKG-02: Cargo, flake, fixture, Nampplets, trusted-shell, and NMP identities agree with
  the realized closure.
- PKG-03: the store-path launcher uses closure-private payloads and preserves bounded
  process and socket ownership without checkout, ambient package, or `PATH` authority.
- PKG-04: the existing versioned Hello exchange succeeds when compatible and refuses a
  mismatch before shell readiness.
- PKG-05: closure contents, assets, references, lifecycle evidence, and real packaged
  Weston/WebKit recovery are inspectable.

## Validation evidence

The merged verification records 5/5 must-haves, exact output and payload hashes, focused
Rust/Node/Nix checks, hostile closure and socket negatives, sequential/concurrent and
TERM/INT/KILL lifecycle probes, a mismatch-only Weston discriminator, and one stable real
Weston/WebKit package run. Final wrapper-only changes retained the WebKit-passed payload
bytes and passed their affected launcher probes.

Phase 2 cleanup preserved 14 unique rejected-reproducer files at the protected Phase 2
evidence destination, removed the completed and diagnostic worktrees plus rebuildable
output, and retained every branch/ref.
