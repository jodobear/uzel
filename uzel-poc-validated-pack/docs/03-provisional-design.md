# Provisional component design

> This document defines constraints and the smallest expected shape. Gate 0 may replace details with a simpler existing upstream seam.

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

## Runtime state

Keep semantic owners separate:

| State | Owner |
|---|---|
| Nostr events, replaceable selection, relay evidence, freshness | NMP |
| verified app build, session, grant, surface binding | `nampplets`/`napd` |
| active read pubkey, layout, product mode | Uzel runtime metadata |
| napplet private KV | runtime storage provider |
| Svelte transient UI state | Svelte |

Use existing `nampplets` persistence where suitable. Add SQLite only for missing product/KV facts; do not design a broad platform schema during the POC.

## Exact-build fixtures

The repository contains two self-contained production-like bundles plus one hostile test bundle. Each has:

```text
publisher pubkey
dTag
aggregate hash
file hashes
required capability domains
```

The runtime verifies the signed manifest and hashes before execution. Remote scripts, dynamic CDN imports, service-worker updates, and direct browser network dependencies are rejected.

## Identity and Nostr reads

The shell selects one public key. Gate 0 determines the exact pinned identity projection used by #204/packages/`nampplets`; do not invent a second identity API.

Required behavior:

- direct follows from canonical kind `3`/NIP-02 state;
- visible list limited to a measured small window;
- visible/cached profile hints fetched lazily;
- selected profile observed live or refreshed with bounded freshness;
- cache-first rendering;
- no claim of global completeness;
- all observers cancelled when identity/session closes.

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

## Web trust boundary

Expected frame:

```html
<iframe sandbox="allow-scripts" referrerpolicy="no-referrer"></iframe>
```

Expected policy is default-deny, self-contained execution, no `connect-src`, no child frames/workers/forms/navigation, and no Tauri/native bridge in the child. The exact CSP and document construction are chosen by the Gate 0 probe because opaque-origin ESM and WebKit behavior can affect packaging.

Inbound host order:

1. map `MessageEvent.source` to a shell-created frame;
2. attach shell-owned surface/generation;
3. validate message shape and size;
4. ignore caller-supplied principal/session/sender claims;
5. forward through the trusted Tauri backend.

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
