# Uzel

## What This Is

Uzel is a Linux Tauri 2 + Svelte product for composable, local-first napplets. It preserves the proven POC, owns its product runtime and composition policy, and turns that foundation into a compelling Social Home experience.

## Core Value

Uzel must make local-first napplet composition visibly useful without duplicating Nostr truth owned by NMP.

## Requirements

### Validated

- ✓ A Linux Tauri/Svelte shell launches exact-build signed napplets through Uzel's private daemon boundary — existing POC
- ✓ Trusted host source binding prevents untrusted napplet frames from receiving raw network or native Tauri authority — existing POC
- ✓ Uzel's daemon and NMP plane supply read-only profile/follow data, bounded resources, and cache-first recovery — existing POC
- ✓ Independent profile and follow napplets compose through runtime-mediated NAP-INC routing — existing POC
- ✓ Deterministic Chromium tests and real Weston/WebKit hostile-egress evidence cover the core trust boundary — existing POC
- ✓ A pinned Nix development environment supports the current Rust, frontend, Tauri, and WebKit workflow — existing POC

### Active

- [ ] Replay the merged POC and record durable Uzel/upstream ownership, state-retention, and resource-bound dispositions
- [ ] Preserve current acceptance, hostile-isolation, recovery, lifecycle, performance, and resource evidence at Uzel's trusted runtime boundary
- [ ] Produce a reproducible canonical Nix Uzel package that runs from its store path with the exact compatible native runtime dependencies
- [ ] Prove measured PR-fast, path-gated native/package preflight, and merge-group full CI lanes with fail-closed review evidence
- [ ] Preserve assigned-profile, avatar/resource, follow-row, and profile-selection value; add only evidenced stale/partial/refreshing/diagnostic gaps
- [ ] Add missing graph/feed/navigation value through source-bound runtime-mediated composition without rebuilding existing follow/profile paths

### Out of Scope

- Local Files — activates only after Social Home acceptance
- Blossom and local-first authoring — require accepted filesystem and external-signer seams first
- Signing, wallets, and private-key custody — not part of this read-oriented milestone
- FIPS, media, ContextVM, Relatr, search, TUI, WASI, Android, and native napplets — parked beyond the first milestone
- Flatpak packaging — Nix is the canonical Linux build and installation artifact
- A duplicate Nostr engine, relay pool, signer, grants store, or cache — violates NMP ownership
- Public daemon protocol or production multi-WebView/compositor work — not required for the scoped Linux product milestone

## Context

The brownfield POC contains the Tauri shell, Svelte presentation, Uzel-owned private AF_UNIX daemon, exact-build fixtures, trusted surface host, rich NMP-backed profile/follow paths, NAP-INC profile selection, bounded resource handling, recovery behavior, hostile-frame tests, deterministic Chromium coverage, and real Weston/WebKit smoke evidence. Production baseline `19519c3` is merged. The nested POC pack is bounded replay evidence, not active workflow or architecture authority.

The first milestone has five contextual slices: `SLICE-REF-01`, `SLICE-PKG-01`, `SLICE-CI-01`, `SLICE-SOC-01`, and `SLICE-SOC-02`. Phase 1 validates the current product boundary before package, CI, and Social work. GitHub issues define delivery outcomes, GSD is the execution record, and [`WORKFLOW.md`](../WORKFLOW.md) is process authority.

## Constraints

- **Product stack**: Tauri 2 + Svelte remains locked
- **Ownership**: Uzel owns product runtime, composition policy, trusted surfaces, and its private daemon; Rust owns canonical runtime/layout state and Svelte owns presentation
- **Current upstreams**: `jodobear/nampplets` supplies exact-pinned native runtime crates; `pablof7z/nmp` is Nostr authority; other repositories matter only when current source or a concrete reusable fix proves relevance
- **Nostr boundary**: NMP is the sole Nostr query, relay, canonical-store, signer, freshness, provenance, and publication plane
- **Trust**: Every napplet request is bound to a trusted source-created surface before Uzel's runtime handles it; untrusted surfaces receive no Tauri bridge, secrets, host paths, raw network, or caller-selected principal
- **Dependency pinning**: Cargo and locks identify exact current native-runtime and NMP commits; arbitrary daemon discovery from `PATH` is forbidden
- **Linux release**: The reproducible Nix derivation is canonical; ambient host packages and Flatpak workarounds cannot supply missing dependencies
- **Responsiveness**: Local data renders first; UI-thread work stays non-blocking; WebView count, queues, retries, streams, tasks, resources, and subprocesses stay bounded and cancellable
- **Compatibility**: Preserve exact-build identity, current-green fixtures, POC acceptance, and real WebKit evidence while changing one ownership axis at a time
- **Current scope**: One Uzel instance and one active read profile are the proven baseline; record collisions but defer multi-instance/multi-profile implementation
- **Naming**: Project-owned identifiers are descriptive and at most 21 characters, with narrow documented exceptions
- **Delivery and review**: Follow [`WORKFLOW.md`](../WORKFLOW.md)

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Preserve and replay the POC instead of rebuilding Uzel | Existing source already proves critical runtime, trust, recovery, profile/follows, and composition behavior | Re-audited at `19519c3`; current replay must close remaining native observations |
| Uzel owns its product runtime and composition policy | Matches current source and avoids an artificial external dependency | ✓ Architecture corrected in Phase 1 |
| Keep only source-proven upstream authority | Prevents speculative framework and ownership work | ✓ nampplets and NMP proven by Cargo source/lock |
| Make Nix the canonical Linux package and release closure | Produces a reproducible artifact with exact compatible dependencies | — Pending |
| Measure simple full-workspace CI before affected-crate complexity | Complexity must earn its cost through real evidence | — Pending |
| Keep GitHub as merge authority and GSD as execution record | Separates local orchestration from required checks and canonical merge state | — Pending |

---
*Last updated: 2026-08-14 during Phase 1 product reorientation*
