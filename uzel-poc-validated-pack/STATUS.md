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
slice: 06 hardening and clean demo acceptance
commits: recorded in repository history
commands: reports/slice-06-preflight.md
observable result: a signed exact-build hostile child ran 13 browser egress probes in real Fedora WebKit; a separately control-proven loopback sentinel accepted zero hostile connections; forged raw Tauri transport reached WebKit but executed zero commands; result attribution stayed bound to the exact child source; user mode hid diagnostics and unsafe controls; deterministic smoke passed from a detached clean checkout; pinned Debian Bookworm built the complete locked workspace
known limitation: CSP plus the tested WebKit sandbox is a malicious-JavaScript boundary, not a browser-engine exploit boundary. Bubblewrap 0.11.0 cannot isolate only the hostile WebKit child without a separately designed subprocess policy; whole-process network unsharing would also break trusted loopback/NMP traffic. The Fedora Nix environment's mixed workspace test link can combine Nix WebKitGTK with the host linker and request GLIBC_2.42; isolated app tests and the Debian workspace build pass.
next action: merge Slice 06 after exact-head review; then treat the POC as accepted and begin only the extraction choices in docs/06-extraction.md
```

## Accepted provisional risks

```text
1. Upstream nampplets baseline remains unratified; Uzel owns the exact Linux-only POC adoption decision.
2. inc.channel.opened and NAP-INTENT delivery remain unsupported and unused.
3. Any pin or surface change reopens the affected Gate 0 probes. Apple evidence and Kehto merge are not Uzel blockers.
4. OS-level per-WebKit-child network isolation and trusted local TLS are post-POC hardening, not retroactive claims of this acceptance.
```
