# Phase 1 Ownership Map

Current source/import/lock evidence is authoritative. The nested POC pack supplies bounded replay history only.

| Concern | Owner | Current source evidence | Disposition |
|---|---|---|---|
| Linux product composition, private daemon lifecycle, trusted surfaces, source-binding policy, recovery policy, and presentation | `jodobear/uzel` | `apps/uzel/`, `crates/napd/`, `scripts/linux-run-smoke.sh` | retain as Uzel product runtime |
| Native runtime controller, bridge, FFI, and resource provider crates | `jodobear/nampplets` at `e2f69f325a6b45213accdacfcc125e80e0687b4c` | `Cargo.toml` and `Cargo.lock` git sources | exact-pinned upstream; change only for a concrete Uzel need and reusable fix |
| Nostr queries, relay work, canonical store, freshness, provenance, diagnostics, signer, and publication | `pablof7z/nmp` at `005dc2a5f12aa414961b313d05ebb021934e385c` | transitive sources in `Cargo.lock`; current runner/provider imports | preserve NMP as sole Nostr authority; no duplicate Uzel data plane |
| Napplet protocol messages and lifecycle | protocol packages already imported by current workspace | `package.json`, `pnpm-lock.yaml`, contracts, and napplet imports | keep current locked protocol boundary; upstream work only for a proven reusable fix |
| Exact-build fixture review/install/launch and private state | `jodobear/uzel` | `crates/napd/src/fixtures.rs`, `crates/napd/src/runner.rs`, `fixtures/` | retain; validate restart and exact-build recovery |
| Trusted shell bytes | current Uzel fixtures plus source-proven nampplets provenance | `apps/uzel/public/trusted-shell/`, compatibility lock, pinned digests | preserve byte/digest binding; no speculative repin |
| Protected user inputs and acceptance evidence | human disposition authority; Uzel preservation duty | durable evidence destination and verified hashes below | retain compact unique inputs; Git preserves completed committed work |
| Graphify output | none; advisory local cache | `WORKFLOW.md`, `.gitignore` | ignored, disposable, non-blocking; never commit or refresh for Phase 1 |
| Obsolete external-framework qualification/checker/handoff | none | no current source/import/lock consumer | removed; Git history retains historical context |

## Proven upstream boundary

Only `jodobear/nampplets` and `pablof7z/nmp` are currently proven project upstreams by Cargo source and lock entries. Kehto, a separate protocol repository, or any other repository becomes relevant only when a current import or concrete reusable Uzel fix supplies evidence. No speculative upstream contribution is active.

## Protected evidence disposition

Human-authorized disposition completed on 2026-08-14. Only five verified unique user
inputs were copied to
`/workspace/projects/napplets/napp-uzel/protected/uzel-phase-01/evidence/` before cleanup:

| Input | Verification |
|---|---|
| `UZEL_LEAN_GOAL_PROMPT.md` | SHA-256 `92366a7e0cd696f46366e9803a9c29452f91a421aedcf3aa42e0dcacc044b873` |
| `UZEL_START_PROMPT.md` | SHA-256 `21a8ec797184a5b6585e8dd1a257c1e2627ae4c7911c5c3e55d9072ff31b44ce` |
| `jodobear-uzel-2026-08-09-nix-ci-reaudited.zip` | SHA-256 `54324de9fff621366da03e8ec2ef2c26f52a03e9f8f159c8316a8338a77d7f8b` |
| `uzel-product-incubation-v4-2026-08-10.sha256` | SHA-256 `c4d54cc49bc40eddd24c8c77d466be91ab934875e37b437e9b3e9e5715ca9560` |
| `uzel-product-incubation-v4-2026-08-10/` | 36 files; normalized relative-path/content manifest `4bb4c1e0db2e898201f5b0d50eb9523442641301c0131b534e339080dc73c69d` |

Source and destination names, bytes, modes, owners, hashes, and file counts matched before
and after cleanup. Eleven obsolete worktrees were removed through `git worktree remove`;
`--force` was used only for two dirty Graphify worktrees whose changes were generated
cache/output or byte-identical to committed `d355965`. Every branch/ref was preserved.
The active PR #43 worktree remains until exact-head review and merge complete. Rebuildable
primary `target/`, `node_modules/`, `graphify-out/`, and `.artifacts/` were deleted after
verification; they are not acceptance evidence.
