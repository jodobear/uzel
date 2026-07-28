import { isCanonicalPubkey } from '../../../contracts/profile-open.js';

export function directFollows(values, limit = 40) {
  if (!Array.isArray(values)) return [];
  const unique = new Set();
  for (const value of values) {
    if (isCanonicalPubkey(value)) unique.add(value);
    if (unique.size === limit) break;
  }
  return [...unique];
}

export function shortPubkey(pubkey) {
  return `${pubkey.slice(0, 12)}…${pubkey.slice(-8)}`;
}
