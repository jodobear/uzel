---
phase: 01
requested_reviewers: [codex, coderabbit]
reviewers: [coderabbit]
skipped_reviewers:
  codex: "Current orchestrator is Codex; gsd-review independence gate skips the executing AI's own CLI."
reviewed_at: 2026-08-10T00:10:00+05:30
review_base: 3a571baa58e2fd7db8a2fbf78cd9a5093c359ac6
review_head: 65340702c7d5bb7fce9ae40ce63af23a3133554a
plans_reviewed:
  - 01-01-PLAN.superseded.md
  - 01-02-PLAN.superseded.md
  - 01-03-PLAN.superseded.md
  - 01-04-PLAN.superseded.md
  - 01-05-PLAN.superseded.md
review_scope: ".planning/phases/01-slice-ref-01-poc-replay-accepted-napp-seam"
finding_counts:
  critical: 1
  major: 35
  minor: 1
  total: 37
---

# Cross-AI Plan Review — Phase 01

## Reviewer Status

- **Codex:** Not invoked. The current orchestrator is Codex, so the GSD independence gate excludes a second Codex CLI session from cross-AI consensus.
- **CodeRabbit:** Completed against committed Phase 1 changes from `3a571ba` to `65340702c7d5bb7fce9ae40ce63af23a3133554a`, scoped to the phase directory. It is a diff-only reviewer and did not receive the source-grounding prompt.

## CodeRabbit Review

CodeRabbit returned 37 actionable observations. These are review inputs, not accepted facts: each must be verified against current source and planning decisions before incorporation.

### Critical

1. **01-04, candidate probe execution — arbitrary command risk** (`01-04-PLAN.superseded.md:97`)
   Candidate-declared probes may execute inside a trust boundary without isolation. Restrict execution to an explicit read-only allowlist, or require a disposable sandbox with no network/secrets, read-only mounts, resource limits, and no dependency installation. Unsafe probes must be recorded as skipped rather than run.

### Major — 01-01 POC Replay

2. **Canonical inventory serialization missing** (`01-01-PLAN.superseded.md:106-110`)
   Define exact byte serialization for source, pin, and fixture inventories: separators, newline rules, quoting, and path encoding. Reuse one serializer in harness, validator, and self-test.

3. **Renderer coverage not derived from baseline config** (`01-01-PLAN.superseded.md:117-140`)
   Read `apps/uzel/tests/ui/playwright.config.mjs`, derive the required scenario set from the pinned configuration, and require an outcome for every declared scenario.

4. **Environment capture may leak secrets** (`01-01-PLAN.superseded.md:140`)
   Never persist the full process environment. Define a non-secret allowlist, redact before storage/hashing, preserve exact argv separately, and use restrictive artifact permissions.

5. **Summary/output contradiction** (`01-01-PLAN.superseded.md:190-196`)
   `01-01-SUMMARY.md` is required but excluded by the four-file changed-path assertion. Add it to declared output or explicitly exempt generated summaries.

6. **Work 07 status record under-specified** (`01-01-PLAN.superseded.md:158-160`)
   Define fixed headings and allowed statuses for review, Debian, cancellation, restart, cleanup, and rollback; validate each gate plus the exact PR 30 historical statement.

7. **Native and relocated artifacts can collide** (`01-01-PLAN.superseded.md:134`)
   Give native and relocated smoke invocations distinct artifact/evidence directories, especially under `--mode all`.

8. **Harness provenance bootstrap unresolved** (`01-01-PLAN.superseded.md:136-142`)
   The validator can be new/dirty while `execution_head` names only a commit. Commit harness first and execute from that clean head, or hash and explicitly admit its exact uncommitted bytes.

9. **Deadlines/cancellation not machine-defined** (`01-01-PLAN.superseded.md:134-140`)
   Define per-command/per-mode deadlines, process groups, graceful signal period, forced termination, descendant reaping, final liveness check, and timeout/cancellation records.

10. **Command-discovery pattern incomplete** (`01-01-PLAN.superseded.md:35-43`)
    Add `lint`, `fallow`, and `format:check` to the key-link pattern so discovery matches required fixture commands.

### Major — 01-02 Ownership and Measurement

11. **Measurement extractor is not executable** (`01-02-PLAN.superseded.md:159-163`)
    Define a finite extractor/capture schema and require the validator to derive recorded values from raw bytes; reject opaque descriptions and mismatches.

12. **Summary/output contradiction** (`01-02-PLAN.superseded.md:195-207`)
    `01-02-SUMMARY.md` is required but excluded by the six-file output assertion. Add it or explicitly exempt generated summaries.

13. **Raw-path confinement is not symlink-safe** (`01-02-PLAN.superseded.md:135,159-163`)
    Resolve artifact root and candidate real paths before opening; reject traversal and symlink escapes outside the resolved root.

14. **Committed baseline references ignored evidence** (`01-02-PLAN.superseded.md:100-108,159-163`)
    Fresh checkouts cannot validate ignored `.artifacts/` raw files. Store redacted evidence in a tracked location or bind to an immutable archive and digest.

15. **One-active-profile boundary not enforced** (`01-02-PLAN.superseded.md:23-25,157-164`)
    Record profile identity and active-profile count; fail closed unless exactly one active read profile is observed.

16. **Locked-shell provenance is self-reported** (`01-02-PLAN.superseded.md:157-163`)
    Capture and validate the `nix develop` wrapper, flake/lock identity, and resolved toolchain identity; reject ambient or mismatched invocation.

### Major — 01-03 Authority Admission

17. **Staging boundary not verified** (`01-03-PLAN.superseded.md:196-202`)
    Capture pre-task tracked/untracked state, assert staged paths equal the nine declared files, and prove unrelated dirty/untracked paths remain unchanged.

18. **Threat-model deny checks absent** (`01-03-PLAN.superseded.md:164-168`)
    Auditor must reject raw-frame, native, path, secret, principal, duplicate-store, and private-fallback authority, with fail-closed tests for each class.

19. **[MINOR] Authority provenance incomplete** (`01-03-PLAN.superseded.md:124,212-213`)
    Record every proposed root input, selected or non-normative, with before/after SHA-256 and Git tracked/untracked state.

20. **Auditor result/scope contract under-verified** (`01-03-PLAN.superseded.md:165-169`)
    Parse stable JSON fields for status, errors, admitted-file count, diagram count, and skipped parser capabilities. Prove malformed non-allowlisted Markdown is ignored without being admitted.

21. **Issue/PR template checks are text-only** (`01-03-PLAN.superseded.md:145-149`)
    Parse YAML/template contracts and validate required fields, external repository owner/repository, exact base/head, one issue/one slice, and once-per-candidate review evidence.

22. **Authority acceptance criteria under-proved** (`01-03-PLAN.superseded.md:124-129`)
    Validate admitted revision, decision reconciliation, complete non-normative inventory, and exact nampplets, transitive NMP, NAP, Tauri, and nixpkgs pins from committed sources/locks.

23. **Plan 01-02 dependency missing** (`01-03-PLAN.superseded.md:5-7`)
    CodeRabbit proposes making authority admission wait for ownership/measurement evidence by adding `01-02`. Verify this against the intended independence of REF-01B and REF-01C before accepting.

24. **Required YAML parsing may fail open** (`01-03-PLAN.superseded.md:164-168`)
    If admitted YAML cannot be parsed, return an audit error/nonzero result. Optional parser skipping must not cover required authority files.

### Major — 01-04 Napp Qualification/Handoff

25. **Plan 03 dependency absent** (`01-04-PLAN.superseded.md:5-6`)
    CodeRabbit proposes moving qualification behind committed root authority. Verify against D-02, which allows qualification to proceed independently while only adapter work requires both gates.

26. **Parity matrix handoff not bound** (`01-04-PLAN.superseded.md:108-114`)
    Reference the authoritative Plan 01 artifact and record its exact content/commit digest; otherwise remove the unspecified cross-plan requirement.

27. **Handoff equality checks are text-only** (`01-04-PLAN.superseded.md:115-118`)
    Parse both evidence documents and compare repository, commit, tree digest, result, missing categories, adapter preconditions, exact heads, blocker, next probe, rollback, and D-18 rule.

28. **Candidate validator does not prove reachability/completeness** (`01-04-PLAN.superseded.md:98-101`)
    Verify approved ref reachability, canonical repository, tree digest, every admission category, exact hashes, missing-category records, ownership, command output, `result: stop`, and `working_tree_evidence: excluded`.

29. **Repository mutation check is incomplete** (`01-04-PLAN.superseded.md:116-118`)
    Compare HEAD, index, refs, and porcelain-v2 status before/after in both Uzel and Napp; allow only declared evidence/ignored inventory.

30. **Candidate instructions read mutable working tree** (`01-04-PLAN.superseded.md:88-97`)
    Read `AGENTS.md`, `README.md`, and planning context from pinned committed blobs, recording missing files as committed-tree facts.

### Major — 01-05 Debian Acceptance

31. **Redaction gate incomplete** (`01-05-PLAN.superseded.md:25,95-100,123-125`)
    Validate selected identity, invoke key, secrets, host-private paths/details, raw unredacted logs, and unintended personal data against the staged record immediately before commit.

32. **Acceptance schema is under-verified** (`01-05-PLAN.superseded.md:98-102,121-137`)
    Require exact Debian 13/x86_64 values, both emitted markers, exact-build hashes, evidence run identifier/hashes, environment fields, every named checklist item as pass, and no failed items.

33. **Failure path can skip cleanup** (`01-05-PLAN.superseded.md:95-102`)
    After first falsifier, stop further assertions/source changes but always close window, confirm daemon cleanup, retain raw-log hash, and record failed result before returning.

### Major — Research Contract

34. **Required replay modes can be marked unavailable and pass** (`01-RESEARCH.md:192`)
    Treat unavailable Chromium/Weston/WebKit evidence as blocked/incomplete. Preserve `unavailable` only as D-08 measurement terminology where applicable.

35. **POC replay and accepted-candidate parity are conflated** (`01-RESEARCH.md:55`)
    Keep REF-01A limited to merged-POC replay and leave candidate parity incomplete/conditional until D-01 qualifies a candidate.

36. **Measurement example lacks provenance** (`01-RESEARCH.md:270-281`)
    Add exact command, environment, input/build/fixture identities, limitations, raw-output digest, and exact Nix identifier, or reference the canonical schema containing them.

37. **Candidate admission example omits gates** (`01-RESEARCH.md:253-263`)
    Add explicit outcomes/evidence for reachability, clean committed tree, version, lifecycle, pin, client, events, testkit, and declared probes.

## Consensus Summary

No multi-reviewer consensus can be claimed. Only CodeRabbit ran, and CodeRabbit is a diff-only reviewer. Its plan-level verdict must be down-weighted until source-backed validation or another independent prompt-fed reviewer confirms each item.

### High-Priority Review Inputs

1. Candidate-declared commands need a strict sandbox/allowlist before execution.
2. Evidence validators frequently assert text/shape without proving provenance, reachability, durable raw bytes, exact fields, or cross-document equality.
3. Several plans contradict their own `*-SUMMARY.md` requirements and changed-file allowlists.
4. Sensitive environment/identity/host data needs explicit allowlisted capture and staged-record redaction gates.
5. Dependency suggestions conflict with the committed decision that replay, ownership, document admission, and qualification can proceed independently; those suggestions require decision-level validation, not automatic acceptance.

### Agreed Strengths

No strength qualifies as agreed across two independent reviewers.

### Agreed Concerns

No concern qualifies as agreed across two independent reviewers. CodeRabbit's recurring themes are evidence provenance, machine-verifiable acceptance, cleanup, and trust-boundary enforcement.

### Divergent Views

- **Plan 03 → Plan 02 dependency:** CodeRabbit proposes ordering them; D-02/D-16 and the roadmap describe independent preservation/admission lanes. Validate before changing waves.
- **Plan 04 → Plan 03 dependency:** CodeRabbit proposes ordering qualification after authority admission; committed decisions say absence blocks only adaptation and allow independent qualification. Adapter planning still requires both.
- **Raw evidence durability:** CodeRabbit favors tracked/immutable evidence. Reconcile this with redaction, repository size, retention, and the committed plan's ignored raw-artifact design before choosing a storage contract.

## Recommended Disposition Flow

Run `$gsd-plan-phase 1 --reviews`. The planner must verify every finding against source and current decisions, accept only valid items, explicitly reject stale/incorrect dependency assumptions, and preserve REF-01D's fail-closed block.
