# Phase 1: POC replay and runtime recovery - Pattern Map

**Mapped:** 2026-08-13
**Reoriented:** 2026-08-14

## File Classification

| File | Role | Closest existing pattern |
|---|---|---|
| `evidence/phase-01/poc-replay.md` | exact observation record | `scripts/linux-run-smoke.sh` markers |
| `evidence/phase-01/ownership-map.md` | source-grounded ownership record | Cargo imports/locks and upstream ledger |
| `crates/napd/src/runner.rs` | private runtime and restart replay | existing temp-state reopen tests |
| `scripts/linux-run-smoke.sh` | real Weston/WebKit evidence | existing native smoke and hostile markers |

## Required Patterns

### Exact source binding

Reuse the current fixture verification, trusted surface token, session lookup, authority denial, and hostile probe paths. Bind claims to current commit, source path, exact fixture digest, and emitted marker. Do not add a second runtime or trust path.

### Restart recovery

Use a temporary state root, install and launch exact fixtures, create an observable ambiguous lifecycle state, drop the runner, reopen the same root, then assert identity, cached profile/follows, installed exact build, ambiguity reconciliation, and a fresh surface generation. Extend the current focused runner tests.

### Native WebKit recovery

Extend the existing `scripts/linux-run-smoke.sh` Weston/WebKit surface. Induce one real recoverable failure/restart, require post-recovery shell, source-binding, artifact-response, and hostile-denial markers, and preserve redacted logs on failure. Chromium/mock results cannot satisfy this gate.

### Ownership

Derive active upstreams from `Cargo.toml` and `Cargo.lock`: nampplets supplies native runtime crates; NMP supplies Nostr behavior. Uzel owns its private daemon, product lifecycle, trusted surfaces, composition, and recovery policy. Do not name an upstream without current source or a concrete reusable fix.

### Fail closed

Unavailable native access, malformed state, missing sessions, unmatched identities, missing recovery markers, or denied capabilities return nonzero and stay pending in evidence. Never infer a pass from historic status text.

### Evidence isolation

Use per-run temporary paths, cleanup traps, protected-value redaction, and explicit owner/revisit triggers. Preserve protected worktrees and refs in place until human disposition authority exists.

## No New Analog Needed

No new framework, validator, evidence format, receipt system, daemon protocol, or Graphify artifact is required. Existing runner, UI, smoke, Markdown, and ownership surfaces are sufficient.
