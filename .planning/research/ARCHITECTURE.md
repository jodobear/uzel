# Architecture Research

**Domain:** Local-first Linux napplet desktop product
**Researched:** 2026-08-09
**Confidence:** HIGH for Uzel-owned boundaries; MEDIUM for external build/CI mechanics

## Standard Architecture

### System Overview

```text
┌────────────────────────────────────────────────────────────────────────────┐
│ Trusted Uzel product                                                        │
│ Svelte presentation ──invoke──> Rust surface/layout + Napp client owner    │
│      ▲                          │ private typed client                      │
│      │ projected DTO/events      ▼                                           │
├────────────────────────────────────────────────────────────────────────────┤
│ Trusted surface boundary: source-bound opaque iframes; bounded NAP events  │
│ No Tauri bridge, host paths, raw network, secrets, or principal choice     │
├────────────────────────────────────────────────────────────────────────────┤
│ Exact Napp runtime closure → NMP sole query/store/freshness/provenance     │
│ accepted Napp commit → client → exact runtime output → NMP adapter         │
├────────────────────────────────────────────────────────────────────────────┤
│ Cargo.lock + flake.lock → pin check → Uzel Nix package → CI acceptance     │
└────────────────────────────────────────────────────────────────────────────┘
```

Uzel remains ports-and-adapters desktop product. Rust owns canonical surface/layout
state, source binding, product lifecycle and single Napp client; Svelte renders
projected product state. Napp owns neutral contracts, runtime policy, client/testkit and
runtime process conventions. NMP remains sole Nostr query, canonical-store, relay,
signer, provenance and freshness owner. This moves existing POC seam, not creates
second runtime or public daemon API.

### Component Responsibilities

| Component | Responsibility | Allowed dependencies |
|-----------|----------------|----------------------|
| Svelte product presentation | Render layouts, local-first Social Home state, stale/partial/refreshing/blocked states, diagnostics and user actions | Tauri invoke + typed Uzel DTOs; never Napp/NMP internals |
| Rust product controller | Own layout/surface IDs, source-to-session binding, Napp client lifecycle, bounded cancellation and UI projections | Napp client; Tauri adapter; no NMP implementation |
| Trusted surface host | Materialize approved artifact bytes; map MessageEvent source to Rust-owned surface; deliver routed bounded envelopes | Browser isolation primitives and Rust-projected surface metadata |
| Untrusted napplet surface | Render own content; use capability-projected NAP and INC messages | NAP facade + own code + versioned cross-napplet contract; no Tauri/native/network access |
| Napp client/runtime | Supply neutral runtime contract, policy, testkit vectors, exact runtime launch and product-facing events | Accepted Napp revision only; Napp chooses NMP adapters |
| NMP plane | Own Nostr reads, relay behavior, canonical store, freshness, provenance, signer and publication functions | Accessed only through Napp-owned runtime seam |
| Nix packaging | Build Uzel assets/binary and reference exact Napp runtime output in one closure | flake lock, Cargo lock, frontend lock, pinned source inputs |
| CI classifier/aggregator | Classify change scope, run applicable evidence, fail closed on missing/skipped/cancelled work | Stable job results; no product/runtime authority |

## Recommended Project Structure

Keep current proven POC placement; migration changes dependency direction, not product
topology.

```text
apps/uzel/                         # Svelte product and trusted browser host
apps/uzel/src-tauri/               # Rust Tauri adapter; product controller entry
crates/napd-protocol/              # retire/bridge only through accepted Napp client seam
crates/napd/                       # POC adapter; no new runtime truth
apps/uzel-napd/                    # POC entry until exact Napp runtime replaces it
napplets/                          # portable, untrusted product napplets
contracts/                         # versioned cross-napplet payload schemas only
fixtures/                          # exact signed artifacts and test vectors
flake.nix + flake.lock             # canonical package/closure input
scripts/                           # pin, boundary, package and native acceptance evidence
.github/                           # change classifier, lane jobs, fail-closed aggregator
```

### Structure Rationale

- **Apps:** Uzel product policy, layout state and presentation remain in Uzel. No runtime policy leaks into Svelte.
- **Napp dependency seam:** consume only documented Napp client, product events and committed testkit vectors. Do not publish or reconstruct POC-private protocol as Uzel contract.
- **Napplet contracts:** napplets stay portable; only real cross-napplet messages earn versioned contract.
- **Nix/CI:** delivery architecture, not convenience tooling. Package closure and acceptance use locks, never ambient PATH or dev shell.

## Architectural Patterns

### Pattern 1: Source-bound capability projection

**What:** trusted host maps browser MessageEvent source to Rust-owned surface/session
record, then forwards bounded envelope through Napp. Responses return only through same
mapping.

**When to use:** every napplet request, INC route and resource/event response.

**Trade-offs:** adds routing and lifecycle bookkeeping; prevents caller-selected session,
native bridge and authority confusion.

```text
frame window → trusted-host source map → Rust surface record → Napp client
Napp event → Rust surface record → trusted host → same frame window
```

### Pattern 2: Local-first projection, background freshness

**What:** Social Home renders available NMP-provided profile/follow/feed projection first.
Rust sends explicit freshness/diagnostic projection separately; refresh never turns
unknown data into fabricated empty state.

**When to use:** profile/avatar, follows, graph, feed and cache/pressure views.

**Trade-offs:** UI must model stale, partial, refreshing and blocked states. Avoids
UI-owned cache, duplicate Nostr reads and false live claims.

### Pattern 3: Dual-pin release closure

**What:** one accepted Napp commit appears in Cargo resolution and flake input. Pin check
validates identity before Uzel Nix derivation references exact runtime output.

**When to use:** every Napp update, lock update, package build and release smoke.

**Trade-offs:** pin bumps become deliberate multi-file changes; avoids daemon selection
from PATH or Cargo/Nix runtime mismatch. Nix documents flake check evaluates flake
outputs and builds declared checks, so package-dependent check is canonical CI gate.
[Nix reference](https://releases.nixos.org/nix/nix-2.32.3/manual/command-ref/new-cli/nix3-flake-check.html)

### Pattern 4: Classified CI with one fail-closed required result

**What:** classifier selects docs, frontend, Rust, contract, host/security and package
lanes; always-run aggregator validates expected results. Merge group runs merge-full
canonical package evidence.

**When to use:** all PRs; uncertainty promotes scope.

**Trade-offs:** classifier fixtures and aggregator tests required. GitHub warns
path-filtered required workflows can remain pending and merge queues require distinct
merge-group trigger; aggregator prevents invalid required-check topology.
[GitHub required checks](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks) · [merge-group events](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows)

## Data Flow

### Surface Request and Response

```text
User action / napplet event
  → Svelte presentation or untrusted frame
  → trusted host verifies browser source (frame path only)
  → Rust resolves its own surface/session record
  → exact-pinned Napp client
  → exact Napp runtime
  → NMP/provider plane
  → product-facing Napp event/result
  → Rust DTO + mapped surface route
  → Svelte display or original bound frame
```

No arrow exists napplet → Tauri, Napp client, NMP, host file path, raw network,
signer, or caller-supplied principal. Resource/media follows same runtime-approved
request/result path; Uzel presents outcome and diagnostics.

### Social Home Flow

```text
NMP local canonical state
  → Napp product-facing projection
  → Rust read model + freshness/pressure status
  → Tauri DTO
  → Svelte profile, follows, graph, feed, settings/diagnostics

follow/feed selection → source-bound INC topic → receiving profile surface
```

Social Home read models are short-lived Uzel projections. Durable runtime cache,
provenance, freshness evaluation and signer state do not cross into Uzel ownership.

### Package and CI Flow

```text
accepted Napp commit
  ├─ Cargo dependency + Cargo.lock ─┐
  └─ flake input + flake.lock ──────┼→ pin check → Uzel Nix package
                                    │              → store-path smoke
PR diff → classifier → direct fast lanes / path gates ┘              │
merge group → merge-full package + full evidence → required aggregator
```

Tauri remains Uzel desktop shell; upstream distribution tooling does not require second
artifact strategy. Uzel locked Nix derivation is canonical.
[Tauri distribution docs](https://v2.tauri.app/distribute/)

## Integration Points

### Internal Boundaries

| Boundary | Communication | Required rule |
|----------|---------------|---------------|
| Svelte ↔ Rust | narrow typed Tauri commands/DTOs | Svelte presentation state only; Rust owns layout/surface/client lifecycle |
| Rust ↔ Napp | committed client, product events, testkit vectors | no private daemon API, runtime policy or NMP internals in Uzel |
| Trusted host ↔ napplet | source-bound postMessage / MessageChannel, bounded NAP envelopes | opaque sandbox; no bridge, network or principal selection |
| Napplet ↔ napplet | runtime-mediated INC plus versioned contracts payload | no direct imports, peer refs or shared hidden state |
| Nix ↔ Napp | exact flake input/output plus pin check | no copied daemon/unit, no ambient binary discovery |
| CI ↔ package | package-dependent checks plus store-path smoke | PR preflight for package changes; merge-full normal code |

### Trust Boundaries

| Boundary | Trust decision | Enforcement evidence |
|----------|----------------|----------------------|
| Product WebView → napplet iframe | napplet untrusted, opaque-origin | sandbox without same-origin; no Tauri bridge; CSP/navigation tests |
| Trusted host → Rust | browser sender not identity | MessageEvent source maps to current Rust-owned surface/token |
| Rust → Napp | accepted Napp API/revision only | Cargo/flake same-commit pin check; committed testkit vectors |
| Napp → NMP | runtime-owned adapter/policy | Uzel neither calls nor persists NMP truth |
| CI fork/PR → caches/runners | contribution cannot inherit trusted execution | isolated caches/runners; final merge SHA proves release closure |

## Build Order and Phase Dependencies

1. **REF — replay and acceptance disposition.** Preserve POC outputs, hostile-isolation,
   lifecycle, recovery, resource and performance evidence. Establish exact committed Napp
   candidate, documented public client/testkit seam and migration adapter plan. This is
   Social Napp gate: no Social implementation before pass.

2. **PKG — canonical closure.** Independent from REF implementation where pin evidence
   exists. Add dual pin representation, pin check, package/app/check outputs and
   store-path smoke. Uzel references exact Napp output; does not package daemon/unit source.

3. **CI — measure first, then package acceptance.** REF and PKG may proceed independently.
   Implement PR-fast classifier/lane evidence and stable aggregator early; final package
   preflight and merge-full contract depends on PKG canonical derivation. Add native
   Weston/WebKit only for changed host/security/package paths.

4. **SOC-01 — local profile/avatar and status projection.** Start only after REF Social
   Napp gate plus M0 delivery gates. Consume Napp product events for local-first profile,
   runtime-resource avatars, freshness and pressure/cache diagnostics. No Uzel cache/Nostr client.

5. **SOC-02 — follows, graph, feed and navigation composition.** Extend same projection
   route; profile selection crosses source-bound INC and versioned payload. Recheck bounded
   surfaces, cancellation, no direct peering and real WebKit evidence for host-boundary changes.

**Ordering rationale:** REF defines permitted consumption; PKG makes exact runtime use
reproducible; CI proves delivery contracts without native/package cost on all changes;
Social builds only atop accepted public seam and canonical artifact.

## Scaling Considerations

| Pressure | First response | Do not do |
|----------|----------------|-----------|
| More Social data | paginate/project bounded Napp results; render local state first | duplicate NMP store in Svelte/Rust |
| More active surfaces | fixed count, generation tokens, cancellation and cleanup | native WebView per napplet or unbounded hidden frames |
| Larger artifacts/resources | byte/frame/deadline limits at runtime path; bounded projections | expose artifact paths or raw resource transport |
| Slower CI | measure p50/p95; start simple full workspace | make path-filtered job directly required |
| Larger Nix closure | record closure size; inspect growth; retain one runtime ref | ambient deps or copied Napp binary |

## Anti-Patterns

### Recreating Runtime Truth in Uzel

**What people do:** place Nostr reads, cache/freshness rules, grants, signers or daemon
policy in Svelte, Tauri or new Uzel service.

**Why it's wrong:** creates competing authority, bypasses Napp policy and makes
local-first state impossible to explain honestly.

**Do this instead:** smallest neutral Napp seam proven by visible Uzel slice; Uzel
consumes product event/projection.

### Treating POC Private IPC as Public Product Contract

**What people do:** extend POC protocol as external API because it already works.

**Why it's wrong:** freezes private POC shape, blocks neutral Napp ownership and risks
product policy leaking into runtime.

**Do this instead:** preserve only as migration/replay evidence; move consumer to accepted
Napp client/testkit surface without inventing daemon APIs.

### Independent Cargo and Nix Pins

**What people do:** Cargo builds one Napp revision while package launch finds another from
PATH or separate flake input.

**Why it's wrong:** tests compile against different runtime than release executes.

**Do this instead:** one accepted commit, dual locks, fail-closed pin check and store-path
smoke against exact closure.

### Required Path-filtered Checks

**What people do:** mark path-filtered workflow branch-required.

**Why it's wrong:** GitHub may leave it pending when filters skip it.

**Do this instead:** one unfiltered aggregator required check validates classifier-selected
jobs, including merge-group work.

## Sources

### Project-primary

- [.planning/PROJECT.md](../PROJECT.md)
- [.planning/codebase/ARCHITECTURE.md](../codebase/ARCHITECTURE.md)
- [Programme contract](../../PROGRAMME_CONTRACT.md)
- [Napp consumer profile](../../NAPP_CONSUMER_PROFILE.md)
- [Nix package contract](../../NIX_PACKAGE.md)
- [Social Home](../../docs/03-social-home.md)
- [CI profile](../../docs/12-ci.md)

### External-primary (MEDIUM through available web-search confidence seam)

- [Nix 2.32.3 flake check](https://releases.nixos.org/nix/nix-2.32.3/manual/command-ref/new-cli/nix3-flake-check.html)
- [GitHub Actions concurrency](https://docs.github.com/en/actions/concepts/workflows-and-actions/concurrency)
- [GitHub required-status-check troubleshooting](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks)
- [GitHub workflow events / merge group](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows)
- [Tauri v2 distribution](https://v2.tauri.app/distribute/)

---
*Architecture research for: Uzel local-first napplet runtime integration*
*Researched: 2026-08-09*
