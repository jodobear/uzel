# Uzel agent instructions

Follow [`WORKFLOW.md`](WORKFLOW.md), the sole active delivery-process authority.

Treat [`uzel-poc-validated-pack/`](uzel-poc-validated-pack/) as preserved brownfield POC
evidence, not current workflow or automatic final architecture. Read its nested
instructions and one `work/*.md` slice only when the active issue assigns bounded replay
or evidence work.

For codebase questions, query through `$gsd-graphify query` first. Run
`$gsd-graphify build` once after a successful scoped slice, or when
`$gsd-graphify status` reports stale source. Use the canonical `.planning/graphs/`
artifacts created by the installed skill; never commit `graphify-out/` runtime noise.

Every upstream-bound change must use a dedicated branch in the corresponding `jodobear`
fork and be recorded in
[`uzel-poc-validated-pack/docs/08-upstream-contributions.md`](uzel-poc-validated-pack/docs/08-upstream-contributions.md).
