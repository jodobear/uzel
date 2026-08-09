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

- [ ] Replay the POC from current source and record the durable Uzel/Napp/upstream ownership disposition
- [ ] Preserve current acceptance, hostile-isolation, recovery, lifecycle, performance, and resource evidence while seams move behind Napp
- [ ] Pin one accepted committed Napp client/testkit revision consistently across Cargo, lockfiles, and the Nix runtime closure
- [ ] Produce a reproducible canonical Nix Uzel package that runs from its store path with the exact compatible Napp runtime
- [ ] Prove measured PR-fast, path-gated native/package preflight, and merge-group full CI lanes with fail-closed review evidence
- [ ] Present assigned profile and avatar/resource state from local data first, with honest stale, partial, refreshing, blocked, and diagnostic states
- [ ] Present follows/social graph, feed, navigation, and profile selection through source-bound runtime-mediated composition

### Out of Scope

- Local Files — activates only after Social Home acceptance
- Blossom and local-first authoring — require accepted filesystem and external-signer seams first
- Signing, wallets, and private-key custody — not part of this read-oriented milestone
- FIPS, media, ContextVM, Relatr, search, TUI, WASI, Android, and native napplets — parked beyond the first milestone
- Flatpak packaging — Nix is the canonical Linux build and installation artifact
- A Uzel-owned Napp substitute, Nostr engine, relay pool, signer, grants store, or duplicate cache — violates ownership boundaries
- Public daemon protocol or production multi-WebView/compositor work — not required for the scoped Linux product milestone

## Context

The brownfield POC already contains the Tauri shell, Svelte presentation, private AF_UNIX daemon protocol, exact-build fixtures, trusted surface host, NMP-backed profile/follow paths, NAP-INC composition, bounded resource handling, recovery behavior, hostile-frame tests, deterministic Chromium coverage, and real Weston/WebKit smoke evidence. The re-audited plan makes Nix packaging and lean CI first-class delivery contracts rather than rebuilding the product or front-loading a generic runtime framework.

The first milestone is organized as five contextual slices: `SLICE-REF-01`, `SLICE-PKG-01`, `SLICE-CI-01`, `SLICE-SOC-01`, and `SLICE-SOC-02`. REF and PKG may proceed independently; CI may measure current lanes concurrently but its final package/merge-full contract consumes PKG. Social implementation waits for an accepted committed Napp candidate and the required M0 delivery gates.

GitHub issues define scoped delivery outcomes, GSD is the repository-local execution record, and GitHub remains merge authority. Each PR carries one visible outcome, declares its CI class, runs bounded local review and PR-fast evidence, and enters the merge queue only after exact-head review evidence is clean.

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
- **Naming**: Project-owned identifiers are descriptive and at most 21 characters, with only narrow documented exceptions
- **Delivery**: One contextual issue, one visible outcome, one owning worktree, one PR; out-of-scope findings move to linked future issues
- **Review**: Local CodeRabbit, remote Codex, final remote CodeRabbit, exact-head evidence, then merge queue; missing, stale, skipped, failed, or timed-out review is not approval

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Preserve and replay the POC instead of rebuilding Uzel | Existing code already proves critical runtime, trust, recovery, and composition behavior | — Pending |
| Build only neutral Napp seams proven by visible Uzel slices | Keeps product policy in Uzel and avoids an abstract framework milestone | — Pending |
| Make Nix the canonical Linux package and release closure | Produces a reproducible artifact with exact compatible runtime dependencies | — Pending |
| Measure simple full-workspace CI before adding affected-crate complexity | Complexity must earn its cost through real p50/p95 evidence | — Pending |
| Gate Social work on an accepted committed Napp candidate and M0 delivery evidence | Prevents private contract invention and protects the existing POC baseline | — Pending |
| Keep GitHub as merge authority and GSD as execution record | Separates local orchestration from required checks and canonical merge state | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `$gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `$gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-08-09 after initialization*
