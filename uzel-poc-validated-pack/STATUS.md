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
- [ ] 07 issue-driven product stabilization (active: #21 data-only external corpus; #19 merged)

### Work 07 parallel branch evidence — issue #21

```text
branch: test/external-napplet-corpus
foundation/provenance chain: b0ba7f9 -> 6580d9a -> f872a1c -> 0cae15a -> 0c874d9 -> dc5cf11
accepted-master integration: 75c3d80 merges f068085 without changing the issue #21 corpus lane
review/docs chain: 0022952 -> e88a46d -> 4042c69 -> f8f092d -> a35bb18 -> f24f5fc
hardening/graph chain: 27986c0 -> d80efd4 -> 8df9916 -> 61350a3 -> ed7ba54 -> 8860a6f -> d138b7a -> c379448 -> f9323bb -> 5c45fa6 -> c133560 -> 624b235 -> 1611dcb -> 97a0232 -> 8423569
current review fix: Node trust diagnostics cross the shell boundary as one exact-schema typed JSON result with escaped hostile controls; the lock and every event are opened with no-follow/nonblocking protection, canonical containment is checked from the opened descriptor, exact descriptor metadata must identify a regular file, and fixed capped reads reject evidence above 16,384 bytes even if it grows after stat; EISDIR, ENOTDIR, ELOOP, and ENXIO are invalid-evidence trust failures while EACCES and EIO remain infrastructure failures; focused regressions and this status travel in one commit; use git rev-parse HEAD after commit for its exact object ID because a commit cannot embed its own resulting SHA
exact-head commands: node --test scripts/verify-external-napplet-corpus.test.mjs; nix --extra-experimental-features 'nix-command flakes' develop -c bash scripts/verify-external-napplet-corpus.test.sh; nix --extra-experimental-features 'nix-command flakes' develop -c bash scripts/verify-external-napplet-corpus.sh; shellcheck --severity=warning scripts/verify-external-napplet-corpus.sh scripts/verify-external-napplet-corpus.test.sh; corepack pnpm docs:check; git diff --check
observable result: Node verifier tests pass, including symlinked lock, FIFO, directory/ENOTDIR evidence, pre-stat and grow-after-stat oversize evidence, exact read-error classification, and hostile LF/CR diagnostic encoding; shell classification reports EXTERNAL_NAPPLET_CORPUS_CLASSIFICATION_TEST_OK trust=2 infrastructure=3 version=pinned setup=checked node=typed-bounded jq=bounded nak=bounded transport=lossless; full pinned verifier reports STRUCTURE_OK and CORPUS_OK for four entries at source aa4dc7a0799d95e3066b50055b29685d6e376045; ShellCheck warning gate, documentation audit, and diff check pass
verified behavior: four hzrd149 coordinates retain exact signed events, author, kind, d tag, naddr/relay hints, path digest and byte length, aggregate, audited domains, server list, source license URL, audit date, publication time, and exact trust/infrastructure policy; exact flake nak 0.20.1 verifies every event hash/signature and naddr offline; an immutable verified snapshot crosses into nak; invalid Node evidence exits trust 2 only through one exact-schema JSON result whose controls remain encoded, while nak uses its typed result; lock and event paths are opened O_NOFOLLOW and O_NONBLOCK, checked for canonical containment and exact identity through /proc/self/fd, fstat-checked as regular files, and read through fixed 16,385-byte buffers that reject more than 16,384 evidence bytes without unbounded allocation; path/type errors EISDIR, ENOTDIR, ELOOP, and ENXIO are trust failures, while operational EACCES and EIO remain infrastructure failures; missing, wrong-version, timed-out, crashed, malformed-output, or untyped-exit tooling exits infrastructure 3; both temporary files are checked and cleaned if either creation fails; no artifact HTML, runtime registration, UI, pin, or dependency changed
known limitation: the lock proves retained signed evidence, not current relay/CDN availability or reproducible source-to-artifact linkage; every candidate has one external publisher; no artifact bytes or live launch are claimed in this data-only lane
next action: exact-head review and PR for #21 data-only lane; signed launch integration waits for #13, rendered resource acceptance for #17, and any nampplets pin change for #11
```

## Latest integrated evidence

```text
slice: Work 07 active; issue #10 renderer harness merged in PR #20; issue #19 initialization retry fix merged in PR #22; issue #21 data-only external corpus is the active review lane
commits: PR #20 merge 83e2e1e7e1565b6fb4ab24ac5b8dc4ecb94fbfc0; #19 activation fix 7a173dbd74787fbbf9acc01eb087f87a0e52939a; ordering/recovery hardening ab55861d86679ed38571b908f0b8a65b4951b6c7; exact reviewed head bc4eeb6296a4c9ca285a031fc5ca4dfeb219ea2f; PR #22 merge f068085b2167e4e9c5981f314de97b9b6d6d6c96
commands: pnpm check; pnpm lint; pnpm test; pnpm test:ui; pnpm fallow; pnpm format:check; pnpm docs:check; targeted initialization-empty-identity, initialization-identity-failure, and restart-reconciliation UI scenarios; screenshot inspection; pnpm smoke:linux
observable result: initialization retry no longer calls the public identity helper that is intentionally locked during recovery; it activates the read identity through a focused internal path before launching either base surface. The acceptance mock delays identity completion and rejects any premature surface launch. Empty-identity recovery passes at 1366x768 and 1920x1080; a one-shot identity-selection failure launches zero surfaces, remains locked and recoverable, then a second retry selects the identity and launches exactly two surfaces. Diagnostics refresh remains after user-driven pane restart, preventing a diagnostics failure from pairing a new identity with stale panes. Restart reconciliation waits for the selected profile rather than treating an already-ready shell as completion. The full renderer suite passes 22/22 test nodes with zero unacknowledged console, page, request, or external-network failures. Fallow reports zero issues; documentation and formatting audits pass. Real Weston/WebKit smoke reports LINUX_RUN_SMOKE_OK with daemon and shell ready, hostile egress denied, and both network and native sentinels zero.
known limitation: renderer recovery scenarios use the committed mocked-native browser lane and do not claim NMP, Unix IPC, Tauri, or WebKit behavior. The separate real Linux smoke proves the integrated Tauri/WebKit boundary but does not visually reproduce the injected initialization failures. Visible Debian interactive acceptance remains a separate human-visible gate. The current Uzel nampplets pin remains d533a63d519c14470f900323958509cdea1c6479 until #11 validates the merged-main successor.
next action: exact-head review and merge the #21 data-only corpus lane; validate and repin #11 against merged nampplets successor eef6905f4a53d46ebf45ff428dadc038a6005485; #12 implementation waits for #11 and its recorded specification authority decision.
```

## Accepted provisional risks

```text
1. Upstream nampplets baseline remains unratified; Uzel owns the exact Linux-only POC adoption decision.
2. inc.channel.opened and NAP-INTENT delivery remain unsupported and unused.
3. Any pin or surface change reopens the affected Gate 0 probes. Apple evidence and Kehto merge are not Uzel blockers.
4. OS-level per-WebKit-child network isolation and trusted local TLS are post-POC hardening, not retroactive claims of this acceptance.
```
