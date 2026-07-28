# nampplets Linux reuse map

## Result

`nampplets@839654cd3643b430548765823b783f0b5140b8da` is reusable on Linux. Its entire locked Rust workspace passed formatting, tests, and warning-denying Clippy with Rust 1.89.0. The missing piece is a Linux WebKit host; it is not a missing runtime.

## Crate map

| Crate | Linux result | Uzel use |
|---|---|---|
| `nmp-native-artifact` | pass | reuse manifest, file-hash, aggregate-hash, and exact-coordinate verification |
| `nmp-native-catalog-resolver` | pass | defer; no catalog in the POC |
| `nmp-native-nap-bridge` | pass | reuse bounded envelope validation and runtime-owned session context |
| `nmp-native-nmp-adapter` | pass | reuse as the sole NMP and NAP-Nostr boundary |
| `nmp-native-provider-identity` | pass | reuse the pinned identity projection |
| `nmp-native-provider-inc` | pass | reuse topics/channels and runtime-attested senders |
| `nmp-native-provider-link` | pass | defer unless required by an accepted fixture |
| `nmp-native-provider-resource` | pass | reuse only for runtime-mediated external bytes; never grant raw network |
| `nmp-native-providers` | pass | reuse shell, relay, storage, config, theme, and provider composition |
| `nmp-native-runtime-core` | pass | reuse principals, grants, sessions, cancellation, and public identity |
| `nmp-native-runtime-store` | pass | reuse installed-build, runtime, and app-KV persistence; it does not own Nostr truth |
| `nmp-native-runtime-app` | pass | reuse the Rust composition kernel and bounded observable snapshots |
| `nmp-native-runtime-ffi` | pass | drive the runtime through `RuntimeController`; UniFFI is not an Apple-only API |
| `nmp-native-surface` | pass | optional reuse for surface descriptors/state; it is not a WebKit renderer |
| `nmp-native-test-harness` | pass | reuse for deterministic service/corpus tests |
| `nmp-native-performance-harness` | pass | defer until a measured performance gate exists |

## Apple-only edges

The platform-specific code is confined to:

```text
apps/workbench-macos/
apps/workbench-ios/
platforms/apple/
Packages/NMPNativeRuntime/
```

The trusted-shell JavaScript under `web/trusted-shell/` is portable. Its existing Apple binding is not; Uzel supplies the Linux WebKit/Tauri host projection.

## Runtime entry points

The public controller already supplies the daemon-facing seam:

```text
RuntimeController::open(...)
RuntimeController::verify_artifact(event_json, coordinate)
RuntimeController::install(verified_artifact)
RuntimeController::launch(verified_artifact, execution_profile)
RuntimeController::mapped_envelope(session_id, bytes)
RuntimeController::read_verified(...)
RuntimeController::{stop,suspend,resume,crash}
RuntimeController::{snapshot,observe,relay_diagnostics,close}
```

Upstream runtime-ffi tests exercise verify/install/launch, `shell.ready`, mapped envelopes, provider traffic, session lifecycle, persistence, permissions, and INC. A daemon can call this Rust API; it does not need a second runtime abstraction.

Exact-build authority remains runtime-owned. The verified principal is publisher/author bytes plus `dTag` plus aggregate hash; session IDs and surface bindings are runtime/host assigned. No envelope payload selects them.

## Build evidence

Executed in a disposable checkout:

```sh
RUSTUP_HOME=<tmp>/rustup-home \
CARGO_HOME=<tmp>/cargo-home \
CARGO_TARGET_DIR=<tmp>/nampplets-target \
cargo +1.89.0 fmt --all -- --check

cargo +1.89.0 test --locked --workspace
cargo +1.89.0 clippy --locked --workspace --all-targets -- -D warnings
python3 -m unittest discover -s conformance/tests
python3 scripts/verify_baseline.py
node --test web/trusted-shell/*.test.js
```

All commands passed. The baseline verifier reported 15 Kehto, 1 published, and 4 reference corpus entries, 209 envelopes, 10 falsifiers, and complete blob/relay/signer service scenarios.

## Compatibility rejection

The upstream `compatibility.lock` is unratified and pins core/nap 0.28.0, shim 0.26.8, SDK 0.24.4, conformance 0.13.0, and an older Kehto snapshot. Provider constants also say `napplet-web@0.28.0`. Kehto PR #204 uses the 0.29 line.

Therefore Linux reuse is proven, but Kehto 0.29 compatibility is not. Uzel must wait for a single updated, tested nampplets baseline; it must not add a translation layer between 0.28 and 0.29.
