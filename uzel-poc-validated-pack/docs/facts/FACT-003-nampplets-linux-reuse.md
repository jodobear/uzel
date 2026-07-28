# FACT-003 — Linux-neutral nampplets reuse

- **Claim:** Uzel can reuse the native runtime without creating a parallel Linux runtime.
- **Classification:** verified fact
- **Exact source/pin:** [`pablof7z/nampplets@839654c`](https://github.com/pablof7z/nampplets/commit/839654cd3643b430548765823b783f0b5140b8da), Rust 1.89.0, locked NMP commit `005dc2a5f12aa414961b313d05ebb021934e385c`.
- **Probe/command:** `cargo +1.89.0 fmt --all -- --check`; `cargo +1.89.0 test --locked --workspace`; `cargo +1.89.0 clippy --locked --workspace --all-targets -- -D warnings`; Python baseline verifier; trusted-shell Node tests.
- **Observed result:** All 16 Rust crates compile and test on Linux with no production Apple cfg edge. The Rust runtime, artifact verifier, providers, NMP adapter, persistence, FFI controller, surface model, trusted-shell assets, and harnesses are reusable. Apple-only code is confined to `apps/workbench-macos`, `apps/workbench-ios`, `platforms/apple`, and `Packages/NMPNativeRuntime`. The `surface` crate is a model/parser, not a Linux WebKit renderer.
- **Decision:** Reuse the crates and trusted-shell assets directly; write only the Linux WebKit host and a thin Uzel process adapter. Do not copy provider, session, storage, exact-build, INC, or NMP logic.
- **Affected documents/code:** `reports/nampplets-linux-map.md`, `docs/02-architecture.md`, `work/02-linux-runner.md`, `work/04-daemon-nmp.md`.
- **Revalidate when:** The nampplets commit, Rust toolchain, provider line, runtime controller, or platform directories change.
