# Pitfalls Research

**Domain:** Brownfield local-first Linux napplet product: Uzel consuming exact-pinned Napp/NMP seams
**Researched:** 2026-08-09
**Confidence:** MEDIUM — current Uzel evidence is specific and primary; Gate 0 still must establish accepted Napp revision, activation path, package outputs, and measured CI behavior.

## Critical Pitfalls

### Pitfall 1: Uzel absorbs runtime or Nostr ownership during extraction

**What goes wrong:** A convenience adapter grows into Uzel-owned grants, Nostr queries, relay selection, signer state, cache/freshness logic, `nappd` lifecycle policy, or a second product/runtime source of truth. The Social Home then works only through a private Uzel contract and extraction becomes a rewrite rather than a seam replacement.

**Why it happens:** The POC has large, cross-cutting state machines and a private protocol version `0`; moving code file-by-file looks faster than defining the smallest consumer-facing Napp client/testkit seam. UI needs can tempt product policy into Napp too.

**How to avoid:** Start every request at visible Uzel outcome, write its implementation/instance/profile/surface/identity/operation scope, then require one Napp-owned neutral contract. Keep Rust as Uzel surface/layout and Napp-client lifecycle owner; keep Svelte presentation-only; keep NMP as sole Nostr query, canonical-store, freshness, provenance, relay, signer and publication plane. Do not broaden parked Files, authoring, signing, wallets, media, FIPS, Android, native napplets, or multi-WebView work.

**Warning signs:** New Uzel tables/cache directories mirror profile, follow, grant, or event data; `apps/uzel` imports NMP/nampplets internals; a Napp API knows Social Home layout; PR description lacks one visible outcome; a seam has no pinned testkit vector.

**Recovery / stop condition:** Stop slice before merging when ownership is ambiguous. Revert the duplicate state/API, record it as a Napp contract question, and replay the existing POC acceptance from exact fixtures before selecting a smaller seam. Do not begin Social implementation until accepted Napp candidate and M0 delivery gates exist.

**Phase to address:** `SLICE-REF-01` first; enforce again in `SLICE-SOC-01` and `SLICE-SOC-02`.

---

### Pitfall 2: Cargo, flake, fixture, and runtime revisions drift apart

**What goes wrong:** Cargo resolves one `napp-client` revision while `flake.lock` supplies a different `nappd`/`nappctl`; or copied trusted-shell bytes, signed napplet fixtures, Rust coordinates, and compatibility ledger are updated only partly. Individual hashes/builds can still pass while client, provider bounds, source binding, or protocol behavior are mutually incompatible.

**Why it happens:** POC dependencies are fork-pinned, four critical dependencies share a Nampplets SHA, trusted shell is copied material, and a Nix lock plus Cargo lock add independent-looking update paths.

**How to avoid:** Make one accepted committed Napp revision a machine-checked input to Cargo dependency/lock and flake input/lock. `pins/check` must compare exact commits, and package smoke must prove store-path Uzel references that Napp closure. Use a single deterministic repin operation that fetches recorded source commit, verifies provenance, rebuilds fixtures, updates every digest, and prints validation matrix. Change one axis only: never combine NMP/nampplets/package upgrade with extraction, schema migration, or contract redesign.

**Warning signs:** `Cargo.toml`, `Cargo.lock`, `flake.lock`, fixture metadata, and compatibility ledger name different commits; hand-edited `public/trusted-shell` assets; package finds a daemon unrelated to flake closure; protocol mismatch silently launches; source status names historical POC SHA as current.

**Recovery / stop condition:** Fail closed at `pins/check`; do not publish package or advance Social work. Restore previous coherent pin set, regenerate rather than patch copied bytes, then rerun conformance, signed-fixture, source-binding, recovery, live-NMP where applicable, and Weston/WebKit evidence. A pin may advance only with an accepted candidate commit and new evidence tied to it.

**Phase to address:** `SLICE-REF-01` pin/ownership disposition; `SLICE-PKG-01` closure assertion and store smoke; `SLICE-CI-01` makes assertion required.

---

### Pitfall 3: Package works in dev shell, not as canonical Nix product

**What goes wrong:** `pnpm`/Cargo/dev script launches Uzel by finding `nappd` on ambient `PATH`, using checkout-relative assets, development libraries, or copied daemon/systemd unit. The package builds but installed result is not self-contained, starts incompatible daemon, or leaves daemon activation unsupervised.

**Why it happens:** Current POC starts components manually, Tauri bundle is disabled, and Fedora developer environment masks missing runtime closure inputs.

**How to avoid:** Treat Nix derivation as canonical Linux release artifact. Expose `packages.uzel`, default package/app, development shell, and checks only after Gate 0 fixes names. Uzel references exact Napp package output; it does not package `nappd`, `nappctl`, or Napp's `nappd@.service`. Keep generic-Linux service activation explicit and owned by Napp. Build from clean checkout with locked inputs and no derivation network access; run non-GUI smoke from `result/bin`, inspect closure size, and run path-relevant WebKit smoke against package output.

**Warning signs:** Test passes only inside `nix develop`; launcher uses `which`/`PATH`; resulting app reads source tree; Nix check builds an unrelated derivation; Uzel repository adds Napp daemon or unit copies; package smoke uses development binary instead of result path.

**Recovery / stop condition:** Stop package acceptance when result cannot launch with sanitized `PATH` and removed checkout path, or cannot name exact runtime closure. Remove ambient fallback/copies, add declared input/wrapper reference, then rebuild clean and repeat store-path, version/protocol-mismatch, closure-size, and native smoke evidence. No Flatpak workaround or system package may substitute for missing derivation input.

**Phase to address:** `SLICE-PKG-01`; consumed by `SLICE-CI-01` merge-full and package preflight.

---

### Pitfall 4: Local-first Social Home displays remote-like certainty or duplicate cache truth

**What goes wrong:** Feed, graph, avatar, or profile screen clears local data while refresh runs, claims data is current when stale/partial/blocked, treats read identity as authenticated login, or caches NMP results separately in Uzel/napplet state. Users receive a smooth but false story; recovery and provenance drift from runtime truth.

**Why it happens:** UI code conflates rendering state, background work, retry state, and canonical data. Existing profile/follow capacity and retries look small enough to hide state loss in demos.

**How to avoid:** Render assigned local NMP state first. Model `fresh`, `stale`, `partial`, `refreshing`, `blocked`, and diagnostic pressure/cache state as visible, testable presentation derived from runtime events. Use `NAP-RESOURCE` mediated media and runtime storage only for napplet-local state; selection travels through source-bound NAP-INC. Preserve visible fallback for refused SVG and failures. State text must say read context, never account authentication.

**Warning signs:** Spinner replaces usable local profile; no stale timestamp/reason; UI treats refresh failure as empty follow list; direct HTTPS fetch by napplet; duplicated profile/follow persistence; avatar object URLs or retries grow without caps; screen says signed-in/owned for public read identity.

**Recovery / stop condition:** Stop a Social slice when visible state cannot identify data source and freshness or when fallback empties known local data. Remove duplicate cache/fetch path, route through Napp/NMP, restore local snapshot first, and add fixture tests for stale→refreshing→success, stale→blocked, partial profile/resource, selection via INC, and process restart. Do not add sign-in/write semantics to repair this presentation gap.

**Phase to address:** `SLICE-SOC-01` state contract and profile/follows; `SLICE-SOC-02` feed/navigation/selection.

---

### Pitfall 5: Source-binding or WebKit isolation regresses while deterministic tests stay green

**What goes wrong:** A change to trusted shell parsing order, injected CSP, `<base>`, iframe sandbox flags, `postMessage` source check, navigation policy, Tauri capabilities, or asset hash lets an untrusted napplet reach native bridge, raw network, host path, wrong surface, or privileged principal. Chromium/mocked tests pass despite actual WebKit behavior differing.

**Why it happens:** The POC boundary depends on precise composition of inert parsing, opaque origin, exact `MessageEvent.source` mapping, CSP and navigation denial. Tauri's own model separates Rust-core and WebView trust groups; capability overlaps merge permissions, while its documentation notes remaining WebView-hardening limitations.

**How to avoid:** Keep one trusted Tauri WebView and sandboxed child frames; preserve `sandbox="allow-scripts"` without `allow-same-origin`; source-bind every request before Napp; expose no raw Tauri bridge, secrets, host paths, raw network, or caller-selected authority. Modify trusted-shell only through owning upstream source and exact-byte repin. Pair unit/conformance checks with hostile iframe/network, raw Tauri dispatch, navigation, and real Weston/WebKit package evidence. Keep capabilities least-privileged and non-overlapping.

**Warning signs:** `allow-same-origin`, remote navigation, broad `connect-src`, or wildcard message acceptance appears; frame can call `window.__TAURI__`; CSP/asset hashes change without upstream pin; test only says Chromium/mock pass; native/security path classified as frontend-only.

**Recovery / stop condition:** Treat as P0 blocker. Disable changed surface or restore last accepted trusted-shell bytes, invalidate all prior hostile evidence for changed head, then repeat exact-head conformance plus real WebKit hostile egress. Do not claim OS-level browser-exploit isolation: it is parked hardening, not delivered by the iframe boundary.

**Phase to address:** `SLICE-REF-01` preserves accepted boundary; `SLICE-CI-01` classifies host/security paths; `SLICE-SOC-02` proves new INC/navigation paths.

---

### Pitfall 6: Bounded POC limits are raised or bypassed for Social data

**What goes wrong:** A large social graph/feed starts unbounded retries, avatar fetches, object URLs, resource buffering, client operations, surfaces, tasks, or per-connection workers. Current daemon serializes requests, so a provider wait can block status/stop/cleanup; 50 MiB decoded Blob and roughly 100 MiB envelope/event ceilings can create much larger transient memory pressure.

**Why it happens:** Existing limits are POC-scoped and distributed across daemon, host, renderer, replay cache, and tests. A feature team sees capacity limit as missing feature instead of product-admission contract.

**How to avoid:** Preserve current limits until product-visible admission/eviction policy is defined. Keep retries/tasks/resources cancellable with whole-operation deadline; use virtualization and priority scheduling for follows; cap concurrent profile/avatar work and revoke object URLs. Benchmark control latency during 15-second provider wait and peak RSS with maximum-sized single/concurrent resources. If daemon concurrency changes, separate I/O from ordered state mutation with bounded admission—never unbounded threads.

**Warning signs:** Fixed Social composition consumes four daemon surfaces; retry queue/Blob/object-URL count lacks metrics; `bytesMany` succeeds per item but lacks overall deadline; close/identity/status latency spikes during fetch; RSS rises sharply despite accepted payload size; code raises a cap without changing matching test/host/replay cap.

**Recovery / stop condition:** Stop growth work at measured cap or breached responsiveness/RSS budget. Cancel work, reconcile surface/replay state idempotently, retain local rendering with partial indicator, and lower product ceiling until streaming/verified resource boundary or upstream deadline contract is accepted. Do not change daemon scheduling and Social behavior in one PR.

**Phase to address:** `SLICE-REF-01` records baseline limits; `SLICE-SOC-01` implements bounded view; later scoped performance hardening only after Social acceptance.

---

### Pitfall 7: Path-classified CI creates green skips or hides dependency changes

**What goes wrong:** Required workflow is path-filtered and never reports, a dependent job is skipped after failure, classifier misses shared contract/toolchain/lock change, merge queue lacks `merge_group`, or cache hit is treated as proof. PR-fast looks green while package, native boundary, or release derivation was never exercised.

**Why it happens:** Path filtering reduces cost but GitHub has different semantics for skipped workflows and skipped jobs. A custom affected-crate resolver is added before real p95 evidence. Caches are mistaken for build outputs.

**How to avoid:** Use union-of-classes classifier; uncertainty promotes to full scope. Workspace manifests, lock/toolchain files, build scripts, shared contracts and CI tooling always promote. Start with measured simple full Rust workspace, add reverse-dependency classifier only if measured p95 gain and fixture/fail-safe fallback justify it. Require one stable fail-closed aggregator that verifies expected jobs, missing result, skip and cancellation; never make path-filtered jobs independently required. Trigger release gate on `merge_group`; package/toolchain changes run Nix preflight on PR head; ordinary code runs same locked package/full suite on merge-group.

**Warning signs:** Required check pending after docs PR; `if:` skip silently marked success; classifier fixtures absent; lockfile edit stays frontend-only; merge queue has no check; Nix check not dependent on Uzel package; p95/cache restore/save/cancellation not recorded; retry masks flake.

**Recovery / stop condition:** Block merge when aggregator cannot account for each expected lane on exact head/merge-group SHA. Promote affected change to full lane, fix event triggers/classifier fixtures, and rerun. Remove cache or scope resolver when measured p95 worsens; caches may speed rebuilds but must never decide correctness. Restrict trusted cache writes/runners from untrusted forks.

**Phase to address:** `SLICE-CI-01` Gate 0 measurement/classifier/aggregator; `SLICE-PKG-01` supplies canonical check; every subsequent phase supplies path fixtures.

---

### Pitfall 8: Review evidence belongs to a stale SHA or unavailable reviewer is treated as approval

**What goes wrong:** Local CodeRabbit, Codex, final CodeRabbit, PR-fast, package preflight, or native result covers an earlier head. A material fix changes behavior after review, a skipped/timeout review is accepted as clean, or GSD is assumed to merge without GitHub merge-group evidence.

**Why it happens:** Batched review deliberately reduces repetition; that economy fails if head/input identity is not recorded. Review services can timeout, and merge queue produces a new merge-group SHA.

**How to avoid:** Bind each evidence record to immutable base/head SHA, scope/classification, commands, result, and expiry condition. Follow local CodeRabbit → PR-fast → Codex → final-head PR-fast → final CodeRabbit; rerun according to material semantic change. Retry unavailable review once, then only maintainer final-head substitute where issue permits. GitHub remains merge authority; merge-group full CI uses locked release derivation.

**Warning signs:** Review links/output omit head SHA; PR updated after final review; test artifact says branch name only; native result runs a checkout binary; reviewer timeout marked pass; merge queue runs no `merge_group` workflow; another PR's result is cited.

**Recovery / stop condition:** Evidence stale, missing, skipped, cancelled, failed, or timed out is not approval. Stop queue entry, update/freeze final head, rerun applicable deterministic tests and final-head reviews, then request merge queue again. A material final CodeRabbit fix returns through Codex before another final CodeRabbit; do not loop indefinitely on unavailable service.

**Phase to address:** `SLICE-CI-01` evidence schema and aggregator; apply to every slice PR and M0 acceptance.

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Copy trusted-shell bytes by hand | Fast local edit | Provenance/hash/behavior divergence across four assets | Never |
| Uzel fallback to `PATH` `nappd` | Dev machine starts | Wrong daemon/protocol in package; non-reproducible release | Never |
| Uzel profile/follow cache | Faster-looking screen | Competing NMP truth, freshness and recovery behavior | Never |
| Treat protocol v0 as public platform API | Avoid seam design | Locks product into POC client/identity/surface assumptions | Only while private and one-client, then explicitly redesign |
| Raise caps for demo data | Avoid admission UX | RSS/latency leaks, dangling surfaces and untestable overload | Never without product policy + measurement |
| Custom affected-crate resolver first | Smaller apparent CI | False skips and maintenance cost larger than saved time | Only after Gate 0 p95 evidence and fail-safe fixtures |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Napp client/runtime | Cargo client and flake runtime point at compatible-looking but different commits | One accepted commit, automated equality check, runtime closure smoke, clear mismatch failure |
| NMP social data | Render direct relay/HTTPS result or persist product copy | NMP remains source; render local state first; use mediated resource and explicit freshness state |
| Trusted shell/Tauri | Grant child broad Tauri capability or treat iframe sandbox as OS sandbox | Exact source binding + strict CSP/navigation + real WebKit hostile evidence; park OS isolation claim |
| Nix package | Copy `nappd@.service` or daemon into Uzel derivation | Reference Napp-owned output; make generic-Linux activation explicit |
| GitHub Actions | Require path-filtered workflow/job directly | Stable fail-closed aggregator decides classification and observes all expected lane results |
| CI cache | Restore cache as trusted build truth or let fork influence trusted cache | Cache only regenerable inputs, validate build normally, restrict trusted writes |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Serial daemon plus long provider request | Status/stop/cleanup waits behind 15-second provider path | Benchmark control latency; bounded I/O admission; preserve ordered state mutation | First stalled request with user action, not a large user count |
| Base64 resource materialization | RSS spike, renderer stall, OOM despite request cap | Measure end-to-end peak RSS; lower ceiling; move to verified streaming only through owning seam | Around existing 50 MiB Blob / ~100 MiB envelope limits, especially concurrent avatars |
| Unbounded social enrichment | Late rows starve, memory grows, stale work wins race | Virtualize list, priority schedule, cap/revoke/cancel | Existing 1,024 follows, 2 profile queries, 4 avatars, 32 retained object URLs |
| Layered CI caches | Cache save/restore dominates p95, failures disappear under warm cache | Measure each cache layer separately; retain only measured benefit | As soon as Nix store + Rust target + sccache overlap |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Source binding by claimed origin/ID rather than exact host-created `event.source` | Frame can route under another surface/principal | Maintain exact source mapping, opaque sandbox origin and per-request binding before Napp |
| `allow-same-origin`, broad CSP/network, native bridge exposure | Untrusted napplet gains host/network/native authority | Preserve sandbox/CSP/navigation denial; hostile WebKit regression suite |
| Calling public read identity authentication | Future writes/secrets authorized by an unproven context | Label read context honestly; introduce signer-backed authority only in future owned seam |
| Private socket treated as multi-client secure API | Same-user process can control/inspect daemon | Retain one-client/private-dir assumption; add peer credentials/capabilities before expansion |
| Trusted cache/persistent runner exposed to fork code | Cache poisoning or secret exfiltration | Restore-only low-trust paths; no secrets in caches; trusted writers only |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Loading spinner erases local Social Home | Product appears offline/empty though data exists | Keep local profile/follows/feed rendered with `refreshing` overlay/state |
| Same visual for stale, partial and blocked | User acts on unknown-quality information | State-specific text, timestamps/reason and pressure/cache diagnostic route |
| Silent source-binding denial | Selection/navigation looks broken | Safe visible denial/diagnostic; never silently retry with broader authority |
| Resource failure removes person/profile context | Graph/feed seems corrupt | Retain profile row and fallback avatar; represent resource failure separately |

## "Looks Done But Isn't" Checklist

- [ ] **Napp extraction:** Uzel consumes only pinned public client/testkit seam — verify no NMP/nampplets internal imports or duplicate persistence.
- [ ] **Nix package:** Store-path Uzel starts with sanitized `PATH`, removed checkout path, exact Napp closure, and clear client/runtime mismatch — verify against clean checkout result.
- [ ] **Social Home:** Local profile/follows/feed remain visible through stale, partial, refreshing, blocked and restart paths — verify fixture state transitions and visible acceptance.
- [ ] **Trusted composition:** Host/security change preserves exact source mapping, no native/raw-network bridge, CSP/navigation denial — verify hostile real Weston/WebKit, not Chromium mock alone.
- [ ] **Lean CI:** Required aggregator observed every classified lane on exact PR and merge-group heads — verify deliberate docs, frontend, contract, host/security, lock and unknown-path fixtures.
- [ ] **Review:** Local CodeRabbit, Codex, final CodeRabbit and applicable CI cite final head SHA — verify stale-evidence invalidation after material change.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Ownership drift | HIGH | Stop slice; remove duplicated truth; return to visible outcome → minimum Napp seam; replay POC fixtures/boundary evidence. |
| Pin or trusted-asset drift | HIGH | Restore coherent accepted pin set; regenerate assets/fixtures; rerun source, conformance, recovery and WebKit matrix. |
| Ambient runtime package | MEDIUM | Delete PATH/checkout fallback; reference Napp output; build clean; smoke result with sanitized environment. |
| False local-first state | MEDIUM | Remove duplicate cache/fetch; restore local snapshot rendering; add state-transition/restart tests. |
| Isolation regression | HIGH | Disable/revert changed surface; repin exact bytes; rerun hostile exact-head WebKit proof before reopening. |
| CI false skip/cache claim | HIGH | Promote full scope; repair classifier/aggregator/trigger; rerun PR head and merge-group package lane. |
| Stale review evidence | MEDIUM | Freeze final SHA; rerun affected gates/reviews; restart review sequence after material fix. |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Ownership drift/private contract invention | `SLICE-REF-01` | Ownership matrix; accepted client/testkit seam; current POC replay. |
| Cargo/flake/fixture pin mismatch | `SLICE-REF-01` + `SLICE-PKG-01` | Automated commit equality; deterministic repin; exact fixture/source validation. |
| Ambient PATH/noncanonical package | `SLICE-PKG-01` | Clean-checkout build, result-path smoke, sanitized PATH, closure and mismatch checks. |
| Misleading social truth | `SLICE-SOC-01` | Visible/fixture proof for local-first, stale/partial/refreshing/blocked plus fallback resource. |
| Source-binding/WebKit regression | `SLICE-REF-01`, `SLICE-CI-01`, `SLICE-SOC-02` | Conformance + hostile egress/navigation/raw-bridge real Weston/WebKit test on affected output. |
| Unbounded work/resource pressure | `SLICE-SOC-01` | Admission/cancellation/retry/object-URL limits, control-latency and peak-RSS evidence. |
| Misclassified or cache-masked CI | `SLICE-CI-01` | Classifier fixture suite, fail-closed aggregator, PR package preflight and merge-group Nix full lane. |
| Review evidence stale/unavailable | `SLICE-CI-01` and every PR | Base/head-bound evidence record; material-change invalidation; final-head review and merge-group proof. |

## Sources

- [Uzel project contract](../PROJECT.md), [codebase concerns](../codebase/CONCERNS.md), [Nix package contract](../../NIX_PACKAGE.md), [Social Home contract](../../docs/03-social-home.md), [CI profile](../../docs/12-ci.md), and [review workflow](../../REVIEW_WORKFLOW.md) — primary project evidence.
- [Nix flake reference](https://releases.nixos.org/nix/nix-2.25.5/manual/command-ref/new-cli/nix3-flake.html) and [flake check](https://releases.nixos.org/nix/nix-2.13.6/manual/command-ref/new-cli/nix3-flake-check.html) — MEDIUM confidence external verification.
- [GitHub required-check troubleshooting](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks), [concurrency](https://docs.github.com/en/actions/concepts/workflows-and-actions/concurrency), and [dependency-cache security](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching) — MEDIUM confidence external verification.
- [Tauri security model](https://v2.tauri.app/security/), [capabilities](https://v2.tauri.app/security/capabilities/), [CSP](https://v2.tauri.app/security/csp/), and [WebView hardening status](https://v2.tauri.app/security/future/) — MEDIUM confidence external verification.

---
*Pitfalls research for: Uzel Napp-consumer M0 extraction, canonical Nix package, lean CI, and Social Home*
*Researched: 2026-08-09*
