# FACT-005 — Tauri/WebKit child-frame authority

- **Claim:** A sandboxed `srcdoc` child can execute while authenticated Tauri/Wry bridge authority stays in the trusted top frame and host routing binds `MessageEvent.source`.
- **Classification:** verified fact
- **Exact source/pin:** Tauri 2.11.5, tauri-runtime-wry 2.11.4, Wry 0.55.1, WebKitGTK 2.52.5, nixpkgs `38a4887411571457d700c51c64a6e49ead2ed5ab`.
- **Probe/command:** Build a disposable Tauri app; run it under Weston 15.0.1/Mesa 26.1.5; load `<iframe sandbox="allow-scripts" srcdoc=...>`; invoke one trusted command in the parent; inspect child globals; send an invalid raw WebKit IPC message; accept only a source-bound result.
- **Observed result:** Parent invoke returned `trusted-parent-ok`; `sourceBound=true`; child `__TAURI__=false`, `__TAURI_INTERNALS__=false`, `window.ipc=false`, and parent bridge read failed. WebKit exposed its low-level `messageHandlers.ipc`, but Tauri rejected the invalid invoke key and native call count remained exactly one.
- **Decision:** The one-WebView/two-sandboxed-frame projection is feasible. Treat the low-level handler as untrusted input, never expose the invoke key, keep all initialization scripts top-frame-only, and retain payload bounds/rate limits.
- **Affected documents/code:** `reports/webkit-trust-spike.md`, `docs/02-architecture.md`, `docs/03-provisional-design.md`, `work/02-linux-runner.md`.
- **Revalidate when:** Tauri, Wry, WebKitGTK, initialization-script settings, iframe construction, or IPC configuration changes.
