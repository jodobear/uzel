import assert from 'node:assert/strict';
import { execFileSync, spawn } from 'node:child_process';
import { createHash } from 'node:crypto';
import { after, before, test } from 'node:test';
import { access, mkdir, readFile, readdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import { fileURLToPath } from 'node:url';

import { chromium } from 'playwright';
import { createServer } from 'vite';

import { DEFAULT_KEYBINDINGS, KEYBINDING_ACTIONS } from '../../src/preferences.js';
import {
  ARTIFACT_ROOT,
  EXTERNAL_SENTINEL_URL,
  FIXTURE_PATH,
  HARNESS_MODE,
  SCENARIOS,
  TEST_NADDR,
  VIEWPORTS,
} from './playwright.config.mjs';

const APP_ROOT = fileURLToPath(new URL('../..', import.meta.url));
const REPOSITORY_ROOT = fileURLToPath(new URL('../../../..', import.meta.url));
const SELF_PATH = fileURLToPath(import.meta.url);
const FAULT_CHILD = process.env.UZEL_UI_FAULT_PROBE_CHILD === '1';
const requestedScenarios = process.env.UZEL_UI_SCENARIOS
  ? process.env.UZEL_UI_SCENARIOS.split(',').map((value) => value.trim()).filter(Boolean)
  : [...SCENARIOS];
const unknownScenarios = requestedScenarios.filter((scenario) => !SCENARIOS.includes(scenario));
assert.deepEqual(unknownScenarios, [], `unknown UZEL_UI_SCENARIOS: ${unknownScenarios.join(', ')}`);
const selectedScenarios = new Set(requestedScenarios);
const gitSha = execFileSync('git', ['rev-parse', 'HEAD'], {
  cwd: REPOSITORY_ROOT,
  encoding: 'utf8',
}).trim();
const gitDirty = execFileSync('git', ['status', '--porcelain'], {
  cwd: REPOSITORY_ROOT,
  encoding: 'utf8',
}).trim().length > 0;
const outcomes = [];
const screenshots = [];
let baseUrl;
let browser;
let browserChildPid = null;
let browserServer;
let decodedNaddr;
let fixtureRecords;
let server;

function eventTag(event, name, qualifier) {
  const matches = event.tags.filter((tag) =>
    Array.isArray(tag) && tag[0] === name && (qualifier === undefined || tag[2] === qualifier));
  assert.equal(matches.length, 1, `fixture event must have exactly one ${name} tag`);
  return matches[0];
}

function nappletRequires(html) {
  const metas = html.match(/<meta\b[^>]*>/giu) ?? [];
  const requires = metas.flatMap((meta) => {
    const name = /\bname\s*=\s*(["'])napplet-requires\1/iu.exec(meta);
    if (!name) return [];
    const content = /\bcontent\s*=\s*(["'])(.*?)\1/iu.exec(meta)?.[2];
    assert.ok(content, 'napplet-requires meta must have non-empty content');
    return content.split(',').map((domain) => domain.trim()).filter(Boolean);
  });
  assert.equal(new Set(requires).size, requires.length, 'napplet-requires domains must be unique');
  return requires.sort();
}

async function loadFixtureRecord(name) {
  const directory = new URL(`../../../../fixtures/${name}/`, import.meta.url);
  const [htmlBytes, eventText] = await Promise.all([
    readFile(new URL('index.html', directory)),
    readFile(new URL('event.json', directory), 'utf8'),
  ]);
  const html = htmlBytes.toString('utf8');
  const event = JSON.parse(eventText);
  assert.ok(Array.isArray(event.tags), `${name} event tags are missing`);
  const pathTag = eventTag(event, 'path');
  const aggregateTag = eventTag(event, 'x', 'aggregate');
  const dTag = eventTag(event, 'd');
  const titleTag = eventTag(event, 'title');
  const htmlSha256 = createHash('sha256').update(htmlBytes).digest('hex');
  assert.equal(htmlSha256, pathTag[2], `${name} HTML does not match its signed path hash`);
  const requiredCapabilities = nappletRequires(html);
  const manifestCapabilities = event.tags
    .filter((tag) => Array.isArray(tag) && tag[0] === 'requires')
    .map((tag) => tag[1]);
  return Object.freeze({
    html,
    author: event.pubkey,
    eventId: event.id,
    kind: event.kind,
    dTag: dTag[1],
    aggregateHash: aggregateTag[1],
    path: pathTag[1],
    pathSha256: pathTag[2],
    title: titleTag[1],
    requiredCapabilities,
    domains: [...new Set([...manifestCapabilities, ...requiredCapabilities])].sort(),
  });
}

function safeName(value) {
  return value.replaceAll(/[^a-zA-Z0-9._-]+/g, '-');
}

function caseDirectory(viewport, scenario, name) {
  return join(ARTIFACT_ROOT, safeName(scenario), safeName(viewport.name), safeName(name));
}

async function screenshot(page, viewport, scenario, name, label) {
  const directory = caseDirectory(viewport, scenario, name);
  await mkdir(directory, { recursive: true });
  const path = join(directory, `${safeName(label)}.png`);
  await page.screenshot({ path, fullPage: false });
  screenshots.push({
    path,
    gitSha,
    viewport: { ...viewport },
    mode: HARNESS_MODE,
    scenario,
    name,
    label,
  });
  return path;
}

async function capturePageState(page) {
  const frames = [];
  for (const frame of page.frames()) {
    frames.push({
      name: frame.name(),
      url: frame.url(),
      html: await frame.content().catch((frameError) => `<!-- ${String(frameError)} -->`),
    });
  }
  return {
    screenshot: await page.screenshot({ fullPage: false }),
    topDom: await page.content(),
    frames,
  };
}

async function dumpFailure(page, guarded, viewport, scenario, name, error, capturedState = null) {
  const directory = caseDirectory(viewport, scenario, name);
  await mkdir(directory, { recursive: true });
  const state = capturedState ?? await capturePageState(page).catch(() => null);
  if (state?.screenshot) {
    const path = join(directory, 'failure.png');
    await writeFile(path, state.screenshot);
    screenshots.push({
      path,
      gitSha,
      viewport: { ...viewport },
      mode: HARNESS_MODE,
      scenario,
      name,
      label: 'failure',
    });
  }
  await writeFile(join(directory, 'failure-dom.html'), state?.topDom ?? '');
  await writeFile(join(directory, 'failure-frames.json'), `${JSON.stringify(state?.frames ?? [], null, 2)}\n`);
  await writeFile(join(directory, 'failure.json'), `${JSON.stringify({
    gitSha,
    gitDirty,
    viewport,
    mode: HARNESS_MODE,
    scenario,
    name,
    error: error instanceof Error ? { name: error.name, message: error.message, stack: error.stack } : String(error),
    console: guarded.consoleEntries,
    acknowledgedProblems: guarded.acknowledgedProblems,
    acknowledgedExternalRequests: guarded.acknowledgedExternalRequests,
    acknowledgedFailedRequests: guarded.acknowledgedFailedRequests,
    pageErrors: guarded.pageErrors,
    externalRequests: guarded.externalRequests,
    externalWebSockets: guarded.externalWebSockets,
    failedRequests: guarded.failedRequests,
  }, null, 2)}\n`);
}

async function guardedPage(viewport, scenario) {
  const context = await browser.newContext({
    viewport: { width: viewport.width, height: viewport.height },
    colorScheme: 'dark',
    reducedMotion: 'reduce',
  });
  const page = await context.newPage();
  const browserProblems = [];
  const acknowledgedProblems = [];
  const acknowledgedExternalRequests = [];
  const acknowledgedFailedRequests = [];
  const consoleEntries = [];
  const externalRequests = [];
  const externalWebSockets = [];
  const failedRequests = [];
  const pageErrors = [];
  const baseOrigin = new URL(baseUrl).origin;
  const baseHost = new URL(baseUrl).host;

  page.on('console', (message) => {
    const entry = { type: message.type(), text: message.text(), location: message.location() };
    consoleEntries.push(entry);
    if (message.type() === 'error' || message.type() === 'warning') {
      browserProblems.push(`console.${message.type()}: ${message.text()}`);
    }
  });
  page.on('pageerror', (error) => {
    pageErrors.push({ name: error.name, message: error.message, stack: error.stack });
    browserProblems.push(`pageerror: ${error.message}`);
  });
  page.on('requestfailed', (request) => {
    failedRequests.push({ url: request.url(), failure: request.failure()?.errorText ?? 'unknown' });
  });
  page.on('websocket', (socket) => {
    if (new URL(socket.url()).host !== baseHost) externalWebSockets.push(socket.url());
  });
  await page.route('**/*', async (route) => {
    const url = route.request().url();
    const parsed = new URL(url);
    if (parsed.origin === baseOrigin || ['about:', 'blob:', 'data:'].includes(parsed.protocol)) {
      await route.continue();
      return;
    }
    externalRequests.push(url);
    await route.abort('blockedbyclient');
  });
  await page.addInitScript((fixtures) => {
    Object.defineProperty(window, '__UZEL_UI_FIXTURES__', {
      configurable: false,
      enumerable: false,
      writable: false,
      value: Object.freeze(fixtures),
    });
  }, fixtureRecords);
  await page.goto(`${baseUrl}${FIXTURE_PATH}?scenario=${encodeURIComponent(scenario)}`, {
    waitUntil: 'domcontentloaded',
  });
  return {
    acknowledgedExternalRequests,
    acknowledgedFailedRequests,
    acknowledgedProblems,
    browserProblems,
    consoleEntries,
    context,
    externalRequests,
    externalWebSockets,
    failedRequests,
    page,
    pageErrors,
  };
}

async function quiescePage(page) {
  await page.evaluate(() => new Promise((resolve) => {
    requestAnimationFrame(() => requestAnimationFrame(resolve));
  }));
  await page.waitForTimeout(100);
}

async function waitForLocatorAttribute(locator, name, predicate, message, attempts = 100) {
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    const value = await locator.getAttribute(name);
    if (predicate(value)) return value;
    await delay(25);
  }
  assert.fail(message);
}

function assertGuardedCase(guarded) {
  assert.deepEqual(guarded.browserProblems, [], 'browser console/page errors must stay empty');
  assert.deepEqual(guarded.externalWebSockets, [], 'external WebSockets are forbidden');
  assert.deepEqual(
    guarded.externalRequests,
    guarded.acknowledgedExternalRequests,
    'every external request must be explicitly acknowledged by its scenario',
  );
  assert.deepEqual(
    guarded.failedRequests,
    guarded.acknowledgedFailedRequests,
    'every failed request must be explicitly acknowledged by its scenario',
  );
}

async function runCase(name, viewport, scenario, body) {
  const guarded = await guardedPage(viewport, scenario);
  const startedAt = new Date().toISOString();
  let capturedState = null;
  let contextClosed = false;
  try {
    await body(guarded.page, guarded);
    await quiescePage(guarded.page);
    capturedState = await capturePageState(guarded.page);
    await guarded.context.close();
    contextClosed = true;
    await delay(0);
    assertGuardedCase(guarded);
    const outcome = {
      name,
      scenario,
      viewport: { ...viewport },
      mode: HARNESS_MODE,
      gitSha,
      startedAt,
      finishedAt: new Date().toISOString(),
      status: 'passed',
      console: guarded.consoleEntries,
      acknowledgedProblems: guarded.acknowledgedProblems,
      acknowledgedExternalRequests: guarded.acknowledgedExternalRequests,
      acknowledgedFailedRequests: guarded.acknowledgedFailedRequests,
      externalRequests: guarded.externalRequests,
      failedRequests: guarded.failedRequests,
    };
    outcomes.push(outcome);
    const directory = caseDirectory(viewport, scenario, name);
    await mkdir(directory, { recursive: true });
    await writeFile(join(directory, 'case.json'), `${JSON.stringify(outcome, null, 2)}\n`);
  } catch (error) {
    const outcome = {
      name,
      scenario,
      viewport: { ...viewport },
      mode: HARNESS_MODE,
      gitSha,
      startedAt,
      finishedAt: new Date().toISOString(),
      status: 'failed',
      error: String(error),
    };
    outcomes.push(outcome);
    await dumpFailure(guarded.page, guarded, viewport, scenario, name, error, capturedState);
    throw error;
  } finally {
    if (!contextClosed) await guarded.context.close();
  }
}

async function waitForReady(page) {
  await page.getByText('Two exact builds ready through NAP-SHELL', { exact: true }).waitFor();
  await page.getByRole('region', { name: 'Composed napplet workspace' }).waitFor();
  await page.frameLocator('iframe[aria-label="Direct follows"]')
    .getByText('2 latest-known direct follows', { exact: true }).waitFor();
  await page.frameLocator('iframe[aria-label="Profile card"]')
    .getByText(/^Latest-known (?:active identity )?profile\.$/).waitFor();
}

async function openReview(page) {
  await page.getByRole('button', { name: 'Open napplet', exact: true }).click();
  const loader = page.getByRole('region', { name: 'Open napplet' });
  await loader.getByLabel('Napplet naddr').fill(TEST_NADDR);
  await loader.getByRole('button', { name: 'Review', exact: true }).click();
  return loader;
}

async function installReview(page, loader) {
  const fixture = fixtureRecords['good-morning'];
  await loader.getByRole('heading', { name: fixture.title, exact: true }).waitFor();
  await loader.getByRole('button', { name: 'Install exact build', exact: true }).click();
  const loaded = page.getByRole('region', { name: 'Loaded napplet workspace' });
  await loaded.waitFor();
  const body = page.frameLocator(`iframe[aria-label="${fixture.title}"]`).locator('body');
  await body.waitFor();
  assert.doesNotMatch(
    await body.innerText(),
    /Runtime missing NAP APIs|can't start here/,
    'loaded artifact did not receive every reviewed required domain',
  );
  return loaded;
}

async function exerciseAllSettings(page, viewport) {
  const replacementBindings = [
    'Control+Alt+1',
    'Control+Alt+2',
    'Control+Alt+3',
    'Control+Alt+4',
    'Control+Alt+5',
    'Control+Alt+6',
  ];
  await page.getByRole('button', { name: 'Settings', exact: true }).click();
  let settings = page.getByRole('region', { name: 'Settings' });
  await settings.waitFor();
  await screenshot(page, viewport, 'ready', 'complete-renderer-flow', 'settings');
  for (const [index, action] of KEYBINDING_ACTIONS.entries()) {
    const row = settings.getByRole('group', { name: action.label, exact: true });
    await row.getByRole('button', { name: 'Change', exact: true }).click();
    await page.keyboard.press(replacementBindings[index]);
    await row.getByText(replacementBindings[index], { exact: true }).waitFor();
  }
  await settings.getByRole('button', { name: 'Save settings', exact: true }).click();
  await settings.getByText('Saved.', { exact: true }).waitFor();
  await settings.getByRole('button', { name: 'Close', exact: true }).click();
  await page.getByRole('button', { name: 'Settings', exact: true }).click();
  settings = page.getByRole('region', { name: 'Settings' });
  for (const [index, action] of KEYBINDING_ACTIONS.entries()) {
    const row = settings.getByRole('group', { name: action.label, exact: true });
    await row.getByText(replacementBindings[index], { exact: true }).waitFor();
  }
  await settings.getByRole('button', { name: 'Reset defaults', exact: true }).click();
  await settings.getByText('Defaults restored. Save to apply.', { exact: true }).waitFor();
  for (const action of KEYBINDING_ACTIONS) {
    const row = settings.getByRole('group', { name: action.label, exact: true });
    await row.getByText(DEFAULT_KEYBINDINGS[action.id], { exact: true }).waitFor();
  }
  await settings.getByRole('button', { name: 'Save settings', exact: true }).click();
  await settings.getByText('Saved.', { exact: true }).waitFor();
  await settings.getByRole('button', { name: 'Close', exact: true }).click();
}

function pidExists(pid) {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    if (error?.code === 'ESRCH') return false;
    throw error;
  }
}

async function captureProcessTree(rootPid) {
  if (rootPid === null || !pidExists(rootPid)) return [];
  const parentByPid = new Map();
  for (const entry of await readdir('/proc', { withFileTypes: true })) {
    if (!entry.isDirectory() || !/^\d+$/.test(entry.name)) continue;
    const pid = Number(entry.name);
    try {
      const stat = await readFile(`/proc/${pid}/stat`, 'utf8');
      const suffix = stat.slice(stat.lastIndexOf(') ') + 2).split(' ');
      parentByPid.set(pid, Number(suffix[1]));
    } catch (error) {
      if (error?.code !== 'ENOENT' && error?.code !== 'ESRCH') throw error;
    }
  }
  const captured = new Set([rootPid]);
  let added = true;
  while (added) {
    added = false;
    for (const [pid, parentPid] of parentByPid) {
      if (!captured.has(pid) && captured.has(parentPid)) {
        captured.add(pid);
        added = true;
      }
    }
  }
  return [...captured].sort((left, right) => left - right);
}

async function waitForPidsExit(pids, attempts = 80) {
  const remaining = new Set(pids);
  for (let attempt = 0; attempt < attempts && remaining.size > 0; attempt += 1) {
    for (const pid of remaining) {
      if (!pidExists(pid)) remaining.delete(pid);
    }
    if (remaining.size > 0) await delay(50);
  }
  return pids.map((pid) => ({ pid, exited: !remaining.has(pid) }));
}

function processGroupExists(groupId) {
  try {
    process.kill(-groupId, 0);
    return true;
  } catch (error) {
    if (error?.code === 'ESRCH') return false;
    throw error;
  }
}

function signalProcessGroup(groupId, signal) {
  try {
    process.kill(-groupId, signal);
    return true;
  } catch (error) {
    if (error?.code === 'ESRCH') return false;
    throw error;
  }
}

async function waitForProcessGroupExit(groupId, attempts = 40) {
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    if (!processGroupExists(groupId)) return true;
    await delay(50);
  }
  return !processGroupExists(groupId);
}

function unrefDelay(milliseconds) {
  return new Promise((resolve) => {
    const timeout = setTimeout(resolve, milliseconds);
    timeout.unref();
  });
}

async function runFaultProbe() {
  const artifactRoot = join(ARTIFACT_ROOT, 'fault-probe');
  const childEnvironment = { ...process.env };
  delete childEnvironment.NODE_TEST_CONTEXT;
  const child = spawn(process.execPath, ['--test', '--test-concurrency=1', SELF_PATH], {
    cwd: REPOSITORY_ROOT,
    detached: true,
    env: {
      ...childEnvironment,
      UZEL_UI_ARTIFACT_ROOT: artifactRoot,
      UZEL_UI_FAULT_PROBE_CHILD: '1',
      UZEL_UI_SCENARIOS: 'fault-proof',
    },
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  let output = '';
  child.stdout.on('data', (chunk) => { output += chunk; });
  child.stderr.on('data', (chunk) => { output += chunk; });
  const closed = new Promise((resolve, reject) => {
    child.on('error', reject);
    child.on('close', (code, signal) => resolve({ code, signal }));
  });
  const timeoutMs = 30_000;
  const termGraceMs = 2_000;
  let timedOut = false;
  let result = await Promise.race([closed, unrefDelay(timeoutMs).then(() => null)]);
  if (result === null) {
    timedOut = true;
    signalProcessGroup(child.pid, 'SIGTERM');
    result = await Promise.race([closed, unrefDelay(termGraceMs).then(() => null)]);
    if (result === null) {
      signalProcessGroup(child.pid, 'SIGKILL');
      result = await Promise.race([closed, unrefDelay(termGraceMs).then(() => null)]);
    }
  }
  assert.ok(result, 'fault probe process group did not exit after SIGKILL');
  let groupExited = await waitForProcessGroupExit(child.pid, 4);
  if (!groupExited) {
    signalProcessGroup(child.pid, 'SIGTERM');
    groupExited = await waitForProcessGroupExit(child.pid);
  }
  if (!groupExited) {
    signalProcessGroup(child.pid, 'SIGKILL');
    groupExited = await waitForProcessGroupExit(child.pid);
  }
  return {
    artifactRoot,
    code: result.code,
    signal: result.signal,
    output,
    timedOut,
    processGroup: child.pid,
    processGroupExited: groupExited,
  };
}

before(async () => {
  const chromiumExecutable = process.env.UZEL_PLAYWRIGHT_CHROMIUM;
  assert.ok(
    chromiumExecutable,
    'UZEL_PLAYWRIGHT_CHROMIUM is missing; run through `nix develop -c pnpm test:ui`',
  );
  await access(chromiumExecutable);
  await mkdir(ARTIFACT_ROOT, { recursive: true });
  fixtureRecords = Object.freeze({
    testNaddr: TEST_NADDR,
    'profile-card': await loadFixtureRecord('profile-card'),
    'follow-list': await loadFixtureRecord('follow-list'),
    'good-morning': await loadFixtureRecord('good-morning'),
  });
  decodedNaddr = JSON.parse(execFileSync('nak', ['decode', TEST_NADDR], { encoding: 'utf8' }));
  const reviewedFixture = fixtureRecords['good-morning'];
  assert.equal(decodedNaddr.kind, reviewedFixture.kind, 'naddr kind differs from checked-in event');
  assert.equal(decodedNaddr.pubkey, reviewedFixture.author, 'naddr pubkey differs from checked-in event');
  assert.equal(decodedNaddr.identifier, reviewedFixture.dTag, 'naddr identifier differs from checked-in event');
  server = await createServer({
    root: APP_ROOT,
    logLevel: 'error',
    server: { host: '127.0.0.1', port: 0, strictPort: false },
  });
  await server.listen();
  baseUrl = server.resolvedUrls?.local[0]?.replace(/\/$/, '');
  assert.ok(baseUrl, 'Vite did not expose a local acceptance URL');
  browserServer = await chromium.launchServer({ executablePath: chromiumExecutable, headless: true });
  browserChildPid = browserServer.process().pid;
  browser = await chromium.connect(browserServer.wsEndpoint());
});

after(async () => {
  const cleanupErrors = [];
  const browserVersion = browser?.version() ?? null;
  const capturedBrowserPids = await captureProcessTree(browserChildPid)
    .catch((error) => {
      cleanupErrors.push(`browser process-tree capture: ${String(error)}`);
      return browserChildPid === null ? [] : [browserChildPid];
    });
  await browser?.close().catch((error) => cleanupErrors.push(`browser close: ${String(error)}`));
  await browserServer?.close().catch((error) => cleanupErrors.push(`browser server close: ${String(error)}`));
  await server?.close().catch((error) => cleanupErrors.push(`Vite close: ${String(error)}`));
  const browserProcessExits = await waitForPidsExit(capturedBrowserPids);
  const browserChildExited = browserChildPid === null
    || browserProcessExits.find((processState) => processState.pid === browserChildPid)?.exited === true;
  const metadata = {
    generatedAt: new Date().toISOString(),
    gitSha,
    gitDirty,
    mode: HARNESS_MODE,
    browser: browserVersion,
    artifactRoot: ARTIFACT_ROOT,
    selectedScenarios: FAULT_CHILD ? ['console-fault'] : [...selectedScenarios],
    ownedBrowserChild: { pid: browserChildPid, exited: browserChildExited },
    ownedBrowserProcessTree: {
      rootPid: browserChildPid,
      capturedBeforeClose: capturedBrowserPids,
      exits: browserProcessExits,
    },
    reviewedCoordinate: decodedNaddr ? {
      coordinate: TEST_NADDR,
      decoded: decodedNaddr,
      eventId: fixtureRecords?.['good-morning']?.eventId ?? null,
      aggregateHash: fixtureRecords?.['good-morning']?.aggregateHash ?? null,
      path: fixtureRecords?.['good-morning']?.path ?? null,
      pathSha256: fixtureRecords?.['good-morning']?.pathSha256 ?? null,
    } : null,
    cleanupErrors,
    screenshots,
    outcomes,
  };
  await writeFile(join(ARTIFACT_ROOT, 'metadata.json'), `${JSON.stringify(metadata, null, 2)}\n`);
  assert.deepEqual(cleanupErrors, [], 'acceptance harness teardown must be clean');
  assert.equal(
    browserProcessExits.every((processState) => processState.exited),
    true,
    'acceptance harness left a captured Chromium process alive',
  );
});

function scenarioTest(scenario, title, body, viewports = [VIEWPORTS[0]]) {
  if (!selectedScenarios.has(scenario)) return;
  test(`${scenario}: ${title}`, { timeout: 180_000 }, async (t) => {
    for (const viewport of viewports) {
      await t.test(viewport.name, async () => body(viewport));
    }
  });
}

if (FAULT_CHILD) {
  test('console-fault: deliberate fault must fail guarded renderer', { timeout: 120_000 }, async () => {
    const viewport = VIEWPORTS[0];
    await runCase('deliberate-console-fault', viewport, 'console-fault', async (page) => {
      await waitForReady(page);
    });
  });
} else {
  scenarioTest('ready', 'identity, real napplet routing, settings, naddr, and network denial', async (viewport) => {
    await runCase('complete-renderer-flow', viewport, 'ready', async (page, guarded) => {
      await waitForReady(page);
      await screenshot(page, viewport, 'ready', 'complete-renderer-flow', 'base-workspace');

      const followFrame = page.frameLocator('iframe[aria-label="Direct follows"]');
      const profileFrame = page.frameLocator('iframe[aria-label="Profile card"]');
      const beforeRefresh = await page.evaluate(() => window.__UZEL_UI_HARNESS__.envelopes.length);
      await followFrame.getByRole('button', { name: 'Reload', exact: true }).click();
      await profileFrame.getByRole('button', { name: 'Reload', exact: true }).click();
      await page.waitForFunction((count) => window.__UZEL_UI_HARNESS__.envelopes.length >= count + 3, beforeRefresh);

      const requestedIdentity = await page.evaluate(() => window.__UZEL_UI_HARNESS__.requestedIdentity);
      await page.getByLabel('Public read identity').fill(requestedIdentity);
      await page.getByRole('button', { name: 'Use identity', exact: true }).click();
      await waitForReady(page);
      await page.frameLocator('iframe[aria-label="Profile card"]')
        .getByText('Requested npub profile', { exact: true }).waitFor();
      const requestedKind0 = page.frameLocator('iframe[aria-label="Profile card"]').locator('#kind0');
      await requestedKind0.getByText('https://profile.ui-acceptance.invalid', { exact: false }).waitFor();

      const routedProfile = await page.evaluate(() => window.__UZEL_UI_HARNESS__.routedProfile);
      const secondaryProfile = await page.evaluate(() => window.__UZEL_UI_HARNESS__.secondaryProfile);
      const followFrameAfterIdentity = page.frameLocator('iframe[aria-label="Direct follows"]');
      await followFrameAfterIdentity.getByText('Routed follow profile', { exact: true }).waitFor();
      await followFrameAfterIdentity.getByText('Secondary follow profile', { exact: true }).waitFor();
      const followProfileQueries = await page.evaluate(() => window.__UZEL_UI_HARNESS__.envelopes
        .filter((entry) => entry.dTag === 'follow-list' && entry.envelope.type === 'outbox.query')
        .map((entry) => entry.envelope.options?.authors));
      assert.ok(
        followProfileQueries.some((authors) => authors.length === 2
          && authors.includes(routedProfile) && authors.includes(secondaryProfile)),
        'follow enrichment did not attempt the bounded multi-author batch',
      );
      assert.equal(
        followProfileQueries.some((authors) => authors.length === 1 && authors[0] === routedProfile),
        false,
        'follow enrichment retried a profile already validated from partial evidence',
      );
      assert.ok(
        followProfileQueries.some((authors) => authors.length === 1 && authors[0] === secondaryProfile),
        'follow enrichment did not bisect the failed batch for the secondary profile',
      );
      const routedFollowButton = followFrameAfterIdentity
        .getByRole('button', { name: `Open profile ${routedProfile}`, exact: true });
      const routedAvatar = routedFollowButton.locator('img');
      await routedAvatar.waitFor({ state: 'visible' });
      assert.match(await routedAvatar.getAttribute('src'), /^blob:/u);
      const followsList = followFrameAfterIdentity.locator('#follows');
      await followsList.evaluate((element) => {
        element.style.transform = 'translateY(2000px)';
      });
      await waitForLocatorAttribute(
        routedAvatar,
        'src',
        (value) => value === null,
        'off-screen follow avatar retained its decoded image source',
      );
      await followsList.evaluate((element) => {
        element.style.transform = '';
      });
      await waitForLocatorAttribute(
        routedAvatar,
        'src',
        (value) => /^blob:/u.test(value ?? ''),
        'visible follow avatar was not reloaded through NAP-RESOURCE',
      );
      await routedFollowButton.click();
      const routedProfileFrame = page.frameLocator('iframe[aria-label="Profile card"]');
      await routedProfileFrame
        .getByText('Routed follow profile', { exact: true }).waitFor();
      const fullKind0 = await routedProfileFrame.locator('#kind0').textContent();
      assert.match(fullKind0, /"name":"Routed raw name"/u);
      assert.match(fullKind0, /"website":"https:\/\/profile\.ui-acceptance\.invalid"/u);
      assert.match(fullKind0, /"lud16":"routed@payments\.ui-acceptance\.invalid"/u);
      assert.match(fullKind0, /"custom":\{/u);
      assert.match(fullKind0, /<img src=x onerror=/u);
      assert.equal(
        await routedProfileFrame.locator('body').evaluate(() => window.__escapedKind0),
        undefined,
        'kind 0 text executed as markup',
      );
      await screenshot(page, viewport, 'ready', 'complete-renderer-flow', 'routed-profile');

      await exerciseAllSettings(page, viewport);
      await page.getByRole('button', { name: 'Debug', exact: true }).click();
      const diagnostics = page.getByRole('complementary', { name: 'Developer diagnostics' });
      await diagnostics.getByText('wss://relay.ui-acceptance.invalid', { exact: true }).waitFor();
      await screenshot(page, viewport, 'ready', 'complete-renderer-flow', 'diagnostics');
      await diagnostics.getByRole('button', { name: 'Close', exact: true }).click();

      const loader = await openReview(page);
      const reviewedFixture = fixtureRecords['good-morning'];
      for (const domain of reviewedFixture.requiredCapabilities) {
        assert.equal(
          await loader.getByRole('checkbox', {
            name: new RegExp(`${domain}.*Required by verified artifact`, 'i'),
          }).isChecked(),
          true,
        );
      }
      await loader.getByText(reviewedFixture.aggregateHash, { exact: true }).waitFor();
      await screenshot(page, viewport, 'ready', 'complete-renderer-flow', 'naddr-review');
      const loaded = await installReview(page, loader);
      await screenshot(page, viewport, 'ready', 'complete-renderer-flow', 'loaded-napplet');
      await loaded.getByRole('button', { name: 'Close napplet', exact: true }).click();
      await waitForReady(page);

      assert.deepEqual(guarded.externalRequests, [], 'ordinary renderer flow made an external request');
      const denied = await page.evaluate(async (url) => {
        try {
          await fetch(url);
          return false;
        } catch {
          return true;
        }
      }, EXTERNAL_SENTINEL_URL);
      assert.equal(denied, true, 'network sentinel escaped the acceptance route guard');
      assert.deepEqual(guarded.externalRequests, [EXTERNAL_SENTINEL_URL]);
      await page.waitForTimeout(50);
      const denialProblem = guarded.browserProblems.findIndex((problem) =>
        problem.includes('Failed to load resource') && problem.includes('ERR_BLOCKED_BY_CLIENT'));
      assert.notEqual(denialProblem, -1, 'Chromium did not report the deliberately blocked sentinel');
      guarded.acknowledgedProblems.push(guarded.browserProblems.splice(denialProblem, 1)[0]);
      guarded.acknowledgedExternalRequests.push(EXTERNAL_SENTINEL_URL);
      const sentinelFailures = guarded.failedRequests.filter((failure) =>
        failure.url === EXTERNAL_SENTINEL_URL
        && failure.failure.startsWith('net::ERR_BLOCKED_BY_CLIENT'));
      assert.equal(sentinelFailures.length, 1, 'expected exactly one blocked sentinel request failure');
      guarded.acknowledgedFailedRequests.push(sentinelFailures[0]);
      const state = await page.evaluate(() => window.__UZEL_UI_HARNESS__.snapshot());
      assert.equal(state.pendingReviews.length, 0);
      assert.equal(state.activeSurfaces.length, 2);
      assert.equal(state.mountedSurfaces.length, 2);
    });
  }, VIEWPORTS);

  scenarioTest('projection-overflow', 'oversized profile batch is isolated before transport circuit', async (viewport) => {
    await runCase('projection-overflow', viewport, 'projection-overflow', async (page, guarded) => {
      await waitForReady(page);
      const followFrame = page.frameLocator('iframe[aria-label="Direct follows"]');
      await followFrame.getByText('Routed follow profile', { exact: true }).waitFor();
      await followFrame.getByText('Secondary follow profile', { exact: true }).waitFor();
      const queries = await page.evaluate(() => window.__UZEL_UI_HARNESS__.envelopes
        .filter((entry) => entry.dTag === 'follow-list' && entry.envelope.type === 'outbox.query')
        .map((entry) => entry.envelope.options?.authors));
      const routedProfile = await page.evaluate(() => window.__UZEL_UI_HARNESS__.routedProfile);
      const secondaryProfile = await page.evaluate(() => window.__UZEL_UI_HARNESS__.secondaryProfile);
      assert.ok(
        queries.some((authors) => authors.length === 2),
        'projection overflow probe did not start with one bounded multi-author query',
      );
      assert.ok(
        queries.some((authors) => authors.length === 1 && authors[0] === routedProfile),
        'projection overflow did not isolate the routed profile',
      );
      assert.ok(
        queries.some((authors) => authors.length === 1 && authors[0] === secondaryProfile),
        'projection overflow did not preserve unrelated profile enrichment',
      );
      assert.deepEqual(guarded.externalRequests, []);
    });
  });

  scenarioTest('profile-delay', 'profile query survives the removed napplet deadline', async (viewport) => {
    await runCase('delayed-profile-response', viewport, 'profile-delay', async (page) => {
      await waitForReady(page);
      const delayConfiguration = await page.evaluate(() => ({
        delayMs: window.__UZEL_UI_HARNESS__.delayedProfileResponseMs,
        routedProfile: window.__UZEL_UI_HARNESS__.routedProfile,
      }));
      assert.ok(delayConfiguration.delayMs > 4_000, 'mocked response must exceed the old deadline');

      const startedAt = Date.now();
      await page.frameLocator('iframe[aria-label="Direct follows"]')
        .getByRole('button', {
          name: `Open profile ${delayConfiguration.routedProfile}`,
          exact: true,
        }).click();
      const profileFrame = page.frameLocator('iframe[aria-label="Profile card"]');
      await profileFrame.getByText('Routed follow profile', { exact: true }).waitFor();
      assert.ok(
        Date.now() - startedAt >= delayConfiguration.delayMs,
        'profile rendered before the mocked delayed response arrived',
      );
      await profileFrame.getByText('Latest-known profile.', { exact: true }).waitFor();

      const query = await page.evaluate(() => window.__UZEL_UI_HARNESS__.envelopes
        .map((entry) => entry.envelope)
        .findLast((envelope) => envelope.type === 'outbox.query'));
      assert.deepEqual(query.options, { authors: [delayConfiguration.routedProfile] });
      assert.equal('timeoutMs' in query.options, false);
    });
  });

  scenarioTest('initialization-failure', 'accessible initialization retry', async (viewport) => {
    await runCase('initialization-retry', viewport, 'initialization-failure', async (page, guarded) => {
      const recovery = page.getByRole('region', { name: 'Runtime initialization recovery' });
      await recovery.getByText('mocked private runtime unavailable', { exact: false }).waitFor();
      await screenshot(page, viewport, 'initialization-failure', 'initialization-retry', 'recovery');
      await recovery.getByRole('button', { name: 'Retry initialization', exact: true }).click();
      await waitForReady(page);
      assert.deepEqual(guarded.externalRequests, []);
    });
  }, VIEWPORTS);

  scenarioTest('initialization-empty-identity', 'initialization retry activates identity before surfaces', async (viewport) => {
    await runCase('initialization-empty-identity', viewport, 'initialization-empty-identity', async (page, guarded) => {
      const recovery = page.getByRole('region', { name: 'Runtime initialization recovery' });
      await recovery.getByText('mocked private runtime unavailable', { exact: false }).waitFor();
      await screenshot(
        page,
        viewport,
        'initialization-empty-identity',
        'initialization-empty-identity',
        'before-retry',
      );
      assert.equal(
        await page.getByRole('button', { name: 'Open napplet', exact: true }).isDisabled(),
        true,
        'catalog controls must stay locked during initialization recovery',
      );
      assert.equal(
        await page.getByRole('button', { name: 'Waiting for panes…', exact: true }).isDisabled(),
        true,
        'identity controls must stay locked during initialization recovery',
      );
      await recovery.getByRole('button', { name: 'Retry initialization', exact: true }).click();
      await waitForReady(page);
      await screenshot(
        page,
        viewport,
        'initialization-empty-identity',
        'initialization-empty-identity',
        'after-retry',
      );
      assert.equal(await recovery.count(), 0, 'successful initialization must clear recovery state');
      const state = await page.evaluate(() => ({
        snapshot: window.__UZEL_UI_HARNESS__.snapshot(),
        calls: window.__UZEL_UI_HARNESS__.calls,
        fixtureIdentity: window.__UZEL_UI_HARNESS__.fixtureIdentity,
      }));
      const identityCalls = state.calls
        .map((call, index) => ({ ...call, index }))
        .filter((call) => call.command === 'select_read_identity');
      const surfaceCalls = state.calls
        .map((call, index) => ({ ...call, index }))
        .filter((call) => call.command === 'start_fixture');
      assert.equal(identityCalls.length, 1, 'retry must select exactly one read identity');
      assert.equal(surfaceCalls.length, 2, 'retry must launch exactly two base surfaces');
      assert.equal(identityCalls[0].args.publicIdentity, state.fixtureIdentity);
      assert.ok(
        surfaceCalls.every((call) => identityCalls[0].index < call.index),
        'identity selection must precede every surface launch',
      );
      assert.equal(state.snapshot.activeIdentity, state.fixtureIdentity);
      assert.deepEqual(guarded.externalRequests, []);
    });
  }, VIEWPORTS);

  scenarioTest('initialization-identity-failure', 'failed initialization identity stays recoverable', async (viewport) => {
    await runCase('initialization-identity-failure', viewport, 'initialization-identity-failure', async (page, guarded) => {
      const recovery = page.getByRole('region', { name: 'Runtime initialization recovery' });
      await recovery.getByText('mocked private runtime unavailable', { exact: false }).waitFor();
      await recovery.getByRole('button', { name: 'Retry initialization', exact: true }).click();
      await recovery.getByText('mocked identity selection unavailable', { exact: false }).waitFor();
      await screenshot(
        page,
        viewport,
        'initialization-identity-failure',
        'initialization-identity-failure',
        'identity-selection-failure',
      );
      const state = await page.evaluate(() => ({
        snapshot: window.__UZEL_UI_HARNESS__.snapshot(),
        calls: window.__UZEL_UI_HARNESS__.calls,
      }));
      assert.equal(
        state.calls.filter((call) => call.command === 'select_read_identity').length,
        1,
      );
      assert.equal(
        state.calls.filter((call) => call.command === 'start_fixture').length,
        0,
        'failed identity selection must launch no surface',
      );
      assert.equal(state.snapshot.activeIdentity, null);
      assert.deepEqual(state.snapshot.activeSurfaces, []);
      assert.equal(
        await page.getByRole('button', { name: 'Open napplet', exact: true }).isDisabled(),
        true,
      );
      await recovery.getByRole('button', { name: 'Retry initialization', exact: true }).click();
      await waitForReady(page);
      await screenshot(
        page,
        viewport,
        'initialization-identity-failure',
        'initialization-identity-failure',
        'recovered-after-identity-failure',
      );
      assert.equal(await recovery.count(), 0, 'second retry must clear identity-selection recovery');
      const recovered = await page.evaluate(() => ({
        snapshot: window.__UZEL_UI_HARNESS__.snapshot(),
        calls: window.__UZEL_UI_HARNESS__.calls,
        fixtureIdentity: window.__UZEL_UI_HARNESS__.fixtureIdentity,
      }));
      const identityCalls = recovered.calls
        .map((call, index) => ({ ...call, index }))
        .filter((call) => call.command === 'select_read_identity');
      const surfaceCalls = recovered.calls
        .map((call, index) => ({ ...call, index }))
        .filter((call) => call.command === 'start_fixture');
      assert.equal(identityCalls.length, 2, 'recovery must preserve one failed and one successful identity attempt');
      assert.equal(surfaceCalls.length, 2, 'successful second retry must launch both base surfaces');
      assert.ok(
        surfaceCalls.every((call) => identityCalls[1].index < call.index),
        'successful identity selection must precede every recovered surface launch',
      );
      assert.equal(recovered.snapshot.activeIdentity, recovered.fixtureIdentity);
      assert.deepEqual(guarded.externalRequests, []);
    });
  });

  scenarioTest('naddr-denied', 'blocked naddr remains non-installable', async (viewport) => {
    await runCase('naddr-denial', viewport, 'naddr-denied', async (page) => {
      await waitForReady(page);
      const loader = await openReview(page);
      await loader.getByText('BLOCKED', { exact: true }).waitFor();
      await loader.getByText('Policy denied this exact build for renderer acceptance.', { exact: true }).waitFor();
      assert.equal(await loader.getByRole('button', { name: 'Install exact build', exact: true }).isDisabled(), true);
      await screenshot(page, viewport, 'naddr-denied', 'naddr-denial', 'blocked-review');
    });
  });

  scenarioTest('review-ambiguous', 'ambiguous review retries same coordinate', async (viewport) => {
    await runCase('ambiguous-review-retry', viewport, 'review-ambiguous', async (page) => {
      await waitForReady(page);
      const loader = await openReview(page);
      await loader.getByText(/Review outcome is unknown:/).waitFor();
      await loader.getByRole('button', { name: 'Retry review', exact: true }).click();
      await loader.getByRole('heading', {
        name: fixtureRecords['good-morning'].title,
        exact: true,
      }).waitFor();
      const state = await page.evaluate(() => window.__UZEL_UI_HARNESS__.snapshot());
      assert.equal(state.reviewAttempts, 2);
      assert.equal(state.pendingReviews.length, 1);
    });
  });

  scenarioTest('confirmation-ambiguous', 'ambiguous confirmation retries frozen review', async (viewport) => {
    await runCase('ambiguous-confirmation-retry', viewport, 'confirmation-ambiguous', async (page) => {
      await waitForReady(page);
      const loader = await openReview(page);
      await loader.getByRole('heading', {
        name: fixtureRecords['good-morning'].title,
        exact: true,
      }).waitFor();
      await loader.getByRole('button', { name: 'Install exact build', exact: true }).click();
      await loader.getByText(/Confirmation outcome is unknown:/).waitFor();
      await loader.getByRole('button', { name: 'Retry install', exact: true }).click();
      await page.getByRole('region', { name: 'Loaded napplet workspace' }).waitFor();
      const state = await page.evaluate(() => window.__UZEL_UI_HARNESS__.snapshot());
      assert.equal(state.confirmationAttempts, 2);
      assert.equal(state.pendingReviews.length, 0);
    });
  });

  scenarioTest('cleanup-failure', 'failed loaded-session cleanup is retained then retried', async (viewport) => {
    await runCase('retained-cleanup-retry', viewport, 'cleanup-failure', async (page) => {
      await waitForReady(page);
      const loader = await openReview(page);
      const loaded = await installReview(page, loader);
      await loaded.getByRole('button', { name: 'Close napplet', exact: true }).click();
      await page.getByText(/Loaded napplet stop failed:.*mock retained loaded session/, { exact: false }).waitFor();
      let state = await page.evaluate(() => window.__UZEL_UI_HARNESS__.snapshot());
      assert.equal(state.activeSurfaces.length, 3);
      await loaded.getByRole('button', { name: 'Close napplet', exact: true }).click();
      await waitForReady(page);
      state = await page.evaluate(() => window.__UZEL_UI_HARNESS__.snapshot());
      assert.equal(state.activeSurfaces.length, 2);
    });
  });

  scenarioTest('restart-reconciliation', 'startup identity reconciliation and identity switch', async (viewport) => {
    await runCase('identity-restart-reconciliation', viewport, 'restart-reconciliation', async (page) => {
      await waitForReady(page);
      const requestedIdentity = await page.evaluate(() => window.__UZEL_UI_HARNESS__.requestedIdentity);
      assert.equal(await page.getByLabel('Public read identity').inputValue(), requestedIdentity);
      await page.frameLocator('iframe[aria-label="Profile card"]')
        .getByText('Requested npub profile', { exact: true }).waitFor();
      const fixtureIdentity = await page.evaluate(() => window.__UZEL_UI_HARNESS__.fixtureIdentity);
      await page.getByLabel('Public read identity').fill(fixtureIdentity);
      await page.getByRole('button', { name: 'Use identity', exact: true }).click();
      await waitForReady(page);
      await page.frameLocator('iframe[aria-label="Profile card"]')
        .getByText('Fixture identity profile', { exact: true }).waitFor();
      await screenshot(
        page,
        viewport,
        'restart-reconciliation',
        'identity-restart-reconciliation',
        'identity-switched',
      );
      const state = await page.evaluate(() => ({
        snapshot: window.__UZEL_UI_HARNESS__.snapshot(),
        calls: window.__UZEL_UI_HARNESS__.calls,
      }));
      assert.equal(state.snapshot.activeIdentity, fixtureIdentity);
      assert.equal(state.snapshot.activeSurfaces.length, 2);
      assert.equal(state.calls.filter((call) => call.command === 'stop_fixture').length, 2);
      assert.equal(state.calls.filter((call) => call.command === 'start_fixture').length, 4);
    });
  });

  if (selectedScenarios.has('fault-proof')) {
    test('fault-proof: isolated deliberate browser fault exits nonzero and tears down', { timeout: 180_000 }, async () => {
      const result = await runFaultProbe();
      await writeFile(join(ARTIFACT_ROOT, 'fault-probe.tap'), result.output);
      assert.equal(result.timedOut, false, 'deliberate browser fault probe timed out');
      assert.equal(result.processGroupExited, true, 'fault probe left its detached process group alive');
      assert.notEqual(result.code, 0, 'deliberate browser fault unexpectedly passed the harness');
      assert.match(result.output, /UZEL_UI_DELIBERATE_FAULT/);
      const metadata = JSON.parse(await readFile(join(result.artifactRoot, 'metadata.json'), 'utf8'));
      assert.equal(metadata.ownedBrowserChild.exited, true, 'fault probe leaked its Chromium child');
      assert.equal(
        metadata.ownedBrowserProcessTree.exits.every((processState) => processState.exited),
        true,
        'fault probe leaked a captured Chromium descendant',
      );
      assert.deepEqual(metadata.cleanupErrors, []);
      outcomes.push({
        name: 'isolated-deliberate-fault',
        scenario: 'fault-proof',
        viewport: { ...VIEWPORTS[0] },
        mode: HARNESS_MODE,
        gitSha,
        status: 'passed',
        childExitCode: result.code,
        childSignal: result.signal,
        childTimedOut: result.timedOut,
        childProcessGroup: result.processGroup,
        childProcessGroupExited: result.processGroupExited,
      });
    });
  }
}
