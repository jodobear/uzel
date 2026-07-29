export const PROBE_NAMES = Object.freeze([
  'fetch',
  'xhr',
  'websocket',
  'eventsource',
  'image',
  'worker',
  'serviceWorker',
  'beacon',
  'media',
  'iframe',
  'form',
  'navigation',
  'popup',
  'tauriInternals',
  'tauriGlobal',
  'wryIpc',
  'parentReadable',
  'rawWebkitTransport',
  'rawInvokeAttempted',
  'identityMutationApi',
]);

export function sentinelTargets(configured) {
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

export function nativeSurface(environment = globalThis) {
  let parentReadable = false;
  try {
    parentReadable = Boolean(environment.parent.document);
  } catch {
    // Opaque-origin sandbox must throw here.
  }
  return {
    tauriInternals: typeof environment.__TAURI_INTERNALS__ !== 'undefined',
    tauriGlobal: typeof environment.__TAURI__ !== 'undefined',
    wryIpc: typeof environment.ipc !== 'undefined',
    parentReadable,
    rawWebkitTransport: Boolean(environment.webkit?.messageHandlers?.ipc),
    identityMutationApi: [
      'setPublicKey',
      'selectPublicKey',
      'setIdentity',
      'selectIdentity',
      'switchIdentity',
    ].some((name) => typeof environment.napplet?.identity?.[name] === 'function'),
  };
}

export function attemptRawWebKitInvoke(
  handler = globalThis.webkit?.messageHandlers?.ipc,
) {
  if (!handler || typeof handler.postMessage !== 'function') return false;
  handler.postMessage(JSON.stringify({
    cmd: 'hostile_native_probe',
    callback: 91_001,
    error: 91_002,
    payload: {},
    options: null,
    __TAURI_INVOKE_KEY__: 'invalid-child-key',
  }));
  return true;
}

export function boundedAttempt(operation, timeoutMs = 3_000) {
  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => reject(new Error('probe produced no success')), timeoutMs);
    Promise.resolve()
      .then(operation)
      .then(
        (value) => {
          clearTimeout(timeout);
          resolve(value);
        },
        (error) => {
          clearTimeout(timeout);
          reject(error);
        },
      );
  });
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
