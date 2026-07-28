# Upstream contribution ledger

Track every Uzel-discovered upstream change from validation through submission. An entry is required before Uzel depends on an unpublished fix.

## Authority and ownership

| Concern | Authority or implementation owner |
|---|---|
| Napplet protocol messages and lifecycle | `napplet/naps` |
| Napplet manifest, artifact, and sandbox semantics | pinned NIP-5D revision |
| Packaged web projection | `napplet/web` |
| Rust Nostr data plane, relay work, cache, freshness, and evidence | `pablof7z/nmp` |
| Reusable native runtime implementation | `pablof7z/nampplets` |
| Linux product composition | `jodobear/uzel`, later reusable `kehto/napd` seams |

Uzel uses implementation repositories as dependencies, not protocol authorities. `napd` composes exact nampplets crates and NMP through their public facades; it does not copy them.

## Active contributions

| Upstream | Fork branch | Exact commit | Uzel validation | Submission | Repin rule |
|---|---|---|---|---|---|
| `kehto/web` | `jodobear/kehto-web:fix/napplet-conformance-no-modulepreload` | `62241de0b4526ba4fdc8a7b3c766c2499d3ae24d` | build 32/32; typecheck 17/17; unit 1,576/1,576; gateway 15/15; chat/feed conformance 6/0/4 each; E2E and AI-slop green | [kehto/web#218](https://github.com/kehto/web/pull/218), open | continue on exact fork SHA; if review changes it, regenerate nampplets corpus and rerun V-01/V-02/V-06 before repinning |
| `pablof7z/nampplets` | `jodobear/nampplets:compat/napplet-0.29` | `08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf` | all 16 Rust crates; 22 Python tests; four trusted-shell tests; digests and file-growth gate; exact Kehto repository binding | not submitted; validate integrated Linux POC first | use exact fork revision; submit after integrated Uzel evidence; repin only to a revalidated reviewed successor |

## Entry template

```text
upstream repository:
jodobear fork:
branch:
exact commit:
Uzel dependency or affected seam:
validation commands and result:
upstream PR URL/state:
known review changes:
Uzel repin/revalidation action:
```

Do not mark a contribution complete merely because a PR exists. Completion means Uzel has either repinned to the accepted upstream commit or deliberately retained a documented fork pin.
