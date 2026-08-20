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

## Code Review Rules

GitHub Codex and CodeRabbit are the default PR reviewers. Apply these rules to the exact
PR head and report only actionable defects with priority, file/line, failure path, and
required proof. Do not restate passing tests, narrate process, or raise formatting/style
comments already enforced mechanically.

- Preserve the canonical NAPS/NIP contracts and every pinned source/digest. A PR may not
  silently reinterpret source authority, protocol identity, or compatibility evidence.
- Treat path existence, URL text after parsing, PID liveness, payload identity fields, and
  caller-provided tokens as correlation, never authority. Establish authority at the owned
  pre-normalization or post-bind boundary and bind it to exact current identity/state.
- For process/socket lifecycle changes, challenge pre-existing objects, bind races, stale
  identity, replacement, remount/restart, interruption, cleanup, and sibling isolation.
  Foreign objects must survive unchanged; cleanup may remove only proven owned resources.
- For trusted-shell/WebKit changes, preserve canonical napplet `srcdoc`,
  `sandbox="allow-scripts"` without `allow-same-origin`, strict outer and earliest inner CSP,
  exact `MessageEvent.source` and current session binding, bounded typed messages, and zero
  child Tauri/Nostr authority.
- Package/runtime evidence must resolve the exact current derivation and store output.
  Historical or caller-selected closures cannot satisfy current acceptance. Product source
  filters must include all build inputs while excluding planning, review, cache, graph, and
  preserved evidence; planning-only edits must not change the product derivation.
- Prefer deterministic tests for stable invariants. Request focused native evidence only
  where engine, packaging, lifecycle, or platform behavior cannot be established statically.
- Reviewer severity labels are advisory. A verified nitpick that does not affect correctness,
  security, accessibility acceptance, compatibility, or the selected issue never blocks the
  current slice. Record it once in `.planning/REVIEW-BACKLOG.md`; do not fix it under current
  delivery pressure or duplicate its full review text. Promote it to an issue only when
  scheduled. A mislabeled substantive defect still follows normal blocking rules.
