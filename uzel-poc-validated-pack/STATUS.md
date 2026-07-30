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
commits: 98a5c35 through 8c6c233 on feat/live-identity-ui; graph refresh 8301ea6
commands: pnpm check; pnpm lint; pnpm test; pnpm fallow; pnpm docs:check; pnpm smoke with an asserted exact live-identity test; exact ignored live naddr test; Unix-socket lost-response tests; Playwright click/fault-injection tests; pnpm smoke:linux
observable result: the supplied npub returns its canonical profile, at least 100 direct follows, and raster picture through NMP/native NAP-RESOURCE; cache-only restart retains the profile; pnpm smoke fails unless the named ignored live-identity test reports exactly one running test and its exact pass line; a signed kind 35129 naddr freezes exact review evidence, enforces required capability consent, installs, launches, and cleans up a distinct exact-build session; every post-launch exit now stops identified new sessions or closes an unprojectable runtime fail-closed; transfer plus cleanup failure crosses Tauri as structured retry state and locks identity/catalog work until idempotent cleanup succeeds; loaded stops, review cancellation, identity-change pane teardown, and renderer teardown all issue exact cleanup requests; every fresh renderer authoritatively reconciles all pre-existing daemon surfaces and pending reviews before launching panes, cancels stale hostile-sentinel state, and locks the shell behind an exact typed retry list if the second daemon snapshot still reports any item; a partial base-pane initialization failure stops every created pane and exposes the same exact retry flow; review and confirmation operation IDs survive ambiguous Unix-socket failures and the daemon replays the exact bounded response without creating a second review or session; when both bounded review or confirmation replies are lost, typed ambiguous results lock close, identity, coordinate mutation, capability grants, and conflicting catalog work until the matching Retry action sends identical frozen input; pending reviews are capped at four and ambiguous client operations at 64, while deterministic pre-send failures discard their operation IDs; Uzel exposes no Blossom product configuration or routable fallback, while ordinary public HTTPS remains governed by upstream NAP-RESOURCE policy; each control frame remains capped at 524288 bytes, while routed NAP-RESOURCE responses up to the trusted shell's 100-MiB-plus-64-KiB aggregate bound cross the same connection in validated 256-KiB chunks; delivered runtime responses are consumed once, duplicate debug payloads are dropped, and the remaining event buffer has an aggregate byte ceiling; browser failure injection and real Weston/WebKit smoke are green
known limitation: the Linux resource provider returns a typed refusal for SVG. CSP plus the tested WebKit sandbox is a malicious-JavaScript boundary, not a browser-engine exploit boundary. Nampplets PR #4 merged as 3849595 after exact-head Codex review and all eight CI jobs passed. The current Uzel pin remains d533a63d because its compatibility head also composes still-unmerged PRs #2 and #3; a new combined successor still needs exact integrated Linux validation. The merged cancellation wake path is finite, lifecycle-owned, and thread-free.
next action: merge Uzel PR #8 only after exact-head Codex review is clean; then build a compatibility successor from merged nampplets main plus tracked PRs #2/#3 and advance only after the exact identity, follows, picture, naddr, and Linux smoke probes pass
```

## Accepted provisional risks

```text
1. Upstream nampplets baseline remains unratified; Uzel owns the exact Linux-only POC adoption decision.
2. inc.channel.opened and NAP-INTENT delivery remain unsupported and unused.
3. Any pin or surface change reopens the affected Gate 0 probes. Apple evidence and Kehto merge are not Uzel blockers.
4. OS-level per-WebKit-child network isolation and trusted local TLS are post-POC hardening, not retroactive claims of this acceptance.
```
