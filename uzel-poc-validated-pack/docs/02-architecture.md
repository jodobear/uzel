# POC architecture

> Gate 0 validated this topology on Linux. The exact provisional pins and accepted risks are recorded in [`../reports/preflight.md`](../reports/preflight.md). The architecture and Slice 01 are authorized for the Linux-only POC.

## Runtime topology

```mermaid
flowchart TB
    subgraph Shell[uzel process - trusted]
        TB[Tauri Rust backend]
        UI[Svelte shell]
        HB[host bridge]
        F[follow-list iframe]
        P[profile-card iframe]
        UI --> HB
        HB <--> F
        HB <--> P
        HB <--> TB
        UI <--> TB
    end

    subgraph Daemon[uzel-napd - runtime authority]
        IPC[local IPC]
        RT[nampplets RuntimeController]
        ST[nampplets runtime-store]
        NA[nampplets NmpDataPlane + providers]
        NE[pinned NMP engine + redb]
        IPC --> RT
        RT --> ST
        RT --> NA
        NA --> NE
    end

    TB <--> IPC
    NE <--> RELAYS[Nostr relays]
```

The Svelte shell never talks directly to NMP or runtime storage. The Tauri Rust backend is a thin trusted daemon client. Napplet frames communicate only with the source-binding host bridge and upstream trusted-shell projection.

## Trust domains

| Domain | Authority |
|---|---|
| `uzel-napd` | process ownership and composition of upstream runtime/NMP authorities |
| nampplets runtime | principals, exact builds, sessions, grants, NAP providers, app KV, bounded diagnostics |
| NMP | Nostr event truth, replaceable selection, follows, relay evidence, freshness, canonical cache |
| Tauri Rust backend | window lifecycle and daemon client |
| Svelte parent | layout, source-bound frame routing, user/dev presentation |
| Napplet iframe | granted NAP capabilities only |
| Nostr relay | untrusted remote signed-event source |
| Bundle | untrusted until publisher/hash verification succeeds |

The POC threat target is malicious napplet JavaScript. It does not claim protection from a compromised kernel or a WebKit engine exploit.

## Repository zones

```text
apps/uzel/              Tauri + Svelte product
apps/uzel-napd/         daemon binary
crates/napd/            thin Uzel daemon composition; no copied runtime/provider/NMP logic
crates/napd-protocol/   small internal local protocol
napplets/               portable demo/test napplets
contracts/              shared convention schemas
fixtures/               signed manifests, artifacts and Nostr events
```

Keep the POC small: two reusable crates, not a speculative crate hierarchy.

```mermaid
flowchart LR
    NP[napplets] --> NAP[Published Napplet packages]
    SH[Uzel shell] --> PR[napd-protocol]
    DM[uzel-napd] --> PR
    DM --> NC[napd]
    NC --> NM[nampplets RuntimeController]
    NM --> NMP[pinned NMP via NmpDataPlane]

    NP -. forbidden .-> SH
    NP -. forbidden .-> NC
    NC -. forbidden .-> SH
```

## Session start

```mermaid
sequenceDiagram
    participant S as Uzel shell
    participant D as uzel-napd
    participant R as nampplets runtime
    participant F as napplet frame

    S->>D: start app + shell-owned surface id
    D->>D: verify/install exact build
    D->>R: create principal/session/grants
    R-->>D: session + sealed document/environment
    D-->>S: session started
    S->>F: create sandboxed frame
    F->>S: shell handshake
    S->>D: source-bound envelope
    D->>R: dispatch to assigned session
    R-->>S: outbound envelope
    S-->>F: target only mapped frame
```

No payload field may choose its principal, session, sender, or target frame.

## Accepted upstream seam

The daemon drives the existing source pin through:

```text
RuntimeController::open
RuntimeController::verify_artifact
RuntimeController::install
RuntimeController::launch
RuntimeController::mapped_envelope
RuntimeController::read_verified
RuntimeController lifecycle, snapshot, observation, diagnostics, and close methods
```

The controller composes `nmp-native-artifact`, runtime-core/app/store, NAP bridge/providers, and `NmpDataPlane`. `crates/napd` may translate the private AF_UNIX operation enum to these calls; it must not reproduce their semantics. The Linux-only implementation edge is WebKit host creation and source-window mapping.

Slice 04 moved the controller out of Tauri and into `uzel-napd`. The verified
96172-byte document crosses the private socket in ordered 2048-byte base64
chunks; each request and response retains the 4096-byte control-frame ceiling.
The daemon rejects an unknown transfer, a changed transfer ID, an out-of-order
offset, an oversized aggregate, and an invalid completion marker. Tauri
reassembles only bounded bytes, and WebKit never receives an artifact cache
path. The Svelte application has no daemon protocol types.

Slice 05 extends that same controller to at most four simultaneously active
exact fixtures. Each launch burns a new persisted surface generation before
exposure. The private response identifies the target surface explicitly;
Svelte can deliver it only through the upstream multi-surface host, which maps
the original `MessageEvent.source` to that surface. Cross-surface delivery is
permitted only for runtime-authorized `inc.emit`; other replies remain
source-scoped. No envelope payload chooses its destination.

## Shared Nostr flow

```mermaid
sequenceDiagram
    participant N as napplet
    participant H as host
    participant D as daemon
    participant M as NMP
    participant R as relay

    N->>H: relay/identity request
    H->>D: source-bound NAP envelope
    D->>D: authorize exact-build session
    D->>M: provider-owned demand using public facade
    M-->>D: cached canonical rows + evidence
    D-->>H: authorized result/update
    H-->>N: target mapped frame
    M->>R: bounded network work when freshness requires
    R-->>M: signed events / EOSE / failure
    M-->>D: canonical transition
    D-->>H: authorized update
    H-->>N: target mapped frame
```

NMP owns event truth. nampplets runtime storage owns installed-build, runtime, and app-KV facts. Uzel may store only missing product metadata; it must not add an event/profile/follow cache.

## Composition flow

```mermaid
sequenceDiagram
    participant A as follow-list
    participant H as trusted host
    participant D as runtime INC router
    participant B as profile-card
    A->>H: emit napplet:profile/open {pubkey}
    H->>D: source-bound envelope
    D->>D: validate schema and attest sender
    D-->>H: target delivery
    H-->>B: deliver to mapped handler frame
    B->>H: query selected kind 0
    H->>D: source-bound relay request
```

The sender does not address a session directly. The profile event itself is not copied across INC; the receiver queries canonical data.

## UI boundary

The shell uses one native window and one trusted WebView containing two sandboxed frames. Gate 0 proved that Tauri/Wry bootstrap and the authenticated invoke key remain top-frame-only under WebKitGTK 2.52.5 and that `MessageEvent.source` binding works.

WebKit still exposes its low-level `window.webkit.messageHandlers.ipc` transport to the child. It carries no Tauri command authority without the generated top-frame invoke key; an invalid-key hostile call was rejected. Treat it as attacker-controlled input, keep scripts main-frame-only, preserve size/rate limits, and never use Tauri webview labels as a substitute for inner-frame source binding.

This path does not prove independent browser-process isolation or per-surface native resource accounting; those remain later work.
