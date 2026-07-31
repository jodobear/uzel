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
  retryProfileQueryRequests,
  splitProfileQueryRequest,
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

  assert.deepEqual(batches.map((batch) => batch.filters.length), [8, 1]);
  assert.deepEqual(batches[0].options.authors, authors.slice(0, 8));
  assert.deepEqual(batches[0].filters[0], { kinds: [0], authors: [authors[0]], limit: 1 });
});

test('bisects a failed batch until one oversized author can be isolated', () => {
  const authors = Array.from({ length: PROFILE_QUERY_BATCH_SIZE }, (_, index) => (
    (index + 1).toString(16).padStart(64, '0')
  ));
  const [left, right] = splitProfileQueryRequest(profileQueryBatches(authors)[0]);

  assert.deepEqual(left.options.authors, authors.slice(0, 4));
  assert.deepEqual(right.options.authors, authors.slice(4));
  assert.deepEqual(splitProfileQueryRequest(left).map((request) => request.filters.length), [2, 2]);
  assert.deepEqual(splitProfileQueryRequest(profileQueryRequest(authors[0])), []);
});

test('preserves valid partial rows and retries only unresolved authors', () => {
  const request = profileQueryBatches([A, B])[0];

  assert.deepEqual(retryProfileQueryRequests(request, [A]), [profileQueryRequest(B)]);
  assert.deepEqual(retryProfileQueryRequests(request, [A, B]), []);
  assert.deepEqual(
    retryProfileQueryRequests(request).map((retry) => retry.options.authors),
    [[A], [B]],
  );
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
  assert.equal(profile.contentText, content);
  assert.equal(profile.createdAtIso, '1970-01-01T00:00:30.000Z');
});

test('complete metadata text preserves unsafe integers and duplicate keys byte for byte', () => {
  const content = '{"large":9007199254740993,"duplicate":1,"duplicate":2}';
  const profile = canonicalProfile([row(A, content)], A);

  assert.equal(profile.contentText, content);
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
