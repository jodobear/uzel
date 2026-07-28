# Work 02 — Linux exact-build runner

## Goal

Launch one verified self-contained fixture through the accepted `nampplets` runtime seam in a Linux Tauri/WebKit host.

## Read

- accepted `reports/nampplets-linux-map.md`
- accepted `reports/webkit-trust-spike.md`
- `docs/02-architecture.md`
- `docs/03-provisional-design.md`

## Tasks

- Reuse exact-build verifier/principal/session code.
- Install or load one signed local fixture.
- Create one shell-owned surface/frame mapping.
- Complete NAP-SHELL handshake.
- Route messages by `MessageEvent.source`, never payload identity.
- Prove child lacks Tauri/native access.
- Keep runtime API product-neutral.

## Acceptance

One exact build starts, handshakes, responds through the runtime, and fails hostile identity/Tauri probes.

## Non-goals

No NMP data, second napplet, generic installer, catalog, multi-WebView, or public IPC.
