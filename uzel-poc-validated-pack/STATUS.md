# POC status

> Durable project state. Update after every integrated slice.

## Gate 0 — validated baseline

- [x] Kehto #204 merge commit verified and pinned
- [x] Napplet package/spec revisions pinned, including provisional/open status
- [x] `nampplets` Linux reuse map completed
- [x] NMP API map completed
- [x] Tauri/WebKit source-binding and synthetic egress spike passed
- [x] Nix/Rust/Node/Tauri/Fallow/Mermaid commands recorded
- [x] Provisional design corrected where evidence required it

Gate 0 outcome: **complete; Slice 01 NO-GO**. V-06 now passes after the verified Kehto build correction. V-01 still fails on source reachability, and the unratified nampplets candidate still lacks three required signoffs. Completion means the assumptions were decided with evidence, not that every claim passed.

## Implementation

- [ ] 01 workspace scaffold
- [ ] 02 Linux runner and exact-build fixture
- [ ] 03 two portable napplets and INC fixtures
- [ ] 04 daemon, NMP, and minimal persistence
- [ ] 05 integrated composed demo
- [ ] 06 user/dev modes, hostile tests, clean demo

## Latest integrated evidence

```text
slice: 00 validate
commits: recorded in repository history
commands: reports/preflight.md#validated-command-ledger
observable result: Linux runtime/NMP/IPC/WebKit and corrected 0.29 artifact compatibility pass locally
known limitation: full hostile suite and clean distro acceptance belong to later slices
next action: publish the exact Kehto fix and obtain named review of the reachable nampplets 0.29 candidate
```

## Current blockers

```text
1. Verified Kehto candidate 62241de is not remotely reachable: jodobear cannot write kehto/web and has no jodobear/web fork.
2. Reachable nampplets candidate b1a38f1 remains unratified; compatibility, security, and NMP-boundary signoffs are blank.
3. Slice 01 may not start until both blockers clear and compatibility.lock changes to go.
```
