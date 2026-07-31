# POC scope and acceptance

## Objective

Prove that a native Linux shell can run unchanged, self-contained web napplets as untrusted components while a Rust daemon owns exact-build identity, lifecycle, Nostr access, private KV, inter-napplet routing, and diagnostics.

“Native” means a Linux desktop process, Rust daemon, Tauri shell, Svelte UI, Nix build, and WebKit-hosted napplets. Native-code napplets are not part of this POC.

## User flow

1. Start Uzel.
2. Enter a public hex pubkey or `npub`.
3. Uzel launches two verified fixture napplets.
4. `follow-list` displays direct follows using runtime-mediated Nostr reads.
5. Selecting one emits `napplet:profile/open` over NAP-INC.
6. `profile-card` independently queries and displays that pubkey's latest-known kind `0`.
7. Restart Uzel and retain the selected identity, layout, napplet KV, and NMP cache.
8. Paste one signed kind `35129` naddr, review its exact publisher/build and
   requested capabilities, then explicitly approve or deny installation and launch.

The public key is a **read context**, not login or authentication. No secret key is accepted.

## Two acceptance levels

### Demo-complete

The first public demonstration is complete when:

- `uzel-napd` and the Uzel shell run as separate processes;
- one trusted WebView hosts two sandboxed napplet frames;
- both exact fixture builds verify and start;
- the split layout supports focus and divider resize;
- NAP-SHELL initializes both frames;
- selecting a follow updates `profile-card` through NAP-INC;
- deterministic signed Nostr fixtures render through one NMP engine;
- a small developer drawer shows sessions, NAP traffic, NMP demand, and errors.

### Foundation-complete

The POC is ready to build upon when, in addition:

- live-relay mode works with a configured public identity;
- NMP remains the only Nostr store/cache and preserves provenance;
- app KV and shell state survive restart;
- hostile napplet tests prove no Tauri bridge and deny direct browser egress under the tested host policy;
- Fedora run and Debian build smoke pass from a clean checkout;
- repository quality gates pass;
- extraction boundaries remain clean.

A Bubblewrap networkless-shell proof is valuable but does not block the fast demo. If it is not completed, it remains an explicit production-hardening item rather than being faked.

### Accepted post-foundation extension

The accepted POC may load one caller-supplied signed naddr through a bounded
review/confirm flow. Nampplets/NMP resolve the coordinate, verify the signed
manifest and exact artifact bytes, and own installation/session lifecycle;
Uzel owns only user consent and presentation. This proves portable exact-build
loading. It is not catalog browsing, discovery, unattended installation, or an
update system.

## Required napplets

| Napplet | Owns | Does not own |
|---|---|---|
| `follow-list` | ordered direct-follow list, NMP-resolved names, visible NAP-RESOURCE pictures, selected-pubkey emission | full profile view, replacement selection, ranking, follow mutation |
| `profile-card` | one active or selected complete latest-known canonical kind `0`, friendly summary, and available provenance | follow list, profile editing, direct remote asset fetching, napplet-owned freshness claims |
| `hostile-egress` | test-only probes | product UI |

No god napplet and no hidden direct coupling.

## Architectural invariants

1. Reuse `nampplets` runtime semantics unless Gate 0 proves a specific gap.
2. NMP is the sole Nostr network and fact plane.
3. Exact-build identity is runtime-assigned from verified publisher bytes.
4. Napplet messages are source-bound by the trusted host.
5. Napplet networking is runtime-mediated; direct browser egress is denied in the tested projection.
6. NAP-INC is the only cross-napplet path in the demo.
7. Runtime metadata is not a second Nostr database.
8. Product policy does not leak into reusable `napd` code.

## Non-goals

- FIPS, ContextVM, Relatr, Open Ranking, wallets, signers, payments;
- Blossom, files, mounts, clipboard, media focus;
- browsable app catalogs, discovery, unattended installation, updates;
- extension/plugin protocol;
- native/WASI napplets or Android;
- Plasma widgets, tray, notifications, MPRIS;
- Hyprland/KWin/X11 adapters or a custom compositor;
- workspaces, overview, tags, Lua, or a full WM;
- public/stable daemon IPC;
- production multi-WebView isolation.

## Stop rule

When a requested feature is not required by the acceptance levels above, record it in `docs/06-extraction.md` and do not implement it in this POC.
