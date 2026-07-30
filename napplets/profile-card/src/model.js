import { isCanonicalPubkey } from '../../../contracts/profile-open.js';

export const PROFILE_RESULT_LIMIT = 1;
export const MAXIMUM_DATE_SECONDS = 8_640_000_000_000;

function optionalText(value) {
  return typeof value === 'string' && value.length > 0 ? value : undefined;
}

export function canonicalIdentityProfile(profile, pubkey) {
  if (
    !isCanonicalPubkey(pubkey)
    || profile === null
    || typeof profile !== 'object'
    || Array.isArray(profile)
  ) return null;
  return {
    pubkey,
    name: optionalText(profile.displayName) ?? optionalText(profile.name) ?? 'Unnamed profile',
    about: optionalText(profile.about) ?? '',
    picture: optionalText(profile.picture),
    nip05: optionalText(profile.nip05),
  };
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
  if (
    event?.kind !== 0
    || event.pubkey !== pubkey
    || !Number.isSafeInteger(event.created_at)
    || event.created_at < 0
    || event.created_at > MAXIMUM_DATE_SECONDS
  ) return null;
  try {
    const content = JSON.parse(event.content);
    if (content === null || typeof content !== 'object' || Array.isArray(content)) return null;
    const observedAt = new Date(event.created_at * 1_000).toISOString();
    return {
      pubkey,
      eventId: event.id,
      createdAt: event.created_at,
      observedAt,
      name: optionalText(content.display_name) ?? optionalText(content.name) ?? 'Unnamed profile',
      about: optionalText(content.about) ?? '',
      picture: optionalText(content.picture),
      nip05: optionalText(content.nip05),
    };
  } catch {
    return null;
  }
}
