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
- [x] 04 daemon, NMP, and minimal persistence
- [ ] 05 integrated composed demo
- [ ] 06 user/dev modes, hostile tests, clean demo

## Latest integrated evidence

```text
slice: 04 daemon, NMP, and minimal persistence
commits: recorded in repository history
commands: reports/slice-04-preflight.md
observable result: Tauri is a thin AF_UNIX client; the daemon owns one pinned RuntimeController/NMP engine/store, transfers the 96172-byte verified fixture in ordered bounded chunks, parses and restores one public read identity through NMP, and returns canonical kind-0/direct-follow data from a disposable live relay and cache-first restart without a second Nostr cache
known limitation: profile indexer/app preferences remain wss-only; the disposable ws loopback relay correctly uses NMP's fallback lane and explicit local-host allowlist. Full two-pane composition and hostile WebKit execution remain Work 05 and Work 06
next action: execute work/05-integrate.md and compose follow-list/profile-card through the completed daemon seam
```

## Accepted provisional risks

```text
1. Upstream nampplets baseline remains unratified; Uzel owns the exact Linux-only POC adoption decision.
2. inc.channel.opened and NAP-INTENT delivery remain unsupported and unused.
3. Any pin or surface change reopens the affected Gate 0 probes. Apple evidence and Kehto merge are not Uzel blockers.
```
