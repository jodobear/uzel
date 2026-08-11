# Uzel agent instructions

## Active programme gate

The product-incubation v4 pack under
[`docs/plans/uzel-product-incubation-v4-2026-08-10/`](docs/plans/uzel-product-incubation-v4-2026-08-10/)
is the active planning authority. If that path is not yet present, do not resume or start
implementation; wait for its planning PR to merge.

Follow `06-START-RUNBOOK.md`. Reorient the existing GSD project with
`prompts/01-reorient-current-gsd.md`, then independently review the revised Phase 1 plans
with `prompts/02-review-phase-1.md`. Do not implement Phase 1 until that review passes.

## Brownfield POC evidence

Treat [`uzel-poc-validated-pack/`](uzel-poc-validated-pack/) as preserved brownfield
evidence, not disposable legacy and not automatic final architecture. Read its nested
`AGENTS.md`, `STATUS.md`, and one `work/*.md` slice only when the revised Phase 1 plan
explicitly assigns bounded POC replay or evidence work. Its old active-slice marker does
not override the v4 planning gate.

For codebase questions, query the existing Graphify graph first. After modifying code, run `graphify update .` and commit the refreshed graph separately.

Every upstream-bound change must use a dedicated branch in the corresponding `jodobear` fork and be recorded in [`uzel-poc-validated-pack/docs/08-upstream-contributions.md`](uzel-poc-validated-pack/docs/08-upstream-contributions.md).
