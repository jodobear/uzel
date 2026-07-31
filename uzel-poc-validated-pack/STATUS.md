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

Gate 0 outcome: **complete; Slice 01 GO for the Linux-only POC**. Every V-01 through V-08 probe passes. Uzel pins validated Linux runtime `jodobear/nampplets@e539378...` and exact portable shell assets from fork contribution head `fc68bce...` provisionally without claiming upstream ratification. The later Rust runtime is not accepted because it refuses the deterministic plaintext loopback relay. Apple evidence is outside scope. Kehto PR #218 merged as `4fd4aff...`.

## Implementation

- [x] 01 workspace scaffold
- [x] 02 Linux runner and exact-build fixture
- [x] 03 two portable napplets and INC fixtures
- [x] 04 daemon, NMP, and minimal persistence
- [x] 05 integrated composed demo
- [x] 06 user/dev modes, hostile tests, clean demo
- [ ] 07 issue-driven product stabilization (active: #19 initialization retry identity ordering)

## Latest integrated evidence

```text
slice: Work 07 active; issue #10 renderer harness merged in PR #20; issue #19 initialization retry fix is under exact-head review in PR #22
commits: PR #20 merge 83e2e1e7e1565b6fb4ab24ac5b8dc4ecb94fbfc0; #19 activation fix 7a173dbd74787fbbf9acc01eb087f87a0e52939a; ordering/recovery hardening ab55861d86679ed38571b908f0b8a65b4951b6c7; graph refresh ed28af5c5ab70a31436b87e70b7948434a2ccf5b
commands: pnpm check; pnpm lint; pnpm test; pnpm test:ui; pnpm fallow; pnpm format:check; pnpm docs:check; targeted initialization-empty-identity, initialization-identity-failure, and restart-reconciliation UI scenarios; screenshot inspection; pnpm smoke:linux
observable result: initialization retry no longer calls the public identity helper that is intentionally locked during recovery; it activates the read identity through a focused internal path before launching either base surface. The acceptance mock delays identity completion and rejects any premature surface launch. Empty-identity recovery passes at 1366x768 and 1920x1080; a one-shot identity-selection failure launches zero surfaces, remains locked and recoverable, then a second retry selects the identity and launches exactly two surfaces. Diagnostics refresh remains after user-driven pane restart, preventing a diagnostics failure from pairing a new identity with stale panes. Restart reconciliation waits for the selected profile rather than treating an already-ready shell as completion. The full renderer suite passes 22/22 test nodes with zero unacknowledged console, page, request, or external-network failures. Fallow reports zero issues; documentation and formatting audits pass. Real Weston/WebKit smoke reports LINUX_RUN_SMOKE_OK with daemon and shell ready, hostile egress denied, and both network and native sentinels zero.
known limitation: renderer recovery scenarios use the committed mocked-native browser lane and do not claim NMP, Unix IPC, Tauri, or WebKit behavior. The separate real Linux smoke proves the integrated Tauri/WebKit boundary but does not visually reproduce the injected initialization failures. Visible Debian interactive acceptance remains a separate human-visible gate. The current Uzel nampplets pin remains d533a63d519c14470f900323958509cdea1c6479 until #11 validates the merged-main successor.
next action: address PR #22 exact-head review and merge #19; in parallel finish #11 successor validation and the data-only #21 external corpus lock. #12 implementation waits for #11 and its recorded specification authority decision.
```

## Accepted provisional risks

```text
1. Upstream nampplets baseline remains unratified; Uzel owns the exact Linux-only POC adoption decision.
2. inc.channel.opened and NAP-INTENT delivery remain unsupported and unused.
3. Any pin or surface change reopens the affected Gate 0 probes. Apple evidence and Kehto merge are not Uzel blockers.
4. OS-level per-WebKit-child network isolation and trusted local TLS are post-POC hardening, not retroactive claims of this acceptance.
```
