export const PROBE_NAMES = Object.freeze([
  'fetch',
  'xhr',
  'websocket',
  'eventsource',
  'image',
  'worker',
  'serviceWorker',
  'beacon',
  'tauriInternals',
  'tauriGlobal',
  'wryIpc',
  'parentReadable',
  'rawWebkitTransport',
]);

export function nativeSurface() {
  let parentReadable = false;
  try {
    parentReadable = Boolean(parent.document);
  } catch {
    // Opaque-origin sandbox must throw here.
  }
  return {
    tauriInternals: typeof globalThis.__TAURI_INTERNALS__ !== 'undefined',
    tauriGlobal: typeof globalThis.__TAURI__ !== 'undefined',
    wryIpc: typeof globalThis.ipc !== 'undefined',
    parentReadable,
    rawWebkitTransport: Boolean(globalThis.webkit?.messageHandlers?.ipc),
  };
}
