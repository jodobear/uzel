# Work 03 — portable napplets

## Goal

Build two small runtime-agnostic napplets and their convention fixtures.

## Read

- pinned Napplet package/conformance baseline
- `docs/00-scope.md`
- `docs/03-provisional-design.md`

## Tasks

- Implement `follow-list` and `profile-card` as separate self-contained builds.
- Define `contracts/profile-open-v1.schema.json`.
- Use pinned NAP-SHELL, INC, relay/identity/storage behavior only.
- Test both in Kehto/Paja or the accepted reference harness where capabilities exist.
- Build signed local manifests/artifacts.
- Add `hostile-egress` test napplet separately.

## Acceptance

- independent manifests and builds;
- no Uzel imports or direct network;
- exact queryless `napplet:profile/open` behavior;
- conformance and malformed-payload tests pass.
