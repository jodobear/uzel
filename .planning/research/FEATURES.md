# Feature Research

**Domain:** Linux local-first Social Home composed from sandboxed Nostr napplets
**Researched:** 2026-08-09
**Confidence:** MEDIUM — high-confidence local POC/contract evidence; primary external standards were verified through web search and classified MEDIUM.

## Scope and Baseline

This is a first-milestone feature map, not a greenfield social-client wishlist.
Existing POC behavior is a preservation constraint: Linux Tauri/Svelte shell, exact-build
napplet loading, source binding, NMP-owned read-only profile/follow state, NAP-INC
composition, bounded resources, recovery, hostile-frame tests, and Chromium plus
Weston/WebKit evidence. It must replay unchanged before the Napp seam moves.

New user-visible work is limited to Social Home profile/resource presentation
(`SLICE-SOC-01`) and social graph/feed composition (`SLICE-SOC-02`). New delivery work
is limited to POC reference/replay (`SLICE-REF-01`), canonical Nix packaging
(`SLICE-PKG-01`), and lean CI (`SLICE-CI-01`).

## Feature Landscape

### Table Stakes (Users Expect These)

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| POC replay with no trust/recovery regression | Brownfield product cannot trade already working profile/follow behavior for an unproven seam. | HIGH | **Existing POC; preserve, do not rebuild.** Test deterministic fixtures, hostile egress denial, restart/recovery, lifecycle/resource bounds, and real Weston/WebKit evidence against the Napp candidate. Confidence: HIGH. |
| Local-first assigned profile | Social Home needs an immediate, readable identity surface even while background work is unavailable. | MEDIUM | **SOC-01.** Render NMP-projected local profile first; show display name, authored profile text and avatar/resource outcome. No Uzel profile cache or freshness engine. Kind `0` is Nostr user metadata and is replaceable, so “latest known” must not mean universally current. Confidence: MEDIUM. |
| Honest state and recovery language | A stale/partial profile, graph, or feed must remain useful and must not look fresh or complete. | MEDIUM | **SOC-01/SOC-02.** Every visible data region distinguishes `local`, `refreshing`, `stale`, `partial`, `blocked`, and diagnosable error; valid local rows stay visible during refresh failure. Expose updates as non-focus-stealing accessible status messages. Confidence: MEDIUM. |
| Safe profile media/resource state | Avatar failure cannot break identity or create uncontrolled network work. | MEDIUM | **SOC-01; existing bounded-resource behavior preserved.** Use Napp/NMP mediated resource path only; show loading, unavailable, and recovered image states; cancel off-screen work and retain fixed concurrency/object-URL bounds. No raw frame network access. Confidence: HIGH. |
| Follow/social graph with profile selection | A Social Home needs an understandable path from follows to an inspected person. | HIGH | **SOC-02.** Render known follows and graph edges from NMP projection; selecting a node/row emits source-bound runtime-mediated `napplet:profile/open`, then updates profile pane. Never infer relationship completeness. Kind `3` is replaceable; graph is a snapshot/projection, not global truth. Confidence: MEDIUM. |
| Local feed and navigation | A user needs a primary social orientation surface, not only a profile card. | HIGH | **SOC-02.** Feed and navigation render available local data first, retain selection while refreshing, and show empty/partial/blocked outcomes. Feed ordering/filters must be explicitly described as local projection policy rather than an invisible global ranking claim. Confidence: HIGH for scope, MEDIUM for ecosystem expectation. |
| Keyboard and assistive-technology navigation | A graphical social product is incomplete if refresh state or selection only works by pointer. | MEDIUM | **SOC-01/SOC-02.** Use semantic controls and accessible names for profile/follow selection; focus remains stable when data refreshes. Use `role="status"` or equivalent polite status announcement for advisory updates, not an alert storm. Confidence: MEDIUM. |
| Canonical Linux Nix package | Linux user must run a reproducible installed artifact, not a checkout/dev-shell ritual. | HIGH | **PKG-01.** Expose locked `packages.uzel`, default package/app, dev shell, and checks; prove store-path startup, exact Napp runtime closure, clear protocol mismatch failure, no ambient `PATH`/source-tree dependency, and recorded closure size. Confidence: HIGH. |
| Lean, fail-closed delivery evidence | Fast feedback is expected; skipped security/native/package coverage is not evidence. | HIGH | **CI-01.** Change class selects direct fast gates; package/toolchain paths run package preflight; host/trust paths run Weston/WebKit; merge-group runs canonical Nix/full suite. Stable aggregator fails on missing, cancelled, or unexpected skipped work. Confidence: HIGH. |

### Differentiators (Competitive Advantage)

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Composable independent napplets, one visible Social Home | Profile, graph, and feed demonstrate genuine composition rather than a monolithic social UI. | HIGH | Source-bound NAP-INC connects user action to profile selection; exact-build identity and runtime-assigned authority prevent an iframe from choosing its own principal or target. Preserve bounded WebView count. Confidence: HIGH. |
| Truthful local-first degraded experience | Shows useful cached/local state while making freshness, completeness, pressure, and failure legible. | MEDIUM | This is stronger than a spinner or a false “live” badge: valid data remains, qualifiers remain visible, refresh is cancellable, and recovery is actionable. Confidence: HIGH (POC/contract) and MEDIUM (accessibility standard). |
| Source-bound trust visible in product behavior | Demonstrates sandboxed social composition without raw Nostr/network/native authority in napplet frames. | HIGH | Keep trust control observable through hostile-frame regression evidence and a developer view that reports bounded diagnostics, never secrets or internal authority tokens. Confidence: HIGH. |
| Pressure/cache developer view | Makes local-first runtime constraints demonstrable to evaluators without hiding backpressure, queue limits, or cache recovery. | MEDIUM | Compact opt-in developer surface: pressure/refresh/diagnostic summary only. It must remain a product diagnostic, not a second runtime-state store or a general admin console. Confidence: HIGH. |
| Exact Napp closure in a Nix artifact | Reproducible Uzel + compatible runtime lets Linux users and reviewers validate the real product binary. | HIGH | This is delivery differentiation for a napplet runtime: Cargo and flake pins must name same committed Napp candidate; Uzel must not discover arbitrary `nappd` from `PATH`. Confidence: HIGH. |

### Anti-Features (Commonly Requested, Often Problematic)

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Uzel-owned profile/follow/feed cache, relay client, or evidence reducer | Seems faster to make UI query and “own” its data. | Duplicates NMP truth, loses provenance/freshness/cancellation semantics, creates reconciliation bugs and false completeness. | Consume Napp/NMP product-facing projections; maintain only presentation state in Uzel. |
| Raw network, Tauri bridge, host paths, or caller-selected principal in napplet frame | Makes media/data integration appear easy. | Breaks source-bound sandbox authority and invalidates hostile-frame proof. | Bind request to trusted source before Napp; use mediated providers/resources only. |
| “Live”, complete, or globally ranked social graph/feed | Familiar social-client language. | Nostr kind `0`/`3` projections are replaceable and relay behavior varies; local read may be stale or incomplete. | Label local/latest-known, refreshing, partial, stale, and blocked states; describe any ordering as local policy. |
| Files, import/export, filesystem browser | Natural next UX request. | Requires accepted filesystem seam and activates only after Social acceptance. | Park through this milestone; start only after Social acceptance. |
| Blossom upload/download workflow | Needed for rich media eventually. | Depends on accepted filesystem plus external-signer seam; current Social scope only uses mediated resource display. | Park until filesystem and signer acceptance; do not create a “temporary” direct HTTP path. |
| Signing, authoring, key custody, wallets | Turns a read-only demo into a full client. | Adds authority, key, publication, recovery, and consent contracts outside milestone. | Read-only Social Home; defer until external signer and product authority are accepted. |
| Flatpak workaround or ambient packages | Convenient installation shortcut. | Violates canonical Nix release contract and hides undeclared dependency drift. | Fix locked Nix derivation and test resulting store-path artifact. |
| Search, media suite, ContextVM, Relatr, TUI, WASI, Android, production multi-WebView | Broadens demo surface. | Dilutes one compelling Social Home outcome and invalidates lean validation scope. | Keep linked parked issues; add only after current acceptance evidence. |

## Feature Dependencies

```text
POC replay + ownership disposition (REF-01)
    └──requires──> accepted committed Napp client/testkit SHA
                           └──requires──> Cargo pin == flake input == runtime closure
                                                  └──enables──> Social profile/resource (SOC-01)
                                                                    └──enables──> graph/feed composition (SOC-02)

Canonical Nix package (PKG-01)
    └──enables──> merge-full canonical release verification

Measured CI lanes (CI-01)
    └──requires──> PKG-01 for final merge-full package contract
    └──gates──> each social slice by changed scope

Accepted Social Home
    └──enables──> Local Files
                           └──then requires──> external signer acceptance before Blossom/authoring
```

### Dependency Notes

- **SOC-01 requires REF-01 and accepted Napp SHA:** otherwise Uzel would invent or target an uncommitted runtime contract. Pin committed client/testkit vectors first, then replay existing POC behavior against it.
- **SOC-02 requires SOC-01:** graph/feed selection needs a stable profile/resource target surface and retained selection/recovery semantics.
- **PKG-01 and CI-01 are early delivery work:** REF-01 and PKG-01 may progress in parallel; CI can measure early but its merge-full package contract consumes PKG-01.
- **Every Social UI outcome requires source binding:** trusted host supplies source/surface identity before a request reaches Napp; UI must not expose a bypass.
- **Files conflicts with first-milestone scope:** it starts after Social acceptance. Blossom, signing and authoring remain blocked by filesystem plus external-signer acceptance.

## MVP Definition

### Launch With (v1)

- [ ] **REF-01 POC preservation/replay** — baseline acceptance, hostile isolation, recovery, resource/performance/lifecycle evidence remains green after the exact Napp seam is selected.
- [ ] **PKG-01 canonical Nix artifact** — user can build/run locked Uzel and exact compatible Napp closure from store path, with no ambient runtime discovery.
- [ ] **CI-01 lean required lanes** — direct PR-fast evidence, relevant package/native preflight, merge-group canonical full gate, and a fail-closed aggregator.
- [ ] **SOC-01 profile/resource** — local profile first; readable resource status; visible and accessible stale/partial/refreshing/blocked/error qualifiers.
- [ ] **SOC-02 graph/feed composition** — local follow graph and feed/navigation, source-bound profile selection, retained useful data during refresh/recovery.

### Add After Validation (v1.x)

- [ ] **Local Files** — only after Social acceptance proves composition and selected profile/resource behavior; require a dedicated approved filesystem seam.
- [ ] **Blossom workflow** — only after Files and external signer acceptance; include explicit cache/recovery evidence, never direct-frame network shortcuts.
- [ ] **Authoring/signing** — only after separate signer, authority, consent, publication and recovery design/acceptance.

### Future Consideration (v2+)

- [ ] **Search, richer media, ContextVM, Relatr** — parked; add only with a single contextual issue and measured impact.
- [ ] **TUI, WASI, Android/native napplets, multi-WebView/compositor work** — outside Linux first-milestone demonstration and needs independent trust/performance evidence.

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Preserve/replay existing POC | HIGH | HIGH | P1 |
| Exact-pinned Napp seam + canonical Nix package | HIGH | HIGH | P1 |
| Local-first profile/resource with honest states | HIGH | MEDIUM | P1 |
| Source-bound graph/feed → profile composition | HIGH | HIGH | P1 |
| Lean CI and package/native evidence | HIGH | HIGH | P1 |
| Compact developer pressure/cache view | MEDIUM | MEDIUM | P2 |
| Local Files | MEDIUM | HIGH | P3 (post-Social) |
| Blossom, signing, authoring | MEDIUM | HIGH | P3 (separate authority milestone) |

**Priority key:** P1 = required first milestone; P2 = include only when it supports current demonstration; P3 = explicitly deferred.

## Competitor Feature Analysis

| Baseline | Relevant expectation | Uzel approach |
|----------|----------------------|---------------|
| Nostr protocol | User metadata (`kind 0`) and follows (`kind 3`) are replaceable and relay return behavior can vary. | Present NMP-owned local projections; preserve uncertainty/freshness qualifiers; do not market a complete graph as authoritative. |
| Accessible desktop-web UI | Dynamic advisory state is communicated without stealing focus. | Semantic profile/graph/feed controls plus polite status announcements for refresh/degraded state. |
| Linux application delivery | Package/check behavior is reproducible from locked inputs, not a mutable workstation. | Nix package is canonical artifact; direct commands only accelerate development/PR-fast evidence. |
| CI delivery | Superseded runs should not waste scarce resources; dependency caches are accelerators, not correctness. | Per-PR cancellation, lock-keyed caches, changed-scope lanes, and required fail-closed aggregator. |

## Sources

### Project-primary

- [Uzel project definition](../PROJECT.md) — validated POC, active milestone requirements, ownership and exclusions.
- [Social Home contract](../../docs/03-social-home.md) — profile, graph/feed, state, settings and pressure/cache scope.
- [Programme contract](../../PROGRAMME_CONTRACT.md) — product-first Napp/Uzel ownership boundary.
- [Nix package contract](../../NIX_PACKAGE.md) and [CI profile](../../docs/12-ci.md) — delivery/package acceptance.
- [Issue seed](../../docs/11-issue-seed.md) and [milestone sequencing](../../docs/10-gsd-seed.md) — slice ordering.

### External primary sources (verified; MEDIUM confidence through web-search seam)

- [NIP-01: basic protocol, kind 0 metadata and replaceable event behavior](https://github.com/nostr-protocol/nips/blob/master/01.md)
- [Nostr NIPs registry: kind 3 follows](https://github.com/nostr-protocol/nips)
- [WAI-ARIA 1.2 `status` role](https://www.w3.org/TR/wai-aria/#status)
- [W3C ARIA25 status/progress technique](https://www.w3.org/WAI/WCAG21/Techniques/aria/ARIA25)
- [Nix `flake check` manual](https://releases.nixos.org/nix/nix-2.32.3/manual/command-ref/new-cli/nix3-flake-check.html)
- [GitHub Actions concurrency](https://docs.github.com/en/actions/concepts/workflows-and-actions/concurrency) and [dependency caching](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching)

---
*Feature research for: Uzel first-milestone Linux Social Home*
*Researched: 2026-08-09*
