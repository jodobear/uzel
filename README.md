# Uzel

Linux-only native napplet-runtime proof of concept. The validated scope, architecture, source pins, and work slices live in [`uzel-poc-validated-pack/`](uzel-poc-validated-pack/README.md).

## Development

Enter the pinned environment and install the locked frontend dependencies:

```sh
nix --extra-experimental-features 'nix-command flakes' develop
pnpm install --frozen-lockfile
```

Start the empty Tauri shell and daemon together:

```sh
pnpm dev
```

Run the Slice 01 checks:

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

This scaffold contains no runtime session, NMP query, napplet, persistence schema, or product UI. Those belong to later slices.
