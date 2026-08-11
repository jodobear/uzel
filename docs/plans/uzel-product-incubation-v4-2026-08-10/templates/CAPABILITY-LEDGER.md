# Capability ledger — <capability-id>

- **Owner:**
- **Supported profile/hash scheme/hash:**
- **Compatibility-kit surface:** none | exact manifest/digest
- **Maturity:** L0 | L1 | L2 | L3 | L4 | L5
- **Last evidence revision:**
- **Next promotion gate:**
- **Guest trust tier:**
- **Visibility:** public | internal | embargoed

## Claim and journeys

Exactly what is supported and what is not.

## Trust and identity

Request identity, authority boundary, grants, secrets and guest trust tier.

## Contract, negotiation and state

Schemas, required/optional capability negotiation, handshake-transcript binding, state
machine, unknown/cancel/timeout/restart behavior.

## Bounds, admission, fairness and operations

Admission principal; global/per-principal in-flight, queue, byte, wakeup and retained-result limits; fair scheduling or explicit anti-starvation policy; typed overload behavior; cancellation/cleanup; memory/CPU/size/time bounds; privacy-safe diagnostics and safe controls.

## Persistence and recovery

State owner, migration, backup, restore, corruption and rollback truth.

## Evidence matrix

| Evidence | Location/revision | Result | Gap |
|---|---|---|---|
| Contract/conformance | | | |
| Clean-room external-source fixture | | | |
| Independently maintained peer | | | |
| Adversarial/fuzz | | | |
| Native/package | | | |
| Interop/version skew | | | |
| Performance/soak/fairness | | | |

## Upstream and known gaps

Exact dependencies, upstream records, local patches, risks and removal/review triggers.
