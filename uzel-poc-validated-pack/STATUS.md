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
- [x] 03 two portable napplets and INC fixtures
- [ ] 04 daemon, NMP, and minimal persistence
- [ ] 05 integrated composed demo
- [ ] 06 user/dev modes, hostile tests, clean demo

## Latest integrated evidence

```text
slice: 03 portable napplets and INC fixtures
commits: recorded in repository history
commands: reports/slice-03-preflight.md
observable result: follow-list and profile-card build independently, pass released conformance, verify as signed single-file NIP-5D fixtures, and exchange the exact queryless profile-open payload through runtime-owned NAP-INC sender binding; profile-card projects exactly one NMP-selected canonical kind-0 row and never selects replaceable events; the hostile fixture requires a configured live loopback sentinel and is pinned for the later WebKit denial lane
known limitation: released conformance skips manifest checks for local directory input; pinned NMP verification covers event signature and exact artifact bytes, while live-sentinel startup, independent zero-accept attestation, and full hostile browser execution remain Work 06
next action: execute work/04-daemon-nmp.md; first prove bounded verified-asset transfer without enlarging the 4096-byte control frame
```

## Accepted provisional risks

```text
1. Upstream nampplets baseline remains unratified; Uzel owns the exact Linux-only POC adoption decision.
2. inc.channel.opened and NAP-INTENT delivery remain unsupported and unused.
3. Any pin or surface change reopens the affected Gate 0 probes. Apple evidence and Kehto merge are not Uzel blockers.
```
