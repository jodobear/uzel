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

test('waits past an accepted cache hit until changed evidence appears', async () => {
  const snapshots = ['cached', 'cached', 'fresh'];
  const attempts = snapshots.length;
  let calls = 0;
  const result = await waitForEvidence(
    async () => {
      calls += 1;
      return snapshots.shift();
    },
    () => true,
    {
      attempts,
      intervalMs: 0,
      isFresh: (value) => value !== 'cached',
    },
  );
  assert.equal(result, 'fresh');
  assert.equal(calls, 3);
});

test('returns the final bounded cache snapshot when no change arrives', async () => {
  let calls = 0;
  const result = await waitForEvidence(
    async () => {
      calls += 1;
      return 'cached';
    },
    () => true,
    { attempts: 3, intervalMs: 0, isFresh: () => false },
  );
  assert.equal(result, 'cached');
  assert.equal(calls, 3);
});
