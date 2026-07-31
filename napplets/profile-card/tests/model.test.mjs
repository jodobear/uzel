import assert from 'node:assert/strict';
import test from 'node:test';

import {
  canonicalProfile, createLatestRequestGate, MAXIMUM_DATE_SECONDS, profileQueryRequest,
  PROFILE_RESULT_LIMIT,
} from '../src/model.js';

const PUBKEY = 'c'.repeat(64);

function result(id, createdAt, content, author = PUBKEY) {
  return { event: { id, pubkey: author, kind: 0, created_at: createdAt, tags: [], sig: '', content } };
}

test('binds profile queries to the selected author and leaves the deadline host-owned', () => {
  const request = profileQueryRequest(PUBKEY);

  assert.deepEqual(request, {
    filters: [{ kinds: [0], authors: [PUBKEY], limit: 1 }],
    options: { authors: [PUBKEY] },
  });
  assert.equal('timeoutMs' in request.options, false);
});

test('projects the single canonical kind 0 returned by NMP', () => {
  const content = JSON.stringify({
    display_name: 'New',
    about: 'Evidence-backed.',
    website: 'https://example.test',
    custom: { nested: true },
  });

  assert.equal(PROFILE_RESULT_LIMIT, 1);
  assert.deepEqual(
    canonicalProfile([result('canonical', 30, content)], PUBKEY),
    {
      pubkey: PUBKEY,
      eventId: 'canonical',
      createdAt: 30,
      createdAtIso: '1970-01-01T00:00:30.000Z',
      name: 'New',
      about: 'Evidence-backed.',
      picture: undefined,
      nip05: undefined,
      content,
      contentText: JSON.stringify(JSON.parse(content), null, 2),
    },
  );
});

test('rejects timestamps outside the JavaScript Date range before projection', () => {
  assert.equal(
    canonicalProfile([result('maximum', MAXIMUM_DATE_SECONDS, '{}')], PUBKEY)?.createdAtIso,
    '+275760-09-13T00:00:00.000Z',
  );
  assert.equal(canonicalProfile([result('too-large', MAXIMUM_DATE_SECONDS + 1, '{}')], PUBKEY), null);
  assert.equal(canonicalProfile([result('negative', -1, '{}')], PUBKEY), null);
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
