# External napplet interoperability corpus

This directory freezes four externally produced napplet coordinates and the
exact signed kind `35129` events resolved during the issue #21 audit. It is
data-only input for later interoperability tests. It is not a product catalog,
runtime fixture registration, grant policy, or source of NAP/NMP authority.

## Provenance

- Source repository: `https://github.com/hzrd149/napplelets`
- Audited source commit: `aa4dc7a0799d95e3066b50055b29685d6e376045`
- Publisher: `266815e0c9210dfa324c6cba3573b14bee49da4209a9456f9484e5106cd408a5`
- Event publication time: `2026-07-26T11:52:04Z`
- Source license: MIT at the exact audited commit

The source commit followed the publication by 22 seconds. That is temporal
association, not source-to-artifact attestation or a reproducible-build claim.
The MIT repository license also does not settle every embedded third-party
asset's redistribution terms. No upstream source or artifact HTML is copied
here; only signed events and audit facts are retained.

The signed events list `cdn.hzrd149.com` and `blossom.ditto.pub`. The audit
downloaded every artifact from the CDN and confirmed the signed path SHA-256.
It did not independently fetch every artifact from the second server.

Good Morning is intentionally a separate external-corpus record. Uzel's
existing `fixtures/good-morning` event is older, although both records name the
same exact artifact path and aggregate hashes.

## Offline verification

Run inside the pinned Nix development shell:

```sh
bash scripts/verify-external-napplet-corpus.sh
node --test scripts/verify-external-napplet-corpus.test.mjs
bash scripts/verify-external-napplet-corpus.test.sh
```

The verifier makes no network calls. It checks the immutable lock, signed event
tuples, exact NIP-19 coordinates, event hashes, and Schnorr signatures. It
requires the flake-pinned `nak 0.20.1`; another PATH version is an infrastructure
failure, not accepted evidence. `nak` remains the owner of Nostr decoding and
cryptographic verification; this code does not reimplement either.

The lock and signed events are opened through protected no-follow,
nonblocking descriptors. The verifier checks each opened descriptor's canonical
contained path and regular-file type, then performs a fixed capped read: each
evidence file must be at most 16,384 bytes, including if it grows after its
initial descriptor stat.

The shell accepts a successful Node snapshot only when its exact interface
schema contains the four audited entries in their fixed order and the copied
lock entries match them exactly. Node verification, `nak --version`, and each
`nak decode` stream stdout and stderr into kernel-limited files; either stream
is rejected above 65,536 bytes before its content enters a shell variable.

Artifact blobs are deliberately absent. Therefore offline verification proves
the retained signed evidence and coordinate mapping, not current relay/CDN
availability or a new source-to-binary build. Later live tests must fetch through
the native napd/NMP boundary and verify the exact byte length, path digest, and
aggregate before launch.

## Failure meaning

- Relay or artifact server unavailable is an **infrastructure** result. Preserve
  it separately; do not call it trust drift.
- Invalid signature, coordinate/event replacement, path, byte-length, or
  aggregate mismatch, or audited capability change is a **trust** result. Fail
  closed until an explicit refresh PR explains and accepts the new tuple.

The verifier exits `2` for trust failures and `3` for missing, wrong-version,
crashed, or malformed-output verifier infrastructure. Node trust diagnostics
cross the shell boundary as a schema-checked JSON result so hostile control
characters stay encoded. Oversized subprocess output is also an infrastructure
failure. It never turns either category into success.

## Deliberate refresh

Do not automatically rewrite this lock from a replaceable `naddr`.

1. Clone the upstream repository at the proposed exact commit. Recheck its
   license, source manifests, package lock, build commands, and relevant code.
2. Resolve each author/kind/`d` coordinate from more than one relay with pinned
   `nak`. Save the candidate event outside this directory first.
3. Run `nak verify` and `nak decode` offline. Confirm author, kind `35129`, `d`,
   event ID, relay hints, signed servers, path tuple, and aggregate tuple.
4. Fetch `/index.html` only through a trusted native probe from the event's
   signed HTTPS servers. Do not allow redirects. Check exact length and SHA-256.
5. Use pinned NIP-5D/NMP tooling to validate the aggregate and extract the
   declared domains. Do not duplicate its aggregate or capability rules here.
6. Compare source and artifact behavior, conformance results, and licenses.
   Record honestly when reproducible source-to-artifact linkage remains absent.
7. Update the event record, lock tuple, exact-value tests, audit date, and source
   commit in one focused PR. A relay/CDN outage alone is not permission to
   refresh anything.
8. Run the offline verifier, repository tests, documentation audit, and real
   Linux acceptance required by the integration issue before merging.

Kehto/Paja behavior is not protocol authority: deriving grants from `requires`
is a compatibility behavior, and its development `iframe.src`/HMR path is not
production isolation evidence. NAP behavior stays owned by `napplet/naps`; NMP
stays the Nostr data-plane owner.
