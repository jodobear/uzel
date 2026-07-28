# POC architecture

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
        RT[nampplets adapter]
        ST[package + settings + app KV]
        NA[NMP adapter]
        NE[NMP engine/store]
        IPC --> RT
        RT --> ST
        RT --> NA
        NA --> NE
    end

    TB <--> IPC
    NE <--> RELAYS[Nostr relays]
```

The Svelte shell never talks directly to NMP or runtime storage. The Tauri Rust backend is the trusted daemon client. Napplet frames communicate only with the host bridge using the pinned web projection.

## Trust domains

| Domain | Authority |
|---|---|
| `uzel-napd` | principals, sessions, package verification, KV, NMP, diagnostics |
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
crates/napd/            reusable runtime, NMP and storage adapters
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
    NC --> NM[nampplets]
    NC --> NMP[NMP]

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
    D->>M: live query using public facade
    M-->>D: cached canonical rows + evidence
    D-->>H: authorized result/update
    H-->>N: target mapped frame
    M->>R: bounded network work when freshness requires
    R-->>M: signed events / EOSE / failure
    M-->>D: canonical transition
    D-->>H: authorized update
    H-->>N: target mapped frame
```

NMP owns event truth. SQLite/runtime state may store package, product, and app-KV facts only.

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

The shell uses one native window and one trusted WebView containing two sandboxed frames. This is the fastest web-projection path. It does not prove independent browser-process isolation or per-surface native resource accounting; those remain later work.
