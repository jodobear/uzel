#!/usr/bin/env node

import { constants } from 'node:fs';
import { open, readFile, realpath } from 'node:fs/promises';
import { dirname, isAbsolute, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '..');
export const DEFAULT_CORPUS_LOCK = resolve(
  ROOT,
  'fixtures/external-napplet-corpus/corpus.lock.json',
);

const HEX_64 = /^[0-9a-f]{64}$/;
const HEX_128 = /^[0-9a-f]{128}$/;
const EXPECTED_FORMAT = 'uzel.external-napplet-corpus.v1';
const EXPECTED_NAK_VERSION = '0.20.1';
const EXPECTED_SOURCE_COMMIT = 'aa4dc7a0799d95e3066b50055b29685d6e376045';
const EXPECTED_SOURCE_COMMIT_URL =
  `https://github.com/hzrd149/napplelets/commit/${EXPECTED_SOURCE_COMMIT}`;
const VERIFIED_SNAPSHOT_FORMAT = 'uzel.verified-external-napplet-corpus.v1';
const EXPECTED_SAFE_AUTOMATION = new Map([
  ['good-morning', 'control'],
  ['rubik-cube', 'zero-capability-render-input'],
  ['nap-feed', 'read-only-config-outbox'],
  ['wifi-map', 'read-only-storage-outbox-no-link-click'],
]);
const EXPECTED_EVENT_IDS = new Map([
  ['good-morning', 'caef62ea8feb506d621f7f4c514fcb4322280cf3041d9e4801354b5d615f9d3b'],
  ['rubik-cube', '312bbf17fef533107d24c5682a9d58a62b7582252e912f90fb41d8d283038706'],
  ['nap-feed', 'e4445830b411d920e1f5fa4f74f58144f085a43e370ed39703e60207be9c7c9d'],
  ['wifi-map', '49e111745256e32af6a5b3dd02e54b731cd8848a2951e0cf6de0888ab52e1b99'],
]);
const EXPECTED_DOMAINS = new Map([
  ['good-morning', ['identity', 'inc', 'link', 'outbox', 'resource', 'theme']],
  ['rubik-cube', []],
  ['nap-feed', ['config', 'identity', 'outbox', 'theme']],
  ['wifi-map', ['link', 'outbox', 'storage', 'theme']],
]);

export class CorpusVerificationError extends Error {
  constructor(category, code, message) {
    super(message);
    this.name = 'CorpusVerificationError';
    this.category = category;
    this.code = code;
  }
}

function trust(code, message) {
  throw new CorpusVerificationError('trust', code, message);
}

function infrastructure(code, message) {
  throw new CorpusVerificationError('infrastructure', code, message);
}

function requireCondition(condition, code, message) {
  if (!condition) {
    trust(code, message);
  }
}

function requireHex(value, pattern, field) {
  requireCondition(
    typeof value === 'string' && pattern.test(value),
    'invalid-lock',
    `${field} is not canonical lowercase hex`,
  );
}

function requireString(value, field) {
  requireCondition(
    typeof value === 'string' && value.length > 0,
    'invalid-lock',
    `${field} must be a non-empty string`,
  );
}

function requireUniqueStrings(values, field, { sorted = false } = {}) {
  requireCondition(Array.isArray(values), 'invalid-lock', `${field} must be an array`);
  for (const value of values) {
    requireString(value, field);
  }
  requireCondition(
    new Set(values).size === values.length,
    'invalid-lock',
    `${field} contains duplicates`,
  );
  if (sorted) {
    requireCondition(
      values.every((value, index) => index === 0 || values[index - 1] < value),
      'invalid-lock',
      `${field} must be sorted`,
    );
  }
}

function exactTag(event, name, predicate = () => true) {
  const matches = event.tags.filter(
    (tag) => Array.isArray(tag) && tag[0] === name && predicate(tag),
  );
  requireCondition(
    matches.length === 1,
    'signed-event-drift',
    `${event.id}: expected exactly one ${name} tag`,
  );
  return matches[0];
}

function verifyFailurePolicy(policy) {
  requireCondition(
    policy && typeof policy === 'object',
    'invalid-lock',
    'failurePolicy must be an object',
  );
  requireUniqueStrings(policy.infrastructure, 'failurePolicy.infrastructure', { sorted: true });
  requireUniqueStrings(policy.trust, 'failurePolicy.trust', { sorted: true });
  const overlap = policy.infrastructure.filter((code) => policy.trust.includes(code));
  requireCondition(
    overlap.length === 0,
    'invalid-lock',
    `failure classifications overlap: ${overlap.join(', ')}`,
  );
}

function verifyLockEntry(entry, publisher) {
  requireCondition(
    entry !== null && typeof entry === 'object' && !Array.isArray(entry),
    'invalid-lock',
    'entry must be an object',
  );
  requireString(entry.name, 'entry.name');
  requireString(entry.title, `${entry.name}.title`);
  requireCondition(
    EXPECTED_SAFE_AUTOMATION.has(entry.name),
    'invalid-lock',
    `${entry.name}: name is not in the audited automation allowlist`,
  );
  requireCondition(
    entry.safeAutomation === EXPECTED_SAFE_AUTOMATION.get(entry.name),
    'invalid-lock',
    `${entry.name}: automation scope is not the audited fail-closed value`,
  );
  requireString(entry.naddr, `${entry.name}.naddr`);
  requireString(entry.eventFile, `${entry.name}.eventFile`);
  requireHex(entry.eventId, HEX_64, `${entry.name}.eventId`);
  requireCondition(
    entry.eventId === EXPECTED_EVENT_IDS.get(entry.name),
    'event-id-drift',
    `${entry.name}: event id is not the exact audited value`,
  );
  requireHex(entry.author, HEX_64, `${entry.name}.author`);
  requireCondition(
    entry.author === publisher,
    'coordinate-drift',
    `${entry.name}: author differs from corpus publisher`,
  );
  requireCondition(entry.kind === 35129, 'coordinate-drift', `${entry.name}: kind must be 35129`);
  requireCondition(
    Number.isSafeInteger(entry.createdAt) && entry.createdAt > 0,
    'invalid-lock',
    `${entry.name}.createdAt must be a positive integer`,
  );
  requireString(entry.dTag, `${entry.name}.dTag`);
  requireUniqueStrings(entry.relayHints, `${entry.name}.relayHints`);
  requireCondition(
    entry.relayHints.every((relay) => relay.startsWith('wss://')),
    'invalid-lock',
    `${entry.name}: relay hints must use wss`,
  );
  requireCondition(entry.artifact && typeof entry.artifact === 'object', 'invalid-lock', `${entry.name}.artifact must be an object`);
  requireCondition(
    entry.artifact.logicalPath === '/index.html',
    'invalid-lock',
    `${entry.name}: only the audited single-file path is allowed`,
  );
  requireHex(entry.artifact.sha256, HEX_64, `${entry.name}.artifact.sha256`);
  requireHex(
    entry.artifact.aggregateSha256,
    HEX_64,
    `${entry.name}.artifact.aggregateSha256`,
  );
  requireCondition(
    Number.isSafeInteger(entry.artifact.sizeBytes) && entry.artifact.sizeBytes > 0,
    'invalid-lock',
    `${entry.name}.artifact.sizeBytes must be a positive integer`,
  );
  requireUniqueStrings(entry.domains, `${entry.name}.domains`, { sorted: true });
  requireCondition(
    JSON.stringify(entry.domains) === JSON.stringify(EXPECTED_DOMAINS.get(entry.name)),
    'capability-drift',
    `${entry.name}: domains are not the exact audited capability set`,
  );
  requireCondition(
    entry.domainsSource === 'audited-artifact-meta',
    'invalid-lock',
    `${entry.name}: domains must retain their evidence classification`,
  );
  requireUniqueStrings(entry.servers, `${entry.name}.servers`);
  requireCondition(
    entry.servers.every((server) => server.startsWith('https://')),
    'invalid-lock',
    `${entry.name}: artifact servers must use https`,
  );
}

export function verifySignedEvent(entry, event) {
  requireCondition(event && typeof event === 'object', 'invalid-event', `${entry.name}: event must be an object`);
  requireCondition(Array.isArray(event.tags), 'invalid-event', `${entry.name}: event tags must be an array`);
  requireHex(event.id, HEX_64, `${entry.name}.event.id`);
  requireHex(event.pubkey, HEX_64, `${entry.name}.event.pubkey`);
  requireHex(event.sig, HEX_128, `${entry.name}.event.sig`);

  requireCondition(event.id === entry.eventId, 'event-id-drift', `${entry.name}: event id drifted`);
  requireCondition(event.pubkey === entry.author, 'coordinate-drift', `${entry.name}: event author drifted`);
  requireCondition(event.kind === entry.kind, 'coordinate-drift', `${entry.name}: event kind drifted`);
  requireCondition(
    event.created_at === entry.createdAt,
    'event-id-drift',
    `${entry.name}: event timestamp drifted`,
  );
  requireCondition(event.content === '', 'event-id-drift', `${entry.name}: event content drifted`);

  const titleTag = exactTag(event, 'title');
  requireCondition(
    titleTag.length === 2 && titleTag[1] === entry.title,
    'signed-event-drift',
    `${entry.name}: signed title drifted`,
  );

  const dTag = exactTag(event, 'd');
  requireCondition(dTag.length === 2 && dTag[1] === entry.dTag, 'coordinate-drift', `${entry.name}: d tag drifted`);

  const pathTag = exactTag(event, 'path');
  requireCondition(
    pathTag.length === 3 &&
      pathTag[1] === entry.artifact.logicalPath &&
      pathTag[2] === entry.artifact.sha256,
    'artifact-path-digest-mismatch',
    `${entry.name}: signed artifact path tuple drifted`,
  );

  const aggregateTag = exactTag(event, 'x', (tag) => tag[2] === 'aggregate');
  requireCondition(
    aggregateTag.length === 3 && aggregateTag[1] === entry.artifact.aggregateSha256,
    'aggregate-drift',
    `${entry.name}: signed aggregate drifted`,
  );

  const eventServers = event.tags
    .filter((tag) => Array.isArray(tag) && tag[0] === 'server')
    .map((tag) => tag[1]);
  requireCondition(
    JSON.stringify(eventServers) === JSON.stringify(entry.servers),
    'signed-event-drift',
    `${entry.name}: signed artifact servers drifted`,
  );

  requireCondition(
    entry.naddr.startsWith('naddr1'),
    'invalid-lock',
    `${entry.name}: coordinate must be an naddr`,
  );
}

function readFailure(error, description) {
  if (error?.code === 'ENOENT') {
    trust('missing-corpus-data', `${description}: ${error.message}`);
  }
  infrastructure('corpus-read-failed', `${description}: ${error.message}`);
}

function parseJson(text, description) {
  try {
    return JSON.parse(text);
  } catch (error) {
    trust('invalid-json', `${description}: ${error.message}`);
  }
}

async function readJson(path, description) {
  let text;
  try {
    text = await readFile(path, 'utf8');
  } catch (error) {
    readFailure(error, description);
  }
  return parseJson(text, description);
}

function isContainedPath(pathFromRoot) {
  return (
    pathFromRoot.length > 0 &&
    pathFromRoot !== '..' &&
    !pathFromRoot.startsWith(`..${sep}`) &&
    !isAbsolute(pathFromRoot)
  );
}

async function canonicalDirectory(path, description) {
  try {
    return await realpath(path);
  } catch (error) {
    readFailure(error, description);
  }
}

async function readEventJson(path, description, canonicalCorpusDirectory, expectedCanonicalPath) {
  let handle;
  try {
    handle = await open(path, constants.O_RDONLY | constants.O_NOFOLLOW);
  } catch (error) {
    if (error?.code === 'ELOOP') {
      trust('invalid-lock', `${description}: symlinked event evidence is not allowed`);
    }
    readFailure(error, description);
  }

  try {
    let canonicalPath;
    try {
      canonicalPath = await realpath(`/proc/self/fd/${handle.fd}`);
    } catch (error) {
      infrastructure('corpus-read-failed', `${description}: ${error.message}`);
    }
    requireCondition(
      isContainedPath(relative(canonicalCorpusDirectory, canonicalPath)),
      'invalid-lock',
      `${description}: event file escapes corpus directory`,
    );
    requireCondition(
      canonicalPath === expectedCanonicalPath,
      'invalid-lock',
      `${description}: symlinked event evidence is not allowed`,
    );

    let text;
    try {
      text = await handle.readFile('utf8');
    } catch (error) {
      infrastructure('corpus-read-failed', `${description}: ${error.message}`);
    }
    return {
      event: parseJson(text, description),
      text,
    };
  } finally {
    await handle.close();
  }
}

async function readVerifiedCorpus(lockPath = DEFAULT_CORPUS_LOCK) {
  const resolvedLockPath = resolve(lockPath);
  const corpusDirectory = dirname(resolvedLockPath);
  const lock = await readJson(resolvedLockPath, 'corpus lock');
  requireCondition(
    lock !== null && typeof lock === 'object' && !Array.isArray(lock),
    'invalid-lock',
    'corpus lock root must be an object',
  );
  requireCondition(lock.format === EXPECTED_FORMAT, 'invalid-lock', 'unknown corpus lock format');
  requireCondition(lock.source && typeof lock.source === 'object', 'invalid-lock', 'source provenance is required');
  requireCondition(lock.source.repository === 'https://github.com/hzrd149/napplelets', 'invalid-lock', 'unexpected source repository');
  requireCondition(
    lock.source.commit === EXPECTED_SOURCE_COMMIT,
    'invalid-lock',
    `source.commit must remain ${EXPECTED_SOURCE_COMMIT}`,
  );
  requireCondition(
    lock.source.commitUrl === EXPECTED_SOURCE_COMMIT_URL,
    'invalid-lock',
    `source.commitUrl must remain ${EXPECTED_SOURCE_COMMIT_URL}`,
  );
  requireCondition(lock.source.license === 'MIT', 'invalid-lock', 'source license must remain explicit');
  requireString(lock.source.licenseUrl, 'source.licenseUrl');
  requireString(lock.source.auditedOn, 'source.auditedOn');
  requireString(lock.source.publishedAt, 'source.publishedAt');
  requireCondition(
    lock.toolchain?.nakVersion === EXPECTED_NAK_VERSION,
    'invalid-lock',
    `toolchain.nakVersion must be ${EXPECTED_NAK_VERSION}`,
  );
  requireHex(lock.publisher, HEX_64, 'publisher');
  verifyFailurePolicy(lock.failurePolicy);
  requireCondition(
    Array.isArray(lock.entries) && lock.entries.length === EXPECTED_SAFE_AUTOMATION.size,
    'invalid-lock',
    `entries must contain exactly ${EXPECTED_SAFE_AUTOMATION.size} audited napplets`,
  );

  const canonicalCorpusDirectory = await canonicalDirectory(corpusDirectory, 'corpus directory');

  const names = new Set();
  const coordinates = new Set();
  const eventIds = new Set();
  const verifiedEntries = [];
  for (const entry of lock.entries) {
    verifyLockEntry(entry, lock.publisher);
    requireCondition(!names.has(entry.name), 'invalid-lock', `duplicate name: ${entry.name}`);
    requireCondition(!coordinates.has(entry.naddr), 'invalid-lock', `duplicate naddr: ${entry.name}`);
    requireCondition(!eventIds.has(entry.eventId), 'invalid-lock', `duplicate event id: ${entry.name}`);
    names.add(entry.name);
    coordinates.add(entry.naddr);
    eventIds.add(entry.eventId);

    const eventPath = resolve(corpusDirectory, entry.eventFile);
    const eventPathFromCorpus = relative(corpusDirectory, eventPath);
    requireCondition(
      isContainedPath(eventPathFromCorpus),
      'invalid-lock',
      `${entry.name}: event file escapes corpus directory`,
    );
    const verifiedEvent = await readEventJson(
      eventPath,
      `${entry.name} event`,
      canonicalCorpusDirectory,
      resolve(canonicalCorpusDirectory, eventPathFromCorpus),
    );
    verifySignedEvent(entry, verifiedEvent.event);
    verifiedEntries.push({
      entry,
      eventText: verifiedEvent.text,
    });
  }

  return {
    format: VERIFIED_SNAPSHOT_FORMAT,
    lock,
    entries: verifiedEntries,
  };
}

export async function verifyCorpus(lockPath = DEFAULT_CORPUS_LOCK) {
  const verifiedCorpus = await readVerifiedCorpus(lockPath);
  return verifiedCorpus.lock;
}

async function main() {
  const snapshotMode = process.argv[2] === '--snapshot-json';
  const lockArgument = process.argv[snapshotMode ? 3 : 2];
  const lockPath = lockArgument ? resolve(lockArgument) : DEFAULT_CORPUS_LOCK;
  try {
    const verifiedCorpus = await readVerifiedCorpus(lockPath);
    if (snapshotMode) {
      process.stdout.write(JSON.stringify(verifiedCorpus));
      return;
    }
    console.log(
      `EXTERNAL_NAPPLET_CORPUS_STRUCTURE_OK entries=${verifiedCorpus.lock.entries.length} commit=${verifiedCorpus.lock.source.commit} mode=offline`,
    );
  } catch (error) {
    if (error instanceof CorpusVerificationError) {
      console.error(
        `EXTERNAL_NAPPLET_CORPUS_${error.category.toUpperCase()} code=${error.code} message=${error.message}`,
      );
      process.exitCode = error.category === 'trust' ? 2 : 3;
      return;
    }
    throw error;
  }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  await main();
}
