import { isCanonicalPubkey } from '../../../contracts/profile-open.js';

export const PROFILE_RESULT_LIMIT = 1;

function optionalText(value) {
  return typeof value === 'string' && value.length > 0 ? value : undefined;
}

export function createLatestRequestGate() {
  let generation = 0;
  return Object.freeze({
    begin() {
      generation += 1;
      return generation;
    },
    isCurrent(requestGeneration) {
      return requestGeneration === generation;
    },
  });
}

export function canonicalProfile(results, pubkey) {
  if (!isCanonicalPubkey(pubkey) || !Array.isArray(results) || results.length !== 1) return null;
  const event = results[0]?.event;
  if (event?.kind !== 0 || event.pubkey !== pubkey || !Number.isSafeInteger(event.created_at)) return null;
  try {
    const content = JSON.parse(event.content);
    if (content === null || typeof content !== 'object' || Array.isArray(content)) return null;
    return {
      pubkey,
      eventId: event.id,
      createdAt: event.created_at,
      name: optionalText(content.display_name) ?? optionalText(content.name) ?? 'Unnamed profile',
      about: optionalText(content.about) ?? '',
    };
  } catch {
    return null;
  }
}
