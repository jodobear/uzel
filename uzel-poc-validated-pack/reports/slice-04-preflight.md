# Slice 04 preflight — daemon, NMP, and persistence

Date: 2026-07-29

Branch: `feat/slice-04-daemon-nmp`

Implementation commits: `8436b66`, `fc74809`, `2cf7f48`, plus the final
review-hardening commit recorded in repository history.

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
  as its private parent directory, a connection probe proves no daemon is
  listening, and a final device/inode check proves the path was not replaced.
- The socket parent must be a real directory, never a symlink. Server teardown
  removes only the exact device/inode it bound, not a replacement socket.
- A newly created private parent is set to mode 0700. An existing parent must
  already exclude group/other access; the daemon refuses it instead of
  changing caller-owned or shared-directory permissions.
- Missing `XDG_RUNTIME_DIR` is a startup refusal unless an explicit socket path
  is supplied; the daemon never falls back to a shared `/tmp/uzel` path.
- The server accepts one request per private local connection and processes one
  shell client's state serially for the POC. Accepted streams have bounded
  read/write deadlines, so an incomplete client cannot block later requests
  indefinitely.
- Invalid protocol JSON, changed versions, unknown surfaces/transfers,
  out-of-order chunks, oversized runtime responses, and runtime refusals are
  typed bounded errors. The daemon serializes each response into a bounded
  buffer before writing, so it returns `response_too_large` instead of closing
  the connection on an oversized upstream envelope.
- Each fixture restart advances the shell-owned surface and transfer generation,
  so stale IDs cannot authorize a replacement session.
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
only a bounded version-0 product record containing mode, canonical public read
key, and last reserved surface generation, with mode 0600. State loading
refuses symlinks and non-regular files; atomic temporary creation refuses
pre-existing paths. Startup passes the key back through NMP's parser and
activation path. Product-state replacement fsyncs both the file and its parent
directory. A failure before rename rolls a changed NMP identity back to the
previous exact installation; a failure after rename retains the new in-memory
identity so memory and disk cannot be inverted. A generation is burned and
durably reserved before its surface token is exposed, including when a
post-replacement directory sync reports failure. Nostr events, replacement
selection, profiles, follows, evidence, and freshness remain exclusively
NMP-owned.

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
  15 passed; 1 explicit live probe ignored

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

nix --extra-experimental-features 'nix-command flakes' develop --command pnpm smoke:fedora
  FEDORA_RUN_SMOKE_OK daemon=ready shell=ready exact_build=verified
  nap_shell=ready artifact=responded child_native=denied
  compositor=weston-headless-gl

nix --extra-experimental-features 'nix-command flakes' develop --command bash -c \
  'while IFS= read -r event; do nak verify "$event"; done < fixtures/nostr/live-events.jsonl'
  every signed fixture event verified
```

The first Fedora review regression run found an unrelated orphaned prior Vite
process holding port 1420. The harness preserved that failed run under
`reports/probes/slice-02-fedora-failed/`; after terminating only that process
group and confirming the port was free, the single clean rerun emitted the
exact success marker above. Expected headless EGL/cursor warnings did not
affect the assertion.

Codex review then exercised eight previously uncovered local-boundary cases:
an active daemon socket must not be unlinked, accepted streams need deadlines,
existing shared socket parents must not be chmodded, and the historical exact
Fedora readiness line must remain stable. It also found that failed identity
persistence must restore the prior active NMP installation and surface
generations must survive daemon restart. Finally, oversized runtime envelopes
must become typed bounded errors, and atomic state replacement must fsync the
parent directory before its reservation is called durable. All eight were
corrected and covered by the 15-test daemon suite plus the final Fedora run.
The Fedora harness now also uses a disposable `XDG_DATA_HOME`, keeping its
fresh-generation assertion repeatable without touching user product state.

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
