# Ecosystem source scan — 10 August 2026

**Purpose:** dated planning snapshot for Uzel M0. These mutable links are discovery
locators, not execution pins. M0 must capture commit/tree/path/content digests or package
integrity for every used source.

## Sources inspected

| Surface | Discovery locator |
|---|---|
| Nostr NIPs governance | https://github.com/nostr-protocol/nips/blob/master/README.md |
| Current NIP-5A | https://github.com/nostr-protocol/nips/blob/master/5A.md |
| NAP registry/governance | https://github.com/napplet/naps |
| Napplet NIP-5D guide/spec | https://napplet.run/docs/guide/nip-5d.html and https://napplet.run/docs/spec.html |
| Napplet packages/conformance | https://napplet.run/docs/packages/ |
| NMP engine | https://github.com/pablof7z/nmp |
| Current Open GSD commands | Active `open-gsd/gsd-core` sources/docs plus exact installed help/version, as resolved and pinned in M0 |

The older GSD repository has moved/been archived in favor of Open GSD. Active online docs,
a development branch and an installed release may still differ in optional command flags.
Phase 1 therefore records the immutable installed tool/version and full help output; that
phase pin controls execution while online sources remain observation inputs.

## Observations

### Nostr standards posture

The NIPs repository describes NIPs as implementation possibilities rather than a required
checklist. A NIP number or merged filename does not mean that every Uzel surface must
implement it, and relevant deployed work may exist in open proposals or external
implementations.

**Uzel consequence:** select the smallest relevant subset, capture immutable sources,
state optionality/deviations, and require implementation/interoperability evidence.

### Manifest and exact-build identity are moving

Current merged NIP-5A is “Static Websites (nsites)” and defines nsite manifests including
kinds `15128` and `35128` plus snapshot kind `5128`. The still-open NIP-5D proposal has
moved toward separate napplet manifest kinds `15129` and `35129` plus snapshot kind `5129`,
and its bootstrap/domain semantics have continued changing while discussion remains open.
Current napplet documentation and tooling may lag that proposal or use different wording.

This is a live cross-source seam. Uzel must not encode “NIP-5A napplet identity” or
“NIP-5D support” as an unqualified stable contract.

**M0 gate:** create a SIR and canonical machine-readable RCP that fixes, by immutable
source identity:

- manifest/event kinds;
- path and aggregate-hash rules;
- source/publisher/exact-build binding;
- required/optional capability declaration grammar;
- bootstrap and domain projection;
- update, replacement, revocation and previous-profile behavior;
- unknown-field/message handling;
- package/conformance revisions and vectors.

### Capability negotiation is not a safe implicit-presence check

Current napplet guidance includes build-time requirements and runtime domain-presence
checks. For a security-oriented native runtime, object presence alone is insufficient for
required capabilities because it permits ambiguity and accidental downgrade.

**Uzel consequence:** before guest code executes, verify the exact build and active RCP,
parse required/optional declarations, reject missing required capabilities, expose optional
omissions explicitly, and bind a canonical negotiation transcript to the session.

### NAP status and implementation-led movement

The NAP registry contains a mix of active and draft domains and evolves alongside real
implementations. The exact statuses and schemas must be captured at M0 because names and
contracts may change after this dated scan.

**Uzel consequence:** use only the domains/subsets named by the RCP. Draft behavior is a
profiled implementation choice with tests and SIRs, not universal conformance.

### Napplet packages and conformance

The package set provides SDK/shim/Vite manifest/conformance surfaces and is moving with the
specification. Package availability or a standalone conformance pass does not prove that
the installed Uzel package uses the same source verification, negotiation, browser/native
and runtime path.

**Uzel consequence:** pin packages and source integrity, add Uzel-owned behavioral vectors,
and show which conformance checks reach the real packaged path.

### NMP maturity and ownership

NMP currently describes itself as pre-1.0/pre-v2: ownership boundaries and behavioral
invariants are the relatively stable frame, while app-facing names and shapes remain
provisional. It owns network work, relay routing, canonical state, signing and durable
publication and maintains explicit known-gap/issue evidence.

**Uzel consequence:** exact-pin NMP behind a narrow private adapter; preserve its semantic
ownership; do not leak provisional provider types into Uzel durable/public contracts; use
issue-first upstream engagement and evidence-led adoption campaigns.

## Immediate M0 outputs

1. `docs/ecosystem/upstreams.toml`, generated from a validated schema, with immutable pins,
   observation locators and contribution/security routes.
2. `docs/compat/profiles/<profile-id>.toml` as canonical RCP plus generated human rendering.
3. Package binding to canonical profile bytes/hash and trusted diagnostics exposing them.
4. A SIR for manifest/exact-build/capability-negotiation semantics.
5. A capability-domain status map: enabled, optional, profiled draft, experimental or
   unsupported.
6. Canonical launch-negotiation/transcript schema and vectors.
7. Contract/conformance fixtures tied to immutable sources and the packaged path.
8. An interop/version-skew plan including a qualifying independent napplet.
9. Upstream and local-patch records with visibility/disclosure, AI-assistance,
   authorship/signoff and human-approval state.
10. Canonical cross-component terminology registry and executable teaching-witness schema.
11. Per-principal admission/fairness/anti-starvation baseline.
12. A Phase 2 go/no-go decision for the exact RCP.

## Stop condition

Phase 2 must not execute guest code until Uzel can prove:

- immutable executing build/source/manifest identity;
- active canonical RCP bytes/hash;
- exact manifest/hash/bootstrap interpretation;
- fail-closed required/optional negotiation before guest execution;
- transcript/session binding;
- update/revocation and unsupported-profile behavior;
- no silent branch-head or documentation-driven semantics.
