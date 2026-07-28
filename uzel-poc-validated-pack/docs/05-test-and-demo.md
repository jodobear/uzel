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

Expected result: no host/native authority; network-bearing attempts fail under the tested policy; forged identity is ignored; valid NAP traffic still works.

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

Do not copy commands from this document into CI until they run in the pinned environment.

## Deterministic demo

1. Start with no public relay dependency.
2. Launch daemon and shell.
3. Select the fixture identity.
4. Verify both exact builds and open the split layout.
5. Show direct follows from signed NMP fixture events.
6. Select one follow; show NAP-INC delivery and profile-card update.
7. Open developer mode; show sessions, envelopes, one NMP engine, evidence and cache state.
8. Restart; show persisted identity/layout/KV and deterministic NMP state.
9. Run hostile fixture and display the test verdict.

## Live demo

1. Configure a small relay bootstrap set.
2. Enter a public user key.
3. Show cache-first direct follows and lazy profile refresh.
4. Select an author and observe kind `0` updates.
5. Disconnect networking; cached data remains with a degraded status.

## Final acceptance

The POC passes when:

- the demo-complete and foundation-complete criteria in `00-scope.md` pass from a clean checkout;
- no undocumented manual source edits are needed;
- all known shortcuts are recorded in `06-extraction.md`;
- a new developer can follow the README and reproduce the deterministic demo.
