import assert from 'node:assert/strict';
import test from 'node:test';

import {
  PROBE_NAMES,
  attemptRawWebKitInvoke,
  boundedAttempt,
  nativeSurface,
  sentinelTargets,
  workerLoad,
} from '../src/probes.js';

test('hostile fixture keeps required probe inventory explicit', () => {
  assert.deepEqual(PROBE_NAMES, [
    'fetch', 'xhr', 'websocket', 'eventsource', 'image', 'worker', 'serviceWorker',
    'beacon', 'media', 'iframe', 'form', 'navigation', 'popup', 'tauriInternals',
    'tauriGlobal', 'wryIpc', 'parentReadable', 'rawWebkitTransport',
    'rawInvokeAttempted', 'identityMutationApi',
  ]);
});

test('native surface reports read-only identity and the raw transport separately', () => {
  assert.deepEqual(nativeSurface({
    parent: {},
    webkit: { messageHandlers: { ipc: {} } },
    napplet: { identity: { getPublicKey() {} } },
  }), {
    tauriInternals: false,
    tauriGlobal: false,
    wryIpc: false,
    parentReadable: false,
    rawWebkitTransport: true,
    identityMutationApi: false,
  });
  assert.equal(nativeSurface({
    parent: {},
    napplet: { identity: { switchIdentity() {} } },
  }).identityMutationApi, true);
});

test('raw WebKit probe sends one deliberately invalid invoke-key command', () => {
  const messages = [];
  assert.equal(attemptRawWebKitInvoke(
    { postMessage: (message) => messages.push(message) },
  ), true);
  assert.equal(messages.length, 1);
  assert.deepEqual(JSON.parse(messages[0]), {
    cmd: 'hostile_native_probe',
    callback: 91_001,
    error: 91_002,
    payload: {},
    options: null,
    __TAURI_INVOKE_KEY__: 'invalid-child-key',
  });
  assert.equal(attemptRawWebKitInvoke(null), false);
  assert.equal(attemptRawWebKitInvoke({ postMessage: () => { throw new Error('rejected'); } }), true);
});

test('bounded attempts reject silent browser operations', async () => {
  await assert.rejects(boundedAttempt(() => new Promise(() => {}), 5), /no success/);
  await assert.doesNotReject(boundedAttempt(() => Promise.resolve('loaded'), 50));
});

test('hostile fixture requires an explicit unprivileged loopback sentinel', () => {
  assert.deepEqual(
    sentinelTargets('http://127.0.0.1:43129/hostile-egress?run=abc'),
    {
      http: 'http://127.0.0.1:43129/hostile-egress?run=abc',
      websocket: 'ws://127.0.0.1:43129/hostile-egress?run=abc',
    },
  );
  assert.throws(() => sentinelTargets(), /required/);
  assert.throws(() => sentinelTargets('http://127.0.0.1:9/probe'), /unprivileged/);
  assert.throws(() => sentinelTargets('https://127.0.0.1:43129/probe'), /unprivileged/);
  assert.throws(() => sentinelTargets('http://example.com:43129/probe'), /unprivileged/);
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
