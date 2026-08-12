# Uzel

## What This Is

Uzel is a Linux Tauri 2 + Svelte graphical product for composable, local-first napplets. It preserves the proven Uzel POC while moving runtime contracts and policy behind one exact-pinned neutral Napp client/runtime seam, then turns that foundation into a compelling Social Home experience.

## Core Value

Uzel must make local-first napplet composition visibly useful without duplicating runtime, Nostr, trust, or persistence truth owned by Napp and NMP.

## Requirements

### Validated

- ✓ A Linux Tauri/Svelte shell launches exact-build signed napplets through a private daemon boundary — existing POC
- ✓ Trusted host source binding prevents untrusted napplet frames from receiving raw network or native Tauri authority — existing POC
- ✓ One daemon-owned runtime/NMP plane supplies read-only profile and follow data, bounded resources, and cache-first recovery — existing POC
- ✓ Independent profile and follow napplets compose through runtime-mediated NAP-INC routing — existing POC
- ✓ Deterministic Chromium tests and real Weston/WebKit hostile-egress evidence cover the core trust boundary — existing POC
- ✓ A pinned Nix development environment supports the current Rust, frontend, Tauri, and WebKit workflow — existing POC

### Active

- [ ] Replay the merged POC from a clean or relocated target and record durable Uzel/Napp/upstream ownership, state-retention, and resource-bound dispositions
- [ ] Preserve current acceptance, hostile-isolation, recovery, lifecycle, performance, and resource evidence while seams move behind Napp
- [ ] Pin one accepted committed Napp client/testkit revision consistently across Cargo, lockfiles, and the Nix runtime closure
- [ ] Produce a reproducible canonical Nix Uzel package that runs from its store path with the exact compatible Napp runtime
- [ ] Prove measured PR-fast, path-gated native/package preflight, and merge-group full CI lanes with fail-closed review evidence
- [ ] Preserve existing assigned-profile, avatar/resource, follow-row, and profile-selection value through the accepted seam; add only evidenced stale/partial/refreshing/diagnostic gaps
- [ ] Add missing graph/feed/navigation value through source-bound runtime-mediated composition without rebuilding existing follow/profile paths

### Out of Scope

- Local Files — activates only after Social Home acceptance
- Blossom and local-first authoring — require accepted filesystem and external-signer seams first
- Signing, wallets, and private-key custody — not part of this read-oriented milestone
- FIPS, media, ContextVM, Relatr, search, TUI, WASI, Android, and native napplets — parked beyond the first milestone
- Flatpak packaging — Nix is the canonical Linux build and installation artifact
- A Uzel-owned Napp substitute, Nostr engine, relay pool, signer, grants store, or duplicate cache — violates ownership boundaries
- Public daemon protocol or production multi-WebView/compositor work — not required for the scoped Linux product milestone

## Context

The brownfield POC already contains the Tauri shell, Svelte presentation, private AF_UNIX daemon protocol, exact-build fixtures, trusted surface host, rich NMP-backed profile/follow paths, NAP-INC profile selection, bounded resource handling, recovery behavior, hostile-frame tests, deterministic Chromium coverage, and real Weston/WebKit smoke evidence. Production baseline `19519c3` is merged; the nested status that still calls PR #30 active is historical, while its Debian 13 visible-acceptance gap remains unresolved. The lean roadmap preserves visible value and makes Nix packaging and fail-closed delivery outcomes rather than rebuilding the product or front-loading a generic runtime framework.

The first milestone is organized as five contextual slices: `SLICE-REF-01`, `SLICE-PKG-01`, `SLICE-CI-01`, `SLICE-SOC-01`, and `SLICE-SOC-02`. REF replay/ownership/document admission may proceed before Napp; its adapter lane waits for an accepted candidate. Uzel-only package research and CI measurement may run concurrently, but the canonical package needs the Napp output and final CI needs that package. New Social implementation waits for accepted REF, PKG, and CI gates.

GitHub issues define scoped delivery outcomes, GSD is the repository-local execution
record, and GitHub remains merge authority. [`WORKFLOW.md`](../WORKFLOW.md) is the sole
delivery-process authority.

## Constraints

- **Product stack**: Tauri 2 + Svelte remains locked — preserve the proven Linux product shell
- **Ownership**: Rust owns canonical surface/layout state and Napp client lifecycle; Svelte owns presentation — avoid split runtime truth
- **Runtime boundary**: Uzel consumes `napp-client`, product-facing events, and committed testkit vectors only — Napp owns runtime contracts and policy
- **Nostr boundary**: NMP is the sole Nostr query, relay, canonical-store, signer, freshness, provenance, and publication plane — no duplicate data plane
- **Trust**: Every napplet request is source-bound before reaching Napp; untrusted surfaces receive no Tauri bridge, secrets, host paths, raw network, or caller-selected principal
- **Dependency pinning**: Cargo and flake inputs must identify the same accepted Napp commit; arbitrary `nappd` discovery from `PATH` is forbidden
- **Linux release**: The reproducible Nix derivation is canonical; ambient host packages and Flatpak workarounds cannot supply missing dependencies
- **Responsiveness**: Local data renders first; UI-thread work stays non-blocking; WebView count, queues, retries, streams, tasks, resources, and subprocesses stay bounded and cancellable
- **Compatibility**: Preserve exact-build identity, current-green fixtures, POC acceptance, and real WebKit evidence while changing one dependency or ownership axis at a time
- **Current scope**: One Uzel instance and one active read profile are the proven baseline; record collisions but defer multi-instance/multi-profile implementation
- **Migration**: Qualify, adapt one Rust/Tauri boundary, replay parity, then retire compatibility code; no dual store/write and rollback by adapter/pin revert
- **Naming**: Project-owned identifiers are descriptive and at most 21 characters, with only narrow documented exceptions
- **Delivery and review**: Follow [`WORKFLOW.md`](../WORKFLOW.md); this product document does
  not duplicate process rules

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Preserve and replay the POC instead of rebuilding Uzel | Existing code already proves critical runtime, trust, recovery, rich profile/follows, and composition behavior | Re-audited at `19519c3`; current replay material is audit/WIP evidence, not accepted exact-build replay and not authority for Napp transition |
| Build only neutral Napp seams proven by visible Uzel slices | Keeps product policy in Uzel and avoids an abstract framework milestone | — Pending |
| Make Nix the canonical Linux package and release closure | Produces a reproducible artifact with exact compatible runtime dependencies | — Pending |
| Measure simple full-workspace CI before adding affected-crate complexity | Complexity must earn its cost through real p50/p95 evidence | — Pending |
| Split REF into independent preservation and blocked adapter lanes | Keeps useful brownfield work moving without inventing the absent Napp seam | ✓ Accepted by Phase 1 re-audit |
| Gate new Social work on an accepted committed Napp candidate and M0 delivery evidence | Prevents private contract invention and protects the existing POC baseline | — Pending |
| Keep GitHub as merge authority and GSD as execution record | Separates local orchestration from required checks and canonical merge state | — Pending |

---
*Last updated: 2026-08-12 during lean-process reset*
