# Source baseline

> Snapshot: 2026-07-28. Gate 0 replaces branch-level observations with exact commits and command output.

## Observed facts

### Napplet packages

The release page lists convention-capable packages including:

```text
@napplet/core@0.29.0
@napplet/nap@0.29.0
@napplet/sdk@0.25.0
@napplet/conformance@0.14.0
@napplet/conformance-cli@0.2.16
```

The release notes describe queryless exact identities, URI-to-payload transposition, runtime-attested senders, and source-independent target delivery.

Source: <https://github.com/napplet/web/releases>

### Kehto #204

At this snapshot, GitHub displays PR #204 as draft. Its stated work covers convention conformance, shell session integrity, identity/theme parity, exact queryless INC routing, runtime-attested senders, and symmetric channel lifecycle.

Start condition: verify and pin the merge commit before implementation.

Source: <https://github.com/kehto/web/pull/204>

### `nampplets`

The repository describes an in-progress native Rust runtime with a macOS reference host. It states that NMP is the sole Nostr engine while the runtime owns installation, capabilities, WebView isolation, workspace bindings, and product policy. The documented host is a trusted WebView shell with a sandboxed napplet iframe and exact-build installation.

Source: <https://github.com/pablof7z/nampplets>

### NMP

NMP describes an embeddable Nostr engine with canonical redb state, provenance-preserving deduplication, replaceable-event handling, freshness, bounded delivery, finite relay fan-out, NIP-65 routing, and NIP-02 following.

Source: <https://github.com/pablof7z/nmp>

### NAP registry

The registry defines the runtime capability seam and web projection. `shell`, `intent`, `inc`, and `theme` are active; `relay`, `identity`, and `storage` are draft at this snapshot.

Source: <https://github.com/napplet/naps>

## Pin record

Gate 0 creates `compatibility.lock` with exact values for:

```text
Kehto #204 merge commit
Napplet package lock and release commit
NIP-5A / NIP-5D / NAP registry commits
NAP-INC and pinned draft relay/identity/storage revisions
nampplets commit
NMP commit
Rust, Node, package manager, Tauri, WebKitGTK, Nix, Fallow
```

Branches and “latest” are not reproducible pins.
