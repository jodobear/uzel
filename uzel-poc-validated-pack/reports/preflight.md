# Gate 0 preflight and Slice 01 decision

## Decision

**GO for Slice 01 within the Linux-only Uzel POC.** Gate 0 is complete and every V-01 through V-08 probe passes. Kehto candidate [`jodobear/kehto-web@62241de0b4526ba4fdc8a7b3c766c2499d3ae24d`](https://github.com/jodobear/kehto-web/commit/62241de0b4526ba4fdc8a7b3c766c2499d3ae24d) is a durable, conformance-clean source pin, and reachable nampplets candidate `08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf` passes the full Linux evidence suite. Uzel explicitly accepts that exact unratified candidate for the POC. This does not claim upstream ratification or Apple support.

No product implementation was started.

## Gate results

| Gate | Result | Evidence | Consequence |
|---|---|---|---|
| V-01 | pass | candidate `62241de...` yields two released-conformance passes and is reachable from the dedicated `jodobear/kehto-web` fork branch | pin the exact fork SHA; review and merge upstream in parallel |
| V-02 | pass | candidate `08ddb87...` passes 16-crate Linux fmt/test/Clippy, 22 Python tests, 4 trusted-shell tests, digests, and file-growth checks on Rust 1.89.0 | reuse generic crates; implement only Linux host edge after acceptance |
| V-03 | pass | candidate full-workspace tests keep public `RuntimeController` verify/install/launch/mapped-envelope/lifecycle APIs green | daemon is a thin controller client, not a new runtime |
| V-04 | pass | locked NMP adapter signed-fixture probe returns profile, direct follows, evidence, cancellation, shutdown | use `NmpDataPlane` and existing providers only |
| V-05 | pass | real Tauri/Wry/WebKitGTK hostile frame lacks bridge authority; invalid raw IPC rejected; source binding true | one trusted WebView with sandboxed frames remains viable |
| V-06 | pass | strict-CSP child runs with zero network connections; corrected exact `chat` and `feed` artifacts contain no module-preload `fetch` and pass released conformance | preserve this bundle/CSP contract |
| V-07 | pass | real `$XDG_RUNTIME_DIR` AF_UNIX version-0 hello/status; 0700/0600 modes; oversized frame rejected | use the bounded private protocol shape |
| V-08 | pass | exact Nix/Rust/Node/Tauri/Fallow/Mermaid commands executed | create the pinned Slice 01 locks and devShell |

## Confirmed assumptions

- Kehto PR #204 is merged, and the npm 0.29 package line is released with retrievable integrity values.
- Vite's fetch-based module-preload polyfill is not required for these self-contained artifacts; disabling it in Kehto's shared build config removes the forbidden global without a compatibility workaround.
- The corrected `chat` and `feed` artifacts each pass `@napplet/conformance-cli@0.2.16` with 6 passed, 0 failed, and 4 skipped.
- Exact Kehto commit `62241de...` is publicly fetchable from `jodobear/kehto-web`; Uzel can provisionally pin it without waiting for an upstream merge.
- A coherent nampplets 0.29 candidate can be generated and exercised on Linux without duplicating NMP, runtime, provider, storage, or cryptographic functionality.
- nampplets runtime semantics are Linux-neutral; only the WebKit/native-host binding is platform-specific.
- Exact-build identity, grants, sessions, source-bound envelopes, providers, storage, and NMP integration already have reusable owners.
- Pinned NMP covers profile/follow selection and owns freshness/evidence/cancellation/diagnostics without a second cache. Released NAP identity 0.29.0 does not expose scoped evidence to napplet JavaScript, so the POC labels those reads latest-known.
- Tauri/Wry initializes authenticated IPC in the top frame on Linux. A sandboxed opaque-origin child cannot read it.
- A strict self-contained child CSP can deny tested loopback egress while allowing script execution and source-bound `postMessage`.
- A small same-user AF_UNIX protocol is sufficient for the shell/daemon boundary.
- The documented tools exist; pinned `nix develop`, a Corepack shim, and corrected Fallow schema are the working paths.

## Rejected assumptions

- **Rejected:** “Kehto #204 is still draft.” It merged on 2026-07-27.
- **Rejected:** “Changing version constants alone makes nampplets compatible with 0.29.” The candidate needed regenerated authorities, envelope/corpus evidence, explicit registry/package drift records, and executable suites; it still advertises no platform domains.
- **Rejected:** “The original #204 artifact failure requires relaxing CSP or conformance.” The smallest source fix is `build.modulePreload = false`; corrected artifacts pass under the existing strict policy.
- **Rejected:** “Uzel must wait for the Kehto upstream PR to merge.” The exact verified fork commit is durable and fetchable, so it clears V-01 as a provisional source pin; any review-modified successor must be revalidated before repinning.
- **Rejected:** “A green compatibility candidate is automatically ratified.” The candidate's three named review fields are blank.
- **Rejected:** “NAP shell/INC/relay/identity/storage have one unambiguous released status.” Registry, file headers, and open PRs disagree.
- **Rejected:** “NIP-5A is the napplet manifest pin.” It is the nsite contract; draft NIP-5D is the relevant napplet proposal.
- **Rejected:** “The nampplets `surface` crate is a Linux renderer.” It is platform-neutral surface state/descriptor logic.
- **Rejected:** “`nix shell` is enough for WebKitGTK development headers.” It does not run the `mkShell` setup hooks needed for pkg-config; use a pinned flake and `nix develop`.
- **Rejected:** “`corepack pnpm` alone is enough for Kehto's scripts.” nested Turbo tasks need a `pnpm` shim on `PATH`.
- **Rejected:** Fallow 3.9.1 accepts the starter `ignore` field. The released schema requires `ignorePatterns`.

## Required design changes

1. Make the daemon a thin owner of upstream `RuntimeController`/`NmpDataPlane`; remove any wording that suggests reimplementing runtime, provider, storage, or NMP translation logic in `crates/napd`.
2. Treat NAP and NIP-5D revisions as exact provisional pins. Do not claim NIP-5A or spec ratification.
3. Preserve one trusted Tauri WebView with source-bound sandboxed frames, but document that WebKit's raw message handler is visible and protected by Tauri's top-frame-only invoke key.
4. Adopt the tested child CSP verbatim and require conformance-clean self-contained artifacts. Do not add broad network permission to accommodate Vite output.
5. Use four-byte big-endian, bounded version-0 JSON frames over a 0600 AF_UNIX socket in a 0700 runtime directory. Gate 0 proved a 4,096-byte feasibility bound; Slice 04 later established the current 524288-byte per-frame product bound with explicit 65536-byte inbound-envelope escaping headroom. The live-identity extension retained that ceiling and added same-connection chunking for larger NAP-RESOURCE routed responses.
6. Pin a real flake/devShell; use `nix develop`, lock Tauri CLI and crate separately, expose pnpm via Corepack on `PATH`, and use Fallow's released schema.

## Accepted provisional risks

1. `jodobear/nampplets@08ddb87...` remains unratified upstream. Uzel owns only a project-local, exact-commit Linux POC acceptance and must not describe the wider baseline as ratified.
2. Current NAP-INC text requires `inc.channel.opened`, which released 0.29 package types/conformance do not expose. Uzel does not use symmetric channels; that surface remains explicitly unsupported.
3. Package NAP-INTENT behavior leads the older registry text. Uzel uses NAP-INC for the POC and does not promote NAP-INTENT.
4. Apple workbench/catalog evidence is absent and outside Uzel's Linux scope.

Kehto upstream PR [#218](https://github.com/kehto/web/pull/218) and later nampplets contribution work proceed in parallel. Neither upstream merge is a Slice 01 blocker. Draft NAP/NIP status remains a tracked exact-pin risk.

## Exact next steps

1. Execute `work/01-scaffold.md` from the exact accepted pins. Do not begin Slice 02–06 early.
2. Depend on the required nampplets crates from `jodobear/nampplets` at exact revision `08ddb87...`; compose them in `napd` instead of copying runtime logic.
3. If Kehto or nampplets review changes a source SHA, rerun the affected Gate 0 suites before repinning Uzel.
4. Record every upstream contribution in [`../docs/08-upstream-contributions.md`](../docs/08-upstream-contributions.md), using a dedicated branch in the `jodobear` fork and submitting after Uzel validation.
5. Treat full nampplets ratification as a pre-production/post-POC milestone, not a POC start gate.

## Validated command ledger

Representative exact commands that ran:

```sh
# Kehto exact merge
git ls-remote https://github.com/jodobear/kehto-web.git \
  refs/heads/fix/napplet-conformance-no-modulepreload
corepack enable --install-directory "$DEV_SHELL_BIN"
pnpm install --frozen-lockfile
pnpm build
pnpm type-check
pnpm test:unit
pnpm audit:gateway-artifacts
npm exec --yes --package=@napplet/conformance-cli@0.2.16 -- \
  napplet-conformance apps/playground/napplets/chat/dist
npm exec --yes --package=@napplet/conformance-cli@0.2.16 -- \
  napplet-conformance apps/playground/napplets/feed/dist
npx --yes aislop@0.12.0 scan . --json

# nampplets exact toolchain and lock
cargo +1.89.0 fmt --all -- --check
cargo +1.89.0 test --locked --workspace
cargo +1.89.0 clippy --locked --workspace --all-targets -- -D warnings
python3 -m unittest discover -s conformance/tests -p 'test_*.py'
python3 conformance/scripts/verify_baseline.py
node --test web/trusted-shell/tests/*.test.js
python3 conformance/scripts/generate_digests.py --check
python3 scripts/ci/check_file_growth.py --base HEAD --head INDEX

# pinned released tools
nix --extra-experimental-features 'nix-command flakes' run \
  github:NixOS/nixpkgs/38a4887411571457d700c51c64a6e49ead2ed5ab#cargo-tauri -- --version
nix --extra-experimental-features 'nix-command flakes' run \
  github:NixOS/nixpkgs/38a4887411571457d700c51c64a6e49ead2ed5ab#mermaid-cli -- --version

# WebKit headers require a pinned mkShell, then:
nix --extra-experimental-features 'nix-command flakes' develop -- \
  command pkg-config --modversion webkit2gtk-4.1

# Fallow 3.9.1 config validation
XDG_CACHE_HOME=<tmp>/xdg-cache fallow \
  --root uzel-poc-validated-pack \
  --config uzel-poc-validated-pack/config/fallow.jsonc \
  --format json --quiet config

# Documentation audit and graph refresh
python3 uzel-poc-validated-pack/scripts/audit_docs.py
graphify update .
```
