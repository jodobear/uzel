# Provisional component design

> Gate 0 replaced guesses with the validated constraints below. Exact source/tool pins and the Linux-scoped go decision live in [`../compatibility.lock`](../compatibility.lock).

## Local daemon protocol

Use an existing `nampplets` local boundary if one is suitable. Otherwise use a private Unix-domain protocol under:

```text
$XDG_RUNTIME_DIR/uzel/napd.sock
```

Minimum properties:

- same-user access only;
- bounded frames and pending requests;
- explicit protocol version `0`;
- one shell client for the POC;
- typed errors;
- no TCP, remote clients, compatibility promise, or generic plugin protocol.

Validated wire shape for version `0`:

```text
4-byte unsigned big-endian payload length
maximum payload length: 4096 bytes
UTF-8 JSON operation/result/error object
reject an oversized declaration before allocating or reading its body
```

Create `$XDG_RUNTIME_DIR/uzel` as mode 0700 and the socket as mode 0600. Accept one shell client for the POC, enforce bounded pending work, and remove only a verified same-user stale socket. The disposable hello/status probe passed against `/run/user/1000`.

Minimum operations:

```text
hello/status
set/get read identity
install/start/stop fixture app
forward source-bound NAP envelope
read bounded diagnostics
shutdown in developer/test mode
```

The Rust daemon and Tauri backend share `napd-protocol` Rust types. Do not generate a TypeScript copy of daemon IPC; Svelte receives a narrower product-facing model from the trusted Tauri backend.

The 4096-byte frame remains a control-message limit. Slice 04 transfers the
pinned 96172-byte verified `/index.html` as ordered 2048-byte chunks with a
512-KiB aggregate ceiling. Start returns bounded metadata, transfer ID, and
total length; each subsequent request must name the same transfer and exact
next offset. Tauri rejects changed totals, invalid base64, empty or oversized
chunks, offset gaps, and inconsistent completion. `RuntimeController` now lives
only in `uzel-napd`; WebKit receives reassembled bytes and never a cache path.

## Runtime state

Keep semantic owners separate:

| State | Owner |
|---|---|
| Nostr events, replaceable selection, relay evidence, freshness | pinned NMP |
| verified app build, session, grant | nampplets runtime |
| source window, surface, generation mapping | trusted Linux host, attested to nampplets |
| active read pubkey, layout, product mode | Uzel runtime metadata |
| napplet private KV | runtime storage provider |
| Svelte transient UI state | Svelte |

Use `nmp-native-runtime-store` for installed-build/runtime/app-KV facts and NMP redb for Nostr truth. The pinned NMP store did not restore its active read identity in an executable restart probe, so Uzel persists only protocol version, product mode, the canonical public read key, and the last reserved surface generation in a bounded mode-0600 record. Startup sends that key back through `register_read_only_account` and activation, keeping NMP's parser authoritative. The generation is product security metadata, not runtime or Nostr truth; reserving it before exposing a surface prevents a stale host mapping from becoming valid after daemon restart. Do not duplicate app KV or design a broad platform schema during the POC.

## Exact-build fixtures

The repository contains two self-contained production-like bundles plus one hostile test bundle. Each has:

```text
publisher pubkey
dTag
aggregate hash
file hashes
required capability domains
```

The Vite plugin creates each single-file artifact and its manifest inputs. Its
sidecar alone is not launchable through the pinned NMP resolver because it lacks
a signed source tag. The released `@napplet/cli@0.0.0` dry-run path creates the
final source-bearing signed event with upstream `nostr-tools`; Uzel never
implements manifest hashing or signing. The runtime then verifies that event
and exact artifact bytes before execution. Remote scripts, dynamic CDN imports,
service-worker updates, module-preload network helpers, and direct browser
network dependencies are rejected.

## Identity and Nostr reads

The shell selects one lower-case hex key or `npub`. Register it through `RuntimeController::register_read_only_account`/`NmpDataPlane`; NIP-05 lookup is not part of the accepted parser. Use the pinned nampplets identity/Nostr providers rather than inventing a second identity API.

Required behavior:

- direct follows from canonical kind `3`/NIP-02 state;
- visible list limited to a measured small window;
- visible/cached profile hints fetched lazily;
- selected profile observed live or refreshed with bounded freshness;
- cache-first rendering;
- no claim of global completeness;
- all observers cancelled when identity/session closes.
- source evidence and shortfalls displayed without a global completeness claim.

Initial fixture sizes should be small enough to falsify behavior, not benchmark the entire network. Performance ceilings are selected after the NMP probe rather than guessed in documentation.

## Napplet convention

```text
napplet:profile/open
```

Payload schema owner:

```text
contracts/profile-open-v1.schema.json
```

Payload:

```json
{"version":1,"pubkey":"64 lowercase hex characters"}
```

Use the exact queryless convention and runtime-attested sender behavior from the pinned packages/#204. Do not build a private event bus.

This direct NAP-INC convention does not require an archetype tag or
NAP-INTENT. `profile-card` intentionally declares only `inc` and `outbox`; the
POC continues to treat `intent.deliver` as unsupported.

## Web trust boundary

Expected frame:

```html
<iframe sandbox="allow-scripts" referrerpolicy="no-referrer"></iframe>
```

Use a verified `srcdoc` document with policy before executable content:

```text
default-src 'none'; script-src 'unsafe-inline'; style-src 'unsafe-inline';
img-src data: blob:; font-src data:; connect-src 'none'; worker-src 'none';
child-src 'none'; frame-src 'none'; media-src 'none'; object-src 'none';
manifest-src 'none'; base-uri 'none'; form-action 'none'
```

Gate 0 proved self-contained child execution and denied fetch, XHR, WebSocket, and external image loads with zero loopback connections. It also proved no Tauri globals, Wry `window.ipc`, or readable parent bridge. The raw WebKit message handler is visible but has no authenticated Tauri authority; keep the generated invoke key top-frame-only and treat invalid messages as hostile input.

Inbound host order:

1. map `MessageEvent.source` to a shell-created frame;
2. attach shell-owned surface/generation;
3. validate message shape and size;
4. ignore caller-supplied principal/session/sender claims;
5. forward through the trusted Tauri backend.

Never switch initialization scripts to all frames, add `allow-same-origin`, or relax `connect-src` to accommodate a bundler. A candidate artifact that contains a network helper is incompatible and must be rebuilt upstream.

## Shell UI

POC layout only:

- two panes;
- horizontal/vertical orientation;
- directional focus;
- draggable divider;
- one-pane fullscreen;
- persisted ratio/orientation;
- compact top bar and status indicator;
- developer drawer hidden in user mode.

Rust/daemon own runtime truth. Svelte owns rendering and local animation only. No workspaces, overview, tags, Lua, external-WM adapters, or internal compositor framework.

## Developer mode

One product, two presentations. Developer mode may show:

```text
installed exact builds
sessions/surfaces
envelope log
NMP query/relay/evidence summary
KV keys/size, not secret values
errors and timing
fixture/live switch in local development
```

It does not grant new napplet capabilities or weaken sandbox rules.
