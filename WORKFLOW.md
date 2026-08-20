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

The selected GitHub issue and roadmap outcome are the default discussion context; autonomous
execution skips a separate discuss round. Use at most one bounded discuss pass only when the
issue leaves a consequential product, security, or source-authority choice unresolved.

## Lean defaults

Keep GSD auto-advance, source-grounded plan check, execution verification, bounded repair,
and worktree isolation enabled. Research, pattern mapping, Nyquist, internal GSD code review,
UI/AI/API design gates, security enforcement, Intel, Graphify, and release tagging are
default-off. Activate one explicitly only when the selected issue needs that specialist;
do not enable it milestone-wide or retain its artifacts after the question is answered.

GitHub Codex and CodeRabbit are the normal code-review pair. Put durable domain invariants
under `## Code Review Rules` in the applicable `AGENTS.md`; give Codex only a short
change-specific focus when requesting exact-head review. Do not run Haven or Meadow as a
routine pre-review. Route Haven only when native/platform behavior needs an independent
reproducer, a reviewer or CI finding needs root-cause debugging, or a new mechanism needs a
focused oracle or fork contribution. Route Meadow only for an unresolved material security
or normative decision, cryptographic/key/secret work, an incident, or a security finding
that tests and repository rules cannot disposition. Human authority remains required for
source/spec changes. UI work uses focused Playwright interaction, accessibility checks, and
screenshots; create a separate UI design contract only for genuinely new interaction design.

The canonical per-phase records are the issue, one plan, one summary, and one verification.
Add one security record only when the phase touches a security boundary. Do not create
research summaries, pattern reports, gap reports, duplicated receipts, or regenerated
evidence unless an explicitly activated specialist produces information required to ship.

## Team coordination

- `@quinn-codex` is PM and owns scope, sequencing, assignments, source authority,
  acceptance, publication gates, and team tracking.
- `@dawn-codex` is developer/integrator and owns Uzel implementation, phase execution,
  product validation, product PRs, and integration of approved handoffs.
- `@haven-codex` is debugger/tester/upstream contributor and owns independent
  reproduction, root-cause analysis, focused oracles, and dedicated fork contributions.
- `@meadow-codex` is security expert and owns security questions, threat analysis,
  sandbox/CSP/source-binding and Nostr-cryptography review, and textual security verdicts.

Assign work to the exact agent identity. Activation is `👍` accepted then `🟢` working;
a tag, idle state, or acceptance alone is not active execution. Each owner uses an isolated
lane and never edits another owner's worktree. A handoff states owner, path/branch/head,
clean or dirty state, evidence and commands, findings, required output, permissions, and
prohibited actions. Quinn accepts and routes handoffs; Dawn alone integrates into Uzel.

Use Mosaico Emoji Protocol v1 as the canonical reaction language. Emoji never grants
authority for destructive or host actions, external disclosure, publishing, merging,
spending, secrets, or normative security/product decisions; those require explicit plain
text from the proper authority. Meadow's security verdict informs but never replaces a
human normative decision.

For upstream work, develop, test, and review locally in a dedicated `jodobear` fork
worktree. Publish only a clean, surgical upstream issue and PR after PM approval and any
required Meadow verdict; keep bot-review chatter, AI/process narration, incremental review
debris, and unnecessary comments off upstream repositories. Publication proceeds only
when explicit human authority already covers that action.

## Locked command entrypoint

Use the repository flake and lock as the canonical toolchain environment. Run normal
repository commands as:

```sh
nix --extra-experimental-features 'nix-command flakes' develop --command pnpm <script>
```

`package.json` defines `<script>`. `Cargo.lock`, `pnpm-lock.yaml`, `deno.lock`,
`flake.lock`, and `uzel-poc-validated-pack/compatibility.lock` remain authoritative for
their dependency domains. Do not introduce another toolchain or dependency lock.

Planning, review, cache, and preserved evidence files are not product build inputs. Package
source filters must include every runtime source, asset, manifest, lock, and pin while
excluding `.planning/`, VCS metadata, advisory graphs, caches, and preserved evidence.
A planning-only edit must not change the product derivation or invalidate native evidence.

## Phase loop

1. Read this file, `.planning/STATE.md`, the selected roadmap outcome, its GitHub issue,
   applicable repository instructions, and due review-backlog issues.
2. Inspect Git source directly. Explicitly enable and use `$gsd-graphify query` only when a
   dependency/community graph materially answers the selected issue faster than source search.
3. Treat the selected issue as shaped unless a consequential ambiguity remains. Create one
   bounded plan and run one source-grounded plan-check pass.
4. Work from current integrated base in the issue's dedicated branch/worktree.
5. Implement the smallest complete vertical slice. During debugging use focused tests,
   affected checks, and narrow probes.
6. If Graphify was explicitly activated, treat it as an advisory disposable cache. Build at
   most once after a stable scoped slice. Git source, tests, evidence, requirements, and
   decisions override it. Never let graph absence or refresh failure block work, and never
   commit `.planning/graphs/` or `graphify-out/`.
7. Run one complete affected candidate validation, complete GSD verification, push, and
   open one linked draft PR.
8. Request GitHub Codex review on a coherent exact head, applying `AGENTS.md` Code Review
   Rules plus a short focus list for the changed boundaries. Batch-fix valid findings. Call
   Haven or Meadow only on the exception triggers above. When Codex is clean, request
   CodeRabbit. Any semantic/code change invalidates both approvals; reacquire GitHub Codex
   then CodeRabbit on the same exact SHA.
9. Run final complete affected validation once on that approved exact head. Merge only
   when GSD verification, required CI, GitHub Codex, and CodeRabbit are green on that SHA
   and repository policy permits automation.
10. Close the issue, leave one final GSD summary and verification result, auto-prune
    `.planning/STATE.md` to the next pointer, review only backlog items due now, and continue
    automatically through ready phases.

Keep one worktree per active issue/PR. After merge or completion, verify its work is
committed and its tree is clean, remove rebuildable output, then remove the Git worktree.
After abandonment, preserve only verified unique human-authored work before removing the
worktree. Never retain completed worktrees indefinitely or automatically delete dirty or
user-authored content. Branch/ref cleanup is separate and remains human-gated wherever a
ref is protected.

Diagnostic/reproducer worktrees expire with their owning phase. After required evidence is
committed or moved to its approved durable location, verify the tree and registration, then
remove the diagnostic worktree in the same phase cleanup. Do not carry dormant scratch lanes
into the next phase.

Expensive native WebKit/Weston, packaging, and complete conformance gates run only when
affected and only on stable candidates. Durable manifests, evidence, screenshots, and
reports are generated after inputs stabilize, then regenerated only after invalidating
semantic changes.

## Reviewer findings

P0/P1 and acceptance-breaking findings block merge and are `FIXED-NOW` with focused proof.
An actionable non-blocking P2/P3 may be deferred only in one concise GitHub issue containing
the reviewer link, reviewed SHA, impact, owner, target phase, and observable close condition.
A demonstrably false, outdated, duplicate, or non-actionable finding receives one concise
evidence-backed PR disposition and no backlog issue. Do not duplicate reviewer text or create
separate finding summaries. Review only items whose target/revisit condition is now due.

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

Never mutate user-owned dirty work. Preserve verified unique human-authored input and
acceptance evidence at its assigned durable destination; Git commits/branches preserve
completed committed work. Every upstream-bound change uses a dedicated branch in the
corresponding `jodobear` fork and is recorded in
`uzel-poc-validated-pack/docs/08-upstream-contributions.md`.
