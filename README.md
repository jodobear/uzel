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
pnpm test:conformance
pnpm lint
pnpm format:check
pnpm docs:check
pnpm fallow
pnpm smoke
pnpm smoke:fedora
```

Run the immutable-digest Debian build smoke from the Fedora host:

```sh
bash scripts/debian-build-smoke.sh
```

Slices 03 and 04 add independent `follow-list` and `profile-card` napplets plus
the Linux daemon boundary. Tauri is now a thin private-socket client; one
daemon-owned upstream `RuntimeController` owns one NMP engine and redb store.
The 96,172-byte verified fixture crosses the 4,096-byte control seam as bounded,
ordered chunks and no artifact path reaches WebKit. The daemon persists only
the selected public read key and mode outside upstream-owned stores, restoring
the key through NMP's parser after restart.

The napplets use
the exact queryless `napplet:profile/open` convention through NAP-INC and
runtime-mediated identity/outbox APIs. Their signed single-file fixtures, plus
the test-only `hostile-egress` fixture, verify through the pinned upstream
runtime. The explicit live smoke starts `nak serve`, proves NMP canonical
kind-0 selection and direct-follow projection, then stops the relay and proves
cache-first restart from the same NMP store. The integrated two-pane demo and
full hostile WebKit run remain Slices 05 and 06.
