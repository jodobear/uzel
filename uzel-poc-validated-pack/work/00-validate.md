# Work 00 — validate assumptions

## Goal

Turn the POC from a design hypothesis into an executable plan. No product implementation beyond disposable probes.

## Read

- `docs/00-scope.md`
- `docs/01-validation.md`
- `docs/02-architecture.md`
- `docs/07-source-baseline.md`

## Tasks

1. Verify Kehto #204 merge state and pin the merge commit.
2. Pin compatible Napplet packages/spec revisions and run relevant conformance fixtures.
3. Inspect `nampplets` source and produce a Linux reuse map:
   - generic crates;
   - Apple-only edges;
   - exact-build install/session APIs;
   - shell bridge assets;
   - NMP adapter and storage seams.
4. Compile the reusable Rust workspace on Fedora/Debian or record exact failures.
5. Build a minimal NMP Rust probe for fixture kind `0`, direct follows, freshness/evidence, cancellation, and shutdown.
6. Build a minimal Tauri/WebKit probe proving:
   - sandboxed self-contained frame runs;
   - `MessageEvent.source` binding;
   - child lacks Tauri bridge;
   - tested direct egress paths fail under strict policy.
7. Verify Nix, Rust, Node/package manager, Tauri, Fallow, and documentation tooling.
8. Correct `docs/02-architecture.md` or `docs/03-provisional-design.md` where evidence disagrees.

## Outputs

```text
compatibility.lock
reports/preflight.md
reports/nampplets-linux-map.md
reports/nmp-api-map.md
reports/webkit-trust-spike.md
docs/facts/FACT-*.md
```

## Done when

- every V-01 through V-08 gate has evidence and a decision;
- later work files reference exact pins/APIs rather than guesses;
- no material contradiction remains hidden in an “LLDD”.

## Stop conditions

Stop product work if #204 is not merged, exact-build identity cannot be retained, `nampplets` reuse would require a parallel runtime, NMP would be bypassed, or the frame requires raw network/Tauri authority.
