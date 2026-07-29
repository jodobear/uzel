# Slice 04 preflight — daemon, NMP, and persistence

Date: 2026-07-29  
Branch: `feat/slice-04-daemon-nmp`  
Implementation commits: `8436b66`, `fc74809`

## Outcome

**PASS. Work 04 acceptance is complete. Slice 05 may start.**

Tauri no longer links `napd` or the pinned runtime. It links only
`napd-protocol` and acts as a thin AF_UNIX client. `uzel-napd` owns one
`LinuxRunner`, which owns one upstream `RuntimeController`; that controller
owns the one `NmpDataPlane`, NMP engine, runtime store, artifact cache, and NMP
redb store.

No Uzel event, profile, or follow table/cache exists.

## Private protocol evidence

- Protocol version remains `0`.
- Every control frame retains the four-byte big-endian length prefix and
  4096-byte maximum; an oversized declaration is rejected before body read or
  allocation.
- `$XDG_RUNTIME_DIR/uzel` is mode 0700 and `napd.sock` is mode 0600.
- A stale path is removed only when it is a Unix socket owned by the same UID
  as its private parent directory.
- The server accepts one request per private local connection and processes one
  shell client's state serially for the POC.
- Invalid protocol JSON, changed versions, unknown surfaces/transfers,
  out-of-order chunks, and runtime refusals are typed bounded errors.
- Shutdown exits the accept loop; runtime and relay observations stop
  idempotently before `RuntimeController::close`, which closes NMP.

The pinned verified `/index.html` is 96172 bytes. Start returns only bounded
surface metadata, a transfer ID, and total length. Tauri then pulls ordered
2048-byte base64 chunks. Both sides enforce a 512-KiB aggregate limit; Tauri
also rejects changed transfer IDs/totals, offset gaps, empty or oversized
chunks, invalid base64, and inconsistent completion. No artifact path crosses
the protocol or reaches WebKit.

## NMP and persistence evidence

The explicit live probe starts pinned-Nix `nak 0.20.1` as a disposable
in-memory loopback relay. Its committed signed fixture contains:

- an older kind-0 profile;
- a newer canonical kind-0 replacement;
- duplicated relay rows for that replacement;
- a kind-3 direct-follow event, also duplicated at the relay boundary.

The daemon selects one read-only identity through
`RuntimeController::register_read_only_account`, then activates the returned
opaque installation handle. The existing nampplets identity provider returns
only the newer `Alice` profile and exactly two unique direct follows. It first
returns the cache snapshot and then satisfies the bounded refresh from the
local relay. After the relay is stopped, reopening the same runtime root
returns the same canonical profile from NMP redb.

The executable restart probe rejected one original assumption: NMP redb at the
pinned revision does not restore the active read identity. Uzel now persists
only a bounded version-0 product record containing mode and canonical public
read key, with mode 0600. Startup passes that key back through NMP's parser and
activation path. Nostr events, replacement selection, profiles, follows,
evidence, and freshness remain exclusively NMP-owned.

The live loopback probe also established that profile-owned indexer/app relay
preferences correctly reject `ws://` before NMP's local-host allowlist. The
disposable relay therefore uses NMP's fallback lane with an explicit
`allowed_local_relay_hosts = ["127.0.0.1"]`. Normal indexer/app preferences
remain `wss://` only.

An asynchronous `identity.changed` delivery initially arrived before a profile
result. The runner now correlates provider responses by the caller's request ID
(and `shell.ready` specifically to `shell.init`) so an unsolicited push cannot
satisfy another request.

## Commands and results

All commands ran from the repository root. Commands requiring Unix sockets or
the Nix daemon ran with host permission rather than the restricted tool
sandbox.

```text
cargo test -p napd --lib
  6 passed; 1 explicit live probe ignored

cargo test -p napd live_nmp_refreshes_then_restarts_cache_first_without_a_second_cache -- --ignored
  1 passed

cargo test -p napd-protocol
  4 passed

cargo test -p uzel-napd
  1 passed

nix --extra-experimental-features 'nix-command flakes' develop --command cargo check --workspace
  passed

nix --extra-experimental-features 'nix-command flakes' develop --command cargo clippy --workspace --all-targets -- -D warnings
  passed

nix --extra-experimental-features 'nix-command flakes' develop --command cargo fmt --all -- --check
  passed

nix --extra-experimental-features 'nix-command flakes' flake check
  passed

nix --extra-experimental-features 'nix-command flakes' develop --command pnpm check
  passed

nix --extra-experimental-features 'nix-command flakes' develop --command pnpm test
  passed

nix --extra-experimental-features 'nix-command flakes' develop --command pnpm docs:check
  40 Markdown documents; zero errors; zero warnings

nix --extra-experimental-features 'nix-command flakes' develop --command pnpm fallow
  zero dead-code/import/boundary issues; zero duplicated lines

nix --extra-experimental-features 'nix-command flakes' develop --command pnpm smoke
  SLICE_04_DAEMON_NMP_SMOKE_OK

nix --extra-experimental-features 'nix-command flakes' develop --command bash -c \
  'while IFS= read -r event; do nak verify "$event"; done < fixtures/nostr/live-events.jsonl'
  every signed fixture event verified
```

## Remaining boundaries

- No signer or publish path was added.
- No graph beyond direct follows was added.
- No remote daemon client, public IPC compatibility promise, broad product
  schema, or extension surface was added.
- The two-pane follow-list/profile-card shell composition is Work 05.
- Full hostile WebKit execution, independent zero-accept network attestation,
  and user/developer presentation modes remain Work 06.

## Exact next step

Execute `work/05-integrate.md`: launch both signed napplets through the completed
daemon seam, preserve source-to-surface binding for each frame, route the exact
queryless `napplet:profile/open` convention through runtime-owned NAP-INC, and
show fixture/live source evidence without claiming global completeness.
