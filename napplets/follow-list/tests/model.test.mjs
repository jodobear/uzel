import assert from 'node:assert/strict';
import test from 'node:test';

import { directFollows, shortPubkey } from '../src/model.js';

const A = 'a'.repeat(64);
const B = 'b'.repeat(64);

test('direct follows remain canonical, unique, ordered, and bounded', () => {
  assert.deepEqual(directFollows([A, A, B, B.toUpperCase(), 'bad'], 2), [A, B]);
  assert.equal(shortPubkey(A), `${'a'.repeat(12)}…${'a'.repeat(8)}`);
});
