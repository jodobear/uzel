# Requirements: Uzel

**Defined:** 2026-08-09
**Core Value:** Uzel must make local-first napplet composition visibly useful without duplicating Nostr truth owned by NMP.

## v1 Requirements

Requirements for the first Uzel milestone. Each maps to exactly one roadmap phase.

### POC Reference and Runtime Recovery

- [x] **REF-01**: Developer can replay exact-build review, confirmation, launch, rich profile/follow rendering, and multi-surface composition from the current merged POC at its exact current pins.
- [ ] **REF-02**: Release reviewer can verify every napplet request is bound to the trusted source-created surface before Uzel's private runtime handles it, with no raw frame network, Tauri bridge, host path, secret, or caller-selected principal.
- [ ] **REF-03**: Operator can restart Uzel and recover selected read identity, installed exact builds, useful local profile/follow state, and ambiguous lifecycle outcomes without duplicate runtime or Nostr state.
- [ ] **REF-04**: Developer can replay deterministic Chromium and real Weston/WebKit hostile-egress, native-bridge denial, recovery, and exact-fixture evidence without regression.
- [x] **REF-05**: Maintainer can inspect a durable ownership map classifying retained Uzel code, current source-proven upstreams, compatibility seams, and obsolete POC-only behavior.
- [x] **REF-06**: Maintainer can inspect a small reproducible baseline for startup/ready time, local profile render, process/WebView pressure, resource flow, queue bounds, cancellation, and lifecycle recovery, with build/Nix materialization separated from runtime and unsupported measurements marked unavailable.

### Canonical Nix Package

- [ ] **PKG-01**: Linux user can build `packages.uzel` and the default package/app from locked inputs on `x86_64-linux`, with the development shell and package-dependent checks exposed by the flake.
- [ ] **PKG-02**: Release reviewer can prove Cargo resolution, `Cargo.lock`, flake inputs, `flake.lock`, fixtures, and packaged runtime closure identify the same accepted native-runtime and NMP commits.
- [ ] **PKG-03**: Linux user can start the packaged Uzel artifact from its Nix store path without the checkout, development shell, ambient system packages, or arbitrary daemon discovery from `PATH`.
- [ ] **PKG-04**: Operator receives a clear failure when packaged Uzel runtime components are incompatible instead of an incompatible daemon starting silently.
- [ ] **PKG-05**: Release reviewer can inspect package closure contents and size, exact native-runtime/NMP references, desktop assets, and path-relevant package/WebKit smoke evidence.

### Lean Delivery and Review

- [ ] **CI-01**: Contributor receives changed-scope PR-fast results for the conservative union of docs, frontend, Rust, contract, native-host/security, and package/toolchain change classes.
- [ ] **CI-02**: Contributor receives direct formatter, typecheck, unit, Fallow, Rust, and targeted Chromium results only when affected, with uncertain/shared inputs promoted to full scope.
- [ ] **CI-03**: Packaging/toolchain changes run canonical Nix package preflight on the PR head, while host/source-binding/security/native/package paths run the applicable Weston/WebKit proof.
- [ ] **CI-04**: Merge queue validates the exact `merge_group` SHA using the canonical Nix package and full required suite before GitHub merges.
- [ ] **CI-05**: One stable required aggregator fails on missing, cancelled, failed, or unexpectedly skipped classified jobs rather than treating path-filter silence as success.
- [ ] **CI-06**: Maintainer can inspect measured cold/warm p50/p95, cache restore/save cost, cancellation behavior, and full-workspace versus affected-scope Rust evidence before retaining CI complexity.
- [ ] **CI-07**: Merge authority can verify local CodeRabbit, remote Codex, final CodeRabbit, PR-fast, applicable preflight, disposition, and zero-blocker evidence is bound to the required exact head SHA.

### Profile and Resource Experience

The merged POC already demonstrates substantial PROF behavior. These requirements preserve
that behavior through Uzel's current runtime boundary and close only evidenced product gaps; they do
not authorize a profile/resource rebuild.

- [ ] **PROF-01**: User sees the assigned profile from NMP-projected local state before background refresh completes, without a duplicate Uzel profile or freshness cache.
- [ ] **PROF-02**: User sees mediated profile image/resource loading, success, failure, cancellation, and recovery while existing concurrency, byte, retry, object-URL, and viewport cleanup bounds remain enforced.
- [ ] **PROF-03**: User can distinguish local, refreshing, stale, partial, blocked, and diagnosable error states while valid local profile data stays visible through refresh failure.
- [ ] **PROF-04**: Keyboard and assistive-technology users can inspect and operate profile/resource state with stable focus, semantic controls, accessible names, and non-disruptive status announcements.
- [ ] **PROF-05**: Developer can opt into compact pressure, cache, freshness, and failure diagnostics without exposing secrets, authority tokens, raw runtime internals, or creating a second state owner.

### Social Graph, Feed, and Navigation

The merged POC already demonstrates follow rows and source-bound profile selection. Phase 5
adds graph/feed value and revalidates existing composition; it does not recreate existing
follow/profile paths.

- [ ] **SOC-01**: User sees known follows and social graph edges from NMP-projected local state first, with explicit incomplete/stale/refreshing semantics rather than a global-completeness claim.
- [ ] **SOC-02**: User sees a local feed and navigation surface that retains useful items and selection through refresh, partial results, failure, and recovery, with ordering described as local projection policy.
- [ ] **SOC-03**: User can select a profile from graph/feed navigation through a source-bound runtime-mediated `napplet:profile/open` interaction that cannot choose its own principal or bypass the trusted host.
- [ ] **SOC-04**: Profile, graph, and feed remain independent exact-build napplets composed into one visible Social Home without direct frame peering or duplicated runtime/Nostr data paths.
- [ ] **SOC-05**: Keyboard and assistive-technology users can navigate graph, feed, and profile selection with stable focus and accessible state changes.
- [ ] **SOC-06**: Release reviewer can verify bounded WebView, queue, subscription, resource, retry, cancellation, recovery, hostile-egress, and affected real-WebKit behavior for the completed Social Home.

## v2 Requirements

Deferred beyond the first milestone and excluded from the current roadmap.

### Local Files

- **FILE-01**: User can browse and operate on local files through an accepted dedicated filesystem seam after Social Home acceptance.

### Blossom and Authoring

- **BLOS-01**: User can complete a mediated Blossom round trip after filesystem and external-signer acceptance.
- **AUTH-01**: User can author and publish through an accepted external signer, consent, recovery, and publication contract without Uzel holding private keys.

### Later Platforms and Capabilities

- **LATR-01**: User can access separately scoped search, richer media, ContextVM, Relatr, TUI, WASI, Android, or native-napplet capabilities after independent trust and performance evidence.

## Out of Scope

Explicit exclusions prevent first-milestone scope creep.

| Feature | Reason |
|---------|--------|
| Local Files implementation | Activates only after Social Home acceptance and an approved filesystem seam |
| Blossom, signing, wallets, key custody, and authoring | Require accepted filesystem, external-signer, consent, publication, and recovery contracts |
| FIPS, richer media, ContextVM, Relatr, search, TUI, WASI, Android, and native napplets | Parked beyond the first milestone |
| Flatpak packaging | Nix is the canonical Linux artifact |
| Duplicate Nostr/grants/signer/cache replacement | Violates NMP semantic ownership |
| Public daemon protocol or arbitrary `PATH` runtime discovery | Breaks private runtime and package ownership boundaries |
| Production multi-WebView, compositor, or window-manager framework work | Not required for the scoped Linux Social Home demonstration |

## Definition of Done

- All v1 requirements map to one roadmap phase and have merged, exact-head evidence.
- Current POC replay and native recovery gates pass before new Social implementation begins; Social work is executable only after REF, PKG, and CI gates are accepted.
- Exact current nampplets and NMP revisions are pinned consistently in source, locks, fixtures, and the Nix closure.
- Canonical Nix package, PR-fast, applicable preflight, merge-group full, and review-evidence gates pass without unexpected skips.
- Social Home visibly demonstrates local-first profile/resource and graph/feed composition while preserving source binding, bounded work, recovery, accessibility, and real WebKit evidence.
- A milestone go/no-go records whether Local Files may begin; no deferred capability is silently pulled into v1.

## Traceability

Every v1 requirement maps to exactly one roadmap phase.

| Requirement | Phase | Status |
|-------------|-------|--------|
| REF-01 | Phase 1 | Complete |
| REF-02 | Phase 1 | Pending |
| REF-03 | Phase 1 | Pending |
| REF-04 | Phase 1 | Pending |
| REF-05 | Phase 1 | Complete |
| REF-06 | Phase 1 | Complete |
| PKG-01 | Phase 2 | Pending |
| PKG-02 | Phase 2 | Pending |
| PKG-03 | Phase 2 | Pending |
| PKG-04 | Phase 2 | Pending |
| PKG-05 | Phase 2 | Pending |
| CI-01 | Phase 3 | Pending |
| CI-02 | Phase 3 | Pending |
| CI-03 | Phase 3 | Pending |
| CI-04 | Phase 3 | Pending |
| CI-05 | Phase 3 | Pending |
| CI-06 | Phase 3 | Pending |
| CI-07 | Phase 3 | Pending |
| PROF-01 | Phase 4 | Pending |
| PROF-02 | Phase 4 | Pending |
| PROF-03 | Phase 4 | Pending |
| PROF-04 | Phase 4 | Pending |
| PROF-05 | Phase 4 | Pending |
| SOC-01 | Phase 5 | Pending |
| SOC-02 | Phase 5 | Pending |
| SOC-03 | Phase 5 | Pending |
| SOC-04 | Phase 5 | Pending |
| SOC-05 | Phase 5 | Pending |
| SOC-06 | Phase 5 | Pending |

**Coverage:**

- v1 requirements: 29 total
- Mapped to phases: 29
- Unmapped: 0

---
*Requirements defined: 2026-08-09*
*Last updated: 2026-08-09 after source-backed milestone re-audit*
