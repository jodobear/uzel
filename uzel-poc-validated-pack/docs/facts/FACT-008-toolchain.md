# FACT-008 — executable Linux toolchain

- **Claim:** The proposed Nix, Rust, Node, Tauri, Fallow, and Mermaid commands exist and can be made reproducible.
- **Classification:** verified fact
- **Exact source/pin:** Tool pins in [`../../compatibility.lock`](../../compatibility.lock); nixpkgs `38a4887411571457d700c51c64a6e49ead2ed5ab` with nar hash `sha256-kh35kIx7el4Jk8Ki3BH9/Pn1eZYSYLJ6LMALos0zOy0=`.
- **Probe/command:** Version commands; pinned `nix run ...#cargo-tauri`; pinned `nix run ...#mermaid-cli`; Mermaid SVG render; pinned `mkShell` plus `nix develop -c pkg-config`; Corepack pnpm install/build/test; signed Fallow config resolution.
- **Observed result:** Nix 2.34.1, Rust host 1.92.0/nampplets 1.89.0, Node 22.22.0, npm 10.9.4, Corepack 0.34.0, pnpm 10.8.0, Bun 1.3.3, Tauri CLI 2.11.4, Fallow 3.9.1, and Mermaid CLI 11.16.0 ran. WebKitGTK resolved to 2.52.5 in a pinned dev shell.
- **Decision:** Commit a flake/devShell and all language locks in Slice 01 after it is unblocked. Use `nix develop`, not `nix shell`, for pkg-config setup; expose Corepack's pnpm shim on `PATH`; use Fallow's `ignorePatterns` field.
- **Affected documents/code:** `config/fallow.jsonc`, `work/01-scaffold.md`, `reports/preflight.md`.
- **Revalidate when:** The flake lock, runtime source pins, Fallow binary/config schema, Tauri CLI/crate, or package-manager lock changes.
