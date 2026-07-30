# NMP API and ownership map

## Accepted pin

Use NMP commit `005dc2a5f12aa414961b313d05ebb021934e385c`, because that is the exact git revision in the accepted nampplets source and Cargo lock. Current NMP main (`0c07e018aa1ff0d16fb368f5f4b8d68caf496fb0` at validation time) is evidence of later work, not a compatible substitute.

## Public facade

The POC-required stable seam at the pin is:

```rust
Engine::new(EngineConfig)
Engine::observe(LiveQuery, Option<Window>)
Engine::observe_diagnostics()
Engine::set_active_account(Option<PublicKey>)
Engine::active_account()
Engine::shutdown()
```

`publish`, signer, and write-receipt APIs exist but are out of POC scope. Observation objects provide bounded frame receipt, row-window requests, cancel handles, and idempotent cancellation.

Every demand explicitly carries:

```text
SourceAuthority
AccessContext
CacheMode
Freshness: Live | MaxAge | CacheOnly
```

Every observation frame carries window contents/deltas and evidence. Evidence identifies actual sources and shortfalls; there is intentionally no process-wide “synced” or “globally complete” boolean.

## nampplets adapter seam

Uzel should normally enter through the already-tested adapter, not call raw NMP for napplet traffic:

```text
NmpDataPlane::open(...)
NmpDataPlane::register_read_only_account(hex-or-npub)
NmpDataPlane::set_active_public_identity(...)
NmpDataPlane::manifest_catalog()
NmpDataPlane::relay_diagnostics()
NmpDataPlane::close()
NapNostrProviderSet / NapNostrProvider
RuntimeController account, provider, read_verified, diagnostics, and close APIs
```

The read-only account parser accepts lower-case hex or `npub`. It does not accept NIP-05 names. Profile JSON and direct follows are adapter projections over canonical NMP observations, not new NMP database nouns.

## Ownership boundary

| Concern | Sole owner | Uzel responsibility |
|---|---|---|
| signed events and provenance | NMP | display bounded projections only |
| replaceable kind 0 selection | NMP | request through runtime provider |
| direct kind 3 follows | NMP + nampplets projection | render returned public keys |
| freshness and source shortfalls | NMP frame evidence, retained internally by nampplets | do not classify freshness in a napplet until released NAP identity APIs expose that evidence |
| relay routing/fan-out | NMP | configuration only |
| accounts/read identity | nampplets `NmpDataPlane` | set one public read context |
| NAP relay/identity envelopes | nampplets providers | source-bind and forward bytes |
| artifact/session/grants/KV | nampplets runtime | product presentation and lifecycle intent |
| layout/product mode | Uzel | persist non-Nostr metadata only |

Forbidden duplications include a Uzel event table, profile cache, follow cache, relay pool, evidence reducer, or direct NAP-to-Nostr translator.

## Executable probe

A disposable example compiled against the locked adapter and used official `nostr` signing for local fixtures. It observed a cache-only kind 0 and direct-follow demand, inspected evidence, cancelled twice, and shut down:

```text
nmp_commit=005dc2a5f12aa414961b313d05ebb021934e385c
cache_only_rows=2
profile={"about":"local-only","displayName":"Uzel Probe","name":"Gate Zero"}
follows=1
evidence.sources=[]
evidence.shortfall=no_planned_source
cancellation=idempotent
shutdown=closed
```

This proves V-04. It does not prove live public-relay behavior, completeness, write/signing, or NIP-05 lookup; none is needed for Slice 01.
