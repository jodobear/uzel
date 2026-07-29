# Uzel

Linux-only native napplet-runtime proof of concept. The validated scope, architecture, source pins, and work slices live in [`uzel-poc-validated-pack/`](uzel-poc-validated-pack/README.md).

## Development

Enter the pinned environment and install the locked frontend dependencies:

```sh
nix --extra-experimental-features 'nix-command flakes' develop
pnpm install --frozen-lockfile
```

Start the Linux shell, readiness daemon, and pinned exact-build fixture together:

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

Slice 03 adds independent `follow-list` and `profile-card` napplets. They use
the exact queryless `napplet:profile/open` convention through NAP-INC and
runtime-mediated identity/outbox APIs. Their signed single-file fixtures, plus
the test-only `hostile-egress` fixture, verify through the pinned upstream
runtime. Daemon-owned runtime state, integrated NMP data, product persistence,
and the full hostile WebKit run remain later slices.
