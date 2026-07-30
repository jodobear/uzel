import assert from 'node:assert/strict';
import test from 'node:test';

import {
  bindingFromEvent, bindingMatches, defaultPreferences, parsePreferences, validateKeybindings,
} from './preferences.js';

test('normalizes editable accelerators deterministically', () => {
  const event = { key: 'd', ctrlKey: true, altKey: false, metaKey: false, shiftKey: true };
  assert.equal(bindingFromEvent(event), 'Control+Shift+D');
  assert.equal(bindingMatches(event, 'Control+Shift+D'), true);
  assert.equal(bindingFromEvent({ ...event, key: 'Control' }), null);
});

test('preferences reject duplicate, incomplete, and oversized state', () => {
  const defaults = defaultPreferences();
  assert.equal(validateKeybindings(defaults.keybindings), true);
  assert.equal(validateKeybindings({ ...defaults.keybindings, focusNext: 'ArrowLeft' }), false);
  assert.deepEqual(parsePreferences('{'), defaults);
  assert.deepEqual(parsePreferences('x'.repeat(4_097)), defaults);
});

test('valid preferences survive serialization', () => {
  const expected = defaultPreferences();
  expected.showEvidence = true;
  expected.keybindings.focusPrevious = 'ArrowUp';
  assert.deepEqual(parsePreferences(JSON.stringify(expected)), expected);
});
