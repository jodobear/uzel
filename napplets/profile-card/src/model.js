import { isCanonicalPubkey } from '../../../contracts/profile-open.js';

export const PROFILE_CANDIDATE_LIMIT = 5;

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

export function latestProfile(results, pubkey) {
  if (!isCanonicalPubkey(pubkey) || !Array.isArray(results)) return null;
  const candidates = results
    .map((result) => result?.event)
    .filter((event) => event?.kind === 0 && event.pubkey === pubkey && Number.isSafeInteger(event.created_at))
    .sort((left, right) => right.created_at - left.created_at);
  for (const event of candidates) {
    try {
      const content = JSON.parse(event.content);
      if (content === null || typeof content !== 'object' || Array.isArray(content)) continue;
      return {
        pubkey,
        eventId: event.id,
        createdAt: event.created_at,
        name: optionalText(content.display_name) ?? optionalText(content.name) ?? 'Unnamed profile',
        about: optionalText(content.about) ?? '',
      };
    } catch {
      // Ignore malformed newer candidate; continue to next valid signed event.
    }
  }
  return null;
}
