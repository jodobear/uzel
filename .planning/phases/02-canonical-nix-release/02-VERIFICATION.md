---
phase: 02-canonical-nix-release
verified: 2026-08-20T15:53:09Z
status: passed
score: 5/5 must-haves verified
behavior_unverified: 0
---

# Phase 2: Canonical Nix release verification

**Phase Goal:** Ship one exact-pinned store-path Linux artifact with compatible native
runtime dependencies.
**Verified implementation head:** `dfbbe3457d6dbb849e63a49ee75a31317bbb61d3`
**Verified tree:** `66541a264f281d69edf512fd72e1c0082f718ee9`
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

- Output: `/nix/store/w6n7fwjk2b2f1qa527k8sfincf39bh68-uzel-0.0.0`.
- Derivation: `/nix/store/zq1brpb7f9vq3z7l3yw6dr56y62vy5jn-uzel-0.0.0.drv`.
- Packaged shell SHA-256:
  `2ea11a4fac775edea14006989c865828687daa6a6e1ede336eb009ec3dbe357d`.
- Packaged daemon SHA-256:
  `ea6605b9578410a7117e48e9161fb8fe906123821b8698047ca12ade698e0222`.
- Focused launcher evidence passed four pre-existing-path refusals, deterministic
  post-check/pre-bind substitution refusal, sequential and concurrent ownership, exact
  TERM/INT child reaping, unexpected-daemon paired shutdown, and truthful
  `webkit=not-run` reporting.
- The focused mismatch-only Weston discriminator refused the injected incompatible Hello
  response before `UZEL_SHELL_READY`. The client connect path now has the same bounded
  deadline as framed reads and writes; its positive and saturated-listener tests pass.
- Because that correction changed the packaged shell bytes, one affected full packaged
  Weston/WebKit run was repeated at this exact output with development loader/data paths
  scrubbed; it emitted `PACKAGE_SMOKE_OK`, `LINUX_RUN_SMOKE_OK`, and
  `UZEL_WEBKIT_RECOVERY_OK`.

## Review and delivery gate

Haven mechanism review and Meadow security review were CLEAN on the security-relevant
implementation through `a28f331b`; the later bounded client-connect deadline, paired-child
supervision, and evidence-environment correction change no trust boundary and passed their
affected unit and native package gates. PR
[#48](https://github.com/jodobear/uzel/pull/48) carries the phase. Required
CI, GitHub Codex, CodeRabbit, and final exact-head merge checks remain delivery gates; they
do not alter this implementation-verification result and must pass before merge.

## Verification metadata

**Approach:** goal-backward verification against the frozen candidate, recorded focused
results, reviewed source/digest bindings, the affected launcher/runtime probes, and the
single stable native package run.
**Affected validation:** `cargo test -p napd-protocol`, strict package clippy, the Linux
smoke-script contract, exact Nix package build, launcher-only lifecycle and daemon-exit matrix,
mismatch-only Weston discriminator, and one exact-output packaged Weston/WebKit run.
