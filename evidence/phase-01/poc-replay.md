# Phase 01 POC replay

**Execution:** 2026-08-13T16:57:52Z  
**Execution HEAD:** `44a2399edcb5cf91fc57ca3b46325f0f01c0c488` (`phase/01-poc-replay-napp-seam`)  
**Accepted comparison commit:** `19519c378c2e775c6ad4b042cfd9aadd89f766b9` — 2026-07-31, `Render rich follow rows and complete canonical profiles (#30)`

This is one source-bound replay attempt, not a byte-identical-HEAD claim and not a reuse of historical pass counts. No source, pin, fixture, runner, lock, or smoke-harness byte changed during it.

## Exact source and pins

Before any baseline command, this preflight ran:

```sh
git diff --name-status 19519c378c2e775c6ad4b042cfd9aadd89f766b9...HEAD -- apps crates contracts fixtures napplets Cargo.toml Cargo.lock pnpm-lock.yaml deno.lock flake.nix flake.lock rust-toolchain.toml scripts/linux-run-smoke.sh scripts/check-pinned-assets.sh
```

Observed result: `M\tfixtures/README.md` only. The documentation delta names the locked entrypoint and `pnpm fixtures:build`; it does not make current HEAD byte-identical to the accepted commit.

The preflight also observed:

| Binding | Command / source | Observed result | Falsifier |
| --- | --- | --- | --- |
| Locked fixture route | `rg -n -F "nix --extra-experimental-features 'nix-command flakes' develop --command" fixtures/README.md`; `rg -n -F 'pnpm fixtures:build' fixtures/README.md` | lines 16–17 name the locked Nix command then `pnpm fixtures:build` | Missing command or fixture script reference |
| Fixture script | `node -e 'const scripts = require("./package.json").scripts; ...'` | `fixtures:build` is exactly `bash scripts/build-signed-napplet-fixtures.sh` | Any other script value |
| Payload / pin parity | `git diff --quiet 19519c...HEAD -- fixtures/{good-morning,follow-list,profile-card,hostile-egress}/{event.json,index.html} crates/napd/src/fixtures.rs scripts/check-pinned-assets.sh scripts/linux-run-smoke.sh` | exit `0` | Any payload, binding, checker, or smoke-harness diff |
| Locked inputs | `sha256sum Cargo.lock pnpm-lock.yaml deno.lock flake.lock uzel-poc-validated-pack/compatibility.lock` | Cargo `9dfbec75b807276b78eb9ee9b8384be47c9a736ab63f7fc0ec8d73d080251603`; pnpm `e2f17fd846e92cca00b7e77a526ac1b99ba0f243e3bb7fedb07b4a5b87695317`; Deno `23209bc013d259aafd7dd06eb8111a646e84cc13defa701b7cb9c3fd3e5d1287`; flake `07a836d15a009b9960eab20f0269fb36efcd300d51a68b6093a123a80673ec25`; compatibility `e1ed29ab44f404663680e23eb45482a7094e96ddd207e9c6a8ee6a8da19d816d` | Digest mismatch |
| Runtime pins | `Cargo.toml`, `Cargo.lock`, and `uzel-poc-validated-pack/compatibility.lock` | current nampplets `e2f69f325a6b45213accdacfcc125e80e0687b4c`; historical pre-catalog runtime `e539378ef735ce06651fd94b71e06f9ce757cb13`; NMP `005dc2a5f12aa414961b313d05ebb021934e385c`; Tauri crate `2.11.5`; lock records Fedora WebKitGTK `2.52.5` | Pin/source change or incompatible successor |

Fixture inventory from `sha256sum` was: good-morning event `66d2a7ed73973e422c86119c3b5c5f1914cb15bad1bfbddecb61cc2edf1c9c17` / index `ffd35eea5c84d03cdda74c23e1bbb2c40500f503833503aa688036faa52f3808`; follow-list event `b145fd991e1a7c9600962351103bdb1464a2fe85af2cb644cade6db2377fcf25` / index `cb331fee5ca80e58b8cecfbaec8fc6c74960bad0f6fdb5133fd0d17203bfd204`; profile-card event `bf93d3adec14237e799bb507464c4c3175e8525db460c7f87f6be54331295980` / index `5b570417414fc9cfc81cf6124893b2a4833175693da27103c240754106d63954`; hostile-egress event `c6183534dc7d46b33c722f9d1771c62ed2a41fc92cfaae07030c6b04608b8bb3` / index `749d4742bde8d42a85f0719f12248203a86ebb9f7f0ace408951f08eb8e15285`. `crates/napd/src/fixtures.rs` binds these four exact sources, authors, d-tags, aggregate hashes, and `nmp-artifact://` bases.

Tool observations before entering Nix: Nix `2.34.1`, Git `2.51.2`, Node `v22.22.0`. Host `pnpm` emitted no usable version; baseline commands intentionally enter the flake instead.

## REF-01 through REF-04 replay

The following locked commands were each run once, starting at 2026-08-13T16:57:52Z:

```sh
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm build
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm check
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm test
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm test:conformance
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm test:ui
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm smoke:linux
```

Each command failed before `pnpm` started, in 0.01–0.02 seconds, with exact non-secret error:

```text
error: opening lock file "/nix/var/nix/db/big-lock": Read-only file system
```

| Requirement | Bound source / intended evidence | Current result | Evidence path | Owner and revisit trigger | Falsifier |
| --- | --- | --- | --- | --- | --- |
| REF-01 exact build, confirmation, launch, render, composition | `pnpm build`, `pnpm check`, `pnpm test`; exact fixtures in `crates/napd/src/fixtures.rs` | **unavailable** — locked Nix could not acquire its database lock, so no build/test command reached pnpm | terminal command output for this replay; sources above | environment owner; rerun one clean locked baseline only after writable Nix DB is provided | successful Nix entry plus command result differs |
| REF-02 trust / denial boundary | `apps/uzel/tests/ui/acceptance.test.mjs`; `scripts/linux-run-smoke.sh` markers `UZEL_HOSTILE_PROBE_OK`, native denial and source binding | **unavailable** — deterministic UI and live smoke did not start; no historic probe count is carried forward as current evidence | existing source paths only; no new runtime artifact claimed | Uzel runtime owner; repeat with a writable locked Nix environment | new successful probe shows marker absence/order failure or a denied capability succeeds |
| REF-03 selected read identity, profile/follow render, lifecycle recovery | UI acceptance cases for profile/follow, retry, restart reconciliation; `crates/napd/src/runner.rs` lifecycle | **unavailable** — `pnpm test:ui` did not enter pnpm; no runtime/Nostr state was added | `apps/uzel/tests/ui/acceptance.test.mjs` (existing test definition) | Uzel runtime owner; rerun after Nix materialization works | successful replay finds incorrect identity, duplicate state, or recovery failure |
| REF-04 Chromium plus real Weston/WebKit hostile/recovery/fixture proof | `pnpm test:ui` is deterministic Chromium class; `pnpm smoke:linux` is real Weston/WebKit class with redacted failed-log handling | **unavailable** — both commands stopped at Nix lock acquisition. Chromium and Weston/WebKit remain distinct, neither is substituted for the other | `scripts/linux-run-smoke.sh`; no smoke result directory was produced by this attempt | environment + Uzel runtime owners; rerun only once when Nix DB access is restored | either successful class fails its independent marker/check |

No raw smoke logs, credentials, invoke material, or failure artifacts were copied into this report. The existing smoke script remains evidence producer and redacts its protected invoke material before retained failure logs.

## REF-06 baseline

Build/Nix materialization is separate from runtime measurements. This execution observed only materialization failure; every runtime value below is deliberately unavailable rather than inferred from `STATUS.md`.

| Dimension | Observed value | Method / source | Owner | Revisit trigger |
| --- | --- | --- | --- | --- |
| Nix materialization / build | unavailable: all six entrypoints failed before `pnpm`, exit `1`, elapsed 0.01–0.02 s | six commands above | environment owner | writable `/nix/var/nix/db` lock path |
| Startup-to-ready | unavailable | `pnpm smoke:linux` never started | Uzel runtime owner | successful materialization |
| Local profile/follow render | unavailable | `pnpm test:ui` never started | Uzel product owner | successful materialization |
| Chromium hostile egress / native bridge | unavailable | deterministic UI command never started | Uzel trust-boundary owner | successful materialization |
| Weston/WebKit process and WebView pressure | unavailable | real smoke command never started | Uzel runtime owner | successful materialization |
| Resource flow, queue bounds, cancellation | unavailable | UI/smoke commands never started | Uzel runtime owner | successful materialization |
| Lifecycle recovery / stop / restart | unavailable | test/UI/smoke commands never started | Uzel runtime owner | successful materialization |

## Unavailable or failed

- **Locked Nix database access failed.** The six required commands were executed once and are not repeated while this evidence is edited. Cause: the execution sandbox exposed `/nix/var/nix/db/big-lock` read-only. Owner: environment provider. Revisit trigger: a new execution environment with writable Nix database locking.
- **No current runtime success claim exists.** Historical POC status and compatibility entries remain sources for pin/probe intent only; they are not current result counts.
- **No accepted-Napp adaptation is authorized.** This replay failure cannot establish or bypass the external candidate requirement.
