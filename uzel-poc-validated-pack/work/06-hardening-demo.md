# Work 06 — hardening and demo acceptance

## Goal

Make the POC reproducible, honest, and presentable.

## Tasks

- Run the hostile frame suite across all listed egress/native/identity probes.
- Retain the Gate 0 top-frame-only Tauri/Wry configuration and exact child CSP; fail if the raw WebKit handler can execute a command without the invoke key or if any sentinel connection occurs.
- Verify user mode hides diagnostics and unsafe fixture controls.
- Run clean Fedora and Debian gates.
- Run Rust, frontend, Fallow, conformance, and documentation checks.
- Follow the deterministic and live demo scripts from a clean checkout.
- Record known limitations and extraction debt.
- Attempt the Bubblewrap networkless-shell proof only if it does not distort the accepted architecture; otherwise record it as the first hardening follow-up.

## Acceptance

All foundation-complete criteria pass, or the only outstanding item is explicitly labeled post-POC hardening and does not invalidate the malicious-JavaScript boundary claimed by the demo.
