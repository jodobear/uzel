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

test('native smoke accepts only an exact packaged launcher override', () => {
  assert.match(script, /UZEL_SMOKE_LAUNCHER=\$\{UZEL_SMOKE_LAUNCHER:-\}/);
  assert.match(script, /UZEL_SMOKE_LAUNCHER must be an exact packaged/);
  assert.match(script, /setsid "\$UZEL_SMOKE_LAUNCHER"/);
});

test('packaged launcher serializes ownership and removes only its exact socket', () => {
  const flake = readFileSync(join(process.cwd(), 'flake.nix'), 'utf8');
  assert.match(flake, /lock_file="\\\$runtime_dir\/uzel-launcher\.lock"/);
  assert.match(flake, /flock -n 9/);
  assert.match(flake, /shell_pid=\\\$!/);
  assert.match(flake, /trap 'handle_signal INT' INT/);
  assert.match(flake, /trap 'handle_signal TERM' TERM/);
  assert.match(flake, /kill -s "\\\$signal" "\\\$shell_pid"/);
  assert.match(flake, /kill -s "\\\$signal" "\\\$daemon_pid"/);
  assert.equal((flake.match(/set -m/g) ?? []).length, 2);
  assert.equal((flake.match(/set \+m/g) ?? []).length, 2);
  assert.match(flake, /if \[ -z "\\\$received_signal" \]; then\s+signal_children TERM/s);
  assert.match(flake, /wait "\\\$shell_pid"/);
  assert.match(flake, /socket_identity=.*stat -Lc '%d:%i'/);
  assert.match(flake, /current_socket_identity.*= "\\\$socket_identity"/s);
  assert.match(flake, /rm -f -- "\\\$socket"/);
  assert.doesNotMatch(flake, /rm -rf/);
});

test('launcher-only evidence cannot claim packaged WebKit execution', () => {
  const packageScript = readFileSync(join(process.cwd(), 'scripts/package-smoke.sh'), 'utf8');
  assert.equal((packageScript.match(/PACKAGE_LAUNCHER_ONLY_OK/g) ?? []).length, 1);
  assert.equal((packageScript.match(/PACKAGE_SMOKE_OK/g) ?? []).length, 1);
  assert.match(packageScript, /UZEL_PACKAGE_LAUNCHER_ONLY:-0} == 1[\s\S]*PACKAGE_LAUNCHER_ONLY_OK[^\n]*webkit=not-run[\s\S]*else[\s\S]*webkit=weston[\s\S]*PACKAGE_SMOKE_OK/);
  assert.match(packageScript, /run_signal_probe TERM 143/);
  assert.match(packageScript, /run_signal_probe INT 130/);
  assert.match(packageScript, /PACKAGE_SIGNAL_OK signal=%s status=%s children=reaped socket=retired/);
});
