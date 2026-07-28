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
| fixture author | `ba01ffeffa841a7e48beb131a08d4e81d07232eacc7c178558b84e5ad9a8ca86` |
| follow-list | event `d1d7e93c...`; aggregate `eaf4e565...`; index `3ae0e253...`; 58881 bytes |
| profile-card | event `2cfd84bc...`; aggregate `de31a655...`; index `a5c9880d...`; 59597 bytes |
| hostile-egress | event `f23d4570...`; aggregate `0514a281...`; index `2687d474...`; 57701 bytes |
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

`follow-list` owns selection and payload emission. `profile-card` owns strict
payload validation and latest-known kind-0 projection. The runtime owns sender
identity, session/surface mapping, capability routing, Nostr evidence, and exact
build verification. Neither napplet imports Uzel, Tauri, `napd`, NMP,
nampplets, or the other napplet.

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
- napplet unit tests: follow-list 1, profile-card 2, hostile-egress 1 pass;
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

One noncanonical retry wrapped the Nix command in `bash -lc`; the user login
profile selected host GCC/glibc instead of the pinned compiler and link failed,
matching the environment failure already preserved by Slice 02. The documented direct command
`nix ... develop --command pnpm smoke:fedora` passed without a product change.

## Hostile fixture scope

Work 03 proves that `hostile-egress` is separate, single-file, unit-covered,
signed, byte-pinned, and accepted by the exact-build verifier. It does not
claim the full hostile browser matrix passed. Work 06 must execute it under the
strict CSP and attest network/native denials.

## Upstream result

No defect or missing API required an upstream change. Slice 03 created no fork
branch. The contribution ledger records this explicitly; any later
upstream-bound change must use a dedicated branch in the relevant `jodobear`
fork before Uzel depends on it.

## Next step

Proceed only to [`../work/04-daemon-nmp.md`](../work/04-daemon-nmp.md). First
prove bounded, ordered, exact verified-asset transfer. Do not enlarge the
4096-byte daemon control frame or start integrated composed-demo work early.
