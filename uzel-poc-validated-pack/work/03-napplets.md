# Work 03 — portable napplets

## Goal

Build two small runtime-agnostic napplets and their convention fixtures.

**Complete.** The portable builds, signed fixtures, convention tests, runtime
INC proof, and exact evidence are recorded in
[`../reports/slice-03-preflight.md`](../reports/slice-03-preflight.md).

## Entry status and pins

**Gate 0 line accepted.** The exact line is core/nap 0.29.0, shim 0.27.0, SDK 0.25.0, vite-plugin 0.12.0, conformance 0.14.0, and conformance-cli 0.2.16, all with the integrity values in [`../compatibility.lock`](../compatibility.lock). Kehto candidate artifacts built with this line pass released conformance without forbidden `fetch`.

## Read

- pinned Napplet package/conformance baseline
- `docs/00-scope.md`
- `docs/03-provisional-design.md`

## Tasks

- [x] Implement `follow-list` and `profile-card` as separate self-contained builds.
- [x] Define `contracts/profile-open-v1.schema.json`.
- [x] Use the exact provisional NAP revisions in `compatibility.lock`; do not describe draft relay/identity/storage or NIP-5D as ratified.
- [x] Test both in the accepted released conformance and pinned native-runtime harnesses where capabilities exist.
- [x] Build signed local manifests/artifacts using upstream formats and existing cryptographic libraries; do not invent signing or hashing rules.
- [x] Add `hostile-egress` test napplet separately.

## Acceptance

- [x] independent manifests and builds;
- [x] no Uzel imports or direct product network;
- [x] exact queryless `napplet:profile/open` behavior;
- [x] conformance and malformed-payload tests pass.
