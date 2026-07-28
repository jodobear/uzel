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
- [ ] 02 Linux runner and exact-build fixture
- [ ] 03 two portable napplets and INC fixtures
- [ ] 04 daemon, NMP, and minimal persistence
- [ ] 05 integrated composed demo
- [ ] 06 user/dev modes, hostile tests, clean demo

## Latest integrated evidence

```text
slice: 01 scaffold
commits: recorded in repository history
commands: reports/slice-01-preflight.md
observable result: pinned shell, empty napd process, Tauri/WebKit shell, Fedora headless run, and Debian Bookworm build all pass
known limitation: scaffold has no runtime session, NMP query, napplet, persistence, or product UI by design
next action: execute work/02-linux-runner.md; keep upstream contributions on fork branches and record them in docs/08-upstream-contributions.md
```

## Accepted provisional risks

```text
1. Upstream nampplets baseline remains unratified; Uzel owns the exact Linux-only POC adoption decision.
2. inc.channel.opened and NAP-INTENT delivery remain unsupported and unused.
3. Any pin or surface change reopens the affected Gate 0 probes. Apple evidence and Kehto merge are not Uzel blockers.
```
