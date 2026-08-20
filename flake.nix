{
  description = "Uzel exact-pinned Linux package and development shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/38a4887411571457d700c51c64a6e49ead2ed5ab";

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      lib = pkgs.lib;
      source = lib.fileset.toSource {
        root = ./.;
        fileset = lib.fileset.unions [
          ./Cargo.lock
          ./Cargo.toml
          ./package.json
          ./pnpm-lock.yaml
          ./pnpm-workspace.yaml
          ./rust-toolchain.toml
          ./apps
          ./crates
          ./fixtures
          ./napplets
        ];
      };
      pnpmDeps = pkgs.fetchPnpmDeps {
        pname = "uzel";
        version = "0.0.0";
        src = source;
        pnpm = pkgs.pnpm_10;
        fetcherVersion = 3;
        hash = "sha256-sY7QJprBZGiQ3LXna5RA+sZLAG7UuC5Z1F3x8tmMC4A=";
      };
      uzel = pkgs.rustPlatform.buildRustPackage {
        pname = "uzel";
        version = "0.0.0";
        src = source;
        cargoLock = {
          lockFile = ./Cargo.lock;
          outputHashes = {
            "nmp-0.1.0" = "sha256-v0/GYb11zNBhxsBTCuKzsJ1KuRthS21+q+b8fN/iY3A=";
            "nmp-engine-0.1.0" = "sha256-v0/GYb11zNBhxsBTCuKzsJ1KuRthS21+q+b8fN/iY3A=";
            "nmp-executor-0.1.0" = "sha256-v0/GYb11zNBhxsBTCuKzsJ1KuRthS21+q+b8fN/iY3A=";
            "nmp-grammar-0.1.0" = "sha256-v0/GYb11zNBhxsBTCuKzsJ1KuRthS21+q+b8fN/iY3A=";
            "nmp-resolver-0.1.0" = "sha256-v0/GYb11zNBhxsBTCuKzsJ1KuRthS21+q+b8fN/iY3A=";
            "nmp-router-0.1.0" = "sha256-v0/GYb11zNBhxsBTCuKzsJ1KuRthS21+q+b8fN/iY3A=";
            "nmp-signer-0.1.0" = "sha256-v0/GYb11zNBhxsBTCuKzsJ1KuRthS21+q+b8fN/iY3A=";
            "nmp-store-0.1.0" = "sha256-v0/GYb11zNBhxsBTCuKzsJ1KuRthS21+q+b8fN/iY3A=";
            "nmp-transport-0.1.0" = "sha256-v0/GYb11zNBhxsBTCuKzsJ1KuRthS21+q+b8fN/iY3A=";
            "nmp-native-artifact-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-catalog-resolver-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-nap-bridge-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-nmp-adapter-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-provider-identity-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-provider-inc-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-provider-link-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-provider-resource-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-providers-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-runtime-app-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-runtime-core-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-runtime-ffi-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-runtime-store-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
            "nmp-native-surface-0.1.0" = "sha256-hRSxPSHBgcxUXjcX5nWvZIIEMZP9urh5HUIAG1LWZts=";
          };
        };
        inherit pnpmDeps;

        nativeBuildInputs = [
          pkgs.pkg-config
            pkgs.nodejs_22
            pkgs.pnpmConfigHook
            pkgs.pnpm_10
        ];
        buildInputs = with pkgs; [
          gtk3
          libayatana-appindicator
          librsvg
          openssl
          webkitgtk_4_1
        ];

        # The shell's frontend is a package input, not a dev-server dependency.
        buildPhase = ''
          runHook preBuild
          pnpm --offline --frozen-lockfile --filter @uzel/shell build
          cargo build --locked --release --package uzel --package uzel-napd
          runHook postBuild
        '';

        doCheck = false;
        installPhase = ''
          runHook preInstall
          install -Dm755 target/release/uzel "$out/libexec/uzel-shell"
          install -Dm755 target/release/uzel-napd "$out/libexec/uzel-napd"
          install -Dm644 apps/uzel/src-tauri/icons/icon.png \
            "$out/share/icons/hicolor/512x512/apps/uzel.png"
          install -Dm644 -T /dev/stdin "$out/share/applications/uzel.desktop" <<'EOF'
          [Desktop Entry]
          Type=Application
          Name=Uzel
          Exec=uzel
          Icon=uzel
          Categories=Network;
          Terminal=false
          EOF
          install -Dm755 -T /dev/stdin "$out/bin/uzel" <<EOF
          #!${pkgs.runtimeShell}
          set -eu

          daemon="$out/libexec/uzel-napd"
          shell="$out/libexec/uzel-shell"
          daemon_pid=
          shell_pid=
          received_signal=
          owns_lock=
          owns_socket=
          socket_identity=
          runtime_dir=\''${XDG_RUNTIME_DIR:?XDG_RUNTIME_DIR is required}
          ready_fifo="\$runtime_dir/uzel-launcher-ready.\$\$"
          socket="\$runtime_dir/uzel/napd.sock"
          lock_file="\$runtime_dir/uzel-launcher.lock"

          umask 077
          exec 9>"\$lock_file"
          ${pkgs.util-linux}/bin/flock -n 9 || {
            echo 'another Uzel launcher owns this runtime directory' >&2
            exit 1
          }
          owns_lock=1
          if [ -e "\$socket" ] || [ -L "\$socket" ]; then
            echo 'Uzel launcher refuses a pre-existing runtime socket' >&2
            exit 1
          fi
          ${pkgs.coreutils}/bin/mkfifo -m 600 "\$ready_fifo"
          exec 8<>"\$ready_fifo"
          exec 7<"\$ready_fifo"
          ${pkgs.coreutils}/bin/rm -f -- "\$ready_fifo"

          signal_children() {
            signal=\$1
            if [ -n "\$shell_pid" ]; then
              kill -s "\$signal" "\$shell_pid" 2>/dev/null || true
            fi
            if [ -n "\$daemon_pid" ]; then
              kill -s "\$signal" "\$daemon_pid" 2>/dev/null || true
            fi
          }

          stop_child() {
            child_pid=\$1
            initial_signal=\$2
            [ -n "\$child_pid" ] || return 0
            if [ -n "\$initial_signal" ]; then
              kill -s "\$initial_signal" "\$child_pid" 2>/dev/null || true
            fi
            attempt=0
            child_state=
            while kill -0 "\$child_pid" 2>/dev/null && [ "\$attempt" -lt 50 ]; do
              if [ -r "/proc/\$child_pid/stat" ]; then
                read -r _ _ child_state _ < "/proc/\$child_pid/stat" || true
                [ "\$child_state" = Z ] && break
              fi
              ${pkgs.coreutils}/bin/sleep 0.1
              attempt=\$((attempt + 1))
            done
            if kill -0 "\$child_pid" 2>/dev/null && [ "\$child_state" != Z ]; then
              kill -KILL "\$child_pid" 2>/dev/null || true
            fi
            wait "\$child_pid" 2>/dev/null || true
          }

          handle_signal() {
            received_signal=\$1
            trap - INT TERM
            signal_children "\$received_signal"
            case "\$received_signal" in
              INT) exit 130 ;;
              TERM) exit 143 ;;
            esac
          }

          cleanup() {
            status=\$?
            trap - EXIT INT TERM
            exec 7<&- 8>&- 2>/dev/null || true
            cleanup_signal=
            [ -n "\$received_signal" ] || cleanup_signal=TERM
            stop_child "\$shell_pid" "\$cleanup_signal"
            stop_child "\$daemon_pid" "\$cleanup_signal"
            shell_pid=
            daemon_pid=
            current_socket_identity=
            if [ -S "\$socket" ]; then
              current_socket_identity=\$(${pkgs.coreutils}/bin/stat -Lc '%d:%i' "\$socket")
            fi
            if [ -n "\$owns_lock" ] && [ -n "\$owns_socket" ] && \
                [ -n "\$socket_identity" ] && \
                [ "\$current_socket_identity" = "\$socket_identity" ]; then
              ${pkgs.coreutils}/bin/rm -f -- "\$socket"
            fi
            exit "\$status"
          }
          trap cleanup EXIT
          trap 'handle_signal INT' INT
          trap 'handle_signal TERM' TERM

          set -m
          "\$daemon" --live \
            --ready-fd 8 \
            --indexer-relay wss://purplepag.es \
            --app-relay wss://purplepag.es \
            --app-relay wss://nos.lol &
          daemon_pid=\$!
          set +m
          exec 8>&-
          ready_identity=
          IFS= read -r -t 8 ready_identity <&7 || exit 1
          exec 7<&-
          [ -S "\$socket" ] || exit 1
          kill -0 "\$daemon_pid" 2>/dev/null || exit 1
          socket_identity=\$(${pkgs.coreutils}/bin/stat -Lc '%d:%i' "\$socket")
          [ "\$ready_identity" = "UZEL_NAPD_BOUND \$socket_identity" ] || exit 1
          owns_socket=1
          set -m
          if [ -n "\''${UZEL_LAUNCHER_TEST_HOLD_SECONDS:-}" ]; then
            case "\''${UZEL_LAUNCHER_TEST_HOLD_SECONDS}" in
              *[!0-9]*|"") exit 2 ;;
            esac
            ${pkgs.coreutils}/bin/sleep "\''${UZEL_LAUNCHER_TEST_HOLD_SECONDS}" &
          else
            "\$shell" "\$@" &
          fi
          shell_pid=\$!
          set +m
          set +e
          completed_pid=
          wait -n -p completed_pid "\$shell_pid" "\$daemon_pid"
          completed_status=\$?
          set -e
          if [ "\$completed_pid" = "\$shell_pid" ]; then
            shell_pid=
            stop_child "\$daemon_pid" TERM
            daemon_pid=
            exit "\$completed_status"
          fi
          if [ "\$completed_pid" = "\$daemon_pid" ]; then
            daemon_pid=
            stop_child "\$shell_pid" TERM
            shell_pid=
            [ "\$completed_status" -ne 0 ] || completed_status=1
            exit "\$completed_status"
          fi
          exit 1
          EOF
          runHook postInstall
        '';
      };
    in
    {
      packages.${system} = {
        inherit uzel;
        default = uzel;
      };
      apps.${system} = {
        uzel = {
          type = "app";
          program = "${uzel}/bin/uzel";
        };
        default = {
          type = "app";
          program = "${uzel}/bin/uzel";
        };
      };
      checks.${system}.package = pkgs.runCommand "uzel-package-check" {
        nativeBuildInputs = [ pkgs.coreutils ];
      } ''
        test -x ${uzel}/bin/uzel
        test -x ${uzel}/libexec/uzel-shell
        test -x ${uzel}/libexec/uzel-napd
        test -f ${uzel}/share/applications/uzel.desktop
        test -f ${uzel}/share/icons/hicolor/512x512/apps/uzel.png
        test "$(find ${uzel}/bin -mindepth 1 -maxdepth 1 | wc -l)" -eq 1
        test -f ${uzel}/bin/uzel
        test ! -L ${uzel}/bin/uzel
        touch "$out"
      '';

      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo-tauri
          chromium
          corepack_22
          curl
          deno
          gtk3
          jq
          libayatana-appindicator
          librsvg
          mesa
          mermaid-cli
          nak
          nodejs_22
          openssl
          pkg-config
          python3
          ripgrep
          rustup
          util-linux
          webkitgtk_4_1
          weston
        ];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
          gtk3
          libayatana-appindicator
          librsvg
          webkitgtk_4_1
        ]);
        LIBGL_ALWAYS_SOFTWARE = "1";
        LIBGL_DRIVERS_PATH = "${pkgs.mesa}/lib/dri";
        __EGL_VENDOR_LIBRARY_FILENAMES = "${pkgs.mesa}/share/glvnd/egl_vendor.d/50_mesa.json";
        PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD = "1";
        UZEL_PLAYWRIGHT_CHROMIUM = "${pkgs.chromium}/bin/chromium";

        shellHook = ''
          export UZEL_COREPACK_BIN="$PWD/.cache/corepack-bin"
          mkdir -p "$UZEL_COREPACK_BIN"
          corepack enable --install-directory "$UZEL_COREPACK_BIN" >/dev/null
          export PATH="$UZEL_COREPACK_BIN:$PATH"
        '';
      };
    };
}
