# Debian 13 live test

Use an x86_64 Debian 13 desktop or VM with systemd, internet access, at least
8 GiB RAM, and roughly 15 GiB free disk. Headless mode also works over SSH.
The scripts use Debian's Nix package only as the launcher; `flake.lock` supplies
the exact Node, Rust, Tauri, WebKitGTK, Weston, Mesa, `nak`, and pnpm toolchain.

## 1. Clone and install the launcher

Run as the normal desktop user:

```sh
git clone git@github.com:jodobear/uzel.git
cd uzel
bash scripts/debian13-setup.sh --install
```

The setup script inventories every system prerequisite, including the systemd
Nix daemon unit, socket file, boot enablement, Debian's `nixbld` group and ten
package-owned build users, and an actual daemon-store connection. It prints the
exact apt/group/builder/daemon change plan and asks `[y/N]`
before using `sudo`. It installs only missing Debian packages and enables or
rebinds `nix-daemon.socket` only when its readiness checks fail. Rebinding uses
an explicit daemon/socket stop followed by a socket start, then waits up to five
seconds for the filesystem endpoint and proves a client connection. If the current
shell does not yet carry the configured `nix-users` group, the live-test script
re-executes itself through Debian's `newgrp` and continues without a logout or
reboot.

The socket directory is intentionally `0770 root:nix-users`. Before group
re-exec, setup reports its pathname as `socket=restricted` instead of treating
an unprivileged `test -S` false result as proof that the socket is missing.
Systemd state proves system readiness; an actual daemon-store connection after
group activation proves client readiness.

Debian's `nix-setup-systemd` post-install script owns the `nixbld` group and
`nixbld1` through `nixbld10` system users. If any are absent, setup asks before
running the package-owned, idempotent `dpkg-reconfigure nix-setup-systemd`
repair and rebinding the daemon. It does not invent or directly manage those
accounts.

Expected terminal marker:

```text
DEBIAN13_SETUP_OK os=debian-13 arch=x86_64 nix=ready flake=locked
```

`--check` makes no system changes. `--install` asks before any system change.
Use `--install --yes` only for an explicitly approved unattended installation.
Once ready, repeating `--install` runs no apt update, package install, group
change, or systemd mutation. A configured but inactive group is process-local
state; it does not cause another install or a logout loop.

## 2. Run automated real-WebKit acceptance

```sh
bash scripts/debian13-live-test.sh headless
```

This command also runs the setup inventory. If system dependencies are missing,
it shows and asks approval for the exact change plan. After Nix is ready, it
uses a non-mutating Nix dry run to show missing locked closure paths and asks
approval before fetching locked flake inputs or realizing the closure. Inside
the locked shell, network-denied pnpm and Cargo probes separately verify their
package caches. Missing package cache entries produce a second exact plan and
approval prompt before either package manager may use the network. When both
caches are complete, the actual install and build remain forced offline. Use
`--yes` only for an explicitly approved unattended run.

First run downloads the locked Nix closure and builds Rust dependencies. Later
runs reuse the Nix and Cargo stores. The low-disk Cargo profile used by the
workspace prebuild is exported into the dev runtime so Cargo does not discard
that cache and rebuild every dependency with a different profile.

The test starts headless Weston, the local signed Nostr fixture relay,
`uzel-napd`, Tauri, and real WebKitGTK. Dependency installation, workspace
prebuild, and runtime startup share one bounded ten-minute deadline. Each
prebuild command receives only the remaining budget. After `UZEL_SHELL_READY`,
runtime acceptance has a separate two-minute deadline. A timeout reports every
required runtime marker as present or missing before preserving logs. The test
verifies three exact builds, both product napplets, NAP-SHELL/NAP-INC routing,
all 13 hostile browser-egress denials, zero sentinel accepts, zero native calls,
source binding, and clean user mode.

Pass ends with:

```text
DEBIAN13_LIVE_SMOKE_OK daemon=ready shell=ready exact_builds=3 nap_shell=3 shell_accepted=2 artifact=responded source_bound=multi hostile=denied sentinel=zero native=zero user_mode=hidden compositor=weston-headless-gl
DEBIAN13_EVIDENCE_OK path=.../.artifacts/debian13-live/<UTC timestamp>
```

Keep `environment.txt`, `prebuild.log`, `run.log`, `uzel.log`, and `weston.log`
from that evidence directory. The directory is created before dependency and
workspace prebuild, so a prebuild timeout retains its log. On runtime failure,
redacted WebKit/Uzel and Weston logs are stored beneath its `failure/`
directory. Each run creates a unique directory and never overwrites earlier
evidence. Interrupting the outer live-test PID forwards the signal to the active
prebuild or smoke child, waits for cleanup, and exits nonzero.

## 3. Run visible desktop demo

From a Debian graphical terminal:

```sh
bash scripts/debian13-live-test.sh interactive
```

Manual checklist:

1. Uzel window opens and reaches `NAP-SHELL 2/2 READY`.
2. Both exact-build hashes appear beneath the two panes.
3. Click `Use identity` with the prefilled public fixture key.
4. Follow pane shows two direct follows.
5. Select a follow; profile pane updates through `napplet:profile/open`.
6. Toggle side-by-side/stacked, pane focus, resize, and fullscreen controls.
7. Open `Developer`; confirm one NMP runtime, exact sessions, relay evidence,
   and bounded envelope diagnostics.
8. Close the Uzel window. Script stops private daemon and fixture relay.

This interactive lane uses only committed signed fixtures and a loopback relay;
it does not require or contact a public Nostr relay.
