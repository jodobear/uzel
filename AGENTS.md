# Uzel agent instructions

Follow [`WORKFLOW.md`](WORKFLOW.md), the sole active delivery-process authority.

Treat [`uzel-poc-validated-pack/`](uzel-poc-validated-pack/) as preserved brownfield POC
evidence, not current workflow or automatic final architecture. Read its nested
instructions and one `work/*.md` slice only when the active issue assigns bounded replay
or evidence work.

Graphify is an advisory, disposable local navigation cache. Use `$gsd-graphify query`
when helpful; build on demand when the local graph is missing or stale, at most once
after a stable scoped slice. Git source, tests, evidence, requirements, and decisions
override graph output. Graph absence, age, provenance, labels, or refresh failure never
blocks execution, review, or merge. Never commit `.planning/graphs/` or `graphify-out/`.

Every upstream-bound change must use a dedicated branch in the corresponding `jodobear`
fork and be recorded in
[`uzel-poc-validated-pack/docs/08-upstream-contributions.md`](uzel-poc-validated-pack/docs/08-upstream-contributions.md).
