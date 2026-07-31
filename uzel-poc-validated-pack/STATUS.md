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
- [ ] 07 issue-driven product stabilization (active: #25 profile/follow avatar transport)

## Latest integrated evidence

```text
slice: Work 07 active; issue #25 implementation is locally accepted on fix/profile-follow-click-timeout; upstream provider-push configuration is tracked in jodobear/nampplets PR #11
commits: author-bound host-deadline query eb15650ed6dbbf37038ba1950ea78b9b4aad1dc8; delayed profile regression ffecb6bfb808cced164f31a8db271b1dccb55a47 and c9a1a3ddab6486ddbd8cd45d3199c2f65790f02b; disproven timeout change reverted in 4c6c01e; bounded provider-push integration e2638f9958a4dfdb125e16f711e5296640349dae; Nampplets dependency e2f69f325a6b45213accdacfcc125e80e0687b4c
commands: pnpm check; pnpm lint; pnpm test; pnpm test:ui; pnpm test:conformance; pnpm fallow; pnpm format:check; pnpm docs:check; pnpm smoke:linux; strict native tauri-driver/WebKit supplied-npub and follow-click probe; screenshot inspection
observable result: the supplied npub renders yo, 435 latest-known direct follows, and its avatar. Clicking the first follow renders nopara73 and its 768069-byte JPEG through NAP-RESOURCE as a WebKit object URL. The shell remains at Two exact builds ready through NAP-SHELL and reports no runtime refusal. The true defect was base64 expansion above Nampplets default provider-push bounds after a successful resource fetch; Uzel now opts into the existing finite 104923136-byte trusted-shell aggregate ceiling through the new RuntimeConfig fields. All compile, strict lint, unit, Rust, Tauri, 24/24 renderer, conformance, Fallow, format, documentation, real Weston/WebKit, hostile-egress, and screenshot gates pass.
known limitation: Nampplets PR #11 exact-head review and CI remain a separate merge gate; Uzel deliberately pins its exact compatibility commit and records that unpublished dependency. Nampplets issue #9 still tracks a whole-operation deadline for sequential resource.bytesMany. The accepted native proof covers resource.bytes. Visible Debian interactive acceptance remains a separate human-visible gate.
next action: push and review the Uzel issue #25 branch; fix any exact-head Codex findings and merge only with clean CI. Re-run the same supplied-npub flow on Debian 13 interactive mode before closing #25. PR #11 must independently clear its Codex/CI gates before merge.
```

## Accepted provisional risks

```text
1. Upstream nampplets baseline remains unratified; Uzel owns the exact Linux-only POC adoption decision.
2. inc.channel.opened and NAP-INTENT delivery remain unsupported and unused.
3. Any pin or surface change reopens the affected Gate 0 probes. Apple evidence and Kehto merge are not Uzel blockers.
4. OS-level per-WebKit-child network isolation and trusted local TLS are post-POC hardening, not retroactive claims of this acceptance.
```
