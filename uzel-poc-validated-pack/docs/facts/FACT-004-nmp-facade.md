# FACT-004 — NMP facade and ownership

- **Claim:** The pinned NMP/nampplets public APIs cover Uzel's profile, direct-follow, freshness, evidence, cancellation, diagnostics, and shutdown needs.
- **Classification:** verified fact
- **Exact source/pin:** `pablof7z/nmp@005dc2a5f12aa414961b313d05ebb021934e385c`, as locked by `nampplets@839654cd3643b430548765823b783f0b5140b8da`.
- **Probe/command:** Compile the public `nmp::Engine` and `NmpDataPlane` path with signed Nostr fixture events; observe cache-only kind 0/direct follows; inspect evidence; cancel twice; close the adapter and engine.
- **Observed result:** The probe returned two cache-only rows, the expected profile, one explicit follow, evidence with an explicit `no_planned_source` shortfall, idempotent observation cancellation, and clean shutdown. The facade exposes `Engine::new`, `observe`, diagnostics/account operations, and `shutdown`; nampplets adds read-only identity and provider projections. It does not expose a global synced/complete flag.
- **Decision:** NMP owns events, replaceable selection, follows, relay evidence, freshness, and diagnostics. Uzel uses `NmpDataPlane`/`RuntimeController`; it must not create profile/follow caches or translate NAP relay messages independently.
- **Affected documents/code:** `reports/nmp-api-map.md`, `docs/02-architecture.md`, `docs/03-provisional-design.md`, `work/04-daemon-nmp.md`.
- **Revalidate when:** The nampplets NMP pin, `Engine`, `NmpDataPlane`, NAP providers, or evidence model changes.
