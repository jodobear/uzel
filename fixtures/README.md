# Fixtures

Work 02 preserves the published `good-morning` artifact unchanged. Work 03
adds three local, signed, single-file NIP-5D artifacts:

| source | manifest `d` tag | purpose |
|---|---|---|
| `napplets/follow-list` | `follow-list` | direct-follow selector |
| `napplets/profile-card` | `profile-card` | latest-known kind `0` projection |
| `napplets/hostile-egress` | `egress-probe` | test-only browser/native denial probe |

Generate Work 03 fixtures only with a disposable test key:

```sh
VITE_DEV_PRIVKEY_HEX="$(openssl rand -hex 32)" \
  nix --extra-experimental-features 'nix-command flakes' develop --command \
    pnpm fixtures:build
```

The script uses pinned `@napplet/vite-plugin@0.12.0` for single-file builds and
released `@napplet/cli@0.0.0` for final source-bearing manifests and signing.
The private key is neither printed nor written. The committed public events,
artifact bytes, and their SHA-256 pins are test data. Regeneration intentionally
changes publisher/event identity and requires updating runtime and asset pins.

`https://blossom.invalid/` is a reserved, non-routable source hint for the local
fixture adapter. It satisfies the runtime's requirement for a signed,
policy-approved candidate without adding live Blossom or network behavior.
