---
phase: 01-slice-ref-01-poc-replay-accepted-napp-seam
generated: 2026-08-12
source: adr-ingest-express-path
source_file: docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST-ADR.md
source_authority_sha256: 098a50e58cb77b9363fdb8fab769ae1f592b96bd3efcc33e6b34a6a8e5501f6d
---

# Phase 01 Context — Truthful Baseline and Execution Reset

**Source:** ADR Ingest Express Path (docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST-ADR.md)

<domain>
## Domain Boundary

This file is a parser-facing projection of `00-GSD-INGEST.md`, not an independent authority. Source authority: `docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST.md`. Source SHA-256: `098a50e58cb77b9363fdb8fab769ae1f592b96bd3efcc33e6b34a6a8e5501f6d`. The source remains byte-identical; this projection exists because GSD 1.10.0 narrative ingest recognizes canonical ADR headings but not the source document's programme-specific headings. Supporting contracts remain `01-BASELINE-REPLAY.md`, `02-PRODUCT-ARCHITECTURE.md`, `03-ROADMAP.md`, `04-DELIVERY-QUALITY.md`, `06-START-RUNBOOK.md`, `07-ECOSYSTEM-UPSTREAM.md`, `08-DECISIONS-LEARNING.md`, and `09-PRODUCTION-MATURITY.md`. Owner direction recorded on 2026-08-12 fixes review-tool routing to local CodeRabbit and GitHub Codex only; a recorded CodeRabbit rate limit permits green GitHub Codex to satisfy the gate. Claude, OpenCode, remote CodeRabbit and local Codex self-review are not programme review lanes.
</domain>

<decisions>
## Locked Decisions

- **D-01:** Treat `00-GSD-INGEST.md` at the recorded SHA-256 as programme authority; this ADR is only its lossless planning projection and must be reviewed against that source whenever either changes.
- **D-02:** Uzel is the sole active product implementation repository through M5; build visible vertical product slices while preserving separate daemon, exact-build guest, runtime, engine, shell, and product-service ownership boundaries.
- **D-03:** Use one exact-pinned canonical Nostr engine behind a narrow private adapter; verify current source before assuming its API, state, cache, query, relay, provenance, or diagnostic ownership.
- **D-04:** Keep product-service, runtime, and engine state instance-scoped from their first new schema; additionally local-profile-scope every identity-dependent record and all object bytes/derivatives before A5; daemon-host product semantics and never let the shell open product DB state directly.
- **D-05:** Build no general filesystem or media platform before A5; use trusted selection, opaque handles, bounded reads, explicit import/export, atomic overwrite, and a private no-network resource-limited raster worker when untrusted normalization is required.
- **D-06:** M5 yields a frozen audit candidate only; after delivery phase 7.9 stop feature work, roadmap advancement, dependency churn, and milestone completion until twelve-lane A5 plus remediation passes and the owner decides the next programme.
- **D-07:** After M0 each bounded integer or decimal phase is one contextual issue, branch, human-created worktree, and primary PR; GSD automatic worktrees and auto-advance remain disabled for Codex Phase 1.
- **D-08:** Bind every packaged build to one immutable Uzel Runtime Compatibility Profile; exact-pin moving specs, proposals, conformance tools, and libraries; admit changes only through evidence-led compatibility campaigns.
- **D-09:** Do not begin Phase 2 until NIP-5A, NIP-5D, NAP, tool, manifest, and exact-build identity disagreement has an accepted profile, Spec Interpretation Record, vectors, and explicit go/no-go result.
- **D-10:** Maintain capability-maturity, decision, spec-interpretation, upstream, terminology, learning, education, visibility, and milestone-digest records; M5 needs L4 evidence, while only A5 remediation and human approval can reach L5.
- **D-11:** Every guest-influenced path needs bounded global and per-principal admission, fairness, anti-starvation, cancellation, cleanup, and privacy-safe resource-pressure diagnostics; a successful soak does not prove an unbounded queue safe.
- **D-12:** Preserve blocked commit `b185ad1`, its parent, worktree, durable safety ref, portable checksummed archive, diff, generated files, tests, and assumptions; planning must not merge, delete, amend, cherry-pick, reset, or assign final disposition without execution evidence.
- **D-13:** Correct Phase 1 through two independent evidence tracks: Track A hermetic exact-source replay and Track B current Nix package/native acceptance; neither track repairs, substitutes for, or determines the other track's verdict.
- **D-14:** Track A must inventory exact historical source, package manager, lockfile, runtime/toolchain, scripts, Vite origin, conformance-tool origin, and each command as workspace binary, repository script, Nix app/derivation, exact lock dependency, integrity-bound historical dependency, or unavailable historical tool.
- **D-15:** Track A may realize only lock-, integrity-, or hash-bound bytes; any fetch stage may obtain only already-fixed missing store content, then replay in a disposable exact-source worktree with disposable HOME, XDG, and cache paths.
- **D-16:** Disable dependency lifecycle scripts by default; permit only an exact reviewed historical allowlist, deny public DNS and external egress during scripts/materialization/replay, and allow only explicitly enumerated loopback or Unix-socket fixtures.
- **D-17:** Keep historical source and locks byte-identical; forbid ambient/global binaries, dynamic `npx`, copied dependency trees, package substitution, compatibility edits, undeclared downloads, and artifact-only replacement of source replay.
- **D-18:** Bound Track A to one source/tool inventory, one exact-closure realization path, one isolated replay attempt, and at most one evidence-proven harness-only correction; record exactly one of `reproduced`, `reproduced_with_variance`, `unavailable_exact_closure`, `unavailable_platform`, `failed_behavior`, or `superseded_test`.
- **D-19:** An unavailable historical replay passes only when current-source proof independently replaces every affected critical security/correctness invariant and explicitly retires or downgrades the old claim; otherwise Phase 1 remains blocked.
- **D-20:** Track B must separately test canonical Nix build/package outputs, store-path runtime/resource discovery, clean HOME/XDG, explicit instance IDs, deterministic disjoint fixture paths, user-service start/restart/stop, incompatible local-control failure, native WebKit/Weston, Fedora SELinux evidence, and measured closure/build/startup/resource baselines.
- **D-21:** If current locked source has no installable package output, Track B reports `not_yet_packaged`; Phase 2 owns package acceptance, and Phase 1 must not invent packaging or simultaneous multi-instance product behavior.
- **D-22:** Plan `01-01` owns incident reconciliation, source/tool/lock inventory, bounded exact-closure replay, hostile egress proof, replay verdict, invariant impact table, and evidence-based `b185ad1` disposition; it must not change product behavior, dependencies, historical source, or locks.
- **D-23:** Plan `01-02` owns current Nix/package/native/store-path/user-service/clean-state/instance-path/version-mismatch/SELinux/timing evidence and depends on `01-01` only for current-source claim mapping, not a green historical verdict.
- **D-24:** Plan `01-03` owns source-grounded authority, identity, schema, state, instance/local-profile, trusted-grant, signer, canonical-engine, guest/browser/daemon/OS threat, parked-capability, and product visual/interaction baselines; implemented facts and planned deltas must stay distinct.
- **D-25:** Plan `01-04` owns measured command, CI lane, timing, cache, bottleneck, focused-test, local CodeRabbit and GitHub Codex review on exact candidate SHAs, naming, unsafe, dependency, and artifact-integrity baselines; Claude, OpenCode, remote CodeRabbit and local Codex self-review are excluded, and speculative affected-crate classification waits for measurements.
- **D-26:** Plan `01-05` owns exact source-pin ecosystem verification, immutable upstream registry and compatibility profile, package/profile-hash binding plan, fail-before-guest capability negotiation and transcript vectors, SIR, interop/version-skew matrix, clean-room fixture and independent-peer gate, maturity ledgers, upstream/local-patch records that separate merge/release/Uzel-adoption/patch-removal states, dedicated upstream fork/worktree/contribution-policy process, terminology/decision/learning/education records, admission/fairness baseline, and read-only upstream radar. Plans `01-06` and `01-07` are bounded execution-only decompositions of D-31 closeout: Plan 06 projects public evidence and freezes E through normal atomic task commits; Plan 07 commits packet P, stops for immediate hold or exact-P-review-gated approve, writes the bound decision, and preserves the distinct later final-SHA condition. They add no outcome or scope beyond authoritative ingest.
- **D-27:** Execute `01-01` through `01-07` sequentially in the human-created Phase 1 worktree with strict chain `01-01 -> 01-02 -> 01-03 -> 01-04 -> 01-05 -> 01-06 -> 01-07`; replay may use one separate disposable checkout.
- **D-28:** Preserve `.planning` history, maps, Phase 1 numbering, pause evidence, and integer phases 2 through 7; insert only required decimal phases including 2.7, preserve the exact v4 sequence through 7.9, and do not rerun project creation, milestone creation, onboarding, or mapping absent a separately approved corruption repair.
- **D-29:** Phase 1 planning and execution must not add product features, float/update dependencies, build a broad file/media platform, expose a public API programme, contact upstream, publish external issues/comments/PRs, or claim universal conformance.
- **D-30:** Phase 1 closes only with separate replay and package verdicts, critical-invariant replacement evidence, immutable compatibility-profile/upstream identities, accepted identity/negotiation interpretation, capability/knowledge owners, checkout-independent package evidence or `not_yet_packaged`, measured CI/review evidence, coherent GSD state, and no unresolved blocking review finding.
- **D-31:** Prompt 02 review against the exact plan head is mandatory after planning: attempt local CodeRabbit on the immutable local diff, then GitHub Codex reviews the exact pushed PR SHA. If CodeRabbit returns a recorded `rate_limit` error before findings, green GitHub Codex satisfies the review gate; no other CodeRabbit failure does. Any later commit invalidates review evidence. No other AI reviewer is part of this programme, no Phase 1 implementation occurs before the applicable review path passes, and after reading the Phase 1 evidence matrix a human must explicitly approve Phase 2 before it begins.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

- `docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST.md` — immutable programme authority at SHA-256 `098a50e58cb77b9363fdb8fab769ae1f592b96bd3efcc33e6b34a6a8e5501f6d`
- `docs/plans/uzel-product-incubation-v4-2026-08-10/00-GSD-INGEST-ADR.md` — parser-facing projection; never independent authority
- `docs/plans/uzel-product-incubation-v4-2026-08-10/01-BASELINE-REPLAY.md` — replay/package verdict contract
- `docs/plans/uzel-product-incubation-v4-2026-08-10/02-PRODUCT-ARCHITECTURE.md` — product/runtime/state/trust ownership contract
- `docs/plans/uzel-product-incubation-v4-2026-08-10/03-ROADMAP.md` — delivery map through 7.9 and A5 stop
- `docs/plans/uzel-product-incubation-v4-2026-08-10/04-DELIVERY-QUALITY.md` — CI/review/evidence contract
- `docs/plans/uzel-product-incubation-v4-2026-08-10/06-START-RUNBOOK.md` — worktree, planning, review, execution, verification, and human gates
- `docs/plans/uzel-product-incubation-v4-2026-08-10/07-ECOSYSTEM-UPSTREAM.md` — compatibility and upstream stewardship
- `docs/plans/uzel-product-incubation-v4-2026-08-10/08-DECISIONS-LEARNING.md` — decisions, terminology, learning, and education
- `docs/plans/uzel-product-incubation-v4-2026-08-10/09-PRODUCTION-MATURITY.md` — capability maturity and candidate criteria
- `docs/plans/uzel-product-incubation-v4-2026-08-10/prompts/02-review-phase-1.md` — mandatory independent plan-review gate
</canonical_refs>

<specifics>
## Plan Sequence

- `01-01`: reconcile incident and produce bounded exact-source replay evidence.
- `01-02`: establish current Nix package/native baseline or honest `not_yet_packaged` verdict.
- `01-03`: establish source-grounded authority, schema, state, identity, threat, and UI grammar baseline.
- `01-04`: establish measured CI, testing, review, and artifact-integrity baseline.
- `01-05`: establish ecosystem, compatibility-profile, negotiation, maturity, upstream, terminology, learning, and fairness baseline.
- `01-06`: project public evidence and freeze immutable Task-2 commit E through standard task commits.
- `01-07`: generate/commit P, stop for immediate hold or exact-P-review-gated approve, write decision, and preserve distinct exact-final-SHA gate.
- Stop for Prompt 02 review: attempt local CodeRabbit, then require green GitHub Codex on the exact PR SHA; recorded CodeRabbit rate limiting activates the approved fallback. Revise through `$gsd-plan-phase 1 --reviews` as required; do not execute in review session.

## Success Criteria

- Parser returns accepted status, at least 31 stable decision IDs, explicit scope fences, dependencies, plan sequence, and Phase 1 exit criteria with no programme-specific headings silently unmapped.
- Generated Phase 1 context names both evidence tracks, all seven bounded plans, `b185ad1`, immutable profile/registry work, manifest-identity gate, fairness ownership, approved review/rate-limit fallback, and human stop before Phase 2.
- Plan checker remains enabled and every `M0-01` through `M0-05` requirement plus every trackable decision is covered by an executable plan.
- Original `00-GSD-INGEST.md` remains byte-identical at the recorded SHA-256.
- Pack checksum, manifest, structural audit, and independent authority-delta review pass before this projection is used for planning.

## Risk Summary

- Projection drift could omit or weaken source authority; checksum binding, stable decision IDs, pack audit, local CodeRabbit and GitHub Codex mitigate it.
- Historical exact closure or platform may be unavailable; honest non-green verdicts and invariant replacement rules prevent false success.
- Current package outputs may not exist; `not_yet_packaged` preserves ownership without pulling Phase 2 work into M0.
- Over-broad M0 planning could import parked product capability; explicit scope fences and plan checker must block it.

## Dependencies

- `00-GSD-INGEST.md` at SHA-256 `098a50e58cb77b9363fdb8fab769ae1f592b96bd3efcc33e6b34a6a8e5501f6d` is authoritative.
- `01-BASELINE-REPLAY.md` defines replay and package verdict rules.
- `02-PRODUCT-ARCHITECTURE.md` defines ownership, authority, identity, state, trust, sandbox, and resource boundaries.
- `03-ROADMAP.md` defines the delivery-phase map through 7.9 and mandatory A5 stop.
- `04-DELIVERY-QUALITY.md` defines measured CI, review, validation, rollback, and evidence gates.
- `06-START-RUNBOOK.md` defines worktree, GSD, local-CodeRabbit/GitHub-Codex review, execution, verification, and human-stop sequence.
- `07-ECOSYSTEM-UPSTREAM.md`, `08-DECISIONS-LEARNING.md`, and `09-PRODUCTION-MATURITY.md` define ecosystem, knowledge, terminology, maturity, and upstream records.
</specifics>

<deferred>
## Deferred Items

- Final `b185ad1` disposition remains provisional until Phase 1 replay execution evidence exists.
- Packaging absent from current locked source is owned by Phase 2 after a Phase 1 `not_yet_packaged` verdict.
- Simultaneous packaged multi-instance operation absent today remains a planned Phase 7.1 delta.
- Production approval and L5 capability maturity remain post-A5 human decisions.
- Public upstream interaction remains a separately authorized, human-reviewed action.
</deferred>

<scope_fence>
## Out of Scope

- Product feature implementation during Phase 1 planning.
- Dependency upgrades, floating pins, package substitution, or broad refactoring.
- General filesystem, media, plugin, mount, public remote daemon, or public API platforms.
- Automatic upstream contact, issue/comment/PR publication, release, or adoption.
- Phase 2 through 7.9 implementation and all post-7.9 feature work.
- Automatic A5 approval, production designation, release, or milestone completion.
- Mutation or loss of `b185ad1`, its blocked worktree, safety ref, or checksummed evidence.
</scope_fence>
