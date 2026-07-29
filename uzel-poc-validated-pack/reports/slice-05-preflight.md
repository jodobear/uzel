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

Uzel advanced the Rust runtime from the Gate 0 candidate to validated successor
`jodobear/nampplets@e539378ef735ce06651fd94b71e06f9ce757cb13` and copies the
portable shell from contribution head
`jodobear/nampplets@fc68bce0a4793a8618445e234bcc91d69e8b96de`.
Those shell bytes add a bounded portable multi-surface host without modifying
the NAP/native envelope. Exact asset SHA-256 values are locked in
`compatibility.lock` and `scripts/check-pinned-assets.sh`.

[Fork PR #1](https://github.com/jodobear/nampplets/pull/1) is open and mergeable.
Codex reported no major issue at predecessor `7eccdee...` after fixes for stale
same-ID frame mappings and the moved Apple sandbox assertion. The fork main was
then fast-forwarded to current upstream main and merged into the contribution
branch so PR CI compares only contribution changes. Exact-head review and CI
remain pending; Apple execution is outside Uzel's Linux gate. Uzel does not pin
that merged branch head: current upstream now refuses every plaintext operator
relay, including the explicit loopback fixture. The exact attempt failed before
NMP refresh with `fallback relay ... must use a wss:// address`. This POC keeps
the previously validated `e539378...` runtime pin and records trusted local TLS
as post-POC deployment work rather than weakening upstream relay policy.
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
  shell_accepted=2 artifact=responded source_bound=multi
  compositor=weston-headless-gl
```

The expected headless software-renderer/cursor warnings do not affect the
assertions. No failed Slice 05 run required reinterpretation as a pass. Two
pre-runtime Fedora attempts are preserved locally under
`reports/probes/slice-05-fedora-linker-contamination-failed/` and
`reports/probes/slice-05-fedora-cold-build-timeout-failed/`: the first mixed a
Nix WebKitGTK library requiring `GLIBC_2.42` with a host-linked Cargo target;
the second reached crate 420/426 but exhausted the harness deadline on a cold
isolated target. The accepted command reused only that isolated target after
the failed cold build, launched WebKit, and observed both acknowledgements.

Final self-review and Codex review found that the initial cross-surface wait
accepted any event from any active session after `inc.emit`. An unrelated
asynchronous provider push could therefore satisfy the call before the routed
delivery. The final implementation admits other surfaces only while waiting
for exact `inc.event` with the requested topic; a regression rejects
`identity.changed`, `inc.emit.result`, and another INC topic as substitutes.

Codex also found two shell-state errors. Selecting a new public identity only
changed daemon state, leaving the once-loaded follow list stale. The shell now
remounts both exact frames after a successful runtime identity change, which
forces fresh provider reads while reusing the same authorized sessions and the
upstream host's reviewed remount behavior. Directional and named focus controls
previously changed only styling. They now move DOM focus to a shell pane;
Enter explicitly enters its mapped iframe, so successive directional keys are
not trapped inside the opaque child. The final review also corrected stacked
fullscreen track specificity and preserves the declared 42/58 split when
local storage has no prior value.

The final exact-head review found one remaining presentation-state race: both
handshakes could complete before startup wrote its unconditional waiting text,
leaving a green `2/2 READY` proof beside a stale message and potentially
overwriting a handshake error. The shell now enters waiting before the first
mount, changes to ready only after both `shell.init` envelopes are accepted by
the trusted host, and latches handshake failures against later ready updates.
The shared readiness predicate also includes that failure latch, so neither the
green indicator nor the proof strip can advertise a failed composition as
ready merely because both surface tokens were counted.

The next exact-head review found that synchronous host delivery of
`shell.init` was not proof that the asynchronous child prelude accepted the
projected environment. Fork PR #1 now uses a one-shot transferred
`MessageChannel`; only the captured prelude path acknowledges after exact
acceptance. A rejected environment explicitly reports rejection before closing
the port; the host invokes only the source/remount-bound `onError` callback.
Uzel counts readiness only from `onReady` and latches either rejected
environment or identity-driven remount refusal as a failed handshake.

The final review also separated lifecycle state from ordinary request health.
Only mounting and `shell.ready`/`shell.init` routing may latch a failed
NAP-SHELL proof. A later napplet request or diagnostics refresh failure remains
visible as a runtime error but cannot rewrite two already accepted handshakes
as `FAILED`.

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
