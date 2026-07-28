# Source baseline

> Validated: 2026-07-28. The canonical machine-readable pins are in [`../compatibility.lock`](../compatibility.lock).

## Verified facts

### Napplet packages

Kehto PR #204 uses this exact released line:

```text
@napplet/core@0.29.0
@napplet/nap@0.29.0
@napplet/shim@0.27.0
@napplet/sdk@0.25.0
@napplet/vite-plugin@0.12.0
@napplet/conformance@0.14.0
@napplet/conformance-cli@0.2.16
```

The package release commit is `60889f1c2476e063500c7ab6624af6abe0dbcbe5`; npm integrity and shasum values are locked. The CLI's Playwright dependency is a range, so a reproducible check must also lock the resolved browser dependency.

Source: <https://github.com/napplet/web/releases>

### Kehto #204

PR #204 merged on 2026-07-27 at `b85db51db838866de753b275b9d34ec908785bd2`. Its source head is `59f56ce47e7eec2ec4438393f0c59b55f653cb04`.

The original merged checkout's built `chat` and `feed` bundles fail conformance-cli 0.2.16 because Vite injects a module-preload `fetch`. On current-main base `297b5478ead54508a881909e658fda0c8ee19984`, candidate commit [`jodobear/kehto-web@62241de0b4526ba4fdc8a7b3c766c2499d3ae24d`](https://github.com/jodobear/kehto-web/commit/62241de0b4526ba4fdc8a7b3c766c2499d3ae24d) sets `build.modulePreload = false`. Build 32/32, typecheck 17/17, unit 1,576/1,576, gateway artifact audit 15/15, AI-slop 100/100, and both exact conformance runs pass. The exact branch is publicly reachable. Upstream PR creation from this environment was rejected by the current GitHub token, but upstream merge is not required for Uzel's provisional exact-SHA pin.

Source: <https://github.com/kehto/web/pull/204>

### `nampplets`

Upstream base `839654cd3643b430548765823b783f0b5140b8da` contains 16 Linux-neutral Rust crates plus portable trusted-shell assets. Candidate `jodobear/nampplets@08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf` updates the evidence line to core/nap 0.29.0, shim 0.27.0, SDK 0.25.0, and conformance 0.14.0, and binds its corpus to `jodobear/kehto-web@62241de...`. Its locked workspace, Python baseline suite, trusted-shell suite, digests, and file-growth check pass on Linux with Rust 1.89.0.

The candidate deliberately remains `unratified`, advertises no M0 platform domains, and records `inc.channel.opened` as unsupported because the current NAP-INC registry requires it while released package types/conformance do not expose it. The released package's normalized intent/`intent.deliver` behavior also differs from the older registry text. Compatibility, security, and NMP-boundary signoffs are blank. Apple-specific generated catalog changes require Xcode and were not tested on Linux.

Source: <https://github.com/pablof7z/nampplets>

### NMP

Use commit `005dc2a5f12aa414961b313d05ebb021934e385c`, the exact NMP revision locked by nampplets, rather than current main. A compiling signed-fixture probe confirmed cache-only profile/direct-follow projections, explicit evidence shortfalls, cancellation, diagnostics, and shutdown through public APIs.

Source: <https://github.com/pablof7z/nmp>

### NAP registry

At `napplet/naps@5ac0490461ca6fec2f0d2e45b4835cf9bc08de24`, the registry calls `shell`, `intent`, `inc`, and `theme` active, while `relay`, `identity`, and `storage` are draft. NAP-SHELL.md and NAP-INC.md still self-label as draft. Relay PR #2 and storage PR #3 are open; exact heads are locked.

Source: <https://github.com/napplet/naps>

### NIP-5A and NIP-5D

NIP-5A at document commit `5d6b432267d4046464490b1923b96844ac4559d0` defines nsite kinds 5128/15128/35128. It is not the napplet manifest contract.

NIP-5D PR #2303 remains open at head `eb45dfd7335b7f88cb53781984c553581d2b4c34`. Its draft specifies napplet kinds 5129/15129/35129, `srcdoc`, `sandbox="allow-scripts"`, source-window binding, pre-script `window.napplet`, no `window.nostr`, and a default-deny CSP. Its root/snapshot identity wording still needs upstream clarification because those records do not carry the `dTag` used in its stated tuple.

Source: <https://github.com/nostr-protocol/nips/pull/2303>

## Decision

No branch or “latest” value is an implementation pin. The technical 0.29/conformance line and durable Kehto SHA are proven. Slice 01 remains blocked only on acceptance of nampplets candidate `08ddb87...`: named compatibility, security, and NMP-boundary review plus Apple-host evidence, or an explicit Uzel decision accepting those provisional risks.
