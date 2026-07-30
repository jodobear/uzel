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

## Latest integrated evidence

```text
slice: accepted live-identity and signed-naddr extension
commits: 98a5c35 through 39dee68 on feat/live-identity-ui
commands: pnpm check; pnpm lint; pnpm test; pnpm fallow; pnpm docs:check; exact ignored live identity and naddr tests; Playwright click/fault-injection tests; pnpm smoke:linux
observable result: the supplied npub returns its canonical profile, at least 100 direct follows, and raster picture through NMP/native NAP-RESOURCE; cache-only restart retains the profile; a signed kind 35129 naddr freezes exact review evidence, enforces required capability consent, installs, launches, and cleans up a distinct exact-build session; cleanup failure keeps the session tracked and blocks identity/catalog changes until retry succeeds; real Weston/WebKit reports all runtime, source-binding, hostile-egress, and user-mode markers green
known limitation: the Linux resource provider returns a typed refusal for SVG. CSP plus the tested WebKit sandbox is a malicious-JavaScript boundary, not a browser-engine exploit boundary. The current Uzel pin remains d533a63d while reviewed nampplets PR #4 successor b5a036e awaits fresh review and integrated repin validation.
next action: merge Uzel PR #8 only after exact-head Codex review is clean; repin nampplets PR #4 only after its fresh review and the exact live Linux probes pass again
```

## Accepted provisional risks

```text
1. Upstream nampplets baseline remains unratified; Uzel owns the exact Linux-only POC adoption decision.
2. inc.channel.opened and NAP-INTENT delivery remain unsupported and unused.
3. Any pin or surface change reopens the affected Gate 0 probes. Apple evidence and Kehto merge are not Uzel blockers.
4. OS-level per-WebKit-child network isolation and trusted local TLS are post-POC hardening, not retroactive claims of this acceptance.
```
