# Slice 05 preflight — integrated composed demo

Date: 2026-07-29

Branch: `feat/slice-05-composed-demo`

Core implementation commits: `8edeab6`, `be0377e`, `b3fd3fb`; final evidence
and review-hardening commits are recorded in repository history.

## Outcome

**PASS. Work 05 acceptance is complete. Slice 06 may start.**

One daemon-owned pinned `RuntimeController` now runs both production-like
fixtures through the same NMP engine and stores. Uzel did not add a second
event, profile, follow, or app-state implementation.

## Exact dependency and upstream evidence

Uzel advanced from the Gate 0 candidate to reviewed successor
`jodobear/nampplets@7eccdee76a1afcfc3ff026c8f41b0072a4601840`.
That successor adds a bounded portable multi-surface host without modifying the
single-surface parser/bridge. The exact copied assets match SHA-256 values in
`compatibility.lock` and `scripts/check-pinned-assets.sh`.

[Fork PR #1](https://github.com/jodobear/nampplets/pull/1) is open and mergeable.
Codex reported no major issue at exact head `7eccdee...` after fixes for stale
same-ID frame mappings and the moved Apple sandbox assertion. Offline baseline,
AntiSlop, legacy evidence, file growth, Rust workspace, trusted shell, and
UniFFI Swift-binding CI are green. The Apple package job remained in progress
when this report was recorded; Apple execution is outside Uzel's Linux gate.
Cross-owner PR creation remains blocked by the fine-grained token, so the fork
branch and exact pin are the durable contribution record.

Kehto contribution [#218](https://github.com/kehto/web/pull/218) merged on
2026-07-29 as `4fd4affdd0043ea093c6b56a866f0f9f333e5375` with head
`62241de0b4526ba4fdc8a7b3c766c2499d3ae24d`; its checks are green.

## Runtime composition evidence

- The private fixture catalog contains exactly `good-morning`, `follow-list`,
  `profile-card`, and `hostile-egress`; unknown names and a fifth active fixture
  fail closed.
- Every launch verifies the exact event and bytes through upstream runtime
  APIs, burns a new persisted surface generation before exposure, and serves
  the document in bounded ordered chunks.
- Active fixtures are keyed by shell-owned surface token. Asset transfers are
  independently keyed and bounded.
- Provider replies, pushes, and handshake responses remain source-scoped.
  Only a runtime-authorized `inc.emit` may return another installed surface as
  its target.
- `daemon_routes_inc_delivery_to_the_other_exact_surface` performs both exact
  transfers, both NAP-SHELL handshakes, a profile subscription, and a
  follow-list emit, then observes delivery to profile-card with runtime-owned
  sender `follow-list`.
- The Svelte layer receives only a product-facing target surface and envelope.
  It cannot supply a principal, session, sender, or destination to the runtime.

## Linux shell evidence

The one trusted Tauri WebView mounts two opaque-origin `sandbox="allow-scripts"`
frames through the unchanged upstream host. Its top-frame document listener
receives source-bound host events, calls the thin Tauri client, then projects
the returned envelope only to the exact target surface.

The user-visible POC provides:

- side-by-side and stacked layouts;
- persisted split ratio and orientation;
- pointer and keyboard resize;
- directional focus controls;
- independent pane fullscreen/restore;
- selected public read identity and source/degradation status;
- exact-build proof, NAP-SHELL readiness, and session evidence;
- a developer mode hidden by default, with a bounded 40-entry type-only
  envelope log and no raw payload or secret values.

`pnpm dev` starts pinned `nak 0.20.1` on a configurable unprivileged loopback
port with the committed signed Nostr fixture, starts `uzel-napd --live` with
that explicit fallback relay and local-host allowlist, then starts Tauri. The
Fedora harness selects a run-specific port and disposable runtime/data roots.
No public relay or manual pre-seeding is required for the deterministic demo.

## Commands and results

All commands ran from repository root in the pinned Nix environment.

```text
pnpm check
  all three napplets built; Svelte 0 errors/0 warnings; frontend build and
  Cargo workspace check passed

pnpm test
  contract 2; follow-list 1; profile-card 4; hostile-egress 3;
  napd 17 passed/1 ignored; napd-protocol 4; uzel-napd 1;
  exact pinned trusted-shell assets passed

cargo test -p napd live_nmp_refreshes_then_restarts_cache_first_without_a_second_cache -- --ignored
  1 passed

pnpm test:conformance
  follow-list and profile-card each 6 passed, 0 failed, 4 skipped

pnpm lint
  Clippy warnings denied; BOUNDARIES_OK

pnpm format:check
  passed

pnpm smoke
  SLICE_05_COMPOSED_DEMO_SMOKE_OK

pnpm smoke:fedora
  FEDORA_RUN_SMOKE_OK daemon=ready shell=ready exact_builds=2 nap_shell=2
  artifact=responded source_bound=multi compositor=weston-headless-gl
```

The expected headless software-renderer/cursor warnings do not affect the
assertions. No failed Slice 05 run required reinterpretation as a pass.

Final self-review found that the initial cross-surface wait accepted any event
from any active session after `inc.emit`. An unrelated asynchronous provider
push could therefore satisfy the call before the routed delivery. The final
implementation admits other surfaces only while waiting for exact
`inc.event`; a regression rejects both `identity.changed` and
`inc.emit.result` as substitutes.

## Honest boundary

The contract test proves the queryless payload. Runtime and daemon tests prove
the authorized cross-surface delivery. The live WebKit harness proves both
exact frames, both handshakes, and artifact-authored provider traffic. It does
not synthesize a pointer click inside the nested opaque-origin frame, so this
report does not claim a browser-automation click proof.

Work 06 still owns the signed hostile artifact's complete WebKit probe matrix,
an independently validated ephemeral TCP sentinel with zero probe accepts, the
raw-handler authentication denial, final user/dev presentation assertions,
fresh-checkout demo reproduction, Debian acceptance, limitations, and
extraction debt.

## Exact next step

Execute `work/06-hardening-demo.md`. Configure the signed hostile fixture with
the live sentinel URL through exact-principal/exact-session NAP-CONFIG before
mount, collect each probe verdict, independently assert zero sentinel accepts
and raw WebKit command denial, then run final clean Fedora and Debian gates.
