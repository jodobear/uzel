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

Gate 0 outcome: **complete; Slice 01 GO for the Linux-only POC**. Every V-01 through V-08 probe passes. Uzel now pins reviewed successor `jodobear/nampplets@7eccdee...` provisionally without claiming upstream ratification; Apple evidence is outside scope. Kehto PR #218 merged as `4fd4aff...`.

## Implementation

- [x] 01 workspace scaffold
- [x] 02 Linux runner and exact-build fixture
- [x] 03 two portable napplets and INC fixtures
- [x] 04 daemon, NMP, and minimal persistence
- [x] 05 integrated composed demo
- [ ] 06 user/dev modes, hostile tests, clean demo

## Latest integrated evidence

```text
slice: 05 integrated composed demo
commits: recorded in repository history
commands: reports/slice-05-preflight.md
observable result: one daemon-owned RuntimeController verifies and launches the exact profile-card and follow-list builds; the unchanged upstream host maps each MessageEvent.source to its own surface; runtime-owned INC routing delivers queryless profile/open from follow-list to profile-card; the one-window shell provides persisted orientation/ratio, focus, resize, fullscreen, read identity, source status, and a user-hidden bounded developer drawer
known limitation: Work 05 proves the click path at the napplet contract/runtime/daemon seams and proves both artifacts in real WebKit, but the automated Fedora harness does not synthesize a pointer click inside the nested opaque-origin iframe. Full hostile WebKit execution, zero-accept sentinel attestation, clean Debian demo, and final user/dev acceptance remain Work 06
next action: execute work/06-hardening-demo.md with the signed hostile artifact and live ephemeral sentinel, then run clean Fedora and Debian acceptance from a fresh checkout
```

## Accepted provisional risks

```text
1. Upstream nampplets baseline remains unratified; Uzel owns the exact Linux-only POC adoption decision.
2. inc.channel.opened and NAP-INTENT delivery remain unsupported and unused.
3. Any pin or surface change reopens the affected Gate 0 probes. Apple evidence and Kehto merge are not Uzel blockers.
```
