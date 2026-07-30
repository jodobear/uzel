# Tests, quality gates, and demo

## Required test layers

### Runtime/Rust

- exact-build verification success/failure;
- principal/session source binding;
- local protocol bounds and malformed input;
- KV namespace isolation and restart;
- NMP fixture ingest, deduplication, replaceable profile selection, evidence;
- observer/session cancellation;
- no Uzel dependency from reusable crates.

### Napplet/web

- pinned NAP-SHELL initialization;
- exact NAP-INC convention behavior;
- malformed `profile/open` payload rejection;
- no direct dependency between the two napplets;
- accessible keyboard/focus smoke;
- Fallow changed-code gate after its actual version/config is validated.

### Hostile frame

Gate 0 executed the minimum trust spike for Tauri globals, raw invalid-key IPC, source binding, fetch, XHR, WebSocket, and image loading. The synthetic frame passed. The original Kehto artifacts failed because generated HTML contained `fetch`; exact candidate `62241de...`, later merged by Kehto as `4fd4aff...`, disables module preload and passes released conformance without weakening CSP. Work 03 added a signed, exact-build `hostile-egress` fixture. Work 06 rebuilt and re-signed that fixture, committed a unique live sentinel URL through exact-principal/exact-session NAP-CONFIG before mount, and executed the complete suite in real Fedora WebKit.

The test napplet attempts:

```text
Tauri/native access
fetch and XHR
WebSocket and EventSource
image/media/iframe loads
worker/service worker
beacon/form/navigation/popup
forged principal/session/sender fields
oversized/malformed envelopes
```

Observed result: all 13 browser-egress capabilities were attempted. `sendBeacon()` may report that the browser queued a request; queue acceptance is not transport proof. The independent, control-proven sentinel accepted zero hostile connections after the settle period. The raw WebKit handler observed the forged invalid invoke key, while the application command counter remained zero. The final report arrived from the exact hostile frame source. Valid NAP-SHELL and source-bound NAP-CONFIG traffic still worked, and user mode exposed neither diagnostics nor unsafe fixture controls.

This proves the POC projection against malicious JavaScript. It is not a browser-engine exploit proof.

## Quality commands

Slice 00 records the actual commands. Expected categories:

```text
cargo fmt --check
cargo clippy -- -D warnings
cargo test
frontend format/typecheck/test
fallow audit against explicit base
napplet conformance fixtures
document link/structure audit
```

The exact accepted commands and their outputs are recorded in
`reports/slice-06-preflight.md`.

## Deterministic demo

1. Start with no public relay dependency.
2. Launch daemon and shell.
3. Select the fixture identity.
4. Verify both exact builds and open the split layout.
5. Show direct follows from signed NMP fixture events.
6. Select one follow; show NAP-INC delivery and profile-card update.
7. Open developer mode; show sessions, envelopes, one NMP engine, evidence and cache state.
8. Restart; show persisted identity/layout/KV and deterministic NMP state.
9. Run `UZEL_RUN_HOSTILE_PROBE=1 pnpm smoke:fedora`; display the exact-build,
   control-sentinel, 13-probe, zero-accept, zero-native-call, source-bound, and
   user-mode verdicts.

The accepted clean-checkout reproduction uses a detached worktree, locked
`pnpm install`, the pinned Nix shell, `pnpm smoke`, and `pnpm smoke:fedora`.
The immutable-digest Bookworm check is `bash scripts/debian-build-smoke.sh`.

## Live demo

1. Configure a small relay bootstrap set.
2. Enter a public user key.
3. Show latest-known direct follows and profile reload through NMP-backed NAP providers.
4. Select an author and observe kind `0` updates.
5. Disconnect networking; cached data remains with a degraded status.

## Final acceptance

The POC passes when:

- the demo-complete and foundation-complete criteria in `00-scope.md` pass from a clean checkout;
- no undocumented manual source edits are needed;
- all known shortcuts are recorded in `06-extraction.md`;
- a new developer can follow the README and reproduce the deterministic demo.

Result on 2026-07-29: **PASS for the Linux-only POC**. The optional public-relay
demo is not part of deterministic acceptance and was not used to reinterpret a
local failure. See `reports/slice-06-preflight.md`.
