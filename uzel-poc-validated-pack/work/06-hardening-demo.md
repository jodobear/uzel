# Work 06 — hardening and demo acceptance

## Goal

Make the POC reproducible, honest, and presentable.

## Tasks

- Start a live TCP sentinel on an ephemeral unprivileged `127.0.0.1` port and prove it accepts a control connection. Launch the signed hostile artifact, commit its unique HTTP URL through nampplets' exact-principal, exact-session NAP-CONFIG provider, and only then mount the artifact in the trusted shell. The `srcdoc` child has no useful location query string.
- Run the hostile frame suite across all listed egress/native/identity probes. A missing, dead, privileged-port, or non-loopback sentinel is a test setup failure, never a network-denial pass.
- Retain the Gate 0 top-frame-only Tauri/Wry configuration and exact child CSP; after all probes settle, separately assert that the live sentinel accepted zero probe connections. Also fail if the raw WebKit handler can execute a command without the invoke key.
- Verify user mode hides diagnostics and unsafe fixture controls.
- Run clean Fedora and Debian gates.
- Run Rust, frontend, Fallow, conformance, and documentation checks.
- Follow the deterministic and live demo scripts from a clean checkout.
- Record known limitations and extraction debt.
- Attempt the Bubblewrap networkless-shell proof only if it does not distort the accepted architecture; otherwise record it as the first hardening follow-up.

## Acceptance

All foundation-complete criteria pass, or the only outstanding item is explicitly labeled post-POC hardening and does not invalidate the malicious-JavaScript boundary claimed by the demo.
