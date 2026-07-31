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
| `kehto/web` | `jodobear/kehto-web:fix/napplet-conformance-no-modulepreload` | `62241de0b4526ba4fdc8a7b3c766c2499d3ae24d`; upstream merge `4fd4affdd0043ea093c6b56a866f0f9f333e5375` | build 32/32; typecheck 17/17; unit 1,576/1,576; gateway 15/15; chat/feed conformance 6/0/4 each; E2E and AI-slop green | [kehto/web#218](https://github.com/kehto/web/pull/218), merged 2026-07-29 | keep exact evidence pins; advance dependent corpus only through a separately validated change |
| `pablof7z/nampplets` | `jodobear/nampplets:compat/napplet-0.29` | `08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf` | all 16 Rust crates; 22 Python tests; four trusted-shell tests; digests and file-growth gate; exact Kehto repository binding; Uzel Slice 02 exact fixture verification, NAP-SHELL, runtime response, Fedora WebKit isolation, and Debian build pass | ready to submit from the existing fork branch; no Uzel-side upstream patch was needed | retain exact fork revision until an accepted upstream successor passes the same integrated Slice 02 probes |
| `pablof7z/nampplets` | `jodobear/nampplets:feat/trusted-shell-multi-surface` | `fc68bce0a4793a8618445e234bcc91d69e8b96de`; fork merge `570a01f8ae85c4dccd5452aa764e97d88c2f6611` | focused trusted-shell Node tests; real Chromium legacy host 4/4 and package 4/4 with one declared external-asset not-run; 22 Python conformance tests; 52-file/211-envelope/10-falsifier baseline; digests, file-growth, fmt, Clippy, and full Rust workspace tests; one-shot accepted/rejected environment acknowledgement and terminal host disposal added after Uzel review; exact-head Codex review clean; Linux-relevant CI green | [jodobear fork PR #1](https://github.com/jodobear/nampplets/pull/1), merged 2026-07-29; cross-owner upstream submission remains separately tracked | Uzel consumes the portable shell bytes from `fc68bce...`; Rust is independently pinned at the integrated compatibility head and must pass the same Linux probes |
| `pablof7z/nampplets` | `jodobear/nampplets:feat/catalog-naddr-coordinates` | `d5b9ad6ae9c37bb8aaa395d6908f3a0a9b0e3d17` | 18 Python conformance tests; baseline verification; fmt; workspace all-target Clippy with warnings denied; full workspace tests; raw and NIP-21 naddr acceptance; malformed and non-35129 refusal | [jodobear fork PR #2](https://github.com/jodobear/nampplets/pull/2), open; upstream creation blocked because both the fine-grained PAT and connected GitHub app lack cross-fork PR authority | Uzel uses compatibility branch `jodobear/nampplets:uzel/naddr-compat-e539` at `2cc404cfb52a4fd8bd227a04569c6228e201df96`; repin to an accepted upstream successor only after the full Linux runtime probes pass |
| `pablof7z/nampplets` | `jodobear/nampplets:feat/runtime-rust-provider-composition` | `488afb674a7279ebc4884e964bfcdfd4fa9e6823` | injected Rust provider remains available through exact-build permission review; 18 Python conformance tests; baseline verification; fmt; workspace all-target Clippy with warnings denied; full workspace tests | [jodobear fork PR #3](https://github.com/jodobear/nampplets/pull/3), open | Uzel consumes the compatibility branch composition at `dd64c5a8afa7eb32ed156a8acedca51617680c4e`, folded into current pin `d533a63d519c14470f900323958509cdea1c6479` |
| `pablof7z/nampplets` | `jodobear/nampplets:feat/identity-cache-refresh` | `dc5e974116e38f1945cbe7507a0ff52d38ae50a4`; fork merge `3849595288d08d5b7c46c02987236a4bc2d8dd53` | retains a configured relay observation after an empty cache frame; terminal empty evidence returns immediately; author-bearing identity reads use operator public lanes; cancellation uses at most eight lifecycle-owned callbacks with typed capacity refusal, drop-time unregistering, and no polling or waiter threads; `identity.rs` is 296 lines, `identity_refresh.rs` is 198, and `cancellation.rs` is 240; file-growth ratchet, 18 Python conformance tests, baseline verification, fmt, workspace all-target Clippy, focused runtime/adapter tests, full workspace tests, exact-head Codex review, and all eight CI jobs pass | [jodobear fork PR #4](https://github.com/jodobear/nampplets/pull/4), merged 2026-07-30 | Uzel still pins compatibility head `d533a63d519c14470f900323958509cdea1c6479`, which also composes open PRs #2/#3; build a combined successor from merged main plus those tracked heads, then advance only after exact identity, follows, picture, naddr, and Linux runtime probes pass |
| `pablof7z/nampplets` | `jodobear/nampplets:fix/runtime-provider-push-limits` | `e2f69f325a6b45213accdacfcc125e80e0687b4c` | validates explicit nonzero provider-push envelope/pending limits, overflow, and envelope-to-pending ordering; proves a response above the old 512-KiB ceiling crosses when opted in; full conformance, baseline, focused tests, format, strict Clippy, workspace tests, and generated Swift/UniFFI byte match pass; Uzel native Tauri/WebKit acceptance renders the supplied profile, 435 follows, and both tested JPEG avatars without runtime refusal | [jodobear fork PR #11](https://github.com/jodobear/nampplets/pull/11), open on the exact Uzel compatibility base; clean upstream port remains required because that base diverges from `pablof7z/main` | Uzel pins exact `e2f69f3...` and sets both limits to its existing 104,923,136-byte trusted-shell aggregate ceiling; any reviewed successor must repeat the native supplied-npub and follow-click acceptance before repin |

The final Uzel lifecycle review required no new upstream patch. The public
controller already exposes `snapshot`, exact `stop(session_id)`, and terminal
`close`. Uzel's thin adapter stops every identifiable session created by a
failed catalog launch refinement; if the post-launch projection refuses before
exposing any session identifier, it closes the controller fail-closed. Base
pane token retention and retry are shell-owned product state. Therefore no
additional fork branch or unpublished dependency was created for this round.

PR #8 review later proved one remaining seam mismatch: NMP frames already carry
scoped acquisition evidence and nampplets retains it internally, but released
`@napplet/nap` 0.29.0 identity result envelopes do not expose it to napplet
JavaScript. Uzel issue #12 owns the authority decision and any future upstream
branch. Commit `d907e8a` removes the product-side rendered-value freshness
workaround, so Uzel does not currently depend on an unpublished fix and no
speculative fork branch was created.

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

## Slice 06 upstream result

No new upstream defect was required to complete the malicious-child boundary.
Uzel's live sentinel, exact hostile NAP-CONFIG staging, Tauri command counter,
navigation policy, and acceptance harness are product-side proof code. They do
not change the NAP specs, NMP facade, or reusable nampplets runtime. The only
reusable shell work remains the merged fork contribution above. Any future
per-WebKit-child OS sandbox or trusted-local-TLS runtime change must start on a
separate branch in the relevant `jodobear` fork and be added here before Uzel
depends on it.

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
