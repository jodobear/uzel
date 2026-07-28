# WebKit/Tauri trust spike

## Verdict

The intended trusted-parent/sandboxed-child boundary is feasible on Linux with Tauri 2.11.5, Wry 0.55.1, and WebKitGTK 2.52.5. Authenticated native authority stayed in the top frame; a sandboxed child ran and communicated through source-bound `postMessage`; tested direct network paths were denied.

This does not clear the candidate production fixtures: Kehto's generated `chat` and `feed` HTML contains a forbidden `fetch` helper and fails released conformance.

## Source verification

The exact released sources show:

- Tauri's invoke-key bootstrap, `__TAURI_INTERNALS__`, metadata, IPC initialization, and optional global API are created as `for_main_frame_only: true` initialization scripts.
- `tauri-runtime-wry` forwards that flag to Wry.
- Wry maps main-only scripts to WebKitGTK `UserContentInjectedFrames::TopFrame` at document start.
- Wry's own `window.ipc` shim is top-frame-only.
- Tauri documents the invoke key as secret material that must not be exposed to third-party scripts or iframes.

Relevant released source paths:

```text
tauri-2.11.5/src/manager/webview.rs
tauri-2.11.5/src/app.rs
tauri-2.11.5/scripts/ipc-protocol.js
tauri-runtime-wry-2.11.4/src/lib.rs
wry-0.55.1/src/webkitgtk/mod.rs
```

## Executable probe

The disposable app used:

```text
Tauri crate       2.11.5
tauri-build       2.6.3
tauri-runtime-wry 2.11.4
Wry               0.55.1
WebKitGTK          2.52.5
GTK                3.24.52
Weston             15.0.1
Mesa llvmpipe      26.1.5
```

The final headless run used pinned nixpkgs commit `38a4887411571457d700c51c64a6e49ead2ed5ab`, a Weston headless compositor with the Mesa GL renderer, and `GDK_BACKEND=wayland`. Xvfb was also tested but WebKitGTK's GPU process could not create an EGL display there; it is not the accepted CI command.

The parent invoked `trusted_ping`, installed one `<iframe sandbox="allow-scripts">`, accepted a child report only when `event.source === frame.contentWindow`, then submitted the report through trusted Tauri IPC. The child tried to read both Tauri globals and its parent, inspected Wry IPC, attempted the low-level WebKit message handler with a deliberately invalid key, and exercised four network paths against a live loopback listener.

Observed output:

```text
__TAURI_INVOKE_KEY__ expected <redacted> but received invalid-child-key
GATE0_RESULT native_calls=1 network_connections=0 child={
  "sourceBound": true,
  "parentInvoke": "trusted-parent-ok",
  "tauri": false,
  "tauriInternals": false,
  "wryIpc": false,
  "webkitIpc": true,
  "parentBridgeReadable": false,
  "rawIpcAttempted": true,
  "fetch": "blocked",
  "xhr": "blocked",
  "websocket": "blocked",
  "image": "blocked"
}
```

The actual random invoke key is intentionally omitted. The probe source and HTML hashes are locked so the result remains attributable after disposable files are removed.

## Low-level handler nuance

WebKitGTK exposes `window.webkit.messageHandlers.ipc` inside the child even though Tauri and Wry bootstrap objects are top-frame-only. That is a transport surface, not command authority: the child lacks the generated invoke key, the forged request was rejected, and no second native call ran.

The design must still treat invalid raw IPC as attacker-controlled input. Keep Tauri's key private, never inject custom all-frame scripts, bound and rate-limit messages at existing runtime seams, and fail closed on malformed input. Tauri capability labels scope a webview, not an inner document; they complement but do not replace frame isolation.

## Accepted child CSP

The tested child document placed this policy before executable content:

```text
default-src 'none';
script-src 'unsafe-inline';
style-src 'unsafe-inline';
img-src data: blob:;
font-src data:;
connect-src 'none';
worker-src 'none';
child-src 'none';
frame-src 'none';
media-src 'none';
object-src 'none';
manifest-src 'none';
base-uri 'none';
form-action 'none'
```

`unsafe-inline` is restricted to the self-contained verified document; all default and network-bearing destinations remain denied. The production bundle must contain no remote imports, module-preload network helper, service-worker update, or external media URL.

## Boundary of proof

Passed now:

- top-frame Tauri invoke and child isolation;
- sandboxed self-contained script execution;
- source-window binding;
- raw invalid-key rejection;
- fetch, XHR, WebSocket, and image denial with zero sentinel connections.

Deferred to the full hostile suite:

- EventSource, beacon, media, nested iframe, worker/service worker, form/navigation/popup, malformed/oversized envelope, and sustained invalid-IPC rate tests;
- a clean interactive Fedora desktop run and Debian build smoke;
- browser-engine exploit resistance, which is not a POC claim.
