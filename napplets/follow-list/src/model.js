import { isCanonicalPubkey } from '../../../contracts/profile-open.js';

export const MAXIMUM_RENDERED_FOLLOWS = 1_024;

export function directFollows(values, limit = MAXIMUM_RENDERED_FOLLOWS) {
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
