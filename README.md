# Uzel

Linux-only native napplet-runtime proof of concept. The validated scope, architecture, source pins, and work slices live in [`uzel-poc-validated-pack/`](uzel-poc-validated-pack/README.md).

## Development

Enter the pinned environment and install the locked frontend dependencies:

```sh
nix --extra-experimental-features 'nix-command flakes' develop
pnpm install --frozen-lockfile
```

Start the Slice 02 Linux shell, readiness daemon, and pinned exact-build fixture together:

```sh
pnpm dev
```

Run the Slice 02 checks:

```sh
pnpm check
pnpm test
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

Slice 02 verifies and launches the pinned `good-morning` artifact through the
upstream `RuntimeController`, completes NAP-SHELL, and exercises the sandboxed
WebKit boundary. NMP reads, a second napplet, daemon-owned runtime state, and
product persistence remain later slices.
