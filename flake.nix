{
  description = "Uzel Linux napplet-runtime POC development shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/38a4887411571457d700c51c64a6e49ead2ed5ab";

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo-tauri
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

        shellHook = ''
          export UZEL_COREPACK_BIN="$PWD/.cache/corepack-bin"
          mkdir -p "$UZEL_COREPACK_BIN"
          corepack enable --install-directory "$UZEL_COREPACK_BIN" >/dev/null
          export PATH="$UZEL_COREPACK_BIN:$PATH"
        '';
      };
    };
}
