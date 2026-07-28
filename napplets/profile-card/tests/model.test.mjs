import assert from 'node:assert/strict';
import test from 'node:test';

import { canonicalProfile, createLatestRequestGate, PROFILE_RESULT_LIMIT } from '../src/model.js';

const PUBKEY = 'c'.repeat(64);

function result(id, createdAt, content, author = PUBKEY) {
  return { event: { id, pubkey: author, kind: 0, created_at: createdAt, tags: [], sig: '', content } };
}

test('projects the single canonical kind 0 returned by NMP', () => {
  assert.equal(PROFILE_RESULT_LIMIT, 1);
  assert.deepEqual(
    canonicalProfile([
      result('canonical', 30, JSON.stringify({ display_name: 'New', about: 'Evidence-backed.' })),
    ], PUBKEY),
    {
      pubkey: PUBKEY,
      eventId: 'canonical',
      createdAt: 30,
      name: 'New',
      about: 'Evidence-backed.',
    },
  );
});

test('never selects among provider rows or revives a replaced profile', () => {
  assert.equal(canonicalProfile([], PUBKEY.toUpperCase()), null);
  assert.equal(canonicalProfile([result('canonical', 20, '{')], PUBKEY), null);
  assert.equal(canonicalProfile([
    result('canonical-malformed', 20, '{'),
    result('replaced-valid', 10, JSON.stringify({ name: 'Old' })),
  ], PUBKEY), null);
  assert.equal(canonicalProfile([{ event: { kind: 1, pubkey: PUBKEY, created_at: 1 } }], PUBKEY), null);
});

test('only the latest profile request may update the display', () => {
  const requests = createLatestRequestGate();
  const slowFirst = requests.begin();
  const newerSecond = requests.begin();

  assert.equal(requests.isCurrent(slowFirst), false);
  assert.equal(requests.isCurrent(newerSecond), true);
});
