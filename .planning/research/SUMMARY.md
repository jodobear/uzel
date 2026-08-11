# Project Research Summary

**Project:** Uzel
**Domain:** Linux local-first Tauri napplet client, consuming exact-pinned Napp/NMP seams
**Researched:** 2026-08-09
**Confidence:** MEDIUM

## Executive Summary

Uzel is a brownfield Linux Tauri 2 + Svelte product whose first milestone proves composable, local-first Social Home behavior without becoming a second runtime, Nostr client, cache, signer, or policy layer. Experts preserve the proven trusted shell and private POC evidence, put canonical surface/layout state and the Napp client lifecycle in Rust, keep Svelte presentation-only, and consume NMP truth solely through an accepted, committed Napp client/testkit seam. Every napplet action remains source-bound at the trusted host, while untrusted frames retain opaque origins and no native, network, path, secret, or principal authority.

Gate 0 must accept one full Napp commit and prove it identically in Cargo resolution, `Cargo.lock`, flake input, `flake.lock`, and the packaged runtime closure. In parallel, make the locked Nix derivation the only Linux release artifact: it must start from its store path with a scrubbed `PATH`, no checkout dependency, exact runtime selection, and an explicit mismatch failure. CI should begin simple and measured: classify changes conservatively, run direct PR-fast evidence, require an always-run fail-closed aggregator, and run canonical package/full evidence on `merge_group`.

Main risks are ownership drift, pin/runtime drift, package-only-in-dev-shell behavior, false certainty in Social state, WebKit/source-binding regression, unbounded resource work, green CI skips, and stale review evidence. Mitigation is not new framework work: replay the POC against the accepted seam; retain byte-exact trusted assets and hostile Weston/WebKit evidence; project local data first with explicit freshness qualifiers; preserve bounded admission/cancellation; and bind every test/review record to the exact head SHA.

## Key Findings

### Recommended Stack

Preserve the locked POC stack: Rust `1.89.0` (edition 2024), Tauri `2.11.5` / API `2.11.1`, Svelte `5.56.8`, Node `>=22.12 <23`, pnpm `10.8.0`, locked `nixpkgs` `38a4887411571457d700c51c64a6e49ead2ed5ab`, WebKitGTK 4.1 + Weston, TypeScript `6.0.3`, Vite `8.1.5`, Playwright `1.62.0`, and Fallow `3.9.1`. Do not replace the shell, build tooling, NMP data plane, or existing browser/native evidence.

**Core technologies:**

- Rust + Tauri 2 — owns product surface/layout state, source binding, cancellation, and one Napp-client lifecycle.
- Svelte 5 — renders typed Uzel projections only; never owns runtime, Nostr, cache, or freshness truth.
- Accepted `napp-client` + committed testkit — sole supported consumer seam; pin a full commit, never tag/branch/private IPC.
- Nix flakes — canonical release closure; `packages.uzel`, default package/app, dev shell, and package-dependent checks must use same Napp commit as Cargo.
- WebKitGTK + Weston — real hostile-boundary evidence after host, source-binding, package, or security changes; Chromium is supplementary.
- GitHub Actions — `pull_request` PR-fast plus `merge_group` canonical full lane and one fail-closed required aggregator.

`cargo-nextest` is optional only after Gate 0 proves a material warm-p95 gain over the existing full-workspace runner; retain doctests separately. Do not add Flatpak, ambient system packages, PATH runtime discovery, unmeasured cache stacks, or an alternate frontend/native test stack.

### Expected Features

**Must have (table stakes):**

- Replay POC acceptance, recovery, bounded-resource/lifecycle, hostile isolation, and real Weston/WebKit evidence against accepted Napp seam.
- Exact-pinned Napp consumer seam and canonical Nix package with store-path, scrubbed-PATH, closure, and mismatch smoke evidence.
- Lean delivery gates: classified PR-fast evidence, package/native preflight where relevant, merge-group full package lane, and fail-closed aggregation.
- Local-first assigned profile and mediated avatar/resource state with accessible `local`, `refreshing`, `stale`, `partial`, `blocked`, and error semantics.
- Local follows/graph, feed/navigation, and source-bound profile selection via runtime-mediated INC while retaining useful state during refresh/recovery.

**Should have (competitive):**

- One visibly composable Social Home built from independent napplets rather than a monolith.
- Truthful degraded-state UX plus compact opt-in pressure/cache diagnostic view.
- Demonstrable source-bound authority and exact Napp closure in the package artifact.

**Defer (v2+ / after acceptance):**

- Local Files until Social Home acceptance and a dedicated filesystem seam.
- Blossom, authoring, signing, key custody, wallets until filesystem and external-signer/authority acceptance.
- Search, richer media, ContextVM, Relatr, TUI, WASI, Android, native napplets, multi-WebView/compositor work, FIPS, and Flatpak.

### Architecture Approach

Use ports-and-adapters with strict ownership. Svelte invokes narrow typed Rust DTOs; Rust owns surface/session records and calls only accepted Napp product APIs; trusted host maps `MessageEvent.source` to Rust-owned surfaces before routing bounded NAP/INC envelopes; Napp owns neutral runtime contract/policy; NMP owns all Nostr query, store, relay, provenance, freshness, signer, and publication behavior. Packaging and CI are architecture: dual locks feed a pin check, which gates the one Nix package and its store-path smoke.

**Major components:**

1. Svelte product presentation — local-first Social views, accessible state qualifiers, diagnostics, and actions.
2. Rust product controller — surface/layout authority, source/session binding, Napp lifecycle, bounded cancellation, and read-model projection.
3. Trusted surface host + untrusted napplet frames — approved assets and source-bound bounded message routing; frames have no Tauri/native/network authority.
4. Napp client/runtime + NMP plane — exact-pinned product events and all runtime/Nostr truth; Uzel never imports internals or persists duplicate truth.
5. Nix package + CI classifier/aggregator — one reproducible closure and evidence selection that fails closed.

### Critical Pitfalls

1. **Ownership drift into Uzel** — begin each request from one visible outcome; require a minimal neutral Napp contract/testkit vector; stop before merge if ownership is ambiguous.
2. **Cargo/flake/fixture/runtime pin drift** — machine-check one full accepted commit across manifests, locks, fixtures, and packaged closure; fail closed and deterministically repin.
3. **Dev-shell-only package** — reference Napp-owned package output, never copied daemon/unit or `PATH`; smoke `result/bin` with sanitized environment and no checkout.
4. **False Social truth or unbounded work** — render runtime-projected local data first, expose state/freshness, preserve caps/cancellation/object-URL cleanup, and measure control latency/RSS.
5. **Isolation, CI, or review evidence false green** — preserve opaque sandbox/source mapping/CSP/navigation denial; run affected real WebKit proof; aggregator rejects skipped/cancelled/missing lanes; evidence must cite final head SHA.

## Implications for Roadmap

### Phase 1: REF-01 — POC replay and accepted Napp seam

**Rationale:** Defines legal consumer boundary before product change; Social work without it would invent private contracts.
**Delivers:** Ownership disposition, accepted committed Napp client/testkit candidate, POC replay, exact trusted-asset/fixture evidence, and baseline resource/lifecycle/performance evidence.
**Addresses:** Existing POC preservation; exact-pinned consumer seam.
**Avoids:** Uzel-owned runtime/Nostr truth, private-protocol promotion, pin/fixture drift, and source-binding regression.

### Phase 2: PKG-01 — Canonical Nix release closure

**Rationale:** Independent early delivery work once candidate pin evidence exists; establishes the real artifact before CI and Social rely on it.
**Delivers:** Dual Cargo/flake pin, deterministic `pins/check`, package/default app/check outputs, store-path launch, scrubbed-PATH/closure/mismatch smoke, and closure-size record.
**Addresses:** Canonical Linux delivery and exact compatible Napp runtime.
**Avoids:** Ambient dependencies, copied daemon/unit, dev-shell-only success, and Cargo/Nix mismatch.

### Phase 3: CI-01 — Measured fail-closed delivery lanes

**Rationale:** Implement PR-fast classification and aggregator early, but make final package preflight/merge-full consume PKG-01 rather than a CI-only build.
**Delivers:** Baseline p50/p95 measurements, conservative union-of-classes classifier fixtures, direct fast lanes, package/native path gates, exact-head evidence records, and `merge_group` canonical full lane.
**Addresses:** Lean feedback plus delivery confidence for every following slice.
**Avoids:** Required path-filter skips, cache-as-proof, missing merge-queue checks, stale reviews, and unaudited native/package changes.

### Phase 4: SOC-01 — Local profile, resources, and honest state

**Rationale:** Requires accepted Napp/M0 gates; creates stable profile/resource target and truthful recovery semantics needed by composition.
**Delivers:** Napp-projected local profile first, mediated avatar/resource state, visible accessible freshness/pressure diagnostics, bounded/cancellable work, and fixture transitions.
**Addresses:** Assigned profile, resource display, and local-first degraded UX.
**Avoids:** Duplicate Uzel cache/direct fetch, spinner-erased local data, account/auth claims, and uncapped resource work.

### Phase 5: SOC-02 — Source-bound graph, feed, and navigation

**Rationale:** Builds only after SOC-01 has stable profile and recovery state; extends same projections instead of adding another data path.
**Delivers:** Local follow graph, feed/navigation, source-bound INC profile selection, retained selection/recovery, and affected real-WebKit proof.
**Addresses:** Composable Social Home visible outcome.
**Avoids:** Direct napplet peering, caller-selected authority, false global completeness/ranking, WebView growth, and host-boundary regressions.

### Phase Ordering Rationale

- REF is non-negotiable gate for Social; it establishes source, ownership, exact Napp API/testkit, and preserved POC proof.
- REF and PKG can progress independently; CI can measure early but its final package/merge-full contract depends on PKG.
- SOC-01 precedes SOC-02 because graph/feed selection needs retained profile/resource and state-recovery semantics.
- Keep all parked authority/filesystem/product-expansion work outside this roadmap until Social acceptance.

### Research Flags

Phases likely needing deeper research during planning:

- **REF-01:** Required. Validate live accepted Napp public seam, testkit vectors, exact source/provenance, activation/mismatch behavior, and current POC replay.
- **PKG-01:** Required. Verify actual Napp flake outputs, Nix wrapper/activation contract, clean store-path launch, closure, and mismatch handling.
- **CI-01:** Required. Measure current workflow p50/p95 and inspect live GitHub required-check/merge-queue configuration before choosing scope/caches.
- **SOC-01:** Required. Confirm accepted product events for profile/resource/freshness/diagnostics and benchmark bounded-resource behavior on real fixtures.
- **SOC-02:** Required only for changed INC/host navigation surface; validate versioned payload and real WebKit boundary evidence.

Phases with standard patterns (skip research-phase once live contracts are accepted):

- **Svelte rendering/accessibility portions of SOC-01/SOC-02:** typed DTO rendering, semantic controls, and polite status announcements are established patterns.
- **Baseline Nix flake outputs and GitHub `merge_group` wiring:** standard documented mechanisms; still validate project-specific values and evidence.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | MEDIUM | Locked POC versions and official tool docs are specific; accepted Napp revision and lane timing are intentionally Gate 0. |
| Features | MEDIUM | Milestone scope and POC contract are primary; Social ecosystem/accessibility expectations require implementation validation. |
| Architecture | MEDIUM | Strong project-primary ownership/trust evidence; actual accepted Napp seam and package activation remain unproven. |
| Pitfalls | MEDIUM | Detailed primary POC risks plus official docs; regressions, resource budgets, and CI economics need current-head measurement. |

**Overall confidence:** MEDIUM

### Gaps to Address

- **Accepted Napp identity and public surface:** record one full candidate SHA only after source, exported client/testkit vectors, package outputs, and incompatibility behavior replay successfully.
- **Canonical package activation:** validate Napp-owned generic-Linux lifecycle/activation semantics; Uzel must not copy or own daemon service artifacts.
- **Measured CI design:** capture current warm/cold p50/p95, cache costs, and classifier fixtures before adding nextest, caches, or affected-crate selection.
- **Social read-model contract:** validate concrete profile/follow/feed/resource/freshness/pressure events, explicit local ordering policy, and state transitions against accepted runtime.
- **Acceptance budgets:** carry forward exact surface/resource/retry/object-URL limits and establish measured control-latency/RSS thresholds before Social capacity changes.

## Sources

### Primary (HIGH confidence)

- [Uzel project definition](../PROJECT.md) — scope, gates, ownership, locked stack, and exclusions.
- [Stack research](STACK.md) — versioned baseline, pin/closure, package, and CI recommendations.
- [Feature research](FEATURES.md) — MVP scope, dependencies, differentiators, and explicit deferrals.
- [Architecture research](ARCHITECTURE.md) — component ownership, data flow, trust boundaries, and phase order.
- [Pitfalls research](PITFALLS.md) — stop conditions, prevention evidence, resource/CI/review risks.

### Secondary (MEDIUM confidence)

- Official Cargo, Nix, GitHub Actions, Tauri, Nostr NIP-01, WAI-ARIA, and cargo-nextest documentation cited in the four research files — stack behavior and external standards.

---
*Research completed: 2026-08-09*
*Ready for roadmap: yes*
