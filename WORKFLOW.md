# Uzel delivery workflow

This document is the sole normative process authority for active Uzel delivery. Product
scope lives in `.planning/PROJECT.md` and `.planning/REQUIREMENTS.md`; concise future
outcomes live in `.planning/ROADMAP.md`; `.planning/STATE.md` is only the current pointer.
Historical POC material under `uzel-poc-validated-pack/` is evidence, not active workflow.

## Unit of delivery

One bounded GSD phase equals one contextual GitHub issue, one dedicated branch/worktree,
and one primary PR. The issue is the slice contract. Git commits own implementation. The
PR, required CI, tests, runtime probes, Playwright, and screenshots own execution evidence.
Mosaico may supply awareness and coordination, never authority.

Plan only the next ready phase. Its single plan contains only outcome, appetite,
boundaries, no-gos, risks, dependencies, and three to seven observable acceptance checks.
Future roadmap entries remain short outcome statements until selected. Run one plan-check
pass; reshape once if material ambiguity remains, otherwise request human judgment. Do not
add validator bodies, receipt systems, timestamp choreography, custom review refs,
self-referential commit algebra, transition machinery, or speculative implementation
pseudocode to plans.

## Locked command entrypoint

Use the repository flake and lock as the canonical toolchain environment. Run normal
repository commands as:

```sh
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm <script>
```

`package.json` defines `<script>`. `Cargo.lock`, `pnpm-lock.yaml`, `deno.lock`,
`flake.lock`, and `uzel-poc-validated-pack/compatibility.lock` remain authoritative for
their dependency domains. Do not introduce another toolchain or dependency lock.

## Phase loop

1. Read this file, `.planning/STATE.md`, the selected roadmap outcome, its GitHub issue,
   applicable repository instructions, and due review-backlog issues.
2. Query the existing Graphify graph before source inspection for codebase questions.
3. Shape and plan only the selected issue; run one plan-check pass.
4. Work from current integrated base in the issue's dedicated branch/worktree.
5. Implement the smallest complete vertical slice. During debugging use focused tests,
   affected checks, and narrow probes.
6. After code stabilizes, run the locked `pnpm graphify:refresh` entrypoint once. It runs
   the supported `graphify update .` command and excludes machine-local cache state from
   canonical output. Commit canonical graph output separately. Repeat only when later code
   changes make it stale.
7. Run one complete affected candidate validation, complete GSD verification, push, and
   open one linked draft PR.
8. Request GitHub Codex review on a coherent exact head. Batch-fix valid findings. When
   Codex is clean, request CodeRabbit. Any semantic/code change invalidates both approvals;
   reacquire GitHub Codex then CodeRabbit on the same exact SHA.
9. Run final complete affected validation once on that approved exact head. Merge only
   when GSD verification, required CI, GitHub Codex, and CodeRabbit are green on that SHA
   and repository policy permits automation.
10. Close the issue, leave one final GSD summary and verification result, reduce
    `.planning/STATE.md` to the next pointer, promote due backlog, and continue
    automatically through ready phases.

Expensive native WebKit/Weston, packaging, and complete conformance gates run only when
affected and only on stable candidates. Durable manifests, evidence, screenshots, and
reports are generated after inputs stabilize, then regenerated only after invalidating
semantic changes.

## Reviewer findings

Every finding is either `FIXED-NOW` with code/test evidence or
`DEFERRED-TO-BACKLOG` in a GitHub issue. A deferred item records reviewer and URL, reviewed
SHA, severity, file/line when applicable, deferral reason, current impact, owner, target
phase, revisit trigger, and observable acceptance criterion. Even disputed findings enter
the backlog as `revalidate` until closed with source/runtime evidence. P0/P1 findings and
acceptance-breaking findings block merge. Safe non-blocking P2/P3 findings may be deferred
only through the complete backlog contract above. No item may outlive its target phase;
audit all remaining backlog before milestone completion.

Candidate count is a diagnostic signal, not an automatic stop. Continue fixing new,
isolated valid findings coherently. Stop review cycling only when the same material root
cause remains unresolved across cycles, reviewer requirements materially conflict, or no
safe autonomous disposition exists. Report the repeated evidence, conflict, or missing
disposition path when requesting human judgment.

## Human gates and preservation

Stop only for conflicting/missing normative authority, irreversible or destructive
operations requiring consent, signer/key/payment/publication/financial authority, material
security choices, missing required access or physical interaction, review non-convergence,
or repository policy requiring human merge approval. Ordinary defects, failed tests/CI,
review findings, locked dependency realization, and recoverable tool errors remain
autonomous work.

Never mutate user-owned dirty work. Preserve protected incident refs, worktrees, and
portable evidence until their named replay phase verifies them and assigns a final
disposition. Every upstream-bound change uses a dedicated branch in the corresponding
`jodobear` fork and is recorded in
`uzel-poc-validated-pack/docs/08-upstream-contributions.md`.
