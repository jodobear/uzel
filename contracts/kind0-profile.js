import { isCanonicalPubkey } from './profile-open.js';

export const PROFILE_RESULT_LIMIT = 1;
export const PROFILE_QUERY_BATCH_SIZE = 8;
export const MAXIMUM_DATE_SECONDS = 8_640_000_000_000;

export function profileQueryRequest(pubkey) {
  return {
    filters: [{ kinds: [0], authors: [pubkey], limit: PROFILE_RESULT_LIMIT }],
    options: { authors: [pubkey] },
  };
}

export function profileQueryBatches(pubkeys) {
  if (!Array.isArray(pubkeys)) return [];
  const canonical = [...new Set(pubkeys.filter(isCanonicalPubkey))];
  const batches = [];
  for (let offset = 0; offset < canonical.length; offset += PROFILE_QUERY_BATCH_SIZE) {
    const authors = canonical.slice(offset, offset + PROFILE_QUERY_BATCH_SIZE);
    batches.push({
      filters: authors.map((author) => ({
        kinds: [0],
        authors: [author],
        limit: PROFILE_RESULT_LIMIT,
      })),
      options: { authors },
    });
  }
  return batches;
}

export function splitProfileQueryRequest(request) {
  const authors = request?.options?.authors;
  if (!Array.isArray(authors) || authors.length < 2) return [];
  const middle = Math.ceil(authors.length / 2);
  return [authors.slice(0, middle), authors.slice(middle)].map((part) => ({
    filters: part.map((author) => ({
      kinds: [0],
      authors: [author],
      limit: PROFILE_RESULT_LIMIT,
    })),
    options: { authors: part },
  }));
}

function optionalText(value) {
  return typeof value === 'string' && value.length > 0 ? value : undefined;
}

export function canonicalProfile(results, pubkey) {
  if (!isCanonicalPubkey(pubkey) || !Array.isArray(results) || results.length !== 1) return null;
  const event = results[0]?.event;
  if (
    event?.kind !== 0
    || event.pubkey !== pubkey
    || typeof event.content !== 'string'
    || !Number.isSafeInteger(event.created_at)
    || event.created_at < 0
    || event.created_at > MAXIMUM_DATE_SECONDS
  ) return null;

  try {
    const metadata = JSON.parse(event.content);
    if (metadata === null || typeof metadata !== 'object' || Array.isArray(metadata)) return null;
    return {
      pubkey,
      eventId: event.id,
      createdAt: event.created_at,
      createdAtIso: new Date(event.created_at * 1_000).toISOString(),
      name: optionalText(metadata.display_name) ?? optionalText(metadata.name) ?? 'Unnamed profile',
      about: optionalText(metadata.about) ?? '',
      picture: optionalText(metadata.picture),
      nip05: optionalText(metadata.nip05),
      content: event.content,
      contentText: event.content,
    };
  } catch {
    return null;
  }
}

export function canonicalProfiles(results, pubkeys) {
  if (!Array.isArray(results) || !Array.isArray(pubkeys)) return new Map();
  const expected = new Set(pubkeys.filter(isCanonicalPubkey));
  const rowsByAuthor = new Map([...expected].map((author) => [author, []]));
  for (const row of results) {
    const author = row?.event?.pubkey;
    if (expected.has(author)) rowsByAuthor.get(author).push(row);
  }

  const profiles = new Map();
  for (const [author, rows] of rowsByAuthor) {
    const profile = canonicalProfile(rows, author);
    if (profile !== null) profiles.set(author, profile);
  }
  return profiles;
}
