# Prompt — review revised Phase 1 plans

Review the generated Uzel Phase 1 plans against the exact repository and v4 baseline
contract. Commit and push the complete immutable candidate, then run local CodeRabbit's
required candidate-review mode and ask GitHub Codex to review that same exact PR SHA. The
two reviewer runs may overlap, but acceptance waits for both dispositions. Record CodeRabbit CLI/version/result, GitHub Codex review
identity, base/head SHAs, prompt digest, commands/requests and full findings. Only if that
required local CodeRabbit mode returns a recorded `rate_limit` error before findings does
a green GitHub Codex result become the approved fallback and permit continuation. A later or
alternate-mode rate limit, or any other CodeRabbit failure, does not.
Claude, OpenCode, remote CodeRabbit and local Codex self-review are not part of this
programme. Any later commit invalidates review evidence and starts a new review path.

Read the v4 ingest, baseline replay and delivery-quality documents; all current Phase 1
artifacts; exact package-manager/lock/script/Nix/test files; and Git/worktree/evidence
state.

Verify with `file:line` or command evidence:

1. Vite and conformance-tool origins are inventoried before choosing materialization.
2. Closure realization is exact/hash-locked; historical source and lock remain unchanged.
3. External egress/public DNS are denied during replay while only declared loopback or
   Unix-socket fixtures are allowed.
4. Lifecycle scripts are disabled by default or exactly allowlisted and sandboxed.
5. Globals, dynamic downloads, copied dependency trees and package substitution are
   forbidden.
6. Historical replay and current Nix/native baseline produce separate verdicts; absent
   installable outputs are `not_yet_packaged` with package acceptance owned by Phase 2.
7. Honest unavailable/failed/superseded outcomes and critical-invariant replacement
   rules exist.
8. Replay has one inventory, one realization path, one attempt and at most one proven
   harness-only correction.
9. `b185ad1` has a worktree, safety ref and portable checksummed archive; final
   disposition waits for execution evidence.
10. Codex execution is sequential with GSD automatic worktrees disabled.
11. Phase 1 contains no feature, dependency upgrade, broad file platform or public API.
12. Authority/schema/instance/local-profile/CI baselines are source-grounded.
13. The complete future roadmap already matches phases 2 through 7.9, including 2.7,
    in the v4 map.
14. A canonical machine upstream registry and RCP are planned from immutable commit/tree/
    path/content or package-integrity identities, not mutable labels; the package binds and
    exposes the RCP hash and the human view is generated.
15. The NIP-5A/NIP-5D/NAP/tool manifest/build-identity disagreement has a SIR, vectors and
    Phase 2 go/no-go gate.
16. Required/optional capability negotiation rejects mismatch before guest code, has a
    canonical transcript bound to profile/build/session principals, and forbids implicit
    downgrade or mid-session profile replacement.
17. Initial capability ledgers, clean-room fixture plan, independent-peer blocking rule,
    interop/version-skew backlog and local-patch/upstream records are owned; merge,
    release, adoption and patch removal are separate states.
18. Upstream contribution work uses current repository policy and a dedicated fork/
    worktree/branch, named human submitter/approver, AI-assistance/authorship/signoff
    disposition and human-reviewed public text.
19. Canonical terminology, decision, profile/negotiation, upstream, learning, visibility
    and education deltas are part of phase closeout, with executable teaching witnesses;
    milestone endpoints require a learning digest.
20. Global/per-principal admission, fairness and anti-starvation ownership/tests are part
    of the runtime baseline rather than deferred to M5.
21. Phase 1 has a human stop before Phase 2.
22. Installed command help confirms the required ingest/review forms, plan checking,
    optional validation semantics when present and mandatory post-execution `verify-work`.

Classify Critical/High/Medium/Low. Replan with
`$gsd-plan-phase 1 --reviews` while legitimate findings materially improve the phase.
Stop only when all material findings are fixed and every remainder is a low-benefit P2
backlog item with stable ID, source, severity, owner, deferral rationale, bounded Phase-1
non-impact, and revisit trigger. Do not execute with any Critical/High finding or a Medium finding that threatens
the phase outcome, authority, correctness, data integrity, security or operability.
Record every remaining non-blocking finding under that P2 backlog contract. A green GitHub Codex review with recorded pre-finding `rate_limit`
evidence from local CodeRabbit's required immutable-candidate review mode satisfies this
review gate. Do not execute in this review session.

Do not send keys, pairing URIs, credentials, production content or unredacted private
diagnostics to any reviewer.
