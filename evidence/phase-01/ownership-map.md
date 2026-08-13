# Phase 01 ownership map

**Observed at Uzel HEAD:** `44a2399edcb5cf91fc57ca3b46325f0f01c0c488` on `phase/01-poc-replay-napp-seam`  
**Method:** read-only inspection of `WORKFLOW.md`, `uzel-poc-validated-pack/compatibility.lock`, `uzel-poc-validated-pack/docs/08-upstream-contributions.md`, `git worktree list --porcelain`, and selected refs.

## Ownership and disposition

| Classification | Concern / seam | Owner | Exact evidence | Disposition | Revisit trigger |
| --- | --- | --- | --- | --- | --- |
| Retained Uzel concern | Linux Tauri/Svelte product composition, private daemon client, source-bound surface policy, presentation | `jodobear/uzel` | `WORKFLOW.md`; `.planning/PROJECT.md`; accepted comparison `19519c378c2e775c6ad4b042cfd9aadd89f766b9` | retain; evidence-only slice makes no product change | successful locked replay or later accepted Napp seam |
| Retained Uzel concern | Exact-build fixture verification, install, permission review, launch, stop/restart behavior | Uzel over reusable runtime facade | `crates/napd/src/fixtures.rs`; `crates/napd/src/runner.rs` | retain exact source bindings; do not replace runtime | fixture/pin change or replay failure after Nix access restored |
| Napp consumption / extraction need | Neutral committed Napp client, product events, testkit vectors, version/lifecycle/scope/NMP/pin/probe evidence | `jodobear/napp` source authority | `evidence/phase-01/candidate-qualification.md`; `evidence/phase-01/napp-dependency.md` | blocked; consume only after committed qualifying evidence, never infer it from checkout state | Napp owner publishes qualifying committed evidence |
| Compatibility seam | RuntimeController and exact-pinned nampplets/NMP facades; no duplicate runtime, cache, relay, or persistence truth | nampplets runtime; NMP data plane; Uzel product policy | `uzel-poc-validated-pack/compatibility.lock`; upstream ledger | retain pins and one-way owner boundaries | accepted compatible successor plus full Linux probes |
| Nostr data plane | query, relay, event store, freshness, provenance, diagnostics, signer, publication | `pablof7z/nmp` / pinned NMP | nested AGENTS; upstream ledger | preserve sole NMP ownership; no Uzel/Napp duplicate | authority changes an explicit public facade |
| Upstream candidate | portable trusted-shell bytes | `jodobear/nampplets` fork candidate `fc68bce0a4793a8618445e234bcc91d69e8b96de` | compatibility lock and upstream ledger | preserve provisional exact evidence; no repin | reviewed successor passes same Linux probes |
| Upstream candidate | runtime compatibility revision | current `jodobear/nampplets` exact pin `e2f69f325a6b45213accdacfcc125e80e0687b4c`; historical pre-catalog runtime `e539378ef735ce06651fd94b71e06f9ce757cb13` | `Cargo.toml`, `Cargo.lock`, and `uzel-poc-validated-pack/compatibility.lock` | retain current pin and historical provenance; current slice does not upgrade | accepted successor and repeat integrated probes |
| Upstream authority | protocol messages/lifecycle; manifest/artifact/sandbox; packaged web projection | `napplet/naps`, pinned NIP-5D, `napplet/web` | upstream ledger | Uzel consumes; does not redefine authority | authority revision requires compatibility revalidation |
| Obsolete POC-only concern | historical nested `STATUS.md` PR #30 active-language and old pass counts | preserved POC evidence | `uzel-poc-validated-pack/STATUS.md` | retain as history, never present it as current baseline success | a current replay produces new evidence |
| Obsolete POC-only concern | direct adaptation, new adapter proposal, Cargo/lock/runner/fixture edits before Napp admission | no owner granted | Phase 01 CONTEXT and plan | excluded / prohibited | same Phase 1 delivery unit resumes only after qualification |

## Protected evidence disposition

**Final disposition recorded:** 2026-08-14 at
`b4eeeb45615c4dcf223c5349a0465cee4f7d3ae2` by read-only inspection. “Retain as protected
archive” is the final Phase 01 disposition, not a deferred cleanup task. Any later deletion,
pruning, rewrite, or repurposing is a new human-authorized housekeeping decision outside this
phase.

| Protected ref or worktree | Exact HEAD / branch | Evidence location | Owner | Disposition | Revisit trigger |
| --- | --- | --- | --- | --- | --- |
| Current scoped worktree | `44a2399edcb5cf91fc57ca3b46325f0f01c0c488`; `phase/01-poc-replay-napp-seam` | this Phase 01 evidence directory | Issue #42 / Uzel delivery | retain focused worktree; do not switch or repurpose | Phase 01 closure after external Napp prerequisite |
| Primary incident archive | `763412a3167713b98c6f741641d485d247041934`; `archive/dirty-primary-763412a` | `/workspace/projects/napplets/napp-uzel/uzel` | protected incident owner | **final: retain indefinitely as read-only forensic archive; exclude from active delivery and never clean/reset/mutate from Phase 01** | none; any destructive housekeeping requires a new human decision |
| Prior replay harness | `b185ad1b8d9d034d151406b12aa189f5a6be970f`; `gsd/phase-01-plan-01-replay`, also `wip/phase-1-replay-b185ad1` | `/tmp/uzel-01-01-3qGzwY` | prior Phase 01 planning owner | **final: retain as inactive historical harness; no merge, rewrite, or active authority** | none; any destructive housekeeping requires a new human decision |
| Prior Phase 01 review pause | `227d1fc43c93fec701b384bdbc2e302ec93c157b`; `phase/01-baseline-v4` | `/workspace/projects/napplets/napp-uzel/uzel-phase-1-v4` | prior review owner | **final: retain as inactive paused evidence; not active delivery or source authority** | none; any destructive housekeeping requires a new human decision |
| Lean process reset | `eea91162a498b579cf47055013be6912a5f4a85d`; `chore/lean-process-reset` | `/tmp/uzel-lean-reset` | process-reset owner | **final: retain as inactive process evidence; no Graphify or planning regeneration** | none; any destructive housekeeping requires a new human decision |
| Review-fix worktree | `ed71845eb9cba37e8bc9fcbfee142552753e823e`; `chore/lean-process-reset-review-fixes` | `/tmp/uzel-lean-review-fixes` | review-fix owner | **final: retain as inactive unrelated review evidence; do not merge into Phase 01** | none; any destructive housekeeping requires a new human decision |

Primary archive evidence remains unique and intact: clean tracked/index state at
`763412a3167713b98c6f741641d485d247041934`; five untracked archive inputs; 36-file,
524-KiB incubation directory; 44-KiB zip SHA-256
`54324de9fff621366da03e8ec2ef2c26f52a03e9f8f159c8316a8338a77d7f8b`; prompt SHA-256
values `92366a7e0cd696f46366e9803a9c29452f91a421aedcf3aa42e0dcacc044b873` and
`21a8ec797184a5b6585e8dd1a257c1e2627ae4c7911c5c3e55d9072ff31b44ce`.
Read-only inspection found no authority to delete, prune, rewrite, repurpose, or merge any listed
ref/worktree. Permanent non-active retention closes the Phase 01 disposition gate without a
destructive action.
