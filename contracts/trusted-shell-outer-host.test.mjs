import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import vm from 'node:vm';
import test from 'node:test';

const source = await readFile(
  new URL('../apps/uzel/public/trusted-shell-outer-host.js', import.meta.url),
  'utf8',
);

function harness() {
  const listeners = new Map();
  const timers = new Map();
  const frames = [];
  const attributes = new Map();
  let timerId = 0;
  const document = {
    documentElement: {
      setAttribute(name, value) { attributes.set(name, value); },
      removeAttribute(name) { attributes.delete(name); },
    },
    dispatchEvent() {},
    createElement(name) {
      assert.equal(name, 'iframe');
      const posted = [];
      const contentWindow = { postMessage(message) { posted.push(message); } };
      const frameAttributes = new Map();
      const frame = {
        contentWindow,
        posted,
        frameAttributes,
        removed: false,
        src: '',
        listeners: new Map(),
        setAttribute(name, value) { frameAttributes.set(name, value); },
        addEventListener(type, listener) { frame.listeners.set(type, listener); },
        remove() { frame.removed = true; },
      };
      frames.push(frame);
      return frame;
    },
  };
  const window = {
    document,
    TextEncoder,
    CustomEvent: class CustomEvent { constructor(type) { this.type = type; } },
    addEventListener(type, listener) { listeners.set(type, listener); },
    removeEventListener(type, listener) {
      if (listeners.get(type) === listener) listeners.delete(type);
    },
    dispatchEvent() {},
    setTimeout(callback) { timerId += 1; timers.set(timerId, callback); return timerId; },
    clearTimeout(id) { timers.delete(id); },
  };
  window.window = window;
  vm.runInNewContext(source, { window, TextEncoder, Object }, { filename: 'trusted-shell-outer-host.js' });
  return { window, frames, attributes, timers, listeners };
}

function configuration(overrides = {}) {
  return {
    session: 'session-1',
    artifactBaseURL: 'nmp-artifact://00000000-0000-4000-8000-000000000001/',
    artifactHTML: '<!doctype html><title>fixture</title>',
    title: 'Fixture',
    domains: ['shell'],
    manifestAuthor: 'a'.repeat(64),
    dTag: 'fixture',
    aggregateHash: 'b'.repeat(64),
    artifactDigest: 'c'.repeat(64),
    ...overrides,
  };
}

function target() {
  return { children: [], replaceChildren(...children) { this.children = children; } };
}

test('mount uses one exact sandboxed outer URL and a source-bound binding', () => {
  const { window, frames, listeners } = harness();
  const ready = [];
  assert.equal(window.NMPTrustedShellHost.mount(
    'surface-1', target(), configuration({ onReady: (id) => ready.push(id) }),
  ), true);
  const frame = frames[0];
  assert.equal(frame.src, 'nmp-shell://localhost/trusted-shell.html');
  assert.equal(frame.frameAttributes.get('sandbox'), 'allow-scripts');
  assert.equal(frame.frameAttributes.get('referrerpolicy'), 'no-referrer');
  assert.equal(window.NMPTrustedShellHost.receive('surface-1', { type: 'early' }), false);

  listeners.get('message')({ source: {}, data: { type: 'nmp.outer.ready', version: 1 } });
  assert.equal(frame.posted.length, 0, 'wrong Window cannot trigger a mount');
  listeners.get('message')({
    source: frame.contentWindow,
    data: { type: 'nmp.outer.ready', version: 1 },
  });
  assert.equal(frame.posted[0].type, 'nmp.outer.mount');
  assert.deepEqual(
    { ...frame.posted[0].configuration.binding },
    {
      manifestAuthor: 'a'.repeat(64),
      dTag: 'fixture',
      aggregateHash: 'b'.repeat(64),
      artifactDigest: 'c'.repeat(64),
      surface: 'surface-1',
      session: 'session-1',
    },
  );
  assert.equal(
    window.NMPTrustedShellHost.receive('surface-1', { type: 'shell.init' }),
    true,
    'mapped daemon response must cross before inner acceptance',
  );
  listeners.get('message')({
    source: frame.contentWindow,
    data: {
      type: 'nmp.outer.surface.ready',
      surfaceId: 'surface-1',
      session: 'session-1',
      binding: { ...frame.posted[0].configuration.binding, materializedDigest: 'd'.repeat(64) },
    },
  });
  assert.deepEqual(ready, ['surface-1']);
  assert.equal(window.NMPTrustedShellHost.receive('surface-1', { type: 'shell.ping' }), true);
  assert.equal(frame.posted.at(-1).type, 'nmp.outer.deliver');
});

test('invalid identifiers, authority fields, and capacity fail closed', () => {
  const { window } = harness();
  assert.equal(window.NMPTrustedShellHost.mount('\u0000bad', target(), configuration()), false);
  assert.equal(window.NMPTrustedShellHost.mount(
    'surface', target(), configuration({ artifactDigest: 'not-a-digest' }),
  ), false);
  for (let index = 0; index < 16; index += 1) {
    assert.equal(window.NMPTrustedShellHost.mount(
      `surface-${index}`, target(), configuration({ session: `session-${index}` }),
    ), true);
  }
  assert.equal(window.NMPTrustedShellHost.mount(
    'surface-overflow', target(), configuration({ session: 'session-overflow' }),
  ), false);
});

test('remount, explicit unmount, timeout, and pagehide retire exact mappings', () => {
  const { window, frames, timers, listeners } = harness();
  const errors = [];
  const surface = target();
  assert.equal(window.NMPTrustedShellHost.mount('surface', surface, configuration()), true);
  const oldFrame = frames[0];
  assert.equal(window.NMPTrustedShellHost.mount(
    'surface', surface, configuration({ session: 'session-2' }),
  ), true);
  assert.equal(oldFrame.removed, true);
  listeners.get('message')({
    source: oldFrame.contentWindow,
    data: { type: 'nmp.outer.ready', version: 1 },
  });
  assert.equal(oldFrame.posted.filter((message) => message.type === 'nmp.outer.mount').length, 0);
  assert.equal(window.NMPTrustedShellHost.unmount('surface'), true);
  assert.equal(frames[1].removed, true);

  assert.equal(window.NMPTrustedShellHost.mount(
    'timeout', target(), configuration({ session: 'timeout', onError: (_id, error) => errors.push(error) }),
  ), true);
  Array.from(timers.values()).at(-1)();
  assert.deepEqual(errors, ['outer-timeout']);
  assert.equal(frames[2].removed, true);

  assert.equal(window.NMPTrustedShellHost.mount('pagehide', target(), configuration()), true);
  listeners.get('pagehide')();
  assert.equal(frames[3].removed, true);
  assert.equal(window.NMPTrustedShellHost.mount('after-dispose', target(), configuration()), false);
});

test('binding drift, parent overflow, and outer renavigation invalidate fail closed', () => {
  const { window, frames, listeners } = harness();
  const errors = [];
  assert.equal(window.NMPTrustedShellHost.mount(
    'bound', target(), configuration({ onError: (_id, error) => errors.push(error) }),
  ), true);
  const frame = frames[0];
  frame.listeners.get('load')();
  listeners.get('message')({
    source: frame.contentWindow,
    data: { type: 'nmp.outer.ready', version: 1 },
  });
  listeners.get('message')({
    source: frame.contentWindow,
    data: {
      type: 'nmp.outer.surface.ready',
      surfaceId: 'bound',
      session: 'session-1',
      binding: {
        ...frame.posted[0].configuration.binding,
        aggregateHash: 'e'.repeat(64),
        materializedDigest: 'd'.repeat(64),
      },
    },
  });
  assert.equal(window.NMPTrustedShellHost.receive('bound', { type: 'still-pending' }), true);
  frame.listeners.get('load')();
  assert.deepEqual(errors, ['outer-navigation']);
  assert.equal(frame.removed, true);

  for (const id of ['one', 'two']) {
    assert.equal(window.NMPTrustedShellHost.mount(
      id, target(), configuration({ session: id, onError: (_surface, error) => errors.push(error) }),
    ), true);
  }
  listeners.get('message')({
    source: frames[1].contentWindow,
    data: { type: 'nmp.outer.rate-limited', scope: 'parent' },
  });
  assert.deepEqual(errors.slice(1), ['outer-rate-limited', 'outer-rate-limited']);
  assert.equal(frames[1].removed, true);
  assert.equal(frames[2].removed, true);
});

test('top and immutable outer documents keep separate authority boundaries', async () => {
  const [index, configText, embedded] = await Promise.all([
    readFile(new URL('../apps/uzel/index.html', import.meta.url), 'utf8'),
    readFile(new URL('../apps/uzel/src-tauri/tauri.conf.json', import.meta.url), 'utf8'),
    readFile(new URL('../apps/uzel/public/trusted-shell/trusted-shell-embedded.html', import.meta.url)),
  ]);
  assert.match(index, /src="\/trusted-shell-outer-host\.js"/);
  assert.doesNotMatch(index, /trusted-shell\/trusted-shell(?:-surface-host)?\.js/);
  const config = JSON.parse(configText);
  const productionCsp = config.app.security.csp;
  assert.match(productionCsp, /frame-src 'self' nmp-shell:/);
  assert.match(productionCsp, /script-src 'self'(?:;|$)/);
  assert.doesNotMatch(productionCsp, /script-src[^;]*'unsafe-inline'/);
  const embeddedText = embedded.toString('utf8');
  assert.equal((embeddedText.match(/\.srcdoc\s*=/g) ?? []).length, 1);
  assert.doesNotMatch(embeddedText, /__TAURI__|__TAURI_INTERNALS__|__TAURI_INVOKE_KEY__/);
  assert.equal(
    createHash('sha256').update(embedded).digest('hex'),
    'a3e6c18e8724329332bd15a039282a8a0bcf5ec93577b97752f46721df80fba3',
  );
});
