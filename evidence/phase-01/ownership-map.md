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
| Protected incident refs/worktrees/evidence | human disposition authority; Uzel preservation duty | `evidence/phase-01/poc-replay.md` protected inventory | preserve in place; no move/archive/delete until authorized |
| Graphify output | none; advisory local cache | `WORKFLOW.md`, `.gitignore` | ignored, disposable, non-blocking; never commit or refresh for Phase 1 |
| Obsolete external-framework qualification/checker/handoff | none | no current source/import/lock consumer | removed; Git history retains historical context |

## Proven upstream boundary

Only `jodobear/nampplets` and `pablof7z/nmp` are currently proven project upstreams by Cargo source and lock entries. Kehto, a separate protocol repository, or any other repository becomes relevant only when a current import or concrete reusable Uzel fix supplies evidence. No speculative upstream contribution is active.

## Protected evidence disposition

Durable worktrees under `/workspace` remain protected inactive archives. Replay/process worktrees under `/tmp` are preserved in place but are not durable because routine cleanup or reboot can remove them. No Phase 1 authority exists to move, archive, delete, prune, rewrite, or repurpose them. This human-only final-disposition gate does not block REF-02, REF-03, or REF-04.

| Protected ref or worktree | Exact HEAD / branch | Evidence location | Disposition |
|---|---|---|---|
| Current scoped worktree | current PR head; `phase/01-poc-replay-napp-seam` | `/tmp/uzel-phase-01-replay-seam` | retain focused worktree; resolve exact head before preservation action |
| Primary incident archive | `763412a3167713b98c6f741641d485d247041934`; `archive/dirty-primary-763412a` | `/workspace/projects/napplets/napp-uzel/uzel` | retain indefinitely as read-only forensic archive; never clean/reset/mutate from Phase 1 |
| Prior replay harness | `b185ad1b8d9d034d151406b12aa189f5a6be970f`; `gsd/phase-01-plan-01-replay` | `/tmp/uzel-01-01-3qGzwY` | pending: protect in place; human must authorize non-destructive durable copy/archive |
| Prior Phase 1 review pause | `227d1fc43c93fec701b384bdbc2e302ec93c157b`; `phase/01-baseline-v4` | `/workspace/projects/napplets/napp-uzel/uzel-phase-1-v4` | retain as inactive paused evidence |
| Lean process reset | `eea91162a498b579cf47055013be6912a5f4a85d`; `chore/lean-process-reset` | `/tmp/uzel-lean-reset` | pending: protect in place; human must authorize non-destructive durable copy/archive |
| Review-fix worktree | `ed71845eb9cba37e8bc9fcbfee142552753e823e`; `chore/lean-process-reset-review-fixes` | `/tmp/uzel-lean-review-fixes` | pending: protect in place; human must authorize non-destructive durable copy/archive |

Primary archive evidence remains unique and intact at `763412a`: five untracked archive inputs; 36-file, 524-KiB incubation directory; 44-KiB zip SHA-256 `54324de9fff621366da03e8ec2ef2c26f52a03e9f8f159c8316a8338a77d7f8b`; prompt SHA-256 values `92366a7e0cd696f46366e9803a9c29452f91a421aedcf3aa42e0dcacc044b873` and `21a8ec797184a5b6585e8dd1a257c1e2627ae4c7911c5c3e55d9072ff31b44ce`. No original ref, worktree, or evidence may be removed as part of a future durable-preservation action.
