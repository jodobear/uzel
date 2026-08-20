---
phase: 02-canonical-nix-release
verified: 2026-08-20T15:03:05Z
status: passed
score: 5/5 must-haves verified
behavior_unverified: 0
---

# Phase 2: Canonical Nix release verification

**Phase Goal:** Ship one exact-pinned store-path Linux artifact with compatible native
runtime dependencies.
**Verified implementation head:** `a28f331b70beb8b862ef2505ff75416e35884340`
**Verified tree:** `61b7c3c5c113ef255a32a5adec40af4ac0cde6ed`
**Status:** passed

## Goal achievement

| # | Observable truth | Status | Existing evidence |
|---|---|---|---|
| 1 | Locked named/default package, app, and checks realize one `x86_64-linux` closure | VERIFIED | Stable candidate flake/package validation and `flake.nix` outputs |
| 2 | The store-path launcher uses only closure-private shell and daemon payloads | VERIFIED | Current-flake output binding, hostile-`PATH`, non-checkout, absolute-payload, and socket-mode probes |
| 3 | Private protocol compatibility fails closed before shell readiness | VERIFIED | Focused Rust Hello/version tests plus mismatch package probe |
| 4 | Sequential, signal, isolated-concurrent, and shared-XDG launches preserve ownership | VERIFIED | Launcher probes passed daemon-origin bind receipt, post-check substitution refusal, TERM=143, INT=130, child reaping, owned-socket retirement, isolated concurrency, and shared-root refusal |
| 5 | Exact closure, pins, assets, and real packaged Weston/WebKit recovery are inspectable | VERIFIED | Stable package report and one affected native run emitted `PACKAGE_SMOKE_OK`, `LINUX_RUN_SMOKE_OK`, and `UZEL_WEBKIT_RECOVERY_OK` |

**Score:** 5/5 observable truths verified.

## Requirements coverage

| Requirement | Status | Evidence |
|---|---|---|
| PKG-01 | SATISFIED | Locked flake package/default/app/check evaluation and build |
| PKG-02 | SATISFIED | Pin, lock, fixture, closure, and trusted-shell digest agreement |
| PKG-03 | SATISFIED | Exact current output, non-checkout store-path launch, absolute private payloads, and daemon-origin socket ownership |
| PKG-04 | SATISFIED | Versioned Hello succeeds when matched and refuses incompatibility before readiness |
| PKG-05 | SATISFIED | Closure/source/assets report and affected real Weston/WebKit package evidence |

## Exact inputs

- Nampplets Rust runtime: `e2f69f325a6b45213accdacfcc125e80e0687b4c`.
- Portable trusted shell: `eefa9f9d8aa463b833b4d93723dd770f81408889`.
- Embedded trusted-shell SHA-256:
  `a3e6c18e8724329332bd15a039282a8a0bcf5ec93577b97752f46721df80fba3`.
- NMP: `005dc2a5f12aa414961b313d05ebb021934e385c`.

## Exact package evidence

- Output: `/nix/store/3bknmlmyq5ipjdw44cxjjzvlj11jssvh-uzel-0.0.0`.
- Derivation: `/nix/store/bipvsxs90y7wkb2zahdzyg80w2g3rn9g-uzel-0.0.0.drv`.
- Packaged shell SHA-256:
  `9b8c16fe4b0193ce8bb7e166792e39a0d5202aeff6dddbe8b9ab472d25c6ec8d`.
- Packaged daemon SHA-256:
  `b2d292229f8692f0b6a2676b33bc6e2f5ab5982ecd0746425979db3767c93c3f`.
- Focused launcher evidence passed four pre-existing-path refusals, deterministic
  post-check/pre-bind substitution refusal, sequential and concurrent ownership, exact
  TERM/INT child reaping, and truthful `webkit=not-run` reporting.
- The focused mismatch-only Weston discriminator refused the injected incompatible Hello
  response before `UZEL_SHELL_READY`. The earlier full packaged Weston/WebKit run remains
  applicable to the unchanged packaged shell and trusted-shell bytes; reviewers confirmed
  the daemon bind-readiness delta needs no repeat full WebKit run.

## Review and delivery gate

Haven mechanism review and Meadow security review were CLEAN on the verified implementation
head/tree, including the child-origin bind receipt and substitution negative. Draft PR
[#48](https://github.com/jodobear/uzel/pull/48) carries the phase. Required
CI, GitHub Codex, CodeRabbit, and final exact-head merge checks remain delivery gates; they
do not alter this implementation-verification result and must pass before merge.

## Verification metadata

**Approach:** goal-backward verification against the frozen candidate, recorded focused
results, reviewed source/digest bindings, the affected launcher/runtime probes, and the
single stable native package run.
**Product commands rerun:** none. This record adds no implementation change and intentionally
does not repeat the package/WebKit run.
