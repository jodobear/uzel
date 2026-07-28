import assert from 'node:assert/strict';
import test from 'node:test';

import { PROBE_NAMES, sentinelTargets, workerLoad } from '../src/probes.js';

test('hostile fixture keeps required probe inventory explicit', () => {
  assert.deepEqual(PROBE_NAMES, [
    'fetch', 'xhr', 'websocket', 'eventsource', 'image', 'worker', 'serviceWorker',
    'beacon', 'tauriInternals', 'tauriGlobal', 'wryIpc', 'parentReadable',
    'rawWebkitTransport',
  ]);
});

test('hostile fixture requires an explicit unprivileged loopback sentinel', () => {
  assert.deepEqual(
    sentinelTargets('?sentinel=http%3A%2F%2F127.0.0.1%3A43129%2Fhostile-egress%3Frun%3Dabc'),
    {
      http: 'http://127.0.0.1:43129/hostile-egress?run=abc',
      websocket: 'ws://127.0.0.1:43129/hostile-egress?run=abc',
    },
  );
  assert.throws(() => sentinelTargets(''), /required/);
  assert.throws(() => sentinelTargets('?sentinel=http://127.0.0.1:9/probe'), /unprivileged/);
  assert.throws(() => sentinelTargets('?sentinel=https://127.0.0.1:43129/probe'), /unprivileged/);
  assert.throws(() => sentinelTargets('?sentinel=http://example.com:43129/probe'), /unprivileged/);
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
