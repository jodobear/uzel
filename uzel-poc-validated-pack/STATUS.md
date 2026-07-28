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

Gate 0 outcome: **complete; Slice 01 NO-GO**. V-01 and V-06 pass after publishing and verifying the Kehto build correction. The unratified nampplets candidate still lacks three required signoffs and Apple-host evidence. Completion means the assumptions were decided with evidence, not that every claim passed.

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
next action: open the prepared Kehto upstream PR in parallel; obtain named review and Apple evidence for the reachable nampplets 0.29 candidate, or record explicit Uzel risk acceptance
```

## Current blockers

```text
1. Reachable nampplets candidate 08ddb87 remains unratified; compatibility, security, and NMP-boundary signoffs are blank.
2. Its bundled Apple corpus/catalog changes have not run under Xcode.
3. Slice 01 may not start until these are reviewed or explicitly accepted and compatibility.lock changes to go. Kehto upstream merge is not a blocker.
```
