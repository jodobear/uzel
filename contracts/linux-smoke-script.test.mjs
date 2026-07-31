import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import {
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

const script = readFileSync(
  join(process.cwd(), 'scripts', 'linux-run-smoke.sh'),
  'utf8',
);

function napdReadyPattern() {
  const assignment = script.match(/^NAPD_READY_PATTERN='([^']+)'$/m);
  assert.ok(assignment, 'linux smoke must define one napd readiness pattern');
  return assignment[1];
}

function matchesLog(contents) {
  const directory = mkdtempSync(join(tmpdir(), 'uzel-smoke-marker-'));
  const log = join(directory, 'uzel.log');
  writeFileSync(log, contents);
  try {
    return spawnSync('rg', ['-q', napdReadyPattern(), log]).status === 0;
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

test('napd readiness survives shared-log output before the marker', () => {
  assert.equal(
    matchesLog(
      '\u001b[1mconcurrent cargo outputUZEL_NAPD_READY role=runtime-authority\n',
    ),
    true,
  );
  assert.equal(matchesLog('UZEL_NAPD_READY role=wrong-authority\n'), false);
  assert.equal(
    matchesLog('UZEL_NAPD_READY role=runtime-authority-extra\n'),
    false,
  );
});
