export const PROFILE_OPEN_TOPIC = 'napplet:profile/open';

const PUBKEY_PATTERN = /^[0-9a-f]{64}$/;

export function isCanonicalPubkey(value) {
  return typeof value === 'string' && PUBKEY_PATTERN.test(value);
}

export function parseProfileOpen(value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) return null;
  const keys = Object.keys(value);
  if (keys.length !== 2 || !keys.includes('version') || !keys.includes('pubkey')) return null;
  if (value.version !== 1 || !isCanonicalPubkey(value.pubkey)) return null;
  return Object.freeze({ version: 1, pubkey: value.pubkey });
}

export function profileOpen(pubkey) {
  const parsed = parseProfileOpen({ version: 1, pubkey });
  if (parsed === null) throw new TypeError('profile-open pubkey must be 64 lowercase hex characters');
  return parsed;
}
