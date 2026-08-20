---
phase: 02-canonical-nix-release
slug: canonical-nix-release
status: verified
threats_open: 0
asvs_level: 1
created: 2026-08-20
---

# Phase 2: Canonical Nix release security

## Trust boundaries

| Boundary | Security invariant |
|---|---|
| Nix closure to launcher | One public launcher selects only immutable closure-private shell and daemon payloads |
| Launcher to private daemon | Same-user XDG ownership, private AF_UNIX modes, versioned Hello, and exact child lifecycle |
| Top WebView to outer shell | Strict top CSP, one sealed canonical outer URL, immutable reviewed response bytes |
| Outer shell to napplet | Canonical `srcdoc`, opaque `allow-scripts` sandbox, exact Window/session/binding mapping, zero native or Nostr authority |

## Threat register

| Threat ID | Category | Component | Severity | Mitigation | Status |
|---|---|---|---|---|---|
| T-02-01 | Spoofing / Tampering | Package launcher | high | Absolute closure-private payload paths and hostile-`PATH` decoy proof | closed |
| T-02-02 | Information disclosure / Elevation | Private socket and state | high | Same-user XDG roots, `0700` parent, `0600` socket, identity-matched ownership and takeover refusal | closed |
| T-02-03 | Tampering / Denial of service | Launcher lifecycle | high | Exact INT/TERM forwarding, deterministic wait/reap, owned-socket retirement, bounded concurrency probes | closed |
| T-02-04 | Tampering | Locked package inputs | high | Exact Nix/Cargo/pnpm pins, immutable trusted-shell bytes/digest, closure/source and asset checks | closed |
| T-02-05 | Elevation / Spoofing | WebView trust boundary | high | Strict CSP, sealed outer identity, canonical inner `srcdoc`, sandbox, exact source/session binding, fail-closed teardown | closed |

## Accepted risks

No accepted risks.

## Audit trail

| Date | Scope | Verdict |
|---|---|---|
| 2026-08-20 | Existing exact-head Candidate-B and launcher security review at `66ba4fc7` / tree `a37558be` | Meadow SECURITY CLEAN; 5 threats closed, 0 open |

## Sign-off

- [x] All threats have a mitigation disposition.
- [x] No accepted risk is required.
- [x] `threats_open: 0` confirmed.
- [x] Frozen reviewed trusted-shell digest remains
  `a3e6c18e8724329332bd15a039282a8a0bcf5ec93577b97752f46721df80fba3`.

**Approval:** verified 2026-08-20 from the existing exact-head Meadow verdict. This
documentation-only record changes no security surface; any later security-surface change
requires a fresh Meadow review.
