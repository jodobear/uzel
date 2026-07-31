# Uzel

Linux-only native napplet-runtime proof of concept. The validated scope, architecture, source pins, and work slices live in [`uzel-poc-validated-pack/`](uzel-poc-validated-pack/README.md).

## Development

Enter the pinned environment and install the locked frontend dependencies:

```sh
nix --extra-experimental-features 'nix-command flakes' develop
pnpm install --frozen-lockfile
```

Start the Linux shell, private AF_UNIX daemon, and pinned exact-build fixture together:

```sh
pnpm dev
```

Run the repository checks, including the Slice 03 napplets:

```sh
pnpm check
pnpm test
pnpm test:ui
pnpm test:conformance
pnpm lint
pnpm format:check
pnpm docs:check
pnpm fallow
pnpm smoke
pnpm smoke:linux
pnpm smoke:fedora
```

`pnpm test:ui` drives the real Svelte renderer, checked-in napplet artifacts,
and trusted surface host through a deterministic mocked native boundary. It
does not claim daemon, NMP, relay, Tauri, or WebKit coverage. The exact scenario
matrix, evidence layout, and trust limits are documented in
[`apps/uzel/tests/ui/README.md`](apps/uzel/tests/ui/README.md).

Run the immutable-digest Debian build smoke from the Fedora host:

```sh
bash scripts/debian-build-smoke.sh
```

Run the real desktop/WebKit acceptance on Debian 13 using the setup, headless,
and interactive paths in [`DEBIAN13-LIVE-TEST.md`](DEBIAN13-LIVE-TEST.md).

Slices 03 through 06 add independent `follow-list` and `profile-card` napplets,
the Linux daemon boundary, and the composed two-pane shell. Tauri is a thin private-socket client; one
daemon-owned upstream `RuntimeController` owns one NMP engine and redb store.
The 96,172-byte verified fixture crosses the 524,288-byte control seam as bounded,
ordered chunks; the same ceiling includes JSON-escaping headroom for a 65,536-byte
runtime envelope. No artifact path reaches WebKit. The daemon persists only
the selected public read key and mode outside upstream-owned stores, restoring
the key through NMP's parser after restart.

The napplets use
the exact queryless `napplet:profile/open` convention through NAP-INC and
runtime-mediated identity/outbox APIs. Their signed single-file fixtures, plus
the test-only `hostile-egress` fixture, verify through the pinned upstream
runtime. The hostile fixture executes 13 browser-egress attempts in real
Fedora WebKit against a control-proven loopback sentinel; the accepted run
observes zero sentinel connections and zero native command executions. The
explicit public-live acceptance probes prove NMP canonical kind-0 selection,
direct-follow projection, HTTPS avatar delivery through NAP-RESOURCE, and then
prove cache-first restart with every relay lane disabled. The shell mounts both verified
builds through the unchanged upstream multi-surface trusted host; the daemon
routes `napplet:profile/open` to the profile surface with a runtime-owned
sender. The deterministic demo, hostile Fedora run, and complete locked Debian
build pass from a detached clean checkout. See
`uzel-poc-validated-pack/reports/slice-06-preflight.md` for the exact boundary
and toolchain limitation.

Interactive development and headless acceptance use `wss://purplepag.es` as an
operator indexer and `wss://purplepag.es` plus `wss://nos.lol` as bounded app
relay lanes. NMP owns subscriptions, reconnects, canonical replacement, cache,
and NIP-65 discovery. A plaintext loopback fallback relay is not used for
identity content: under the pinned public-demand contract it receives discovery
demand, while profile and contact content is routed through secure app lanes.
