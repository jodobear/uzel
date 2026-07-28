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

export function sentinelTargets(search = globalThis.location.search) {
  const configured = new URLSearchParams(search).get('sentinel');
  if (!configured) throw new Error('loopback sentinel URL is required');

  const http = new URL(configured);
  const port = Number(http.port);
  if (
    http.protocol !== 'http:'
    || http.hostname !== '127.0.0.1'
    || !Number.isInteger(port)
    || port < 1024
    || port > 65_535
    || http.username
    || http.password
  ) {
    throw new Error('sentinel must be an unprivileged http://127.0.0.1 URL');
  }
  http.hash = '';

  const websocket = new URL(http);
  websocket.protocol = 'ws:';
  return Object.freeze({ http: http.href, websocket: websocket.href });
}

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

export function workerLoad(url, WorkerConstructor = globalThis.Worker, timeoutMs = 3_000) {
  return new Promise((resolve, reject) => {
    const worker = new WorkerConstructor(url);
    let settled = false;
    const finish = (outcome, value) => {
      if (settled) return;
      settled = true;
      clearTimeout(timeout);
      worker.terminate();
      outcome(value);
    };
    const timeout = setTimeout(() => finish(resolve), timeoutMs);
    worker.onmessage = () => finish(resolve);
    worker.onerror = (event) => {
      event.preventDefault?.();
      finish(reject, new Error('worker load rejected'));
    };
  });
}
