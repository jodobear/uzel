import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { cp, mkdtemp, readFile, rename, rm, symlink, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  classifyEvidenceReadError,
  CorpusVerificationError,
  DEFAULT_CORPUS_LOCK,
  readBoundedEvidence,
  verifyCorpus,
  verifySignedEvent,
} from './verify-external-napplet-corpus.mjs';

const DEFAULT_CORPUS_DIRECTORY = dirname(DEFAULT_CORPUS_LOCK);
const VERIFIER_SCRIPT = fileURLToPath(
  new URL('./verify-external-napplet-corpus.mjs', import.meta.url),
);

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

test('missing and non-regular lock evidence are typed trust failures', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-read-'));
  try {
    await assert.rejects(
      verifyCorpus(join(temporaryDirectory, 'missing.json')),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'missing-corpus-data',
    );
    await assert.rejects(
      verifyCorpus(temporaryDirectory),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'invalid-lock' &&
        error.message.includes('evidence must be a regular file'),
    );

    const cli = spawnSync(process.execPath, [VERIFIER_SCRIPT, temporaryDirectory], {
      encoding: 'utf8',
    });
    assert.equal(cli.status, 2);
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('read-error classification rejects malformed evidence but preserves operational failures', () => {
  for (const errorCode of ['EISDIR', 'ENOTDIR', 'ELOOP', 'ENXIO']) {
    assert.deepEqual(classifyEvidenceReadError(errorCode), {
      category: 'trust',
      code: 'invalid-lock',
    });
  }
  for (const errorCode of ['EACCES', 'EIO']) {
    assert.deepEqual(classifyEvidenceReadError(errorCode), {
      category: 'infrastructure',
      code: 'corpus-read-failed',
    });
  }
});

test('null and non-object lock roots fail as invalid lock trust errors', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-root-'));
  const lockPath = join(temporaryDirectory, 'corpus.lock.json');
  try {
    for (const root of [null, [], 'not-an-object']) {
      await writeFile(lockPath, JSON.stringify(root), 'utf8');
      await assert.rejects(
        verifyCorpus(lockPath),
        (error) =>
          error instanceof CorpusVerificationError &&
          error.category === 'trust' &&
          error.code === 'invalid-lock' &&
          error.message.includes('root must be an object'),
      );
    }
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('null and non-object entries fail as invalid lock trust errors', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-entry-'));
  const lockPath = join(temporaryDirectory, 'corpus.lock.json');
  try {
    for (const entry of [null, [], 'not-an-object']) {
      const lock = await loadLock();
      lock.entries[0] = entry;
      await writeFile(lockPath, JSON.stringify(lock), 'utf8');
      await assert.rejects(
        verifyCorpus(lockPath),
        (error) =>
          error instanceof CorpusVerificationError &&
          error.category === 'trust' &&
          error.code === 'invalid-lock' &&
          error.message.includes('entry must be an object'),
      );

      const cli = spawnSync(process.execPath, [VERIFIER_SCRIPT, lockPath], {
        encoding: 'utf8',
      });
      assert.equal(cli.status, 2);
    }
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('source commit and commit URL remain pinned to the audited source', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-source-'));
  const lockPath = join(temporaryDirectory, 'corpus.lock.json');
  try {
    const replacementCommit = '0'.repeat(40);
    const replacedSource = await loadLock();
    replacedSource.source.commit = replacementCommit;
    replacedSource.source.commitUrl =
      `https://github.com/hzrd149/napplelets/commit/${replacementCommit}`;
    await writeFile(lockPath, JSON.stringify(replacedSource), 'utf8');
    await assert.rejects(
      verifyCorpus(lockPath),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'invalid-lock' &&
        error.message.includes('source.commit must remain'),
    );

    const replacedUrl = await loadLock();
    replacedUrl.source.commitUrl = 'https://github.com/hzrd149/napplelets/commit/not-the-pin';
    await writeFile(lockPath, JSON.stringify(replacedUrl), 'utf8');
    await assert.rejects(
      verifyCorpus(lockPath),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'invalid-lock' &&
        error.message.includes('source.commitUrl must remain'),
    );
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('license and audit timestamps remain pinned to the audited provenance', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-provenance-'));
  const lockPath = join(temporaryDirectory, 'corpus.lock.json');
  const mutations = [
    {
      field: 'licenseUrl',
      value: 'https://github.com/hzrd149/napplelets/blob/main/LICENSE',
    },
    { field: 'auditedOn', value: '2026-08-01' },
    { field: 'publishedAt', value: '2026-07-26T11:52:05Z' },
  ];

  try {
    for (const mutation of mutations) {
      const lock = await loadLock();
      lock.source[mutation.field] = mutation.value;
      await writeFile(lockPath, JSON.stringify(lock), 'utf8');
      await assert.rejects(
        verifyCorpus(lockPath),
        (error) =>
          error instanceof CorpusVerificationError &&
          error.category === 'trust' &&
          error.code === 'invalid-lock' &&
          error.message.includes(`source.${mutation.field} must remain`),
      );
    }
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('event ids remain pinned by audited name instead of caller-lock self-consistency', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-event-pin-'));
  const corpusDirectory = join(temporaryDirectory, 'corpus');
  try {
    await cp(DEFAULT_CORPUS_DIRECTORY, corpusDirectory, { recursive: true });
    const lockPath = join(corpusDirectory, 'corpus.lock.json');
    const lock = JSON.parse(await readFile(lockPath, 'utf8'));
    const eventPath = join(corpusDirectory, lock.entries[0].eventFile);
    const event = JSON.parse(await readFile(eventPath, 'utf8'));
    lock.entries[0].eventId = '0'.repeat(64);
    event.id = lock.entries[0].eventId;
    await writeFile(lockPath, JSON.stringify(lock), 'utf8');
    await writeFile(eventPath, JSON.stringify(event), 'utf8');

    await assert.rejects(
      verifyCorpus(lockPath),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'event-id-drift' &&
        error.message.includes('exact audited value'),
    );
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('domains remain pinned to each audited napplet capability set', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-domains-'));
  const corpusDirectory = join(temporaryDirectory, 'corpus');
  try {
    await cp(DEFAULT_CORPUS_DIRECTORY, corpusDirectory, { recursive: true });
    const lockPath = join(corpusDirectory, 'corpus.lock.json');
    const lock = JSON.parse(await readFile(lockPath, 'utf8'));
    lock.entries[1].domains = ['wallet'];
    await writeFile(lockPath, JSON.stringify(lock), 'utf8');

    await assert.rejects(
      verifyCorpus(lockPath),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'capability-drift' &&
        error.message.includes('exact audited capability set'),
    );
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('unsigned coordinate and byte-length facts remain pinned by audited name', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-unsigned-pins-'));
  const lockPath = join(temporaryDirectory, 'corpus.lock.json');
  const mutations = [
    {
      mutate(lock) {
        lock.entries[0].naddr = lock.entries[1].naddr;
      },
      code: 'coordinate-drift',
      message: 'naddr is not the exact audited value',
    },
    {
      mutate(lock) {
        lock.entries[0].relayHints = ['wss://relay.example'];
      },
      code: 'coordinate-drift',
      message: 'relay hints are not the exact audited values',
    },
    {
      mutate(lock) {
        lock.entries[0].artifact.sizeBytes += 1;
      },
      code: 'artifact-size-drift',
      message: 'artifact byte length is not the exact audited value',
    },
  ];

  try {
    for (const mutation of mutations) {
      const lock = await loadLock();
      mutation.mutate(lock);
      await writeFile(lockPath, JSON.stringify(lock), 'utf8');
      await assert.rejects(
        verifyCorpus(lockPath),
        (error) =>
          error instanceof CorpusVerificationError &&
          error.category === 'trust' &&
          error.code === mutation.code &&
          error.message.includes(mutation.message),
      );
    }
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('symlinked event files are rejected even when their target remains inside the corpus', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-symlink-'));
  const corpusDirectory = join(temporaryDirectory, 'corpus');
  try {
    await cp(DEFAULT_CORPUS_DIRECTORY, corpusDirectory, { recursive: true });
    const eventPath = join(corpusDirectory, 'events/good-morning.json');
    const backingPath = join(corpusDirectory, 'events/good-morning.backing.json');
    await rename(eventPath, backingPath);
    await symlink('good-morning.backing.json', eventPath);

    await assert.rejects(
      verifyCorpus(join(corpusDirectory, 'corpus.lock.json')),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'invalid-lock' &&
        error.message.includes('symlinked evidence'),
    );
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('event directories and non-directory path components are invalid evidence trust failures', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-event-types-'));
  const corpusDirectory = join(temporaryDirectory, 'corpus');
  try {
    await cp(DEFAULT_CORPUS_DIRECTORY, corpusDirectory, { recursive: true });
    const lockPath = join(corpusDirectory, 'corpus.lock.json');

    const directoryLock = JSON.parse(await readFile(lockPath, 'utf8'));
    directoryLock.entries[0].eventFile = 'events';
    await writeFile(lockPath, JSON.stringify(directoryLock), 'utf8');
    await assert.rejects(
      verifyCorpus(lockPath),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'invalid-lock' &&
        error.message.includes('evidence must be a regular file'),
    );

    const nonDirectoryLock = JSON.parse(await readFile(DEFAULT_CORPUS_LOCK, 'utf8'));
    nonDirectoryLock.entries[0].eventFile = 'events/good-morning.json/child';
    await writeFile(lockPath, JSON.stringify(nonDirectoryLock), 'utf8');
    await assert.rejects(
      verifyCorpus(lockPath),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'invalid-lock' &&
        error.message.includes('ENOTDIR'),
    );
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('corpus lock rejects symlinks and FIFOs without following or blocking', { timeout: 2000 }, async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-lock-types-'));
  const symlinkCorpus = join(temporaryDirectory, 'symlink-corpus');
  const fifoCorpus = join(temporaryDirectory, 'fifo-corpus');
  try {
    await cp(DEFAULT_CORPUS_DIRECTORY, symlinkCorpus, { recursive: true });
    const symlinkLock = join(symlinkCorpus, 'corpus.lock.json');
    const backingLock = join(symlinkCorpus, 'corpus.lock.backing.json');
    await rename(symlinkLock, backingLock);
    await symlink('corpus.lock.backing.json', symlinkLock);
    await assert.rejects(
      verifyCorpus(symlinkLock),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'invalid-lock' &&
        error.message.includes('symlinked evidence'),
    );

    await cp(DEFAULT_CORPUS_DIRECTORY, fifoCorpus, { recursive: true });
    const fifoLock = join(fifoCorpus, 'corpus.lock.json');
    await rm(fifoLock);
    const mkfifo = spawnSync('mkfifo', [fifoLock], { encoding: 'utf8', timeout: 1000 });
    assert.equal(mkfifo.status, 0, mkfifo.stderr);
    await assert.rejects(
      verifyCorpus(fifoLock),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'invalid-lock' &&
        error.message.includes('evidence must be a regular file'),
    );
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('lock and event evidence enforce fixed byte caps before parsing', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-caps-'));
  const lockCorpus = join(temporaryDirectory, 'lock-corpus');
  const eventCorpus = join(temporaryDirectory, 'event-corpus');
  try {
    await cp(DEFAULT_CORPUS_DIRECTORY, lockCorpus, { recursive: true });
    const oversizedLockPath = join(lockCorpus, 'corpus.lock.json');
    const lockText = await readFile(oversizedLockPath, 'utf8');
    await writeFile(oversizedLockPath, `${lockText}${' '.repeat(16 * 1024)}`, 'utf8');
    await assert.rejects(
      verifyCorpus(oversizedLockPath),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'invalid-lock' &&
        error.message.includes('evidence exceeds 16384-byte limit'),
    );

    await cp(DEFAULT_CORPUS_DIRECTORY, eventCorpus, { recursive: true });
    const eventLockPath = join(eventCorpus, 'corpus.lock.json');
    const eventLock = JSON.parse(await readFile(eventLockPath, 'utf8'));
    const oversizedEventPath = join(eventCorpus, eventLock.entries[0].eventFile);
    const eventText = await readFile(oversizedEventPath, 'utf8');
    await writeFile(oversizedEventPath, `${eventText}${' '.repeat(16 * 1024)}`, 'utf8');
    await assert.rejects(
      verifyCorpus(eventLockPath),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'invalid-lock' &&
        error.message.includes('evidence exceeds 16384-byte limit'),
    );
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('bounded descriptor reads consume at most limit plus one after grow-after-stat', async () => {
  let largestRead = 0;
  const growingHandle = {
    async stat() {
      return { isFile: () => true, size: 1 };
    },
    async read(buffer, offset, length) {
      largestRead = Math.max(largestRead, length);
      buffer.fill(0x20, offset, offset + length);
      return { bytesRead: length };
    },
  };

  await assert.rejects(
    readBoundedEvidence(growingHandle, 'growing event', 8),
    (error) =>
      error instanceof CorpusVerificationError &&
      error.category === 'trust' &&
      error.code === 'invalid-lock' &&
      error.message.includes('evidence exceeds 8-byte limit'),
  );
  assert.equal(largestRead, 9);
});

test('descriptor metadata and read failures remain infrastructure failures', async () => {
  for (const [operation, code] of [
    ['stat', 'EIO'],
    ['read', 'EACCES'],
  ]) {
    const operationalError = Object.assign(new Error(`${operation} failed`), { code });
    const handle = {
      async stat() {
        if (operation === 'stat') {
          throw operationalError;
        }
        return { isFile: () => true, size: 1 };
      },
      async read() {
        throw operationalError;
      },
    };
    await assert.rejects(
      readBoundedEvidence(handle, 'operational event', 8),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'infrastructure' &&
        error.code === 'corpus-read-failed',
    );
  }
});

test('canonical event containment rejects a symlinked parent that escapes the corpus', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-containment-'));
  const corpusDirectory = join(temporaryDirectory, 'corpus');
  const outsideEvents = join(temporaryDirectory, 'outside-events');
  try {
    await cp(DEFAULT_CORPUS_DIRECTORY, corpusDirectory, { recursive: true });
    await cp(join(corpusDirectory, 'events'), outsideEvents, { recursive: true });
    await rm(join(corpusDirectory, 'events'), { recursive: true, force: true });
    await symlink(outsideEvents, join(corpusDirectory, 'events'), 'dir');

    await assert.rejects(
      verifyCorpus(join(corpusDirectory, 'corpus.lock.json')),
      (error) =>
        error instanceof CorpusVerificationError &&
        error.category === 'trust' &&
        error.code === 'invalid-lock' &&
        error.message.includes('escapes corpus directory'),
    );
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
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

test('an unknown name cannot omit its automation scope and pass as undefined', async () => {
  const lock = await loadLock();
  lock.entries[0].name = 'not-audited';
  delete lock.entries[0].safeAutomation;
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
        error.message.includes('audited automation allowlist'),
    );
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});

test('a caller-supplied lock cannot omit an audited corpus entry', async () => {
  const lock = await loadLock();
  lock.entries.pop();
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
        error.message.includes('exactly 4 audited napplets'),
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

test('lock title drift fails closed against the signed title', async () => {
  const lock = await loadLock();
  const entry = lock.entries[0];
  const event = await loadEvent(entry);
  entry.title = 'Misleading replacement title';

  assert.throws(
    () => verifySignedEvent(entry, event),
    (error) =>
      error instanceof CorpusVerificationError &&
      error.category === 'trust' &&
      error.code === 'signed-event-drift' &&
      error.message.includes('signed title drifted'),
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
  assert.deepEqual([...trust].sort(), [
    'aggregate-drift',
    'artifact-path-digest-mismatch',
    'artifact-size-drift',
    'capability-drift',
    'coordinate-drift',
    'event-id-drift',
    'invalid-event-signature',
  ]);
});

test('failure policy rejects reclassification or omission of audited failure codes', async () => {
  const temporaryDirectory = await mkdtemp(join(tmpdir(), 'uzel-corpus-policy-'));
  const lockPath = join(temporaryDirectory, 'corpus.lock.json');
  const mutations = [
    (lock) => {
      lock.failurePolicy.trust = lock.failurePolicy.trust.filter(
        (code) => code !== 'invalid-event-signature',
      );
    },
    (lock) => {
      lock.failurePolicy.trust = lock.failurePolicy.trust.filter(
        (code) => code !== 'invalid-event-signature',
      );
      lock.failurePolicy.infrastructure = [
        'artifact-server-unavailable',
        'invalid-event-signature',
        'relay-unavailable',
      ];
    },
  ];

  try {
    for (const mutate of mutations) {
      const lock = await loadLock();
      mutate(lock);
      await writeFile(lockPath, JSON.stringify(lock), 'utf8');
      await assert.rejects(
        verifyCorpus(lockPath),
        (error) =>
          error instanceof CorpusVerificationError &&
          error.category === 'trust' &&
          error.code === 'invalid-lock' &&
          error.message.includes('exact audited classifications'),
      );
    }
  } finally {
    await rm(temporaryDirectory, { recursive: true, force: true });
  }
});
