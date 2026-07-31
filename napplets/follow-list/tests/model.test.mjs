import assert from 'node:assert/strict';
import test from 'node:test';

import {
  createAvatarObjectUrlStore,
  createBoundedTaskQueue,
  createProfileRetryBudget,
  directFollows,
  MAXIMUM_AVATAR_OBJECT_URLS,
  MAXIMUM_AVATAR_REQUESTS,
  MAXIMUM_PROFILE_REQUESTS,
  MAXIMUM_PROFILE_RETRY_REQUESTS,
  MAXIMUM_RENDERED_FOLLOWS,
  shortPubkey,
} from '../src/model.js';

const A = 'a'.repeat(64);
const B = 'b'.repeat(64);

test('direct follows remain canonical, unique, ordered, and bounded', () => {
  assert.deepEqual(directFollows([A, A, B, B.toUpperCase(), 'bad'], 2), [A, B]);
  assert.equal(shortPubkey(A), `${'a'.repeat(12)}…${'a'.repeat(8)}`);
});

test('the default follow row bound remains 1,024', () => {
  const pubkeys = Array.from({ length: MAXIMUM_RENDERED_FOLLOWS + 1 }, (_, index) => (
    index.toString(16).padStart(64, '0')
  ));
  assert.equal(directFollows(pubkeys).length, 1_024);
  assert.equal(directFollows(pubkeys).at(-1), pubkeys[1_023]);
});

test('avatar object URLs evict the oldest retained blob at the aggregate bound', () => {
  const store = createAvatarObjectUrlStore(2);
  assert.deepEqual(store.remember('blob:first', 'first-row'), []);
  assert.deepEqual(store.remember('blob:second', 'second-row'), []);
  assert.deepEqual(store.remember('blob:third', 'third-row'), [['blob:first', 'first-row']]);
  assert.equal(store.size, 2);
  assert.deepEqual(store.drain(), [
    ['blob:second', 'second-row'],
    ['blob:third', 'third-row'],
  ]);
  assert.equal(store.size, 0);
  assert.equal(MAXIMUM_AVATAR_OBJECT_URLS, 32);
});

test('profile retries consume one finite per-refresh budget', () => {
  const budget = createProfileRetryBudget(2);
  assert.equal(budget.take(), true);
  assert.equal(budget.take(), true);
  assert.equal(budget.take(), false);
  assert.equal(budget.remaining, 0);
  assert.equal(MAXIMUM_PROFILE_RETRY_REQUESTS, 32);
  assert.throws(() => createProfileRetryBudget(-1), RangeError);
});

test('avatar work never exceeds four active tasks', async () => {
  const queue = createBoundedTaskQueue();
  const releases = [];
  let active = 0;
  let maximum = 0;
  const jobs = Array.from({ length: MAXIMUM_AVATAR_REQUESTS + 3 }, () => queue.run(() => (
    new Promise((resolve) => {
      active += 1;
      maximum = Math.max(maximum, active);
      releases.push(() => {
        active -= 1;
        resolve();
      });
    })
  )));

  await Promise.resolve();
  assert.equal(releases.length, MAXIMUM_AVATAR_REQUESTS);
  while (releases.length > 0) {
    releases.shift()();
    await new Promise((resolve) => setImmediate(resolve));
  }
  await Promise.all(jobs);
  assert.equal(maximum, 4);
});

test('profile queries never exceed two active batches', async () => {
  const queue = createBoundedTaskQueue(MAXIMUM_PROFILE_REQUESTS);
  const releases = [];
  let active = 0;
  let maximum = 0;
  const jobs = Array.from({ length: 5 }, () => queue.run(() => new Promise((resolve) => {
    active += 1;
    maximum = Math.max(maximum, active);
    releases.push(() => {
      active -= 1;
      resolve();
    });
  })));

  await Promise.resolve();
  assert.equal(releases.length, MAXIMUM_PROFILE_REQUESTS);
  while (releases.length > 0) {
    releases.shift()();
    await new Promise((resolve) => setImmediate(resolve));
  }
  await Promise.all(jobs);
  assert.equal(maximum, 2);
});

test('clearing avatar work rejects tasks that have not started', async () => {
  const queue = createBoundedTaskQueue(1);
  let release;
  const active = queue.run(() => new Promise((resolve) => { release = resolve; }));
  const pending = queue.run(() => Promise.resolve());
  const reason = new Error('stale generation');

  queue.clear(reason);
  await assert.rejects(pending, reason);
  release();
  await active;
});
