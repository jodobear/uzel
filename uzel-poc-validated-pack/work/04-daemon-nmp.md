# Work 04 — daemon, NMP, and persistence

## Goal

Provide one runtime authority and one canonical Nostr data plane behind the shell.

## Read

- accepted NMP API map
- `docs/02-architecture.md`
- `docs/03-provisional-design.md`

## Tasks

- Implement the smallest accepted local shell/daemon seam.
- Own one NMP engine and store.
- Set/get one public read identity.
- Map pinned napplet read requests to NMP kind `0` and direct-follow queries.
- Support deterministic fixture mode and configured live mode.
- Persist only required product/app-KV facts not owned by NMP/`nampplets`.
- Expose bounded diagnostics.
- Cancel observers/sessions cleanly.

## Acceptance

- fixture events deduplicate and select canonical profiles through NMP;
- live mode returns cache-first data and bounded refresh;
- restart preserves required state;
- no second event/profile/follow cache exists.

## Non-goals

No signer, publish, graph depth >1, broad schema, extension, or remote daemon client.
