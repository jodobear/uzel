# Work 03 — portable napplets

## Goal

Build two small runtime-agnostic napplets and their convention fixtures.

## Entry status and pins

**Blocked by Gate 0.** The candidate line is core/nap 0.29.0, shim 0.27.0, SDK 0.25.0, vite-plugin 0.12.0, conformance 0.14.0, and conformance-cli 0.2.16, all with the integrity values in [`../compatibility.lock`](../compatibility.lock). Do not start until a compatible nampplets baseline exists and a production fixture built with this line passes conformance without forbidden `fetch`.

## Read

- pinned Napplet package/conformance baseline
- `docs/00-scope.md`
- `docs/03-provisional-design.md`

## Tasks

- Implement `follow-list` and `profile-card` as separate self-contained builds.
- Define `contracts/profile-open-v1.schema.json`.
- Use the exact provisional NAP revisions in `compatibility.lock`; do not describe draft relay/identity/storage or NIP-5D as ratified.
- Test both in Kehto/Paja or the accepted reference harness where capabilities exist.
- Build signed local manifests/artifacts using upstream formats and existing cryptographic libraries; do not invent signing or hashing rules.
- Add `hostile-egress` test napplet separately.

## Acceptance

- independent manifests and builds;
- no Uzel imports or direct network;
- exact queryless `napplet:profile/open` behavior;
- conformance and malformed-payload tests pass.
