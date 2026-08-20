# Review backlog

This is the sole intake for verified non-blocking reviewer polish. Entries do not block the
active slice. Keep each entry concise; the immutable review link owns full context. Promote an
entry to a contextual GitHub issue only when scheduled, then replace it with that issue link.

| Status | Source / reviewed SHA | Concise scope | Deferral reason | Revisit trigger |
|---|---|---|---|---|
| VERIFIED-NONBLOCKING | [PR #48 CodeRabbit review](https://github.com/jodobear/uzel/pull/48#pullrequestreview-4984567444) / `5fafeefe0066130af870d551586389292235c73d` | Add upper-bound and non-numeric `--ready-fd` parser cases | Test-coverage polish; accepted range already enforced | When changing daemon option parsing |
| VERIFIED-NONBLOCKING | [PR #48 CodeRabbit review](https://github.com/jodobear/uzel/pull/48#pullrequestreview-4984567444) / `5fafeefe0066130af870d551586389292235c73d` | Replace launcher source-shape assertions with narrower semantic contracts | Maintainability only; executable lifecycle probes own behavior | When changing launcher contracts |
| VERIFIED-NONBLOCKING | [PR #48 CodeRabbit review](https://github.com/jodobear/uzel/pull/48#pullrequestreview-4984567444) / `5fafeefe0066130af870d551586389292235c73d` | Return named `SocketIdentity` from the internal helper | Internal API clarity; no behavior or authority change | When changing socket identity reporting |
| VERIFIED-NONBLOCKING | [PR #48 CodeRabbit review](https://github.com/jodobear/uzel/pull/48#pullrequestreview-4984567444) / `5fafeefe0066130af870d551586389292235c73d` | Move launcher test-hold behavior to a test-only artifact | Test seam cleanup; opt-in variable has no production authority | When changing package test passthroughs |
| VERIFIED-NONBLOCKING | [PR #48 CodeRabbit review](https://github.com/jodobear/uzel/pull/48#pullrequestreview-4984567444) / `5fafeefe0066130af870d551586389292235c73d` | Deduplicate trusted-shell digest literals in pin contracts | Maintainability only; canonical bytes and digest already agree | When changing trusted-shell pins |
| VERIFIED-NONBLOCKING | [PR #48 CodeRabbit review](https://github.com/jodobear/uzel/pull/48#pullrequestreview-4984567444) / `5fafeefe0066130af870d551586389292235c73d` | Reuse bounded `read_exact` in the compatible smoke responder | Local harness hardening; product protocol path is already bounded | When changing mismatch/compatibility probes |
| VERIFIED-NONBLOCKING | [PR #48 CodeRabbit review](https://github.com/jodobear/uzel/pull/48#pullrequestreview-4984567444) / `5fafeefe0066130af870d551586389292235c73d` | Add global abnormal-exit cleanup for all smoke helper PIDs | Harness cleanup hardening; focused failure paths already reap owned processes | When changing package-smoke process orchestration |
