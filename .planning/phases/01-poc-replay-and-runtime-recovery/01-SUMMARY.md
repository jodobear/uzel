# Phase 1: POC replay and runtime recovery summary

## Outcome

Phase 1 is complete. Uzel replayed the accepted POC at its locked inputs, proved the
current trusted-runtime source boundary, restart recovery, and real Weston/WebKit
recovery, recorded source-grounded ownership, and completed the human-authorized compact
retention of unique protected inputs.

## Delivery

- Issue [#42](https://github.com/jodobear/uzel/issues/42) is closed.
- PR [#43](https://github.com/jodobear/uzel/pull/43) merged reviewed head
  `63bd6aaf6d2f5506cec8bcfb98a1a2fd2bda4b3b` as merge commit
  `577e02b0549131a7231a566c73b91cc50cd5e29d`.
- GitHub Codex reported no major issues on the reviewed head.
- CodeRabbit completed on the same reviewed head. Its final stale-status findings are
  resolved by this bounded closeout correction; no Phase 1 product behavior changed.

## Acceptance

- REF-01: locked replay covered exact-build review, confirmation, launch, profile/follow
  rendering, and multi-surface composition.
- REF-02: source-bound requests and denied raw network, native, host, secret, and
  caller-selected authority were observed.
- REF-03: restart replay recovered identity, exact installed builds, follow state,
  ambiguous lifecycle reconciliation, and a fresh surface generation.
- REF-04: deterministic Chromium and real Weston/WebKit recovery passed source-binding,
  hostile-egress, native-bridge, and exact-fixture checks.
- REF-05: `evidence/phase-01/ownership-map.md` records Uzel ownership and the two
  source-proven upstreams.
- REF-06: `evidence/phase-01/poc-replay.md` records the bounded reproducible baseline and
  explicitly unavailable measurements.
- Protected evidence: five unique user inputs were hash-verified at the durable protected
  destination; obsolete worktrees and rebuildable bulk were removed while refs remained.

## Validation evidence

The merged evidence records one locked affected candidate with passing build, check, test,
conformance, Chromium UI, and native Linux smoke lanes. The bounded continuation records
passing affected checks, four selected Chromium scenarios with nine TAP tests, the restart
replay, and `UZEL_WEBKIT_RECOVERY_OK` plus `LINUX_RUN_SMOKE_OK` native markers.

This closeout does not rerun or reinterpret those product gates. It only supplies the
missing canonical summary, passing verification, completed roadmap status, and Phase 2
state pointer.
