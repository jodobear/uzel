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
