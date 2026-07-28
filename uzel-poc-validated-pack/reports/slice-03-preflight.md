# Slice 03 preflight

## Verdict

**PASS.** `follow-list` and `profile-card` are independent portable napplets.
They build to self-contained artifacts, pass released conformance, verify as
signed exact builds through the pinned native runtime, and exchange the exact
queryless profile-open payload with a runtime-owned sender. Work 04 may start.

## Exact pins and fixtures

| Input | Exact value |
|---|---|
| NAP registry | `napplet/naps@5ac0490461ca6fec2f0d2e45b4835cf9bc08de24` |
| packages | `nap@0.29.0`, `shim@0.27.0`, `vite-plugin@0.12.0`, `conformance-cli@0.2.16`, `cli@0.0.0` |
| signing runtime | Deno 2.9.4; frozen `deno.lock`; `nostr-tools@2.24.0` |
| native runtime | `jodobear/nampplets@08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf` |
| NMP | `pablof7z/nmp@005dc2a5f12aa414961b313d05ebb021934e385c` |
| fixture author | `b0e591472ce87429291f883c1101f54b4ba2c082074aded52b97a8dcbc87a4cd` |
| follow-list | event `ce54276d...`; aggregate `eaf4e565...`; index `3ae0e253...`; 58881 bytes |
| profile-card | event `a019be60...`; aggregate `71c7c91d...`; index `f294c630...`; 59872 bytes |
| hostile-egress | event `a4141f41...`; aggregate `6dcafdf3...`; index `94fd9d4e...`; 58909 bytes |
| `pnpm-lock.yaml` | `aea77d4cf2403cf7d0f2e396d4f83164e34d48b31dd69f8b25a2c6cc24e3137b` |
| `deno.lock` | `23209bc013d259aafd7dd06eb8111a646e84cc13defa701b7cb9c3fd3e5d1287` |
| contract schema | `767a1f80409e9357e4a79109747d9feb17060df46e8361a6c9df85efea70b830` |

Full event, artifact, and aggregate values are machine-locked in
[`../compatibility.lock`](../compatibility.lock) and enforced by
`scripts/check-pinned-assets.sh` plus the Rust verifier tests.

## API and ownership evidence

The source uses only published calls:

```text
identityGetFollows()
incEmit("napplet:profile/open", {version: 1, pubkey})
incOn("napplet:profile/open", callback)
outboxQuery(filters, options)
```

`follow-list` owns selection and payload emission. NMP and the nampplets provider
own canonical replaceable kind-0 selection. `profile-card` requests one row,
requires exactly one provider result, then owns only strict payload/content
validation and display projection; it never sorts candidates or falls back to a
replaced event. The runtime owns sender identity, session/surface mapping,
capability routing, Nostr evidence, and exact build verification. Neither
napplet imports Uzel, Tauri, `napd`, NMP, nampplets, or the other napplet.

The native integration test initializes both sessions through NAP-SHELL,
subscribes `profile-card`, emits from `follow-list`, and observes:

```text
topic: napplet:profile/open
payload: {"version":1,"pubkey":"<64 lowercase hex>"}
sender: follow-list
```

Caller-supplied sender identity is not used.

## Manifest correction

The Vite plugin successfully creates a self-contained `index.html`; temporary
asset files are inlined and removed. Its `.nip5a-manifest.json` sidecar has no
signed `server` tag, so the pinned NMP resolver rejects it with `no
policy-approved blob source is available`. That rejection is correct.

The final fixture command uses released `@napplet/cli@0.0.0` in dry-run mode.
The CLI adds the signed source hint and signs with its pinned `nostr-tools`.
Uzel supplies a disposable environment-only key and never implements or stores
private-key, signature, event-ID, file-hash, or aggregate-hash logic. The
committed `https://blossom.invalid/` tag is a reserved non-routable local-fixture
source; it is not a live Blossom claim.

The CLI preserves `requires` from the plugin sidecar but does not manufacture
archetypes. `profile-card` intentionally has no archetype: direct queryless
NAP-INC is sufficient, and Uzel still does not implement NAP-INTENT.

## Commands and observed results

```sh
pnpm install --frozen-lockfile
pnpm check
pnpm test
pnpm test:conformance
pnpm lint
pnpm format:check
pnpm docs:check
pnpm fallow
pnpm smoke
pnpm smoke:fedora
pnpm smoke:debian
```

All commands run in the pinned Nix development shell. Observed:

- contract tests: 2 pass;
- napplet unit tests: follow-list 1, profile-card 4, hostile-egress 3 pass;
- Rust workspace: napd 4 and protocol 1 pass, including three signed-fixture
  verifications and queryless runtime INC delivery;
- conformance: follow-list and profile-card each 6 passed, 0 failed, 4 skipped;
- Clippy `-D warnings`, rustfmt, boundary and exact-asset checks pass;
- Fallow reports `total_issues: 0`; health estimates remain informational;
- Fedora emits `FEDORA_RUN_SMOKE_OK` under real headless WebKit/Weston;
- Debian Bookworm emits `DEBIAN_BUILD_SMOKE_OK` after the full locked build.

The conformance CLI skips manifest checks for local-directory input. This is
not presented as manifest coverage: the Rust test independently verifies all
three signed events and exact artifact bytes through the pinned NMP runtime.

## Automated review corrections

Codex's first review found three P2 defects. The corrected profile card clears
old name/about/evidence immediately when a valid selection starts, and a tested
request-generation gate ignores completions from superseded queries. The
boundary audit now matches side-effect imports as well as static/dynamic
imports and export-from statements, with built-in regression cases. Source
changes triggered official CLI regeneration and repinning of all signed fixture
events; the stale pre-review artifact was not retained.

Codex's second review found that a one-event outbox limit made the tested
malformed-newest fallback unreachable, and that Worker construction completed
before its network load outcome. The query now requests a bounded five
candidates. The worker probe waits for `message` or `error`; silence times out
as not-proven-denied rather than producing a false pass. Both behaviors have
regression coverage, and the official CLI regenerated and repinned the affected
exact-build fixtures again.

Codex's third review found that the hostile fixture's dead port could turn
connection refusal into false network-denial evidence. The fixture now requires
an explicit unprivileged `127.0.0.1` sentinel URL and rejects missing, privileged,
TLS, or non-loopback targets. Work 06 must preflight a live sentinel and assert
its accept counter remains zero independently of browser-side error results.
The exact signed fixtures were regenerated and repinned again.

Codex's fourth review found that `profile-card` had duplicated NMP's
replaceable-event selection and could revive an older event after malformed
canonical content. The query now requests one canonical row from the
NMP-backed provider. The napplet rejects zero or multiple rows and malformed
canonical content; it never sorts or falls back to a replaced event.
Exact pinned source confirms the boundary: `nmp-store/src/address_key.rs`
selects the replaceable winner by timestamp then event ID, and
`nmp-engine/src/core/query.rs` states store queries return only that current
winner. `nampplets` `nmp-adapter/src/nap.rs` projects those NMP observation rows
through `outbox.query`; its filter limit bounds output without moving winner
selection into the napplet.

Codex's fifth review found two boundary errors. The hostile artifact was reading
the sentinel from `location.search`, but the trusted shell executes it as
`about:srcdoc`; it now reads the value through the existing source-bound
NAP-CONFIG provider. Work 06 must commit the live URL to the exact running
artifact session before mounting its frame. Profile projection now rejects
timestamps outside JavaScript's Date range and computes the display timestamp
before returning any content for rendering.

One noncanonical retry wrapped the Nix command in `bash -lc`; the user login
profile selected host GCC/glibc instead of the pinned compiler and link failed,
matching the environment failure already preserved by Slice 02. The documented direct command
`nix ... develop --command pnpm smoke:fedora` passed without a product change.

## Hostile fixture scope

Work 03 proves that `hostile-egress` is separate, single-file, unit-covered,
signed, byte-pinned, and accepted by the exact-build verifier. It does not
claim the full hostile browser matrix passed. Work 06 must execute it under the
strict CSP against an explicitly configured live, preflighted, unprivileged
loopback sentinel, then separately attest that the sentinel accepted zero probe
connections. Missing/dead sentinel configuration fails fixture startup instead
of turning connection refusal into false denial evidence.

## Upstream result

No defect or missing API required an upstream change. Slice 03 created no fork
branch. The contribution ledger records this explicitly; any later
upstream-bound change must use a dedicated branch in the relevant `jodobear`
fork before Uzel depends on it.

## Next step

Proceed only to [`../work/04-daemon-nmp.md`](../work/04-daemon-nmp.md). First
prove bounded, ordered, exact verified-asset transfer. Do not enlarge the
4096-byte daemon control frame or start integrated composed-demo work early.
