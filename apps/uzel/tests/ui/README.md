# Renderer acceptance harness

Run from the repository root:

```sh
nix develop -c pnpm test:ui
```

The command starts an in-process Vite server on an ephemeral loopback port,
drives the Chromium executable pinned by `flake.lock` through the Playwright
library pinned by `pnpm-lock.yaml`, and closes both browser and server in test
teardown. The Nix shell exports the exact browser path as
`UZEL_PLAYWRIGHT_CHROMIUM`; the harness refuses to fall back to a user cache or
runtime browser download. It exercises the existing Svelte UI through
accessible names at fixed 1366×768 and 1920×1080 viewports.

Each run gets a new ignored `.artifacts/ui-acceptance/<UTC>.<pid>/` directory;
earlier evidence is never removed. `metadata.json` records the exact Git SHA,
dirty flag, browser, mode, scenario, viewport, screenshots, and owned Chromium
child teardown. Every failed case retains a screenshot, top-level DOM, frame
DOM, console/page errors, and blocked/failed network requests.

The fixture replaces only Tauri `invoke`. It mounts the checked-in profile,
follow-list, and good-morning artifact HTML through the real
`NMPTrustedShellHost`; deterministic native replies drive NAP-SHELL,
NAP-IDENTITY, NAP-INC, and NAP-OUTBOX renderer paths. The suite covers identity
submission and restart reconciliation, napplet refresh, follow-to-profile
routing, all editable bindings save/reset, naddr denial, ambiguous review and
confirmation retry, retained cleanup retry, and network denial. An isolated
deliberate-console-fault process must fail and still prove its Chromium child
was reaped.

Run one named test without changing source:

```sh
nix develop -c pnpm --filter @uzel/shell exec node --test \
  --test-concurrency=1 \
  --test-name-pattern='cleanup-failure:' \
  tests/ui/acceptance.test.mjs
```

Or select one or more stable scenario names:

```sh
UZEL_UI_SCENARIOS=review-ambiguous,cleanup-failure nix develop -c pnpm test:ui
```

## Evidence boundary

This harness proves renderer state transitions against real built napplet bytes,
accessible controls, trusted-host routing, request guarding, and cleanup of its
own processes. Native and NMP replies are mocked. It does **not** prove daemon
IPC, NMP data correctness, relay transport, exact-build verification, WebKit
behavior, or Tauri/native isolation. Those remain owned by the Rust tests and
the real Linux gate:

```sh
nix develop -c pnpm smoke:linux
```
