# Phase 01 POC replay

**Execution:** 2026-08-13T17:49:24Z
**Execution HEAD:** `b307a297bc7d858066de055837cb5aeeb018c7c4` (`phase/01-poc-replay-napp-seam`)
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

Tool observations: Nix `2.34.1`, Git `2.51.2`, host Node `v22.22.0`, and locked-shell Node `v22.23.1`. Baseline commands used the flake-provided Corepack/pnpm route.

## REF-01 through REF-04 replay

The following locked commands were each run once through an authorized host path, starting at 2026-08-13T17:49:24Z:

```sh
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm build
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm check
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm test
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm test:conformance
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm test:ui
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm smoke:linux
```

All six commands entered Nix and reached pnpm. Exact outcomes were:

| Command | Exit | Elapsed | Current result |
| --- | ---: | ---: | --- |
| `pnpm build` | 1 | 2 s | Failed at napplet builds: `vite: command not found`; clean worktree had no `node_modules` |
| `pnpm check` | 1 | 2 s | Failed at same napplet-build prerequisite before shell/Rust checks |
| `pnpm test` | 0 | 77 s | Passed contract, napplet, shell, Rust, and pinned-asset checks; 55 Rust tests passed, 2 ignored network tests; `PINNED_ASSETS_OK` |
| `pnpm test:conformance` | 1 | <1 s | Failed before conformance execution: `napplet-conformance: command not found` |
| `pnpm test:ui` | 1 | 1 s | Failed before UI cases: package `playwright` missing |
| `pnpm smoke:linux` | 1 | 12 s | Runtime exited before readiness; all required smoke markers missing; failed logs retained by existing redacting smoke producer |

| Requirement | Bound source / intended evidence | Current result | Evidence path | Owner and revisit trigger | Falsifier |
| --- | --- | --- | --- | --- | --- |
| REF-01 exact build, confirmation, launch, render, composition | `pnpm build`, `pnpm check`, `pnpm test`; exact fixtures in `crates/napd/src/fixtures.rs` | **failed / partial** — test lane passed, but build and check could not find clean-worktree JS dependencies | current command results above; sources above | Uzel build owner; establish the existing locked dependency-materialization prerequisite before next candidate baseline | clean locked build/check pass and exact fixture flow succeeds |
| REF-02 trust / denial boundary | `apps/uzel/tests/ui/acceptance.test.mjs`; `scripts/linux-run-smoke.sh` markers `UZEL_HOSTILE_PROBE_OK`, native denial and source binding | **failed / partial** — source-level contract, hostile-egress, and Rust boundary tests passed; UI lacked Playwright and live smoke exited before every marker | current test output; redacted `uzel-poc-validated-pack/reports/probes/linux-failed` from existing producer | Uzel build/runtime owner; restore locked JS dependencies, then investigate smoke startup without reusing this baseline | successful independent UI and live hostile-boundary classes |
| REF-03 selected read identity, profile/follow render, lifecycle recovery | UI acceptance cases for profile/follow, retry, restart reconciliation; `crates/napd/src/runner.rs` lifecycle | **failed / partial** — unit and Rust lifecycle tests passed, but no UI/runtime render claim exists because Playwright was missing and live readiness failed | current test output and existing test definition | Uzel build/runtime owner; successful locked UI plus live readiness | successful replay finds incorrect identity, duplicate state, or recovery failure |
| REF-04 Chromium plus real Weston/WebKit hostile/recovery/fixture proof | `pnpm test:ui` is deterministic Chromium class; `pnpm smoke:linux` is real Weston/WebKit class with redacted failed-log handling | **failed** — Chromium cases did not load without Playwright; Weston/WebKit process exited before readiness and all markers were missing | current command output; existing redacted failed-log directory | Uzel build/runtime owner; next candidate must materialize JS dependencies and pass both independent classes | both classes pass their independent marker/check sets |

No raw smoke logs, credentials, invoke material, or failure artifacts were copied into this report. The existing smoke script remains evidence producer and redacts its protected invoke material before retained failure logs.

## REF-06 baseline

Build/dependency materialization is separate from runtime measurements. Values below come only from this exact execution; no historical POC count is inferred.

| Dimension | Observed value | Method / source | Owner | Revisit trigger |
| --- | --- | --- | --- | --- |
| Nix materialization / build | Nix and pnpm entered; build/check exit `1` after 2 s each because clean-worktree JS dependencies were absent | locked `pnpm build`, `pnpm check` | Uzel build owner | locked dependency-materialization prerequisite exists and succeeds |
| Test lane | exit `0`, 77 s; contract/napplet/shell/Rust/pin checks passed | locked `pnpm test` | Uzel product owner | source or pin change |
| Startup-to-ready | failed after 12 s; runtime exited and all readiness markers were missing | locked `pnpm smoke:linux` | Uzel runtime owner | next frozen candidate after dependency prerequisite is fixed |
| Local profile/follow render | unavailable; UI cases did not load because Playwright was absent | locked `pnpm test:ui` | Uzel product owner | locked JS dependency materialization succeeds |
| Chromium hostile egress / native bridge | unavailable; same pre-test Playwright failure | locked `pnpm test:ui` | Uzel trust-boundary owner | locked UI suite executes |
| Weston/WebKit process and WebView pressure | failed before readiness; output included `/dev/dri/card0` permission warning and cursor warnings, but no sole root cause is claimed | locked `pnpm smoke:linux` | Uzel runtime/environment owners | focused startup diagnosis before next candidate |
| Resource flow, queue bounds, cancellation | unit-level checks passed; UI/live measurement unavailable | locked `pnpm test` plus failed UI/smoke lanes | Uzel product/runtime owners | UI and live smoke execute successfully |
| Lifecycle recovery / stop / restart | unit-level checks passed; no successful live measurement | locked `pnpm test` plus failed smoke | Uzel runtime owner | live smoke reaches and verifies recovery markers |

## Unavailable or failed

- **Clean-worktree JS dependency materialization failed acceptance.** Nix provided its toolchain and pnpm entered, but the locked commands do not themselves install workspace dependencies. Build/check/conformance/UI therefore failed on missing `vite`, `napplet-conformance`, or `playwright`. No install or rerun was added to this evidence-only slice. Owner: Uzel build workflow. Revisit trigger: a separately reviewed existing-entrypoint correction or documented locked prerequisite.
- **Live Linux smoke failed before readiness.** Existing redacting producer retained failed logs; all required markers were missing. GPU/cursor warnings were observed, but this report does not infer a root cause. Owner: Uzel runtime/environment. Revisit trigger: focused startup diagnosis before another frozen candidate baseline.
- **Current success is limited to `pnpm test`.** Historical POC status and compatibility entries remain sources for pin/probe intent only; they are not current result counts.
- **No accepted-Napp adaptation is authorized.** This replay failure cannot establish or bypass the external candidate requirement.
