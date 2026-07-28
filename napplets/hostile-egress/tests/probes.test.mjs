import assert from 'node:assert/strict';
import test from 'node:test';

import { PROBE_NAMES, workerLoad } from '../src/probes.js';

test('hostile fixture keeps required probe inventory explicit', () => {
  assert.deepEqual(PROBE_NAMES, [
    'fetch', 'xhr', 'websocket', 'eventsource', 'image', 'worker', 'serviceWorker',
    'beacon', 'tauriInternals', 'tauriGlobal', 'wryIpc', 'parentReadable',
    'rawWebkitTransport',
  ]);
});

test('worker probe waits for the load outcome', async () => {
  class RejectedWorker {
    constructor() {
      queueMicrotask(() => this.onerror({ preventDefault() {} }));
    }

    terminate() {}
  }

  class LoadedWorker {
    constructor() {
      queueMicrotask(() => this.onmessage());
    }

    terminate() {}
  }

  class SilentWorker {
    terminate() {}
  }

  await assert.rejects(workerLoad('http://127.0.0.1:9/probe.js', RejectedWorker, 50));
  await assert.doesNotReject(workerLoad('http://127.0.0.1:9/probe.js', LoadedWorker, 50));
  await assert.doesNotReject(workerLoad('http://127.0.0.1:9/probe.js', SilentWorker, 5));
});
