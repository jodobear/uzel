import assert from 'node:assert/strict';
import test from 'node:test';

import { createLatestRequestGate, latestProfile } from '../src/model.js';

const PUBKEY = 'c'.repeat(64);

function result(id, createdAt, content, author = PUBKEY) {
  return { event: { id, pubkey: author, kind: 0, created_at: createdAt, tags: [], sig: '', content } };
}

test('selects latest valid kind 0 for requested author', () => {
  assert.deepEqual(
    latestProfile([
      result('older', 10, JSON.stringify({ name: 'Old' })),
      result('wrong-author', 50, JSON.stringify({ name: 'Wrong' }), 'd'.repeat(64)),
      result('malformed', 40, '{'),
      result('newer', 30, JSON.stringify({ display_name: 'New', about: 'Evidence-backed.' })),
    ], PUBKEY),
    {
      pubkey: PUBKEY,
      eventId: 'newer',
      createdAt: 30,
      name: 'New',
      about: 'Evidence-backed.',
    },
  );
});

test('rejects noncanonical author and non-profile results', () => {
  assert.equal(latestProfile([], PUBKEY.toUpperCase()), null);
  assert.equal(latestProfile([{ event: { kind: 1, pubkey: PUBKEY, created_at: 1 } }], PUBKEY), null);
});

test('only the latest profile request may update the display', () => {
  const requests = createLatestRequestGate();
  const slowFirst = requests.begin();
  const newerSecond = requests.begin();

  assert.equal(requests.isCurrent(slowFirst), false);
  assert.equal(requests.isCurrent(newerSecond), true);
});
