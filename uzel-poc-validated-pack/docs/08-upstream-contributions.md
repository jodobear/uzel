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
| `pablof7z/nampplets` | `jodobear/nampplets:compat/napplet-0.29` | `08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf` | all 16 Rust crates; 22 Python tests; four trusted-shell tests; digests and file-growth gate; exact Kehto repository binding; Uzel Slice 02 exact fixture verification, NAP-SHELL, runtime response, Fedora WebKit isolation, and Debian build pass | ready to submit from the existing fork branch; no Uzel-side upstream patch was needed | retain exact fork revision until an accepted upstream successor passes the same integrated Slice 02 probes |
| `pablof7z/nampplets` | `jodobear/nampplets:feat/trusted-shell-multi-surface` | `fe25d1e` | focused trusted-shell Node tests; legacy host 4/4 and package 4/4; 22 Python conformance tests; 52-file/211-envelope/10-falsifier baseline; validator, digests, fingerprint, file-growth gate, Rust fmt/clippy/tests | [jodobear fork PR #1](https://github.com/jodobear/nampplets/pull/1), Codex review requested; cross-owner PR creation blocked by current fine-grained PAT | do not repin Uzel until exact-head review is clear; then rerun integrated Linux shell probes and submit the compatibility base before this dependent change upstream |

## Slice 02 upstream result

Uzel consumed the public `RuntimeController` facade and portable trusted-shell
assets unchanged. No defect or missing Linux API required a new change in
`jodobear/nampplets` or `jodobear/napd`, so Slice 02 created no additional
upstream branch. The existing nampplets compatibility branch now has integrated
Linux evidence and is the contribution to submit. Any later upstream-worthy
change must be made in its own branch in the relevant `jodobear` fork and added
to this ledger before Uzel depends on it.

## Slice 03 upstream result

No upstream code change was required. The napplets use released
`@napplet/nap`, `@napplet/shim`, `@napplet/vite-plugin`, conformance, and deploy
CLI APIs directly. The signed-manifest source-tag requirement was an integration
fact, not an upstream defect: the deploy CLI owns final source-bearing event
creation, while the Vite plugin owns the single-file build and sidecar inputs.
The queryless NAP-INC flow also needs no archetype or NAP-INTENT workaround.
Slice 03 therefore created no fork branch or upstream ledger row.

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
