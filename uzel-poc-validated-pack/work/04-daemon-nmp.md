# Work 04 — daemon, NMP, and persistence

## Goal

Provide one runtime authority and one canonical Nostr data plane behind the shell.

## Entry status and pins

**Gate 0 pins accepted.** Use `jodobear/nampplets@08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf` with NMP `005dc2a5f12aa414961b313d05ebb021934e385c`. Use a successor only after revalidation; do not silently advance to NMP main.

## Read

- accepted NMP API map
- `docs/02-architecture.md`
- `docs/03-provisional-design.md`

## Tasks

- Implement the bounded version-0 AF_UNIX seam accepted in `reports/preflight.md`.
- Own one upstream `NmpDataPlane`/`RuntimeController`, which in turn owns one NMP engine and store.
- Add a bounded chunked verified-asset transfer (or equivalently bounded private custom-scheme stream) from daemon to trusted Tauri host. Keep the 4096-byte control-frame ceiling, reject out-of-order/oversized transfers, and never expose an artifact filesystem path to WebKit.
- Set/get one public read identity with `RuntimeController::register_read_only_account` and the adapter's lower-hex/`npub` parser.
- Reuse `NapNostrProviderSet`/existing provider projections for kind `0` and direct follows; do not independently translate NAP requests or add caches.
- Support deterministic fixture mode and configured live mode.
- Persist only required product/app-KV facts not owned by NMP/`nampplets`.
- Expose bounded diagnostics.
- Cancel observation handles idempotently and call controller/data-plane close, which shuts down NMP.

## Acceptance

- fixture events deduplicate and select canonical profiles through NMP;
- live mode returns cache-first data and bounded refresh;
- restart preserves required state;
- no second event/profile/follow cache exists.

## Non-goals

No signer, publish, graph depth >1, broad schema, extension, or remote daemon client.
