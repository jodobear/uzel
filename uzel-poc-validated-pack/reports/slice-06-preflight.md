# Slice 06 preflight — hardening and clean demo acceptance

Date: 2026-07-29

Branch: `feat/slice-06-hardening-demo`

Core implementation commits: `cb3cb51`, `cd5bf0e`, `e3f7fb3`, and `c271b93`;
evidence-led corrections through `2614341` are recorded in repository history.

## Outcome

**PASS. The Linux-only single-repository POC is accepted.**

The signed hostile child executed the complete browser probe inventory in real
Fedora WebKit. A separately control-proven loopback sentinel accepted zero
hostile connections, and raw invalid-key Tauri transport executed zero
commands. The deterministic demo passed from a detached clean checkout and the
complete locked workspace built in the immutable-digest Debian toolchain.

## Exact hostile evidence

```text
author: be1a049c1b9da66d504a808cbb1141ab37f03b1505eefa43f49894eff379c73f
aggregate: 4f69e62d242a6f0d1d13ff7721325906940491037c79fe4c2f0bd61c0f1e1022
event sha256: 56338f7191589eb490ab3a2cfa44acea4a1c09534d6f00de00bebad303869d58
index sha256: 1843ffc7ee9710c207c7097cab5d2a376bb3b94e96ed4b7872db8403c815a828
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
The independent sentinel remains the transport authority and accepted zero
hostile connections after settlement.

The raw forged Tauri message was visible with `invalid-child-key`, proving that
the negative test reached the transport. The application command counter
remained zero. The hostile report was accepted only from the mounted child's
captured source, not from payload identity fields. The normal exact NAP-SHELL
path and source-bound NAP-CONFIG delivery remained live. User mode hid the
developer drawer and exposed no unsafe fixture launcher.

Accepted real-browser marker:

```text
FEDORA_RUN_SMOKE_OK daemon=ready shell=ready exact_builds=3 nap_shell=3 shell_accepted=2 artifact=responded source_bound=multi hostile=denied sentinel=zero native=zero user_mode=hidden compositor=weston-headless-gl
```

## Commands and results

Commands ran in the pinned Nix environment unless stated otherwise.

```text
hostile JavaScript tests
  6 passed

Tauri hostile/navigation/sentinel tests
  4 passed in an isolated Fedora target

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
installed the locked frontend, built the Svelte shell, and completed
`cargo build --workspace --locked` with WebKitGTK `2.50.6-1~deb12u2`.

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
to `__inet_pton_chk@GLIBC_2.42`. The isolated Tauri test target passes all four
tests, the remaining workspace tests pass, the real Tauri application runs,
and the complete Debian workspace build passes. This is recorded as a Fedora
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
