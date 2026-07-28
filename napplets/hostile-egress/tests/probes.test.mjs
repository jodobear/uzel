import assert from 'node:assert/strict';
import test from 'node:test';

import { PROBE_NAMES } from '../src/probes.js';

test('hostile fixture keeps required probe inventory explicit', () => {
  assert.deepEqual(PROBE_NAMES, [
    'fetch', 'xhr', 'websocket', 'eventsource', 'image', 'worker', 'serviceWorker',
    'beacon', 'tauriInternals', 'tauriGlobal', 'wryIpc', 'parentReadable',
    'rawWebkitTransport',
  ]);
});
