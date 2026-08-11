# Prompt — prepare one evidence-led upstream interaction

A Uzel phase has found a possible upstream defect, ambiguity or generally useful seam.
Prepare the smallest correct interaction; do not submit anything until a human reviews
it.

Read `07-ECOSYSTEM-UPSTREAM.md`, the target repository's current contributor/security
guidance, existing issues/PRs/discussions, Uzel's exact pin/current compatibility profile
and the relevant Upstream Interaction Record.

Required work:

1. reproduce at Uzel's immutable exact pin/profile hash;
2. reproduce, disprove or classify at an immutable current upstream commit/proposal source;
3. search the canonical existing issue/PR/discussion and read `CONTRIBUTING`, `SECURITY`,
   license, AI-assisted-contribution, authorship/signoff, DCO/CLA, style, architecture and test rules;
4. reduce to a synthetic minimal public reproducer with no secrets/private content or
   embargoed vulnerability detail;
5. classify the cause as Uzel misuse, adapter bug, library defect, spec ambiguity,
   generally useful missing seam, docs defect or security vulnerability;
6. select no action, existing-thread comment, issue, discussion, focused PR or private
   disclosure according to upstream policy;
7. separate Uzel product policy from the general upstream contract;
8. use a dedicated upstream fork/worktree/branch; preserve truthful authorship, keep commits minimal and avoid unrelated cleanup;
9. draft concise factual text with exact source identities, expected/actual behavior,
   impact, reproducer and test evidence;
10. for code, include the smallest test-first change and run upstream-required gates;
11. obtain a named human submitter/approver who understands and takes responsibility for
    route, technical claims, tone, privacy/disclosure and patch;
12. record AI assistance, repository AI policy, truthful authorship, signoff and DCO/CLA disposition; never fabricate maintainer consensus or evidence;
13. update the local upstream record, visibility and patch expiry/removal trigger;
14. state separate future events for upstream acceptance, merge, release, exact Uzel
    adoption and local-patch removal.

For NMP and every other fast-moving upstream, verify the current repository-specific
route at the exact observed revision before acting; prefer an existing thread or issue-
first route when current guidance or maintainer practice requires it. For NAP/NIP changes,
provide implementation, security and interoperability evidence rather than preference.
Use private disclosure for security issues.

Output the proposed route, draft interaction/patch, evidence and privacy checklist. Do
not post, push or open the issue/PR automatically.
