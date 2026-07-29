# Slice 06 preflight — hardening and clean demo acceptance

Date: 2026-07-29

Branch: `feat/slice-06-hardening-demo`

Core implementation commits: `cb3cb51`, `cd5bf0e`, `e3f7fb3`, and `c271b93`;
evidence-led corrections and exact-head review fixes are recorded in repository
history.

## Outcome

**PASS. The Linux-only single-repository POC is accepted.**

The signed hostile child executed the complete browser probe inventory in real
Fedora WebKit. A separately control-proven loopback sentinel accepted zero
hostile connections, and raw invalid-key Tauri transport executed zero
commands. The deterministic demo passed from a detached clean checkout and the
complete locked workspace built in the immutable-digest Debian toolchain.

## Exact hostile evidence

```text
author: 79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798
aggregate: d29a7660cd37118f9d619a16854617b0d44b20d16b3f6a45b9f8e28ce5187a16
event id: 8efc7cb0c99c82e115db1009e48af6c66c110e59e87c0782c85cb1786227c3f1
event sha256: c6183534dc7d46b33c722f9d1771c62ed2a41fc92cfaae07030c6b04608b8bb3
index sha256: 749d4742bde8d42a85f0719f12248203a86ebb9f7f0ace408951f08eb8e15285
```

The daemon creates an ephemeral unprivileged `127.0.0.1` listener, proves it
with one control connection, verifies and launches the exact hostile artifact,
commits the unique listener URL through the exact-principal/exact-session
NAP-CONFIG provider, and mounts only after that commit. Missing, dead,
privileged-port, or non-loopback setup fails instead of becoming a denial pass.

The child attempts 13 browser egress surfaces: fetch, XHR, WebSocket,
EventSource, image, worker, service worker, beacon, media, iframe, form,
navigation, and popup. A true `sendBeacon()` return means only that WebKit
accepted the request into a queue; it is not described as a transport denial.
The independent sentinel remains the transport authority. After settlement,
finalization signals stop, drains the listen backlog through `WouldBlock`,
joins the accept thread, rejects any recorded accept-loop error, and only then
samples the count. It accepted zero hostile connections.

The raw forged Tauri message was synchronously dispatched before the final
report and was visible with `invalid-child-key`, proving that the negative test
reached the transport before finalization. The application command counter
remained zero. The host then unmounted the child while the sentinel remained
live, covering WebKit beacon transfer deferred until document teardown, and
only afterward settled, fully drained, joined, and sampled the listener. The hostile report was
accepted only from the mounted child's captured source, not from payload
identity fields. The normal exact NAP-SHELL path and source-bound NAP-CONFIG
delivery remained live. User mode hid the developer drawer and exposed no
unsafe fixture launcher.

Accepted real-browser marker:

```text
FEDORA_RUN_SMOKE_OK daemon=ready shell=ready exact_builds=3 nap_shell=3 shell_accepted=2 artifact=responded source_bound=multi hostile=denied sentinel=zero native=zero user_mode=hidden compositor=weston-headless-gl
```

## Commands and results

Commands ran in the pinned Nix environment unless stated otherwise.

```text
hostile JavaScript tests
  7 passed, including raw-dispatch-before-report ordering

Tauri hostile/navigation/sentinel tests
  5 passed in the pinned Debian target, including accept-loop failure rejection

cargo test --workspace --exclude uzel
  napd 20 passed/1 ignored; napd-protocol 4; uzel-napd 1

pnpm check
  Svelte 0 errors/0 warnings; frontend production build; Cargo workspace check

pnpm test:conformance
  follow-list and profile-card each 6 passed, 0 failed, 4 skipped

pnpm lint
  Clippy workspace/all targets with warnings denied; BOUNDARIES_OK

pnpm format:check
  passed

pnpm fallow
  Fallow 3.9.1 total_issues 0; health findings informational

pnpm smoke
  SLICE_05_COMPOSED_DEMO_SMOKE_OK

pnpm smoke:fedora
  FEDORA_RUN_SMOKE_OK with hostile=denied, sentinel=zero, native=zero,
  source_bound=multi, and user_mode=hidden

detached clean checkout: pnpm install --frozen-lockfile; pnpm smoke;
pnpm smoke:fedora
  passed without source edits

bash scripts/debian-build-smoke.sh
  DEBIAN_BUILD_SMOKE_OK image=uzel-debian-toolchain:local source_mount=overlay
```

The Debian build used the exact `node:22.23.1-bookworm` and
`rust:1.89.0-bookworm` image digests committed in `Containerfile.debian`,
installed the locked frontend, built the Svelte shell, ran
`cargo test -p uzel --locked`, and completed `cargo build --workspace --locked`
with WebKitGTK `2.50.6-1~deb12u2`.

## Failed evidence and toolchain limit

Failed exploratory Fedora logs are preserved locally under
`reports/probes/slice-06-fedora-failed/` with the Tauri invoke key redacted.
They drove corrections to raw-IPC result ordering, task separation, settlement,
diagnostics, and the honest beacon model. They were never reclassified as
passes.

A fresh checkout first placed under `/tmp` failed with `Disk quota exceeded`;
moving the same detached checkout to the spacious workspace filesystem passed.
This is host storage policy, not a source correction.

The combined Fedora `pnpm test` link can mix Nix WebKitGTK libraries requiring
`GLIBC_2.42` with the host linker. The exact failure is an undefined reference
to `__inet_pton_chk@GLIBC_2.42`. The pinned Debian Tauri target passes all five
tests, the remaining workspace tests pass, the real Fedora Tauri application
runs, and the complete Debian workspace build passes. This is recorded as a Fedora
developer-toolchain composition issue; it is not hidden as an all-in-one test
pass.

## Bubblewrap decision

Host Bubblewrap `0.11.0` exists. Wrapping the whole Tauri/WebKit process with
`--unshare-net` would also remove the trusted top frame's required loopback
daemon and NMP relay connectivity. That does not test the accepted
architecture. A per-WebKit-child process policy is therefore the first
post-POC hardening follow-up. This POC claims a tested malicious-JavaScript
boundary, not a browser-engine exploit boundary.

## Upstream result

Kehto PR #218 is merged at
`4fd4affdd0043ea093c6b56a866f0f9f333e5375`. The nampplets portable
multi-surface contribution fork PR #1 is merged at
`570a01f8ae85c4dccd5452aa764e97d88c2f6611` after clean exact-head Codex review
and green Linux-relevant CI. Uzel deliberately keeps Rust runtime
`e539378...`; the later runtime correctly refuses this POC's plaintext local
relay. Trusted local TLS is post-POC work. Slice 06 required no new upstream
change.

## Go/no-go and exact next steps

**GO: the Linux POC is proven.** No product implementation slice remains.

1. Merge this slice after exact-head review.
2. Preserve the exact pins and hostile regression harness during extraction.
3. Decide when a second consumer justifies extracting product-neutral daemon
   seams to `kehto/napd` and portable napplets to `jodobear/napplets`.
4. Before any runtime repin, add trusted local TLS and rerun all affected gates.
5. Design per-WebKit-child OS isolation; do not weaken CSP or NAP ownership to
   simulate it.
