# Gate 0 preflight and Slice 01 decision

## Decision

**NO-GO for Slice 01.** Gate 0 is complete, but the only candidate end-to-end compatibility line is internally split: Kehto PR #204 is on Napplet 0.29 and its generated artifacts fail the released conformance CLI, while current nampplets remains on an unratified Napplet 0.28 baseline. Building the workspace now would encode an unaccepted contract or require a forbidden compatibility workaround.

No product implementation was started.

## Gate results

| Gate | Result | Evidence | Consequence |
|---|---|---|---|
| V-01 | **fail** | #204 merged at `b85db51...`; full unit suite passes; exact `chat` and `feed` builds fail conformance on forbidden `fetch` | upstream fixture/build or conformance must be reconciled |
| V-02 | pass | 16-crate nampplets workspace passes Linux fmt/test/Clippy on Rust 1.89.0 | reuse generic crates; implement only Linux host edge |
| V-03 | pass | public `RuntimeController` verify/install/launch/mapped-envelope/lifecycle APIs compile and are exercised by upstream tests | daemon is a thin controller client, not a new runtime |
| V-04 | pass | locked NMP adapter signed-fixture probe returns profile, direct follows, evidence, cancellation, shutdown | use `NmpDataPlane` and existing providers only |
| V-05 | pass | real Tauri/Wry/WebKitGTK hostile frame lacks bridge authority; invalid raw IPC rejected; source binding true | one trusted WebView with sandboxed frames remains viable |
| V-06 | **fail** | synthetic strict-CSP child runs with zero network connections, but intended Kehto artifacts contain `fetch` | never relax CSP; repair the artifact line first |
| V-07 | pass | real `$XDG_RUNTIME_DIR` AF_UNIX version-0 hello/status; 0700/0600 modes; oversized frame rejected | use the bounded private protocol shape |
| V-08 | pass | exact Nix/Rust/Node/Tauri/Fallow/Mermaid commands executed | create locks/devShell only after no-go clears |

## Confirmed assumptions

- Kehto PR #204 is merged, and the npm 0.29 package line is released with retrievable integrity values.
- nampplets runtime semantics are Linux-neutral; only the WebKit/native-host binding is platform-specific.
- Exact-build identity, grants, sessions, source-bound envelopes, providers, storage, and NMP integration already have reusable owners.
- Pinned NMP covers the POC's profile/follow/freshness/evidence/cancellation/diagnostics needs without a second cache.
- Tauri/Wry initializes authenticated IPC in the top frame on Linux. A sandboxed opaque-origin child cannot read it.
- A strict self-contained child CSP can deny tested loopback egress while allowing script execution and source-bound `postMessage`.
- A small same-user AF_UNIX protocol is sufficient for the shell/daemon boundary.
- The documented tools exist; pinned `nix develop`, a Corepack shim, and corrected Fallow schema are the working paths.

## Rejected assumptions

- **Rejected:** “Kehto #204 is still draft.” It merged on 2026-07-27.
- **Rejected:** “Current nampplets is compatible with the Kehto 0.29 line.” Its lock/constants remain 0.28 and its baseline is unratified.
- **Rejected:** “A built single-file Kehto fixture is necessarily network-free.” Vite inserts a module-preload `fetch` helper and released conformance detects it.
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
5. Use four-byte big-endian, maximum-4,096-byte, version-0 JSON frames over a 0600 AF_UNIX socket in a 0700 runtime directory for the POC.
6. Pin a real flake/devShell; use `nix develop`, lock Tauri CLI and crate separately, expose pnpm via Corepack on `PATH`, and use Fallow's released schema.

## Blockers

1. **Compatibility blocker:** nampplets must publish or accept a tested baseline for core/nap 0.29.0, shim 0.27.0, SDK 0.25.0, and conformance 0.14.0 (or all participants must converge on another single line).
2. **Artifact blocker:** exact Kehto production fixtures must pass `@napplet/conformance-cli@0.2.16` without forbidden `fetch`. Likely resolution is to stop Vite from emitting the module-preload helper, but the fix belongs upstream and must be proven, not assumed.
3. **Acceptance blocker:** the updated nampplets baseline needs its currently blank compatibility/security/NMP signoffs or an explicit Uzel risk acceptance. Uzel must not silently treat “unratified” as accepted.

Draft NAP/NIP status is a tracked risk, not an independent Slice 01 blocker if exact revisions are deliberately accepted after the two compatibility blockers clear.

## Exact next steps

1. In Kehto/napplet upstream, produce `chat` and `feed` single-file builds with no module-preload `fetch`; run the exact released conformance CLI on both and retain output plus bundle hashes.
2. Update nampplets against that exact 0.29 package/spec/Kehto line; update provider constants and its `compatibility.lock`; run its full Rust, corpus, provider, and trusted-shell suites on Linux.
3. Ratify or explicitly accept that new nampplets baseline, including named compatibility, security, and NMP boundary review.
4. Update this repository's `compatibility.lock` and affected facts, then rerun V-01, V-02, V-03, and V-06. Any Tauri/Wry/WebKit change also reruns V-05.
5. Only after every blocking gate passes, change the verdict to go and execute `work/01-scaffold.md`. Do not begin Slice 02–06 early.

## Validated command ledger

Representative exact commands that ran:

```sh
# Kehto exact merge
corepack enable --install-directory "$DEV_SHELL_BIN"
pnpm install --frozen-lockfile
pnpm build
pnpm test:unit
npm exec --yes --package=@napplet/conformance-cli@0.2.16 -- \
  napplet-conformance apps/playground/napplets/chat/dist

# nampplets exact toolchain and lock
cargo +1.89.0 fmt --all -- --check
cargo +1.89.0 test --locked --workspace
cargo +1.89.0 clippy --locked --workspace --all-targets -- -D warnings

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
