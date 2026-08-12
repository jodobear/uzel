# Phase 1 and Milestone Re-audit

**Audited:** 2026-08-09

**Uzel planning head:** `f139682da61babff8da48a756fd0717f896c308e`

**Uzel production baseline:** `19519c3` (`Render rich follow rows and complete canonical profiles (#30)`)

**Observed Napp head:** `0b75b6b4a9ba83598ef8be5ff95dbd40faaf128e`

**Verdict:** **Ready to plan bounded POC-preservation work; not ready to plan or execute Napp integration.**

## Evidence status

- Graphify was queried before direct source inspection. Its ownership and call-path map
  remains applicable because every commit after `19519c3` changes planning documents,
  not production code.
- The root product-first pack is installed but not committed: `PROGRAMME_CONTRACT.md`,
  `NAPP_CONSUMER_PROFILE.md`, `NIX_PACKAGE.md`, `QUALITY.md`, `REVIEW_WORKFLOW.md`,
  `GSD_RUNBOOK.md`, `docs/10-gsd-seed.md`, and related files are untracked. The committed
  `.planning/` derivatives are the only committed product-first planning artifacts.
- Napp's committed observed head contains planning documents, not an executable
  `napp-client`, product-event, or testkit candidate. Dirty sibling-repository audit work
  was observed but is not accepted source or a consumable dependency.
- Current POC dependency truth is exact, not floating: `jodobear/nampplets@e2f69f3`,
  transitive `pablof7z/nmp@005dc2a`, NAP packages from the lockfile, Tauri `2.11.5`,
  and nixpkgs `38a4887`. Live remote heads have moved; no compatibility with those heads
  is inferred.
- Fresh replay on this host passed 37 JavaScript tests, 55 Rust tests with two explicit
  live-network ignores, 34 Chromium acceptance cases, and `BOUNDARIES_OK`.
- Real Weston/WebKit smoke passed from a clean isolated Cargo target with
  `LINUX_RUN_SMOKE_OK`. The default target first failed because generated Tauri metadata
  retained an absolute path from `/workspace/projects/napplets/jodobear/uzel`; a clean
  target passed. This is a reproducibility/cleanliness defect, not a source failure.
- `pnpm docs:check` validates only the nested POC pack. The installed root auditor fails
  on non-UTF-8 Markdown encountered by an unrestricted repository scan, so it does not
  currently validate the product-first pack or `.planning/`.
- `uzel-poc-validated-pack/STATUS.md` still calls Work 07 and PR #30 active even though
  `19519c3` is the merged production baseline. Its Debian 13 interactive acceptance gap
  remains real; its PR state is stale.

## Classification key

1. blocker for current phase
2. current-phase improvement
3. valid future backlog item
4. incorrect/stale assumption
5. already handled

## Lane audit

| Lane | Problem or fact | Class | Disposition |
|---|---|---:|---|
| Product outcome / visible value | Plan treats profile/follows as mostly future work, but merged POC already renders rich profiles, follows, avatars, profile selection, recovery, and hostile-boundary diagnostics. | 4 | Fastest visible value is fresh replay and preservation of this product path. Do not rebuild it in Phase 1. Later Social slices add only evidenced gaps. |
| Product outcome / visible value | Accepted Napp seam does not exist, so a plan that starts with seam adaptation would produce no visible value and invite a private facade. | 1 | Plan POC replay, ownership, measurements, and Napp dependency handoff first. Gate adapter work on an accepted candidate. |
| POC behavior/tests/fixtures | Exact-build review/confirm/launch, multi-surface INC, source binding, recovery, rich profile/follows, bounded resource flow, hostile denial, Chromium, and Weston/WebKit evidence already exist. | 5 | Preserve source, signed fixtures, contracts, and separate browser/native proof. Treat them as regression corpus. |
| POC behavior/tests/fixtures | Legacy status says merged PR #30 is active; Debian 13 interactive acceptance is still missing. | 4 | Record merged source truth separately from unresolved human-platform acceptance. Do not reopen or overwrite historical Work 07. |
| Napp versus Uzel ownership | Existing `crates/napd*` combines reusable runtime mechanisms with a private Uzel daemon protocol; promoting that protocol as public Napp API would reverse dependency ownership. | 2 | Produce file-level disposition. Retain Uzel product/Tauri behavior; consume future Napp client/events/testkit; contribute only proven neutral gaps. |
| Napp versus Uzel ownership | No committed Napp implementation candidate is consumable. | 1 | Repository-qualified Napp blocker. No Uzel facade, copied daemon, or speculative public protocol. |
| NAP/package/Kehto/NMP/nampplets compatibility | POC pins are internally coherent and tested, but current upstream heads differ. “Current compatible” is unproved beyond the exact lock. | 2 | Freeze Phase 1 on current-green pins. Research upgrades separately, one axis at a time; replay all affected evidence before repin. |
| Trusted host/source/sandbox/permissions | `MessageEvent.source` binding, runtime-owned identities, opaque `allow-scripts` frames, no Tauri bridge, bounded envelopes, native navigation policy, socket/path checks, and hostile tests exist. | 5 | Preserve as non-negotiable regression gates before and after any adapter change. |
| Trusted host/source/sandbox/permissions | POC evidence does not prove OS-level per-WebKit-process network isolation or trusted local TLS. | 3 | Keep claim narrow. Track hardening separately; do not expand Phase 1 unless accepted Napp contract requires it. |
| Local-first storage/caching/NMP | NMP owns canonical Nostr state; Rust persists a small selected-identity/product-state record; browser object URLs and retry state are bounded and transient. | 5 | Keep one writable profile owner. Do not add Uzel profile/follow/event/freshness caches. |
| Local-first storage/caching/NMP | Retention, cache invalidation, schema migration, and profile-directory ownership are not expressed as one durable contract across future Napp and current POC. | 2 | Ownership map must record owner, durability, schema/version, retention, recovery, and migration trigger for each state class. No new store in Phase 1. |
| Performance/resource pressure | Surface, request, event, review, retry, subscription, object-URL, envelope, state, and resource bounds exist in code. | 5 | Capture them in one bounds ledger; do not add a framework. |
| Performance/resource pressure | No fresh cold-start, first-frame, RSS/WebView-memory, or pressure baseline exists for current checkout. A clean Nix shell had a large first-use materialization cost. | 2 | Measure a small representative baseline, separating shell setup/build cache from runtime. Mark unavailable honestly. Defer p50/p95 CI economics to delivery work. |
| Performance/resource pressure | `resource.bytesMany` lacks a whole-operation deadline; nampplets issue #9 already tracks it. | 3 | Preserve individual-resource proof; do not import the upstream fix into current phase absent a visible failing slice. |
| Multi-instance/profile assumptions | Current POC exposes one daemon endpoint, one active read identity, one bounded product-state file, and no demonstrated concurrent-profile or multi-instance contract. | 2 | Declare Phase 1 acceptance as single instance plus one active read profile. Probe collision/failure behavior; do not promise multi-instance support. |
| Multi-instance/profile assumptions | Building full multi-instance/profile isolation now would front-load Napp/package policy. | 3 | Resolve in Napp/package work before claiming parallel profiles or instances. |
| Tauri/Svelte/SurfaceHost | Boundary is already sound: Rust/Tauri owns native/runtime state, Svelte owns presentation, trusted shell owns source-bound projection, `SurfaceHost` owns bounded opaque frames. | 5 | Preserve ownership. Adapter work belongs behind Tauri/Rust, never in napplet/Svelte code. |
| Tauri/Svelte/SurfaceHost | `App.svelte` is large and coordinates many product states, but no audited defect requires refactoring it during REF-01. | 3 | Refactor only with a bounded UI outcome or measurable regression; no architecture cleanup slice now. |
| Nix ownership/reproducibility | Current flake is a dev shell only; Tauri bundle is disabled; no `packages.uzel`, app output, exact Napp closure, or store-path launch exists. | 3 | Phase 2 owns canonical package implementation. Phase 1 records pins and the consumer dependency only. |
| Nix ownership/reproducibility | Default Cargo target carried generated absolute metadata from an old checkout and broke Tauri build/smoke until a clean target was used. | 2 | Add clean-target/repository-relocation proof to replay evidence and avoid treating a warm shared target as reproducibility proof. |
| Rust/TS CI latency / duplicate gates | Repository already has direct unit, type/build, conformance, Fallow, boundary, Chromium, and native smoke commands; no committed GitHub workflows exist. | 2 | Phase 3 should compose existing commands, measure them, and use one stable aggregator. Do not create duplicate wrapper suites or run all heavy gates on every push. |
| Rust/TS CI latency / duplicate gates | Separate evidence realities are necessary: deterministic Chromium does not replace Weston/WebKit; package proof does not replace unit checks. | 5 | Deduplicate orchestration, not distinct evidence. |
| Rust/TS CI latency / duplicate gates | Proposed `GSD_RUNBOOK.md` says CI/review are delivery-phase tasks while roadmap has a CI phase. Read literally, this conflicts. | 4 | Phase 3 owns shared workflow/aggregator implementation and measurement. Each feature phase still owns its applicable checks/reviews; no separate ceremony phase per PR. |
| Contextual issue to PR scope | Installed issue/PR templates and issue process specify one issue/one outcome, but they are uncommitted and unenforced. | 1 | Before implementation, adopt/commit the selected product docs/templates or explicitly replace them. Each plan maps to one repository-qualified issue and bounded PR. |
| Review flow | Owner direction uses local CodeRabbit followed by GitHub Codex on the exact pushed PR SHA, with GitHub Codex as fallback when CodeRabbit records a rate limit before findings. | 5 | Attempt CodeRabbit once per immutable candidate. Continue only on CodeRabbit pass plus green GitHub Codex, or recorded CodeRabbit rate limit plus green GitHub Codex. Any later commit invalidates review evidence. |
| Review flow | No committed workflow/evidence implementation currently binds reviews and required checks to exact head/merge-group SHA. | 3 | Phase 3 owns enforcement. Do not block POC evidence capture on future GitHub automation. |
| GSD/Mosaico handoffs/context | Current phase context repeats a large programme library and previously treated all Phase 1 work as blocked by Napp. | 4 | Handoffs should contain exact heads, issue, decisions, evidence paths, blocker, and next probe only. Reference canonical docs; do not copy full packs into prompts. |
| Migration/rollback/restart/cancel | Existing POC has typed ambiguous outcomes, replay cache, cancellation, cleanup, identity rollback, restart reconciliation, and bounded state. | 5 | Preserve tests and idempotent cleanup. |
| Migration/rollback/restart/cancel | No accepted private-protocol-to-Napp migration/rollback contract exists. | 2 | Use one-axis strangler: replay current POC, qualify candidate, adapt one boundary, replay matrix, then retire compatibility code. No dual store/write; rollback by reverting adapter/pin while current POC remains green. |
| Missing evidence/contracts | Product-first root docs are not committed; root doc auditor is not a usable gate; accepted Napp client/events/testkit and pin-parity contract are absent. | 1 | These block full Phase 1 integration planning and any implementation claim. |
| Missing evidence/contracts | Ownership/state-retention/bounds diagrams, clean-checkout replay manifest, Debian 13 visible acceptance, and Napp candidate qualification evidence are missing. | 2 | Produce minimal exact-source artifacts in REF-01; diagrams only where they clarify ownership or migration. |

## Corrected current-phase scope

Phase 1 is a split evidence-and-integration gate, not a rewrite:

1. Reconcile source truth: merged POC baseline, stale Work 07 status, exact pins, and
   installed/committed documentation status.
2. Replay current-green POC from a clean or relocated target: unit/contract tests,
   Chromium, Weston/WebKit, exact fixtures, visible profile/follow composition, hostile
   denial, restart, cancellation, and rollback.
3. Produce three small durable artifacts: file/owner disposition, state/retention and
   resource-bounds ledger, and replay/measurement manifest.
4. Qualify one exact committed Napp candidate for client, product events, testkit,
   version negotiation, lifecycle/recovery, and pin parity. If absent, update the Napp
   dependency handoff and stop that lane.
5. Only after qualification, adapt the narrow Rust/Tauri consumer boundary, replay the
   same matrix, and retire compatibility code proven obsolete. Preserve Uzel product
   behavior and NMP ownership throughout.

## Explicit out of scope

- Social feed/graph feature construction, Files, Blossom, signing, wallets, authoring,
  media, search, ContextVM, Relatr, TUI, WASI, Android, native napplets, and Flatpak.
- Generic runtime/framework redesign, public promotion of `napd-protocol`, duplicate
  Nostr/profile/follow/event/freshness stores, or Uzel-owned Napp substitutes.
- Canonical Nix package implementation and GitHub CI/merge-queue implementation.
- Full multi-instance/multi-profile support, App.svelte refactor, OS-level WebKit child
  isolation, trusted local TLS, or upstream dependency upgrades.
- Closing historical Debian/PR records without fresh direct evidence.

## Safe parallel work lanes

| Lane | May proceed now | Join gate |
|---|---|---|
| A — POC replay | Clean-target tests, Chromium, Weston/WebKit, fixture hashes, visible demo, stale-status reconciliation | Exact Uzel head and raw evidence manifest |
| B — Ownership and pressure | File disposition, state/retention map, bounds ledger, minimal startup/RSS/resource measurements | No production refactor; exact source anchors |
| C — Napp qualification | Read-only source/export/testkit/version/pin probes and repository-qualified handoff | Exact committed candidate exists and all probes pass |
| D — Product-doc admission | Select, audit, and commit authoritative root docs/templates without changing product scope | Root audit passes; no competing normative copies |
| E — Adapter integration | **Blocked** | A–D accepted plus exact Napp candidate |

No two lanes edit the same production boundary. Nix package and CI workflow work remain
their own later slices, even if research runs concurrently.

## Blockers and dependencies on Napp

1. Exact reachable committed candidate containing the supported client API, product
   events, testkit vectors, and executable compatibility probes.
2. Runtime/protocol version authority and explicit mismatch response.
3. Lifecycle contract covering review, confirm, launch, cancellation, lost replies,
   status/reconciliation, cleanup, and bounded events.
4. Instance/profile/XDG/socket ownership sufficient for one Uzel instance now and
   non-colliding future instances.
5. NMP projection contract that leaves query, relay, store, freshness, provenance,
   signer, and publication truth in NMP.
6. Pin-parity input/fixture usable by Cargo and later Nix packaging.

## Facts requiring fresh research

- Compatibility of current remote NAP packages, Kehto, NMP, and nampplets heads with
  this exact POC; live head movement alone proves no compatibility.
- Accepted Napp candidate source and review state after its sibling audit is committed.
- Clean-checkout cold/warm start, first-frame, RSS, WebView memory, Nix materialization,
  and resource-pressure measurements on named hardware/toolchain.
- Debian 13 interactive visible acceptance for merged rich-profile fixtures.
- Current availability and exact-head semantics of local CodeRabbit, GitHub Codex,
  and merge queue before Phase 3 planning.
- Whether Tauri generated permission metadata can be made checkout-location independent
  without requiring a broad clean.

## Recommended issue slices

1. **REF-01A — Replay merged Uzel POC from clean/relocated checkout.** Preserve current
   visible profile/follows behavior and produce one exact evidence manifest.
2. **REF-01B — Classify Uzel POC ownership, storage, retention, and bounds.** Documentation
   and probes only; no extraction.
3. **REF-01C — Admit product-first documentation.** Resolve untracked authority, narrow
   the root auditor, and commit one canonical document set plus issue/PR templates.
4. **NAPP dependency — Publish and accept product client/events/testkit candidate.** Owned
   by Napp; Uzel supplies exact consumer probes and expected lifecycle vectors.
5. **REF-01D — Adapt accepted Napp seam and replay parity matrix.** Blocked on 1–4; one
   boundary, no dual state, explicit rollback.
6. **REF-01E — Debian 13 visible acceptance.** Human/platform evidence only; do not mix
   with architecture changes.

## Planning readiness

**Conditional.** REF-01A, REF-01B, and the Uzel side of REF-01C are ready for detailed
planning now. Napp qualification can be planned as a fail-closed probe/handoff. REF-01D
is not ready to plan or execute until the authoritative product docs are committed and
an exact Napp client/events/testkit candidate exists. Phase 1 cannot be completed before
those blockers clear; parked milestone capabilities remain parked.
