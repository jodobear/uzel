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

Gate 0 outcome: **complete; Slice 01 NO-GO**. V-01 and V-06 remain blocking failures. Completion means the assumptions were decided with evidence, not that every claim passed.

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
observable result: Linux runtime/NMP/IPC/WebKit feasibility passed; compatibility line failed
known limitation: full hostile suite and clean distro acceptance belong to later slices
next action: repair and ratify one upstream Napplet 0.29 compatibility baseline, then rerun blocking gates
```

## Current blockers

```text
1. Kehto #204 chat/feed artifacts fail @napplet/conformance-cli@0.2.16 on Vite modulepreload fetch.
2. nampplets@839654c remains unratified and pinned to Napplet 0.28, not Kehto's 0.29 line.
3. Slice 01 may not start until both are reconciled and compatibility.lock changes to go.
```
