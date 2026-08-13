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
- [ ] 07 issue-driven product stabilization (active: #28 rich follow rows and #29 complete kind-0 profile card)

## Latest integrated evidence

```text
slice: Work 07 active; issues #28 and #29 are implemented on feat/rich-profile-surfaces in PR #30
commits: initial rich rendering 6384de4; renderer proof d21a0da; first-round review corrections 9f2532d and f456e53; second-round retry/avatar/incomplete correction 4895aae; overflow/queue corrections 2595c80 and 6f9a02e; failure-bound correction 8cd6357; final signed fixture repins 4c46674, f829456, 9f59d0d, and 135f49d; artifact UUID normalization 109782b
commands: pinned-Nix `pnpm check`, `pnpm test`, `pnpm test:ui`, `pnpm test:conformance`, `pnpm lint`, `pnpm fallow`, and `pnpm format:check`; documentation audit; pinned asset audit; real Weston/WebKit `pnpm smoke:linux`; routed-profile screenshot inspection
observable result: each follow row immediately renders a short pubkey fallback, then shows its NMP-resolved profile name, friendly accessible label with disambiguating pubkey, and viewport-visible NAP-RESOURCE picture. Initial NAP-OUTBOX batches contain at most eight author-bound filters; errored or incomplete partial results preserve validated rows and retry only unresolved authors. A correlated result rejected by the trusted shell's 64-KiB projection becomes a bounded incomplete terminal result, so adaptive splitting isolates the oversized batch instead of opening the transport-outage circuit. At most 32 adaptive retries can be created per refresh, and an actual transport failure opens a circuit that rejects queued work. At most two profile queries and four image fetches run concurrently; viewport exit cancels matching queued work and aborts an active per-row NAP-RESOURCE request, failed visible requests stay quiescent until an exit/re-entry transition, pending object URLs are capped at 32, successful URLs are revoked after decode, and decoded image sources are removed on exit then reloaded on re-entry. A failed follows reload removes stale rows after disconnecting their observer. Active and clicked profile cards show friendly fields plus the exact authored complete kind-0 JSON as inert, wrapped text outside the concise live-status region; error-only provider results retain the full profile but visibly qualify and diagnose its evidence. Root checks built all three napplets with Svelte 0/0, 37 JavaScript and 55 Rust tests passed with only two explicit live-network ignores, conformance passed 12/0/8, Fallow found zero issues, documentation reported 47 documents/78 links/9 Mermaid blocks with zero errors or warnings, Playwright passed 34/34, and real Weston/WebKit reported every daemon/shell/artifact/isolation marker green. The routed-profile screenshot was inspected.
known limitation: the newly signed fixture bytes still require visible Debian 13 interactive acceptance with the supplied npub. Nampplets issue #9 tracks a whole-operation bound for sequential resource.bytesMany; current proof covers individual resource.bytes. Uzel deliberately remains pinned to exact NMP, Nampplets, and NAP revisions.
next action: push the corrected exact head, request a fresh head-anchored Codex review, merge PR #30 only when that review is clean, then run Debian 13 interactive acceptance before closing #28 and #29.
```

## Preserved evidence maintenance — 2026-08-13

```text
scope: process reset issue #37 / PR #38; no product behavior, fixture payload, dependency, pin, or compatibility change
commits: d98d340 makes documentation validation read-only and adds locked maintenance entrypoints; 260817d deterministically normalizes canonical Graphify metadata; the trailing graph-only commit refreshes source locations after this status note
commands: locked pnpm test:maintenance and pnpm docs:check; explicit pnpm docs:evidence; locked pnpm graphify:refresh
evidence: relocated docs:check leaves a clean tracked tree and emits root "."; two unchanged graph refreshes leave no cache or manifest timestamp diff; Graph Report routes back through the locked wrapper
limitation: Graphify 0.9.13 still creates absolute machine-local cache keys internally, so Uzel purges and ignores that cache; issue #40 tracks the upstream fix
next action: obtain exact-head Codex and substantive CodeRabbit approval, run final affected validation, then merge/close the reset and start Phase 1
```

## Accepted provisional risks

```text
1. Upstream nampplets baseline remains unratified; Uzel owns the exact Linux-only POC adoption decision.
2. inc.channel.opened and NAP-INTENT delivery remain unsupported and unused.
3. Any pin or surface change reopens the affected Gate 0 probes. Apple evidence and Kehto merge are not Uzel blockers.
4. OS-level per-WebKit-child network isolation and trusted local TLS are post-POC hardening, not retroactive claims of this acceptance.
```
