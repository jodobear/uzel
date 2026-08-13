# Phase 01 POC replay

**Execution:** 2026-08-13T18:05:15Z
**Execution HEAD:** `d4c83a93d976e099774a18762b096b8eba5bbd7c` (`phase/01-poc-replay-napp-seam`)
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

The documented prerequisite first passed through the same locked entrypoint:

```sh
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm install --frozen-lockfile
```

It installed the lockfile's 69 packages with pnpm `10.8.0` and changed no tracked file. The following locked baseline commands were then each run once through the authorized host path, starting at 2026-08-13T18:05:15Z:

```sh
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm build
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm check
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm test
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm test:conformance
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm test:ui
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm smoke:linux
```

All six commands passed. Exact outcomes were:

| Command | Exit | Elapsed | Current result |
| --- | ---: | ---: | --- |
| `pnpm build` | 0 | 67 s | Three napplets, shell, and Rust workspace built |
| `pnpm check` | 0 | 36 s | Napplet builds, shell checks/build, and Rust workspace check passed |
| `pnpm test` | 0 | 25 s | 11 contract, 21 napplet, 5 shell, and 55 Rust tests passed; 2 external-network Rust tests ignored; `PINNED_ASSETS_OK` |
| `pnpm test:conformance` | 0 | 5 s | Follow-list and profile-card each CONFORMANT: 6 passed, 0 failed, 4 unmeasured checks skipped |
| `pnpm test:ui` | 0 | 32 s | 34 deterministic Chromium/UI acceptance tests passed |
| `pnpm smoke:linux` | 0 | 23 s | Weston/WebKit runtime reached readiness and emitted the exact non-secret success line below |

```text
LINUX_RUN_SMOKE_OK daemon=ready shell=ready exact_builds=3 nap_shell=3 shell_accepted=2 artifact=responded source_bound=multi hostile=denied sentinel=zero native=zero user_mode=hidden compositor=weston-headless-gl
```

| Requirement | Bound source / intended evidence | Current result | Evidence path | Owner and revisit trigger | Falsifier |
| --- | --- | --- | --- | --- | --- |
| REF-01 exact build, confirmation, launch, render, composition | build/check/test plus exact fixtures, UI acceptance, and live smoke | **passed** — exact builds, confirmation/acceptance, profile/follow rendering, and multi-surface composition completed | command/result table and durable smoke line above; bound source paths | Uzel product owner; rerun after source/pin change | any locked lane or exact-source binding fails |
| REF-02 trust / denial boundary | UI acceptance, hostile-egress tests, runtime boundary tests, and live smoke | **passed at `66c4d8e`** — current Uzel source binding and denial paths pass deterministic, Rust, and native probes | affected validation and native marker below | Uzel trust-boundary owner | any request selects a surface/session/principal, receives raw network/native authority, or bypasses trusted host binding |
| REF-03 selected read identity, profile/follow render, lifecycle recovery | deterministic runner restart plus UI reconciliation/ambiguity scenarios | **passed at `66c4d8e`** — restart preserves identity, exact-build metadata, identical follows, reconciles the unobserved pre-restart surface as inactive, and advances generation | `restart_recovers_identity_installed_build_follows_and_orphan_outcome`; selected Chromium scenarios | Uzel runtime owner | restart loses selected state/build/follows, retains an orphan session, or reuses generation |
| REF-04 Chromium plus real Weston/WebKit hostile/recovery/fixture proof | selected deterministic `test:ui` plus real `smoke:linux` | **passed at `66c4d8e`** — native WebKit stops both trusted surfaces after hostile proof, relaunches generations 4/5, re-handshakes, and preserves hostile/native denial | affected validation and `UZEL_WEBKIT_RECOVERY_OK` below | Uzel runtime/trust owners | real WebKit replacement, handshake, source binding, or hostile/native denial fails |

No raw smoke logs, credentials, or invoke material were copied into this report. The exact non-secret terminal line above is the durable result; the existing smoke script remains the protected evidence producer.

## REF-06 baseline

Build/dependency materialization is separate from runtime measurements. Values below come only from this exact execution; no historical POC count is inferred.

| Dimension | Observed value | Method / source | Owner | Revisit trigger |
| --- | --- | --- | --- | --- |
| Nix materialization / build | install 5 s; build 67 s; check 36 s; all exit `0` | locked install/build/check | Uzel build owner | source, lock, or flake change |
| Test lane | exit `0`, 25 s; 11 contract, 21 napplet, 5 shell, and 55 Rust tests passed; 2 external-network tests ignored | locked `pnpm test` | Uzel product owner | source or pin change |
| Startup-to-ready | unavailable: producer emits phase markers without timestamps and may compile before launch; 23 s is only total smoke-lane wall time | locked `pnpm smoke:linux` | Uzel runtime owner | direct ready-marker timing is instrumented |
| Local profile/follow render | 34 deterministic UI cases passed; live smoke accepted 2 shell surfaces | locked UI plus durable smoke line | Uzel product owner | renderer or fixture change |
| Chromium hostile egress / native bridge | UI suite passed; live hostile denied, sentinel/native zero | locked UI plus durable smoke line | Uzel trust-boundary owner | boundary change |
| Weston/WebKit process and WebView pressure | unavailable: `nap_shell=3` counts iframe surfaces, not WebViews or OS processes; no process count was captured | smoke source plus durable smoke line | Uzel runtime owner | direct WebKit/WebView process counting is instrumented |
| Resource flow, queue bounds, cancellation | contract, napplet, UI, and Rust lanes passed their bounded resource/cancellation cases | locked test/UI lanes | Uzel product/runtime owners | resource-path change |
| Lifecycle recovery / stop / restart | deterministic UI and Rust lifecycle/reconciliation tests passed; live smoke completed | locked test/UI/smoke lanes | Uzel runtime owner | lifecycle change |

## Unavailable or failed

- **Per-marker startup latency remains unavailable.** The existing smoke producer reports phase and success markers but not individual marker timestamps; the reproducible full-smoke elapsed value is 23 seconds. Owner: later performance instrumentation, only when product scope calls for it.
- **Process/WebView pressure remains unavailable.** The smoke's nap-shell and accepted-surface counters describe iframe surfaces inside one configured Tauri window, not WebKit/WebView OS-process pressure. Owner: runtime instrumentation when product scope calls for it.
- **Current architecture:** Uzel owns product runtime/composition and its private daemon; nampplets and NMP remain exact-pinned source-proven upstreams.

## 2026-08-14 bounded continuation

Continuation ran from `b4eeeb45615c4dcf223c5349a0465cee4f7d3ae2` without changing product,
fixture, lock, runner, or smoke-harness bytes. Focused direct tests established the missing
REF-03 restart-state evidence gathered so far:

- `runner::tests::nmp_parses_and_persists_the_active_read_identity`: 1 passed;
- `runner::tests::restarted_daemon_gets_a_new_surface_generation`: 1 passed;
- `server::tests::daemon_serves_ordered_verified_asset_and_shuts_down`: 1 passed;
- `runner::tests::public_identity_profile_follows_and_picture_cross_only_native_providers`
  with `--ignored --exact`: 1 passed. It first resolved useful public profile/follow state and
  native image bytes, then reopened the same state root with no relays and recovered the selected
  identity plus cached profile;
- `napd-protocol` authoritative reconciliation: 1 passed;
- Nix-native Tauri reconciliation: 1 passed, stopping each stale snapshot surface exactly once;
- focused Chromium `restart-reconciliation`, `review-ambiguous`, and
  `confirmation-ambiguous`: 6 tests passed across three scenarios.

Implementation commit `66c4d8e` closed the remaining observable gaps on current Uzel source:

- `pnpm check` passed Svelte checks/builds and `cargo check --workspace`.
- `pnpm test` passed 11 contract tests, 21 napplet tests, 5 shell tests, 31 napd tests
  (2 public-network tests intentionally ignored), 12 protocol tests, 1 daemon-app test, 12 Tauri
  tests, and pinned-asset verification.
- `restart_recovers_identity_installed_build_follows_and_orphan_outcome` passed. The same temporary
  state root retained the selected identity, one exact installed build and identical follow keys;
  the unobserved pre-restart surface reconciled inactive; relaunch used a fresh generation.
- Selected Chromium `ready`, `review-ambiguous`, `confirmation-ambiguous`, and
  `restart-reconciliation` scenarios passed: 4 scenarios / 9 TAP tests.
- The real Weston/WebKit smoke passed after hostile proof, stopped both trusted surfaces, relaunched
  generations 4/5, completed fresh shell handshakes, and emitted:

```text
UZEL_WEBKIT_RECOVERY_OK before=uzel-follow-list-generation-2,uzel-profile-card-generation-1 after=uzel-follow-list-generation-5,uzel-profile-card-generation-4 source_bound=true
LINUX_RUN_SMOKE_OK daemon=ready shell=ready exact_builds=5 nap_shell=5 shell_accepted=4 artifact=responded source_bound=multi hostile=denied sentinel=zero native=zero recovery=passed user_mode=hidden compositor=weston-headless-gl
```

An initial selected-Chromium run exposed that the mock native boundary lacked the new
disabled-by-default probe commands. Production checks and native smoke were already green; the mock
was updated to return `false`/`null`, and the selected Chromium rerun passed. No unavailable native
access or remaining REF-02/03/04 failure was hidden.
