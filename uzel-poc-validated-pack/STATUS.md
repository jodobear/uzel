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

Gate 0 outcome: **complete; Slice 01 GO for the Linux-only POC**. Every V-01 through V-08 probe passes. Uzel accepts exact `jodobear/nampplets@08ddb87...` provisionally without claiming upstream ratification; Apple evidence is outside scope. Kehto PR #218 proceeds independently.

## Implementation

- [x] 01 workspace scaffold
- [x] 02 Linux runner and exact-build fixture
- [ ] 03 two portable napplets and INC fixtures
- [ ] 04 daemon, NMP, and minimal persistence
- [ ] 05 integrated composed demo
- [ ] 06 user/dev modes, hostile tests, clean demo

## Latest integrated evidence

```text
slice: 02 Linux exact-build runner
commits: recorded in repository history
commands: reports/slice-02-preflight.md
observable result: pinned signed fixture verifies, launches, completes NAP-SHELL, answers identity.getPublicKey through the runtime, and denies child native authority in real Fedora WebKit; Debian Bookworm build passes
known limitation: the 4096-byte daemon control frame cannot transfer the 96172-byte verified document; Slice 02 therefore composes RuntimeController in the Tauri process until Work 04 adds bounded chunked verified-asset transfer
next action: execute work/03-napplets.md; do not move runtime ownership to the daemon until the Work 04 asset-transfer gate passes
```

## Accepted provisional risks

```text
1. Upstream nampplets baseline remains unratified; Uzel owns the exact Linux-only POC adoption decision.
2. inc.channel.opened and NAP-INTENT delivery remain unsupported and unused.
3. Any pin or surface change reopens the affected Gate 0 probes. Apple evidence and Kehto merge are not Uzel blockers.
```
