# FACT-006 — strict CSP and direct egress

- **Claim:** A self-contained sandboxed napplet can run under the draft NIP-5D default-deny CSP without direct browser network access.
- **Classification:** verified fact
- **Exact source/pin:** NIP-5D PR #2303 head `eb45dfd7335b7f88cb53781984c553581d2b4c34`; WebKitGTK 2.52.5; probe HTML SHA-256 `9478e24ff9ba69d9e281428e6ddab364befcd699840dbd8c732fea5787881cbc`.
- **Probe/command:** In the sandboxed child, install the NIP-5D CSP and execute `fetch`, XHR, WebSocket, and external image attempts against a live `127.0.0.1:43129` sentinel.
- **Observed result:** Child JavaScript and `postMessage` ran. All four attempts reported blocked, and the native sentinel accepted zero connections. No `allow-same-origin` token or direct network allowance was required. Candidate Kehto commit `62241de...` disables Vite module preload; its exact `chat` and `feed` artifacts contain no module-preload `fetch` and pass released conformance-cli 0.2.16.
- **Decision:** V-06 now passes. Keep the exact default-deny child CSP and runtime-mediated resource/Nostr providers. This proves feasibility for the tested paths, not every browser API. Kehto source reachability is independently satisfied by the exact fork pin; this result does not ratify the nampplets compatibility candidate.
- **Affected documents/code:** `reports/webkit-trust-spike.md`, `reports/preflight.md`, `docs/03-provisional-design.md`, `docs/05-test-and-demo.md`.
- **Revalidate when:** The bundle shape, CSP, WebKitGTK, iframe attributes, Tauri asset protocol, or hostile path list changes.
