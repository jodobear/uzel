import assert from 'node:assert/strict';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

import {
  CorpusVerificationError,
  DEFAULT_CORPUS_LOCK,
  verifyCorpus,
  verifySignedEvent,
} from './verify-external-napplet-corpus.mjs';

async function loadLock() {
  return JSON.parse(await readFile(DEFAULT_CORPUS_LOCK, 'utf8'));
}

async function loadEvent(entry) {
  const eventUrl = new URL(`../fixtures/external-napplet-corpus/${entry.eventFile}`, import.meta.url);
  return JSON.parse(await readFile(eventUrl, 'utf8'));
}

test('external corpus verifies offline with exact audited tuples', async () => {
  const lock = await verifyCorpus();

  assert.equal(lock.source.commit, 'aa4dc7a0799d95e3066b50055b29685d6e376045');
  assert.equal(lock.toolchain.nakVersion, '0.20.1');
  assert.deepEqual(
    lock.entries.map((entry) => entry.name),
    ['good-morning', 'rubik-cube', 'nap-feed', 'wifi-map'],
  );
  assert.deepEqual(
    lock.entries.map((entry) => ({
      name: entry.name,
      safeAutomation: entry.safeAutomation,
      naddr: entry.naddr,
      eventId: entry.eventId,
      author: entry.author,
      dTag: entry.dTag,
      artifact: entry.artifact,
      domains: entry.domains,
    })),
    [
      {
        name: 'good-morning',
        safeAutomation: 'control',
        naddr:
          'naddr1qqxxwmm0vskk6mmjde5kuecpzemhxue69uhhyetvv9ujuurjd9kkzmpwdejhgq3qye5ptcxfyyxl5vjvdjar2ua3f0hynkjzpx552mu5snj3qmx5pzjsxpqqqzynjsul3vr',
        eventId: 'caef62ea8feb506d621f7f4c514fcb4322280cf3041d9e4801354b5d615f9d3b',
        author: '266815e0c9210dfa324c6cba3573b14bee49da4209a9456f9484e5106cd408a5',
        dTag: 'good-morning',
        artifact: {
          logicalPath: '/index.html',
          sha256: 'ffd35eea5c84d03cdda74c23e1bbb2c40500f503833503aa688036faa52f3808',
          sizeBytes: 96172,
          aggregateSha256:
            '828a6df02afd56782ea20f805084acce65c53f7c37554948c1e0a64aa5a2b0a8',
        },
        domains: ['identity', 'inc', 'link', 'outbox', 'resource', 'theme'],
      },
      {
        name: 'rubik-cube',
        safeAutomation: 'zero-capability-render-input',
        naddr:
          'naddr1qq98yatzd94j6cm4vfjsz9nhwden5te0wfjkccte9ec8y6tdv9kzumn9wspzqfngzhsvjggdlgeycm96x4emzjlwf8dyyzdfg4hefp89zpkdgz99qvzqqqyf8y9pvhkw',
        eventId: '312bbf17fef533107d24c5682a9d58a62b7582252e912f90fb41d8d283038706',
        author: '266815e0c9210dfa324c6cba3573b14bee49da4209a9456f9484e5106cd408a5',
        dTag: 'rubik-cube',
        artifact: {
          logicalPath: '/index.html',
          sha256: '12057b8de4b76b931bd880e408630eb5bab58e269a5d8f9711c7ea03cb978ab8',
          sizeBytes: 501383,
          aggregateSha256:
            'e7906e9dc98f8f35359d5b6bf52eb8fd69a0726ad4a7c440c43ee14998ee8821',
        },
        domains: [],
      },
      {
        name: 'nap-feed',
        safeAutomation: 'read-only-config-outbox',
        naddr:
          'naddr1qqyxucts94nx2etyqyt8wumn8ghj7un9d3shjtnswf5k6ctv9ehx2aqzyqnxs90qeyssm73jf3kt5dtnk997ujw6ggy6j3t0jjzw2yrv6sy22qcyqqqgjwglxcuwy',
        eventId: 'e4445830b411d920e1f5fa4f74f58144f085a43e370ed39703e60207be9c7c9d',
        author: '266815e0c9210dfa324c6cba3573b14bee49da4209a9456f9484e5106cd408a5',
        dTag: 'nap-feed',
        artifact: {
          logicalPath: '/index.html',
          sha256: 'a3f03bc23808522d4f5c0c19b7b9567444173ce9105b8eef8ed50880836d28ea',
          sizeBytes: 24165,
          aggregateSha256:
            '4e1248038e9cfb6998a7981d08f5ea30ce21ecbe9b1d14f82c8a30fa18468dd1',
        },
        domains: ['config', 'identity', 'outbox', 'theme'],
      },
      {
        name: 'wifi-map',
        safeAutomation: 'read-only-storage-outbox-no-link-click',
        naddr:
          'naddr1qqy8w6txdykk6ctsqyt8wumn8ghj7un9d3shjtnswf5k6ctv9ehx2aqzyqnxs90qeyssm73jf3kt5dtnk997ujw6ggy6j3t0jjzw2yrv6sy22qcyqqqgjwggda37g',
        eventId: '49e111745256e32af6a5b3dd02e54b731cd8848a2951e0cf6de0888ab52e1b99',
        author: '266815e0c9210dfa324c6cba3573b14bee49da4209a9456f9484e5106cd408a5',
        dTag: 'wifi-map',
        artifact: {
          logicalPath: '/index.html',
          sha256: '3961e34469c74b0e194d260f9ae06e39535bd30d3bd207d28062cbc0833992fb',
          sizeBytes: 291344,
          aggregateSha256:
            '4b5a81c828423aaad3ee146e6a849cf39438f7f25ddf0a93d50362b21b41fbc9',
        },
        domains: ['link', 'outbox', 'storage', 'theme'],
      },
    ],
  );
});

test('caller-supplied relative lock path verifies against the same corpus', async () => {
  const lock = await verifyCorpus('fixtures/external-napplet-corpus/corpus.lock.json');
  assert.equal(lock.entries.length, 4);
});

test('automation scope drift fails closed before later harnesses can consume it', async () => {
  const lock = await loadLock();
  lock.entries[0].safeAutomation = 'upload-and-publish';
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-lock-'));
  const lockPath = join(temporaryDirectory, 'corpus.lock.json');
  try {
    await writeFile(lockPath, JSON.stringify(lock), 'utf8');
    await assert.rejects(
      verifyCorpus(lockPath),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'invalid-lock' &&
        error.message.includes('automation scope'),
    );
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('tuple drift is a trust failure before launch', async () => {
  const lock = await loadLock();
  const entry = lock.entries[0];
  const event = await loadEvent(entry);
  event.id = '0'.repeat(64);

  assert.throws(
    () => verifySignedEvent(entry, event),
    (error) =>
      error instanceof CorpusVerificationError &&
      error.category === 'trust' &&
      error.code === 'event-id-drift',
  );
});

test('failure policy keeps external availability separate from trust failures', async () => {
  const lock = await loadLock();
  const infrastructure = new Set(lock.failurePolicy.infrastructure);
  const trust = new Set(lock.failurePolicy.trust);

  assert.deepEqual([...infrastructure].sort(), [
    'artifact-server-unavailable',
    'relay-unavailable',
  ]);
  assert.equal([...infrastructure].some((code) => trust.has(code)), false);
  assert.equal(trust.has('invalid-event-signature'), true);
  assert.equal(trust.has('artifact-path-digest-mismatch'), true);
  assert.equal(trust.has('capability-drift'), true);
});
