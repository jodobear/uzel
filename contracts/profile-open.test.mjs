import assert from 'node:assert/strict';
import test from 'node:test';

import {
  PROFILE_OPEN_TOPIC,
  isCanonicalPubkey,
  parseProfileOpen,
  profileOpen,
} from './profile-open.js';

const PUBKEY = '0123456789abcdef'.repeat(4);

test('queryless convention identity and valid payload remain exact', () => {
  assert.equal(PROFILE_OPEN_TOPIC, 'napplet:profile/open');
  assert.deepEqual(profileOpen(PUBKEY), { version: 1, pubkey: PUBKEY });
});

test('malformed profile-open payloads fail closed', () => {
  for (const malformed of [
    null,
    [],
    {},
    { version: 2, pubkey: PUBKEY },
    { version: 1, pubkey: PUBKEY.toUpperCase() },
    { version: 1, pubkey: '0'.repeat(63) },
    { version: 1, pubkey: PUBKEY, sender: 'forged' },
  ]) {
    assert.equal(parseProfileOpen(malformed), null);
  }
  assert.equal(isCanonicalPubkey(PUBKEY), true);
});
