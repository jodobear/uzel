# FACT-007 — local daemon IPC

- **Claim:** A small private shell/daemon protocol is feasible over a same-user Unix socket under `$XDG_RUNTIME_DIR`.
- **Classification:** verified fact
- **Exact source/pin:** Fedora Server 43, Linux 7.1.3, Rust 1.89.0; disposable source SHA-256 `48a1d032a9b97677b823341591a121a7210e3a7eded5d55195a50673fd01052c`.
- **Probe/command:** Compile a standard-library AF_UNIX server/client; create a per-process runtime directory; set directory/socket modes; exchange a version-0 hello/status using a four-byte big-endian length prefix; send a 4,097-byte declared frame against a 4,096-byte limit.
- **Observed result:** Directory mode 0700, socket mode 0600, UID 1000, hello/status round trip passed, oversized frame was rejected before allocation/read, and probe files were cleaned.
- **Decision:** Use AF_UNIX at `$XDG_RUNTIME_DIR/uzel/napd.sock`, protocol version 0, bounded length-prefixed JSON, one POC client, and typed operation/error enums. Do not create TCP, remote access, or a public compatibility protocol.
- **Affected documents/code:** `docs/02-architecture.md`, `docs/03-provisional-design.md`, `work/04-daemon-nmp.md`, `reports/preflight.md`.
- **Revalidate when:** Multi-client support, remote access, streaming, file-descriptor passing, or a non-Linux target enters scope.
