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
| Launcher to private daemon | Private XDG/AF_UNIX modes, child-origin post-bind identity receipt, versioned Hello, and exact child lifecycle |
| Top WebView to outer shell | Strict top CSP, one sealed canonical outer URL, immutable reviewed response bytes |
| Outer shell to napplet | Canonical `srcdoc`, opaque `allow-scripts` sandbox, exact Window/session/binding mapping, zero native or Nostr authority |

## Threat register

| Threat ID | Category | Component | Severity | Mitigation | Status |
|---|---|---|---|---|---|
| T-02-01 | Spoofing / Tampering | Package launcher | high | Absolute closure-private payload paths and hostile-`PATH` decoy proof | closed |
| T-02-02 | Information disclosure / Elevation | Private socket and state | high | Same-user XDG roots, `0700` parent, `0600` socket, daemon-origin bind receipt, identity match, and takeover/substitution refusal | closed |
| T-02-03 | Tampering / Denial of service | Launcher lifecycle | high | Exact INT/TERM forwarding, deterministic wait/reap, owned-socket retirement, bounded concurrency probes | closed |
| T-02-04 | Tampering | Locked package inputs | high | Exact Nix/Cargo/pnpm pins, immutable trusted-shell bytes/digest, closure/source and asset checks | closed |
| T-02-05 | Elevation / Spoofing | WebView trust boundary | high | Strict CSP, sealed outer identity, canonical inner `srcdoc`, sandbox, exact source/session binding, fail-closed teardown | closed |

## Accepted risks

No accepted risks.

## Audit trail

| Date | Scope | Verdict |
|---|---|---|
| 2026-08-20 | Existing exact-head Candidate-B and launcher security review at `66ba4fc7` / tree `a37558be` | Meadow SECURITY CLEAN; 5 threats closed, 0 open |
| 2026-08-20 | Supply-chain and launcher correction review at `f6de2985` / tree `4da72e00` | Meadow SECURITY CLEAN; exact current output binding, filtered package source, and pre-existing path refusal verified |
| 2026-08-20 | Daemon-origin readiness delta at `a28f331b70beb8b862ef2505ff75416e35884340` / tree `61b7c3c5c113ef255a32a5adec40af4ac0cde6ed` | Meadow SECURITY CLEAN; post-check substitution refused; runtime evidence sufficient without full WebKit repeat |

## Sign-off

- [x] All threats have a mitigation disposition.
- [x] No accepted risk is required.
- [x] `threats_open: 0` confirmed.
- [x] Frozen reviewed trusted-shell digest remains
  `a3e6c18e8724329332bd15a039282a8a0bcf5ec93577b97752f46721df80fba3`.

**Approval:** verified 2026-08-20 at exact implementation head `a28f331b70beb8b862ef2505ff75416e35884340`,
tree `61b7c3c5c113ef255a32a5adec40af4ac0cde6ed`, and output
`/nix/store/3bknmlmyq5ipjdw44cxjjzvlj11jssvh-uzel-0.0.0`. Packaged shell SHA-256 remains
`9b8c16fe4b0193ce8bb7e166792e39a0d5202aeff6dddbe8b9ab472d25c6ec8d`; packaged daemon
SHA-256 is `b2d292229f8692f0b6a2676b33bc6e2f5ab5982ecd0746425979db3767c93c3f` after the reviewed
bind-readiness change. This records-only update changes no security surface; any later
security-surface change requires fresh Meadow review.
