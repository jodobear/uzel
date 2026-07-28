# Work 02 — Linux exact-build runner

## Goal

Launch one verified self-contained fixture through the accepted `nampplets` runtime seam in a Linux Tauri/WebKit host.

## Entry status and pins

**Blocked by Gate 0.** After compatibility is accepted, use candidate `nampplets@b1a38f1af9191b6742c0be8ddea04159a2755a71` only if its required reviews ratify that exact commit, or use its explicitly accepted successor. Keep Tauri crate 2.11.5, Wry 0.55.1, and WebKitGTK 2.52.5. Re-run the hostile probe if any of those pins changes.

## Read

- accepted `reports/nampplets-linux-map.md`
- accepted `reports/webkit-trust-spike.md`
- `docs/02-architecture.md`
- `docs/03-provisional-design.md`

## Tasks

- Reuse `nmp-native-artifact`, `nmp-native-runtime-core`, `nmp-native-nap-bridge`, `nmp-native-runtime-store`, `nmp-native-runtime-app`, and `RuntimeController`; do not create a parallel runner.
- Install or load one signed local fixture.
- Create one shell-owned surface/frame mapping.
- Complete NAP-SHELL handshake.
- Route messages by `MessageEvent.source`, never payload identity. Keep Tauri/Wry initialization scripts top-frame-only.
- Prove child lacks Tauri/native access.
- Keep runtime API product-neutral.

## Acceptance

One exact build starts, handshakes, responds through the runtime, and fails hostile identity/Tauri probes.

## Non-goals

No NMP data, second napplet, generic installer, catalog, multi-WebView, or public IPC.
