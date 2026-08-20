# Codebase Concerns

**Analysis Date:** 2026-08-09

**Scope:** Primary tracked Uzel sources and current Git relationships. Disposable local graph/cache trees, fixture payload bodies, archives, build output, and `jodobear-uzel-2026-08-09-nix-ci-reaudited/` are excluded from concern discovery.

## Tech Debt

**POC-sized modules combine multiple responsibilities:**
- Issue: Core state machines are concentrated in very large files: `crates/napd/src/runner.rs` is 2,083 lines, `apps/uzel/public/trusted-shell/trusted-shell.js` is 1,494 lines, `apps/uzel/src/App.svelte` is 1,419 lines, and `crates/napd-protocol/src/lib.rs` is 1,156 lines. Runtime composition, persistence, permissions, event correlation, catalog flow, and surface lifecycle share `crates/napd/src/runner.rs`; application orchestration, recovery, preferences, diagnostics, and presentation share `apps/uzel/src/App.svelte`.
- Files: `crates/napd/src/runner.rs`, `apps/uzel/public/trusted-shell/trusted-shell.js`, `apps/uzel/src/App.svelte`, `crates/napd-protocol/src/lib.rs`
- Impact: Small behavior changes cross large review surfaces and can couple otherwise independent failure paths. State-transition regressions become difficult to localize even with the extensive tests in `apps/uzel/tests/ui/acceptance.test.mjs` and the inline Rust test modules.
- Fix approach: Extract cohesive owners without changing contracts: product-state persistence and launch reconciliation from `crates/napd/src/runner.rs`, catalog/recovery state from `apps/uzel/src/App.svelte`, and IPC framing/client concerns from `crates/napd-protocol/src/lib.rs`. Keep the pinned trusted-shell bytes in `apps/uzel/public/trusted-shell/` unchanged until the owning upstream source is repinned and revalidated.

**Internal contracts remain explicitly POC-scoped:**
- Issue: Protocol version `0`, one shell client, one public read identity, a small fixed composition, and one trusted WebView containing sandboxed iframes are recorded shortcuts rather than stable platform contracts.
- Files: `crates/napd-protocol/src/lib.rs`, `crates/napd/src/runner.rs`, `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js`, `uzel-poc-validated-pack/docs/06-extraction.md`
- Impact: New consumers, concurrent clients, identity switching models, or multi-WebView isolation cannot be added by treating current types and limits as a durable public API.
- Fix approach: Keep `crates/napd-protocol` private while version `0` is in use. Define lifecycle, compatibility, and migration behavior only when a second consumer proves the required seam, following `uzel-poc-validated-pack/docs/06-extraction.md`.

**Product-state format has no migration path:**
- Issue: `PRODUCT_STATE_VERSION` is `0`; startup refuses any other version. The persisted `mode` is validated as a known string but is not compared with the requested runtime mode. No upgrade or downgrade migration exists.
- Files: `crates/napd/src/runner.rs`
- Impact: Any state shape change can make an installed runtime fail at startup. The unused persisted-mode distinction can also mislead future code into assuming a mode-consistency check exists.
- Fix approach: Before changing `ProductState`, add explicit versioned decoding and migration tests. Either enforce mode consistency or remove the persisted field and document why mode is intentionally runtime-selected.

**Generated and signed assets require manual coordinated repins:**
- Issue: Four trusted-shell files, three signed napplet fixtures, Rust fixture coordinates, digest checks, and the compatibility ledger must move together. `scripts/check-pinned-assets.sh` verifies hashes but does not establish that copied trusted-shell bytes still match their upstream source commit.
- Files: `apps/uzel/public/trusted-shell/trusted-shell.js`, `apps/uzel/public/trusted-shell/trusted-shell-policy.js`, `apps/uzel/public/trusted-shell/trusted-shell-prelude-domains.js`, `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js`, `fixtures/`, `crates/napd/src/fixtures.rs`, `scripts/build-signed-napplet-fixtures.sh`, `scripts/check-pinned-assets.sh`, `uzel-poc-validated-pack/compatibility.lock`
- Impact: A partial repin can leave valid hashes attached to mutually incompatible runtime, shell, and fixture bytes. Reviewing the 1,494-line injected compatibility layer locally is expensive.
- Fix approach: Add one deterministic repin command that fetches from the recorded exact commit, verifies provenance, rebuilds fixtures, updates all digests, and prints the required validation matrix. Never hand-edit copied trusted-shell assets.

**Durable status and implementation state disagree:**
- Issue: `uzel-poc-validated-pack/STATUS.md` still says PR #30 must be pushed, reviewed, and merged, while the tracked checkout is already merge commit `19519c3` for PR #30 on `master`. `uzel-poc-validated-pack/work/07-stabilize-product.md` names issue #19 as active, while `uzel-poc-validated-pack/STATUS.md` names issues #28 and #29. Historical documents also describe `e539378...` as the current runtime even though `Cargo.toml` and `Cargo.lock` pin `e2f69f...`.
- Files: `uzel-poc-validated-pack/STATUS.md`, `uzel-poc-validated-pack/work/07-stabilize-product.md`, `uzel-poc-validated-pack/docs/06-extraction.md`, `uzel-poc-validated-pack/reports/slice-06-preflight.md`, `Cargo.toml`, `Cargo.lock`
- Impact: Agents following the mandated status/work flow can repeat completed Git operations, select the wrong slice, or plan hardening against an obsolete dependency pin.
- Fix approach: Make `STATUS.md` the current-state authority, clearly label historical preflight reports as snapshots, and update Work 07 or close it with the exact merged head and remaining human acceptance gate.

## Known Bugs

**Failed product-state writes can wedge persistence for the process lifetime:**
- Symptoms: A write or `sync_all` failure before rename leaves `uzel-state.json.<pid>.tmp` behind. Every later persistence attempt in the same daemon uses the same PID-derived path with `create_new(true)` and fails with `AlreadyExists`.
- Files: `crates/napd/src/runner.rs`
- Trigger: Any partial write, file-sync error, injected I/O failure, or process path collision after the temporary file is created but before `fs::rename` completes.
- Workaround: Restart after removing the exact stale temporary file from the selected runtime root. Permanent fix: guard the temporary path and unlink it on every pre-rename error, or use a securely randomized same-directory temporary file with atomic rename.

**Current rich-profile bytes lack visible Debian acceptance:**
- Symptoms: Deterministic Chromium and real headless Weston/WebKit checks pass, but the checked-in status records the exact signed rich-profile fixture hashes as pending a visible Debian 13 interactive run with the supplied public identity.
- Files: `uzel-poc-validated-pack/STATUS.md`, `uzel-poc-validated-pack/compatibility.lock`, `DEBIAN13-LIVE-TEST.md`, `scripts/debian13-live-test.sh`
- Trigger: Treating headless smoke or mocked renderer evidence as proof of the user-visible profile/follow interaction on Debian 13.
- Workaround: Run the interactive checklist in `DEBIAN13-LIVE-TEST.md` against the exact checkout before closing the product issues; preserve its evidence separately from automated headless results.

**SVG profile resources are refused:**
- Symptoms: The Linux resource rasterizer returns a typed refusal for SVG images; current native evidence covers JPEG profile images only.
- Files: `crates/napd/src/resource.rs`, `uzel-poc-validated-pack/docs/facts/FACT-014-live-identity-catalog.md`
- Trigger: A profile picture URL resolves to SVG rather than the tested raster formats.
- Workaround: Keep the visible fallback avatar. Add bounded SVG handling only in the NAP-RESOURCE owner with hostile-content tests; do not let napplets fetch or render remote SVG directly.

## Security Considerations

**Untrusted WebKit children lack an OS-level network sandbox:**
- Risk: The accepted boundary relies on iframe sandboxing, injected CSP, source binding, and Tauri isolation. A browser-engine exploit is outside that boundary and could escape JavaScript-level network denial.
- Files: `apps/uzel/public/trusted-shell/trusted-shell-policy.js`, `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js`, `apps/uzel/src-tauri/src/main.rs`, `uzel-poc-validated-pack/docs/06-extraction.md`, `uzel-poc-validated-pack/reports/slice-06-preflight.md`
- Current mitigation: Frames use `sandbox="allow-scripts"` without `allow-same-origin`; CSP sets `connect-src 'none'`, `form-action 'none'`, `worker-src 'none'`, and `object-src 'none'`; `MessageEvent.source` is mapped to the host-created frame; the hostile real-WebKit probe exercises 13 egress paths.
- Recommendations: Design a per-WebKit-child process policy that removes child network authority without severing trusted shell loopback access. Preserve `scripts/linux-run-smoke.sh` and `fixtures/hostile-egress/` as regression evidence. Do not claim browser-engine exploit isolation until that layer exists.

**Trusted local transport lacks TLS:**
- Risk: The POC cannot advance to the newer runtime behavior that correctly refuses plaintext local relay endpoints. Keeping plaintext loopback transport prolongs the exact old compatibility boundary.
- Files: `scripts/dev.sh`, `uzel-poc-validated-pack/compatibility.lock`, `uzel-poc-validated-pack/docs/06-extraction.md`, `uzel-poc-validated-pack/reports/slice-06-preflight.md`
- Current mitigation: Production-facing relay defaults use `wss://`; plaintext is limited to deterministic loopback validation and explicitly allowlisted local hosts in test paths.
- Recommendations: Terminate the deterministic local relay with trusted local TLS, repin the runtime only after that is proven, and rerun every affected Gate 0, hostile-boundary, live-NMP, and WebKit test.

**Read identity is context, not authentication:**
- Risk: The selected Nostr public key has no signer proof and must not authorize account-specific writes, secrets, or privileged actions.
- Files: `crates/napd/src/runner.rs`, `apps/uzel/src/App.svelte`, `uzel-poc-validated-pack/AGENTS.md`
- Current mitigation: `LinuxRunner::set_read_identity` registers a read-only account, and the POC exposes no wallet, signer, or authenticated write path.
- Recommendations: Keep UI and protocol language explicit about read context. Any future authenticated feature must introduce signer-backed proof through the applicable Nostr/NAP authority rather than reusing `active_read_identity` as login state.

**Private socket authenticates only by filesystem ownership:**
- Risk: Any process running as the same OS user can connect to `napd.sock`, stop surfaces, change the read identity, inspect diagnostics, or request daemon shutdown. There is no per-client handshake or peer-credential authorization.
- Files: `crates/napd/src/server.rs`, `crates/napd-protocol/src/lib.rs`, `apps/uzel-napd/src/main.rs`
- Current mitigation: The socket parent must be a real private directory with no group/other permissions, the socket is mode `0600`, stale-socket replacement is device/inode checked, and no shared `/tmp` fallback exists.
- Recommendations: Preserve the one-user/one-client assumption while the protocol is private. Before supporting multiple clients or privileged methods, validate peer credentials and add a client/session capability model.

## Performance Bottlenecks

**Single-threaded daemon causes head-of-line blocking:**
- Problem: `DaemonServer::serve` accepts and completes one connection at a time. `LinuxRunner::forward_from_surface` can wait up to 15 seconds for a provider response; an incomplete IPC client can hold the server for the 2-second stream timeout. Status, stop, identity, and cleanup requests queue behind that work.
- Files: `crates/napd/src/server.rs`, `crates/napd/src/runner.rs`, `crates/napd-protocol/src/lib.rs`, `uzel-poc-validated-pack/work/07-stabilize-product.md`
- Cause: One mutable `DaemonState` and one blocking accept/request loop serialize all control and data-plane work. Work 07 explicitly lists daemon control responsiveness as a separate issue.
- Improvement path: Benchmark stop/status latency during a stalled envelope request. Separate connection I/O from ordered state mutation, add bounded worker admission, and preserve exact replay and surface-order guarantees. Do not add unbounded per-connection threads.

**Resource delivery permits high transient memory use:**
- Problem: Routed responses may reach 104,923,136 bytes, the event buffer ceiling is 104,988,672 bytes, and the trusted shell permits a 50 MiB decoded Blob. Server, client, JSON/base64 transport, UTF-8 envelope reconstruction, renderer parsing, and Blob decoding can coexist transiently.
- Files: `crates/napd-protocol/src/lib.rs`, `crates/napd/src/runner.rs`, `crates/napd/src/server.rs`, `apps/uzel/public/trusted-shell/trusted-shell.js`, `uzel-poc-validated-pack/compatibility.lock`
- Cause: Large NAP-RESOURCE results are base64-wrapped, chunked only across IPC frames, then reassembled into one string before trusted-shell projection and binary decoding.
- Improvement path: Measure peak RSS with maximum-sized single and concurrent resources. Prefer streaming or file-descriptor-backed verified resource delivery at the trusted native boundary, and lower product-specific ceilings if real profile media does not require the current maximum.

**`resource.bytesMany` lacks a whole-operation deadline:**
- Problem: Each individual resource is bounded, but a sequential bulk request has no accepted whole-operation time bound.
- Files: `apps/uzel/public/trusted-shell/trusted-shell-prelude-domains.js`, `apps/uzel/public/trusted-shell/trusted-shell.js`, `uzel-poc-validated-pack/compatibility.lock`, `uzel-poc-validated-pack/docs/facts/FACT-014-live-identity-catalog.md`
- Cause: The upstream provider contract bounds item count and per-item work but not total elapsed time across the sequence.
- Improvement path: Resolve the upstream Nampplets issue with an operation deadline/cancellation contract, repin the exact dependency, and add timeout tests covering partial completion and cleanup.

## Fragile Areas

**Trusted-shell materialization and source binding:**
- Files: `apps/uzel/public/trusted-shell/trusted-shell.js`, `apps/uzel/public/trusted-shell/trusted-shell-policy.js`, `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js`, `apps/uzel/src-tauri/src/main.rs`
- Why fragile: Correctness depends on inert HTML parsing, CSP and `<base>` insertion preceding authored code, an opaque sandbox origin, wildcard `postMessage` paired with exact `event.source` mapping, and navigation denial. Small ordering or sandbox-token changes alter the security boundary.
- Safe modification: Change only through the upstream trusted-shell owner, pin exact bytes, update all four asset hashes, and rerun hostile iframe/network, raw Tauri dispatch, navigation, conformance, and real WebKit gates.
- Test coverage: Real hostile WebKit evidence exists in `scripts/linux-run-smoke.sh`, while the deterministic renderer suite in `apps/uzel/tests/ui/acceptance.test.mjs` uses Chromium and a mocked native boundary. Neither proves browser-engine exploit containment.

**Runtime response correlation and event buffering:**
- Files: `crates/napd/src/runner.rs`
- Why fragile: Responses share an observer buffer with unsolicited runtime pushes. Correlation uses request IDs, special cases `shell.ready` and cross-surface `inc.emit`, and falls back to accepting any later response from the source session for other shapes.
- Safe modification: Require a documented expectation for every newly forwarded envelope type. Add a regression where unrelated identity/resource pushes arrive before the expected result; keep matched responses consumed exactly once and preserve byte/count eviction.
- Test coverage: `crates/napd/src/runner.rs` tests `inc.emit` against an unrelated push and exercises live identity paths, but the two public-relay/resource tests are ignored by the default Rust test command.

**Catalog review, confirmation, replay, and cleanup state:**
- Files: `crates/napd/src/runner.rs`, `crates/napd/src/server.rs`, `crates/napd-protocol/src/lib.rs`, `apps/uzel/src/App.svelte`, `apps/uzel/tests/ui/acceptance.test.mjs`
- Why fragile: Ambiguous delivery preserves operation IDs, confirmation freezes reviewed identity/hash/grants, asset transfer failure may require a separate surface cleanup, and renderer controls remain locked until authoritative reconciliation.
- Safe modification: Preserve byte-identical replay inputs and idempotent stop/cancel behavior. Test response loss before and after send, partial base-pane launch, cleanup failure, process restart, and replay-cache eviction together.
- Test coverage: Mocked fault-injection coverage is strong in `apps/uzel/tests/ui/acceptance.test.mjs`; native concurrency and crash/restart coverage remains limited by the single-process, serial server model in `crates/napd/src/server.rs`.

**State persistence and identity rollback:**
- Files: `crates/napd/src/runner.rs`
- Why fragile: Identity activation occurs in upstream runtime state, then a separate JSON record is atomically replaced and parent-synced. Pre-replacement and post-replacement failures deliberately require different rollback behavior.
- Safe modification: Preserve mode `0600`, refusal of symlink/non-regular state files, same-directory atomic rename, file and directory sync, and exact previous-account rollback. Add cleanup for failed temporary writes before refactoring this path.
- Test coverage: Existing tests cover symlink refusal, occupied temporary path, rollback, restart, and generation persistence in `crates/napd/src/runner.rs`; injected partial-write and file-sync failure cleanup are not covered.

## Scaling Limits

**Surface and catalog capacity:**
- Current capacity: At most 4 runtime surfaces in `crates/napd/src/fixtures.rs`, 16 host iframe slots in `apps/uzel/public/trusted-shell/trusted-shell-surface-host.js`, 4 pending reviews in `crates/napd/src/runner.rs`, and 64 ambiguous client operations/replay entries in `crates/napd-protocol/src/lib.rs` and `crates/napd/src/server.rs`.
- Limit: The daemon surface cap is reached before the host cap. The fixed two-pane product plus one catalog napplet and hostile/dev surface nearly consumes the POC composition budget.
- Scaling path: Define product-visible admission and eviction rules before raising limits. Keep daemon, host, renderer, replay, and test limits synchronized and bounded.

**Follow/profile presentation:**
- Current capacity: 1,024 rendered follows, 2 concurrent profile queries, 32 adaptive retry queries per refresh, 4 concurrent avatar requests, and 32 retained avatar object URLs.
- Limit: Large follow sets stop at the renderer cap; retries and image work remain bounded but can delay enrichment for later visible rows.
- Scaling path: Preserve NMP as the only data/cache owner. Add virtualized presentation and priority scheduling in `napplets/follow-list/src/main.js` and `napplets/follow-list/src/model.js` rather than introducing a second profile/follow cache.

**Platform and distribution:**
- Current capacity: `flake.nix` defines only `x86_64-linux`; `apps/uzel/src-tauri/tauri.conf.json` sets `bundle.active` to `false`; daemon startup is manually composed by `scripts/dev.sh`.
- Limit: There is no installable application bundle, daemon service/supervision contract, ARM Linux shell, or supported non-Linux target.
- Scaling path: Complete Linux packaging and daemon lifecycle first. Add other architectures only with pinned WebKit/Tauri smoke evidence; do not broaden to excluded platforms from the POC code path without a separate phase.

## Dependencies at Risk

**Fork-pinned Nampplets runtime:**
- Risk: Four critical Rust dependencies pin the same `jodobear/nampplets` SHA `e2f69f...`, while the reviewed trusted-shell bytes come from the separate `jodobear/nampplets` compatibility line. The runtime contract also remains tied to provisional NAP revisions.
- Impact: Upstream divergence, review-modified successors, or a runtime repin can change permissions, resource bounds, NMP behavior, trusted-shell compatibility, and native acceptance together.
- Migration plan: Keep both exact pins until one separately authorized `jodobear/nampplets` candidate proves provider-push limits, current runtime fixes, and the reviewed trusted-shell invariants together. Revalidate source, conformance, signed fixtures, live NMP, and real WebKit before changing `Cargo.toml`, `Cargo.lock`, or `uzel-poc-validated-pack/compatibility.lock`. `pablof7z/nampplets` is historical read-only provenance, never a publication or port target.
- Files: `Cargo.toml`, `Cargo.lock`, `uzel-poc-validated-pack/docs/08-upstream-contributions.md`, `uzel-poc-validated-pack/compatibility.lock`

**Provisional NAP/NIP-5D contract line:**
- Risk: Registry documents and released package behavior are not fully aligned; `inc.channel.opened` and NAP-INTENT delivery remain unsupported, and NIP-5D remains provisional.
- Impact: Treating draft or registry text as implemented can open unsupported capabilities or create a competing translation/runtime layer.
- Migration plan: Keep unsupported surfaces closed. Repin only against exact released packages and source evidence, then rerun conformance and trust-boundary probes.
- Files: `uzel-poc-validated-pack/docs/01-validation.md`, `uzel-poc-validated-pack/docs/07-source-baseline.md`, `uzel-poc-validated-pack/compatibility.lock`, `uzel-poc-validated-pack/AGENTS.md`

## Missing Critical Features

**Production packaging and daemon supervision:**
- Problem: Tauri bundling is disabled, and only `scripts/dev.sh` starts/stops `uzel-napd` alongside the shell. No installed service, activation contract, upgrade path, or crash-restart policy exists.
- Blocks: Reproducible end-user installation, reliable desktop startup, crash recovery, and release distribution.
- Files: `apps/uzel/src-tauri/tauri.conf.json`, `scripts/dev.sh`, `apps/uzel-napd/src/main.rs`, `Containerfile.debian`

**Per-child OS isolation and trusted local TLS:**
- Problem: Both are explicitly deferred hardening items, not properties of the accepted POC.
- Blocks: A production-grade browser-engine containment claim and migration away from the plaintext-loopback-compatible runtime baseline.
- Files: `uzel-poc-validated-pack/docs/06-extraction.md`, `uzel-poc-validated-pack/reports/slice-06-preflight.md`, `uzel-poc-validated-pack/STATUS.md`

**Automated repository CI:**
- Problem: No tracked `.github/workflows/` pipeline runs the extensive commands defined in `package.json`; validation depends on local/manual execution and recorded evidence.
- Blocks: Automatic enforcement of formatting, Clippy, JavaScript/Rust tests, conformance, boundary checks, pinned assets, renderer tests, and platform smoke on every change.
- Files: `package.json`, `README.md`, `scripts/linux-run-smoke.sh`, `scripts/debian-build-smoke.sh`

## Test Coverage Gaps

**Public relay, catalog, and profile-resource paths:**
- What's not tested: The live `naddr` review/install flow and public identity/profile/follows/HTTPS image path are marked `#[ignore]` and do not run in the default Rust suite. `scripts/smoke.sh` runs only the public identity test explicitly.
- Files: `crates/napd/src/runner.rs`, `scripts/smoke.sh`, `package.json`
- Risk: Relay behavior, remote artifact availability, HTTPS image behavior, and upstream service changes can regress while deterministic tests remain green.
- Priority: High

**Visible Debian interaction:**
- What's not tested: Exact current rich-profile fixture bytes have no recorded completed visible Debian 13 interactive checklist.
- Files: `DEBIAN13-LIVE-TEST.md`, `scripts/debian13-live-test.sh`, `uzel-poc-validated-pack/STATUS.md`, `uzel-poc-validated-pack/compatibility.lock`
- Risk: WebKit rendering, focus, resize, keybinding, profile selection, and user-perceived behavior can fail despite headless markers.
- Priority: High

**Daemon concurrency and responsiveness:**
- What's not tested: Sustained concurrent clients, control requests during a 15-second provider wait, shutdown during chunked transfer, and bounded overload behavior under multiple live surfaces.
- Files: `crates/napd/src/server.rs`, `crates/napd/src/runner.rs`, `crates/napd-protocol/src/lib.rs`
- Risk: Head-of-line stalls and cleanup delays can appear only under real contention.
- Priority: High

**Resource maximums and process memory:**
- What's not tested: End-to-end peak memory and latency at the 50 MiB Blob and approximately 100 MiB routed-envelope ceilings, including concurrent avatar/resource responses.
- Files: `apps/uzel/public/trusted-shell/trusted-shell.js`, `crates/napd/src/runner.rs`, `crates/napd/src/server.rs`, `crates/napd-protocol/src/lib.rs`
- Risk: A formally bounded request can still exhaust practical desktop memory or trigger long UI stalls.
- Priority: Medium

**Coverage measurement:**
- What's not tested: No line/branch coverage tool or minimum threshold is configured for Rust, JavaScript, TypeScript, or Svelte.
- Files: `package.json`, `apps/uzel/package.json`, `Cargo.toml`
- Risk: Large inline test suites can pass while important branches in `apps/uzel/src/App.svelte`, `crates/napd/src/runner.rs`, and the trusted-shell projection remain unexecuted.
- Priority: Medium

---

*Concerns audit: 2026-08-09*
