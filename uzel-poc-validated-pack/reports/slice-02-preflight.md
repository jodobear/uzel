# Slice 02 preflight

## Verdict

**PASS.** One pinned signed build verifies, launches, completes NAP-SHELL, and
responds through the upstream nampplets runtime in a real Linux Tauri/WebKit
process. Hostile surface identity and native-bridge probes fail closed. No NMP
data, second napplet, generic installer, catalog, or public IPC was added.

## Exact dependency and asset record

| Input | Exact value |
|---|---|
| nampplets | `jodobear/nampplets@08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf` |
| transitive NMP | `pablof7z/nmp@005dc2a5f12aa414961b313d05ebb021934e385c` |
| `Cargo.lock` SHA-256 | `03d1d6fd58e013cbe845c2b315c088148cc38281b1d422bfb7966eabc942ffa0` |
| `pnpm-lock.yaml` SHA-256 | `a4870aa91e70d6e79e3fce822f0b9deffb9bdae0383d3eb84bcea3e3fd8dce2c` |
| Tauri JS API / crate | `2.11.1` / `2.11.5` |
| fixture author | `266815e0c9210dfa324c6cba3573b14bee49da4209a9456f9484e5106cd408a5` |
| fixture `d` tag | `good-morning` |
| fixture aggregate | `828a6df02afd56782ea20f805084acce65c53f7c37554948c1e0a64aa5a2b0a8` |
| event / index SHA-256 | `66d2a7ed...` / `ffd35eea...` |
| verified index size | 96172 bytes |
| trusted shell / policy / domains SHA-256 | `32cb57cd...` / `d130c028...` / `d4c930f6...` |

Fixture and trusted-shell bytes are copied unchanged from the exact nampplets
revision because the Rust crate does not package its web resources. Their full
digests are enforced by `scripts/check-pinned-assets.sh` and recorded in
[`../compatibility.lock`](../compatibility.lock).

## Runtime evidence

Uzel calls the public upstream facade only:

```text
RuntimeController::open
RuntimeController::verify_artifact
RuntimeController::install
RuntimeController::set_grant
RuntimeController::launch
RuntimeController::read_verified
RuntimeController::mapped_envelope
RuntimeController::observe/snapshot/close
```

The launched session advertised `identity`, `inc`, `outbox`, and `shell`.
`link`, `resource`, and `theme` were reported unavailable. Uzel did not fake
providers or duplicate NMP/nampplets behavior to make the fixture appear fully
capable.

The trusted shell maps `MessageEvent.source` to its own surface and emits only a
shell-owned token to Rust. Rust maps that token to the internal session ID and
ignores payload `session`, `principal`, or sender claims. Unit tests prove an
unknown token is rejected and a forged handshake obtains no response.

## Commands and observed results

```sh
cargo +1.89.0 test -p napd --locked
nix --extra-experimental-features 'nix-command flakes' develop --command bash -c 'pnpm check'
nix --extra-experimental-features 'nix-command flakes' develop --command bash -c 'pnpm test'
nix --extra-experimental-features 'nix-command flakes' develop --command bash -c 'pnpm lint'
nix --extra-experimental-features 'nix-command flakes' develop --command bash -c 'pnpm format:check'
nix --extra-experimental-features 'nix-command flakes' develop --command bash -c 'pnpm docs:check'
nix --extra-experimental-features 'nix-command flakes' develop --command bash -c 'pnpm fallow'
nix --extra-experimental-features 'nix-command flakes' develop --command bash -c 'pnpm smoke'
nix --extra-experimental-features 'nix-command flakes' develop --command bash -c 'pnpm smoke:fedora'
bash scripts/debian-build-smoke.sh
```

Observed:

- all runner tests and the complete Rust workspace pass;
- Svelte reports zero errors and warnings; Vite and Cargo build;
- Clippy `-D warnings`, rustfmt, boundary checks, documentation audit, exact
  asset digests, and Fallow's zero-issue gate pass;
- Fedora emits exact verification, NAP-SHELL, artifact response, and hostile
  child-isolation markers before `FEDORA_RUN_SMOKE_OK`;
- Debian Bookworm compiles the locked frontend and complete Tauri, nampplets,
  and NMP Rust graph before `DEBIAN_BUILD_SMOKE_OK`.

## Preserved failed Fedora probe

The first real run is preserved under
[`probes/slice-02-fedora-failed/`](probes/slice-02-fedora-failed/). It exposed
two environment assumptions:

1. released `@tauri-apps/api` is 2.11.1, not the initially tested 2.8.0;
2. `nix develop --command bash -lc ...` starts the user's login shell and
   prepends a host GCC 14 path, which could not link the pinned WebKitGTK
   libraries requiring `GLIBC_2.42`.

The package was corrected to the released 2.11.1 API and runtime probes use a
non-login `bash -c`, preserving the pinned Nix GCC 15 toolchain. The next real
run passed. This is a probe-command correction, not a product workaround.

## Required design correction

The Gate 0 AF_UNIX probe proved a 4096-byte bounded control protocol. It did not
prove transport of a verified document, and the exact fixture document measures
96172 bytes. Increasing the frame limit, exposing a cache path to WebKit, or
claiming daemon ownership would preserve a false assumption.

Slice 02 therefore keeps `RuntimeController` in the trusted Tauri process.
Work 04 must add a bounded chunked verified-asset transfer, or an equivalently
bounded private custom-scheme stream, with size/order rejection before moving
runtime authority into `uzel-napd`. Control messages retain the 4096-byte limit.

## Upstream result

No new upstream code change was required. Uzel uses the exact public nampplets
facade and copied portable trusted-shell bytes unchanged. The existing
`jodobear/nampplets:compat/napplet-0.29` branch now has integrated Linux evidence
and is ready for upstream submission; it remains tracked in
[`../docs/08-upstream-contributions.md`](../docs/08-upstream-contributions.md).

## Next step

Proceed to [`../work/03-napplets.md`](../work/03-napplets.md). Keep the two
napplets local and self-contained, use the upstream package/spec line, and do
not start Work 04 daemon/NMP ownership early.
