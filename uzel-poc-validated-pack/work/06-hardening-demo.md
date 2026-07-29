# Work 06 — hardening and demo acceptance

## Goal

Make the POC reproducible, honest, and presentable.

## Tasks

- [x] Start a live TCP sentinel on an ephemeral unprivileged `127.0.0.1` port and prove it accepts a control connection. Launch the signed hostile artifact, commit its unique HTTP URL through nampplets' exact-principal, exact-session NAP-CONFIG provider, and only then mount the artifact in the trusted shell. The `srcdoc` child has no useful location query string.
- [x] Run the hostile frame suite across all listed egress/native/identity probes. A missing, dead, privileged-port, or non-loopback sentinel is a test setup failure, never a network-denial pass.
- [x] Retain the Gate 0 top-frame-only Tauri/Wry configuration and exact child CSP; after all probes settle, separately assert that the live sentinel accepted zero probe connections. Also fail if the raw WebKit handler can execute a command without the invoke key.
- [x] Verify user mode hides diagnostics and unsafe fixture controls.
- [x] Run clean Fedora and Debian gates.
- [x] Run Rust, frontend, Fallow, conformance, and documentation checks.
- [x] Follow the deterministic demo scripts from a clean checkout. The public-relay live demo remains optional and is not part of deterministic acceptance.
- [x] Record known limitations and extraction debt.
- [x] Evaluate Bubblewrap without distorting the accepted architecture. Whole-process `--unshare-net` would also sever the trusted shell's loopback daemon and relay access, so OS-level per-WebKit-child isolation is recorded as the first post-POC hardening follow-up.

## Acceptance

All foundation-complete criteria pass, or the only outstanding item is explicitly labeled post-POC hardening and does not invalidate the malicious-JavaScript boundary claimed by the demo.

Result: **PASS**. Exact commands, pins, failures, and limitations are recorded in
`reports/slice-06-preflight.md` and `compatibility.lock`.
