import assert from 'node:assert/strict';
import test from 'node:test';

import {
  canonicalProfile,
  canonicalProfiles,
  MAXIMUM_DATE_SECONDS,
  profileQueryBatches,
  profileQueryRequest,
  PROFILE_QUERY_BATCH_SIZE,
  PROFILE_RESULT_LIMIT,
} from './kind0-profile.js';

const A = 'a'.repeat(64);
const B = 'b'.repeat(64);

function row(author, content = '{}', createdAt = 30, id = `event-${author[0]}`) {
  return { event: { id, pubkey: author, kind: 0, created_at: createdAt, tags: [], sig: '', content } };
}

test('binds one profile query to one author and leaves the deadline host-owned', () => {
  assert.equal(PROFILE_RESULT_LIMIT, 1);
  assert.deepEqual(profileQueryRequest(A), {
    filters: [{ kinds: [0], authors: [A], limit: 1 }],
    options: { authors: [A] },
  });
});

test('batches canonical authors within the NAP-OUTBOX filter ceiling', () => {
  const authors = Array.from({ length: PROFILE_QUERY_BATCH_SIZE + 1 }, (_, index) => (
    index.toString(16).padStart(64, '0')
  ));
  const batches = profileQueryBatches([...authors, authors[0], 'bad']);

  assert.deepEqual(batches.map((batch) => batch.filters.length), [64, 1]);
  assert.deepEqual(batches[0].options.authors, authors.slice(0, 64));
  assert.deepEqual(batches[0].filters[0], { kinds: [0], authors: [authors[0]], limit: 1 });
});

test('retains the complete canonical kind 0 while projecting its friendly summary', () => {
  const content = JSON.stringify({
    name: 'Base name',
    display_name: 'Display name',
    website: 'https://example.test',
    lud16: 'yo@example.test',
    custom: { nested: true },
  });
  const profile = canonicalProfile([row(A, content)], A);

  assert.equal(profile.name, 'Display name');
  assert.equal(profile.content, content);
  assert.deepEqual(JSON.parse(profile.contentText), JSON.parse(content));
  assert.equal(profile.createdAtIso, '1970-01-01T00:00:30.000Z');
});

test('maps only one valid provider-owned canonical row per expected author', () => {
  const profiles = canonicalProfiles([
    row(A, '{"name":"A"}'),
    row(B, '{"name":"B-new"}', 30, 'new'),
    row(B, '{"name":"B-old"}', 20, 'old'),
    row('c'.repeat(64), '{"name":"unexpected"}'),
  ], [A, B]);

  assert.equal(profiles.get(A)?.name, 'A');
  assert.equal(profiles.has(B), false);
});

test('rejects malformed rows and timestamps outside the JavaScript Date range', () => {
  assert.equal(
    canonicalProfile([row(A, '{}', MAXIMUM_DATE_SECONDS)], A)?.createdAtIso,
    '+275760-09-13T00:00:00.000Z',
  );
  assert.equal(canonicalProfile([row(A, '{}', MAXIMUM_DATE_SECONDS + 1)], A), null);
  assert.equal(canonicalProfile([row(A, '{')], A), null);
  assert.equal(canonicalProfile([row(A, '[]')], A), null);
  assert.equal(canonicalProfile([row(A, '{}'), row(A, '{}')], A), null);
});
