# Requirements: Uzel product incubation through M5

## Scope rule

Each requirement maps to exactly one active GSD phase. The detailed acceptance language
is authoritative in
`docs/plans/uzel-product-incubation-v4-2026-08-10/01-BASELINE-REPLAY.md`,
`02-PRODUCT-ARCHITECTURE.md`, and `03-ROADMAP.md`.

## M0 — truthful baseline

- [ ] **M0-01:** Preserve `b185ad1` and produce one bounded exact-source/exact-lock replay verdict with claim-by-claim replacement/blocker evidence.
- [ ] **M0-02:** Produce a separate current package-output and Nix/native verdict; use `not_yet_packaged` and assign complete package acceptance to Phase 2 when outputs are absent.
- [ ] **M0-03:** Record current authority, durable-format, instance/local-profile, request-identity, storage/retention, trust and threat facts with planned deltas clearly separated.
- [ ] **M0-04:** Record measured CI/test/review behavior and a phase-pinned Codex/GSD/CodeRabbit/Rust/Node/Nix command profile without duplicate speculative gates.
- [ ] **M0-05:** Establish immutable upstream/RCP/terminology registries, manifest/exact-build SIR, negotiation vectors, capability ledgers, fairness baseline, interop/local-patch lifecycle, visibility and learning/closeout process.

## M1 — coherent composable Social Home

- [ ] **M1-20:** Package the trusted shell and hostile exact-build guest with compatibility-profile binding, fail-before-guest negotiation, transcript binding and bounded fair admission.
- [ ] **M1-21:** Deliver local read-only profile/actor binding plus cached/live text Home with honest freshness and cancellation.
- [ ] **M1-22:** Deliver semantic destination-mediated bounded resource fetch with SSRF/DNS/redirect/proxy defenses.
- [ ] **M1-23:** Deliver isolated low-authority raster normalization and bounded profile-local resource cache.
- [ ] **M1-24:** Deliver accessible People/Profile guest projection with partial-source and offline behavior.
- [ ] **M1-25:** Deliver typed bounded cross-surface intents without capability transfer.
- [ ] **M1-26:** Close M1 packaged diagnostics, native, hostile, accessibility and resource evidence.
- [ ] **M1-27:** Publish the compatibility kit and prove black-box composition with an external-source clean-room fixture while retaining the independent-peer M5 gate.

## M2 — local-first offline authoring

- [ ] **M2-30:** Add one daemon-hosted product-service owner and versioned crash-safe draft schema.
- [ ] **M2-31:** Deliver offline Composer create/edit/save/reopen through a scoped guest capability.
- [ ] **M2-32:** Deliver draft organization and explicit conflict semantics.
- [ ] **M2-33:** Close packaged offline recovery, corruption, migration and previous-green evidence.

## M3 — external signer and deliberate text publication

- [ ] **M3-40:** Pair an external signer and bind its reported key to a local profile under an explicit client-key lifecycle.
- [ ] **M3-41:** Commit an exact-scoped publication grant and trusted canonical event-template review.
- [ ] **M3-42:** Validate final signer output before every relay write and retain engine-owned per-relay evidence.
- [ ] **M3-43:** Close refusal, replay, timeout, restart, partial-relay and unknown publication recovery/UX.

## M4 — verified static-image attachment

- [ ] **M4-50:** Import one supported static raster through trusted selection, opaque handles, isolated parsing and a bounded profile-local object store.
- [ ] **M4-51:** Validate Blossom authorization before upload bytes and perform one bounded policy-mediated verified upload.
- [ ] **M4-52:** Verify remote bytes before atomic cache commit and support offline reopen.
- [ ] **M4-53:** Complete attachment, publication, fetch, offline reopen and atomic export without conflating outcomes.

## M4.5 — scheduling and cross-domain recovery

- [ ] **M45-60:** Persist exact-revision schedule intent and a narrow future-action grant under one bounded daemon scheduler.
- [ ] **M45-61:** Revalidate at due time and truthfully reconcile signer/relay/restart ambiguity.
- [ ] **M45-62:** Close the coherent Social-to-authoring-to-publication flow, optional bounded notification and scheduler resources.

## M5 — production-candidate hardening

- [ ] **M5-70:** Prove multi-profile actor/authority isolation.
- [ ] **M5-71:** Prove two simultaneous packaged instances with disjoint state and operations.
- [ ] **M5-72:** Deliver exact-build register/launch/revoke/remove lifecycle and trust tiers.
- [ ] **M5-73:** Deliver exact-build update/quarantine/rollback without authority inheritance.
- [ ] **M5-74:** Deliver narrow bounded operations and redacted diagnostics.
- [ ] **M5-75:** Close schema migration, integrity and corruption recovery.
- [ ] **M5-76:** Close backup/restore/profile-deletion and truthful package rollback.
- [ ] **M5-77:** Close native surfaces, accessibility and supported-Linux evidence.
- [ ] **M5-78:** Close L4 fuzz, interop/version-skew, independent-peer, supply-chain, reproducibility, security-review and resource evidence.
- [ ] **M5-79:** Freeze one exact not-release-approved candidate and A5 evidence bundle, then stop.

## Programme constraints

- [ ] Every listed phase after M0 is one contextual issue, manual worktree/branch and primary PR.
- [ ] One canonical Nostr engine remains the sole Nostr query/relay/store/freshness/sign/write owner.
- [ ] First-party guests receive no privileged bridge.
- [ ] The shell never opens daemon product/runtime/engine databases.
- [ ] Every guest-influenced queue has global/per-principal admission, fairness, anti-starvation, cancellation and cleanup.
- [ ] A5 is mandatory after Phase 7.9; milestone completion and the next programme require explicit human approval.

## Traceability

| Phase | Requirement(s) |
|---|---|
| 1 | M0-01, M0-02, M0-03, M0-04, M0-05 |
| 2–2.7 | M1-20 through M1-27, one per listed phase |
| 3–3.3 | M2-30 through M2-33, one per listed phase |
| 4–4.3 | M3-40 through M3-43, one per listed phase |
| 5–5.3 | M4-50 through M4-53, one per listed phase |
| 6–6.2 | M45-60 through M45-62, one per listed phase |
| 7–7.9 | M5-70 through M5-79, one per listed phase |

**Coverage:** 38 active requirements mapped across 34 active phases; no active requirement is
unmapped. Earlier REF/PKG/CI/PROF/SOC requirement text remains in Git history as
superseded pre-v4 planning evidence.

---
*Reoriented from committed v4 authority: 2026-08-11*
