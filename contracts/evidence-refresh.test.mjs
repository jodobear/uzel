import assert from 'node:assert/strict';
import test from 'node:test';

import { waitForEvidence } from './evidence-refresh.js';

test('repeats cache-first reads until evidence appears', async () => {
  let calls = 0;
  const result = await waitForEvidence(
    async () => ++calls,
    (value) => value === 3,
    { attempts: 4, intervalMs: 0 },
  );
  assert.equal(result, 3);
  assert.equal(calls, 3);
});

test('returns the final bounded snapshot when evidence stays absent', async () => {
  let calls = 0;
  const result = await waitForEvidence(
    async () => ++calls,
    () => false,
    { attempts: 2, intervalMs: 0 },
  );
  assert.equal(result, 2);
  assert.equal(calls, 2);
});
