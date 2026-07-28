# FACT-002 — NAP and NIP revision status

- **Claim:** The package/runtime contracts can be identified by released, active specifications.
- **Classification:** verified fact
- **Exact source/pin:** `napplet/naps@5ac0490461ca6fec2f0d2e45b4835cf9bc08de24`; `nostr-protocol/nips@6d2979b3f503a8539c983efbcdcf901bbcf9ed23`; exact PR heads and merge commits in [`../../compatibility.lock`](../../compatibility.lock).
- **Probe/command:** GitHub PR API plus direct inspection of the pinned registry and specification documents.
- **Observed result:** The NAP registry calls shell/intent/inc/theme Active and relay/identity/storage Draft, while NAP-SHELL.md and NAP-INC.md still label themselves draft. Relay PR #2 and storage PR #3 remain open. NIP-5D PR #2303 is open and its document is draft. NIP-5A defines nsite kinds 5128/15128/35128, not napplet manifests; NIP-5D proposes napplet kinds 5129/15129/35129.
- **Decision:** Treat these as exact provisional revisions, not released stable contracts. NIP-5D supplies the hostile-frame baseline, but the POC must not claim NIP-5A compatibility or spec ratification.
- **Affected documents/code:** `compatibility.lock`, `docs/01-validation.md`, `docs/03-provisional-design.md`, `docs/07-source-baseline.md`, `work/03-napplets.md`.
- **Revalidate when:** Any listed PR merges, the NAP registry/status headers converge, or NIP-5D changes identity, manifest, iframe, or CSP rules.
