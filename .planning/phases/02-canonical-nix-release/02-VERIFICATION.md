---
phase: 02-canonical-nix-release
verified: 2026-08-20T13:54:01Z
status: passed
score: 5/5 must-haves verified
behavior_unverified: 0
---

# Phase 2: Canonical Nix release verification

**Phase Goal:** Ship one exact-pinned store-path Linux artifact with compatible native
runtime dependencies.
**Verified implementation head:** `66ba4fc7f420b92a5dcf33a23de4732fafc95f36`
**Verified tree:** `a37558be29ae67dd007f514b9ef9d3949e65198c`
**Status:** passed

## Goal achievement

| # | Observable truth | Status | Existing evidence |
|---|---|---|---|
| 1 | Locked named/default package, app, and checks realize one `x86_64-linux` closure | VERIFIED | Stable candidate flake/package validation and `flake.nix` outputs |
| 2 | The store-path launcher uses only closure-private shell and daemon payloads | VERIFIED | Package smoke hostile-`PATH`, non-checkout, absolute-payload, and socket-mode probes |
| 3 | Private protocol compatibility fails closed before shell readiness | VERIFIED | Focused Rust Hello/version tests plus mismatch package probe |
| 4 | Sequential, signal, isolated-concurrent, and shared-XDG launches preserve ownership | VERIFIED | Launcher probes passed TERM=143, INT=130, child reaping, owned-socket retirement, isolated concurrency, and shared-root refusal |
| 5 | Exact closure, pins, assets, and real packaged Weston/WebKit recovery are inspectable | VERIFIED | Stable package report and one affected native run emitted `PACKAGE_SMOKE_OK`, `LINUX_RUN_SMOKE_OK`, and `UZEL_WEBKIT_RECOVERY_OK` |

**Score:** 5/5 observable truths verified.

## Requirements coverage

| Requirement | Status | Evidence |
|---|---|---|
| PKG-01 | SATISFIED | Locked flake package/default/app/check evaluation and build |
| PKG-02 | SATISFIED | Pin, lock, fixture, closure, and trusted-shell digest agreement |
| PKG-03 | SATISFIED | Non-checkout store-path launch, absolute private payloads, and bounded launcher ownership |
| PKG-04 | SATISFIED | Versioned Hello succeeds when matched and refuses incompatibility before readiness |
| PKG-05 | SATISFIED | Closure/source/assets report and affected real Weston/WebKit package evidence |

## Exact inputs

- Nampplets Rust runtime: `e2f69f325a6b45213accdacfcc125e80e0687b4c`.
- Portable trusted shell: `eefa9f9d8aa463b833b4d93723dd770f81408889`.
- Embedded trusted-shell SHA-256:
  `a3e6c18e8724329332bd15a039282a8a0bcf5ec93577b97752f46721df80fba3`.
- NMP: `005dc2a5f12aa414961b313d05ebb021934e385c`.

## Review and delivery gate

Haven mechanism review and Meadow security review were CLEAN on the verified implementation
head/tree. Draft PR [#48](https://github.com/jodobear/uzel/pull/48) carries the phase. Required
CI, GitHub Codex, CodeRabbit, and final exact-head merge checks remain delivery gates; they
do not alter this implementation-verification result and must pass before merge.

## Verification metadata

**Approach:** goal-backward verification against the frozen candidate, recorded focused
results, reviewed source/digest bindings, and the single stable native package run.
**Product commands rerun:** none. This record adds no implementation change and intentionally
does not repeat the package/WebKit run.
