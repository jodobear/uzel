export const PREFERENCES_STORAGE_KEY = 'uzel.shell-preferences.v1';

export const KEYBINDING_ACTIONS = Object.freeze([
  Object.freeze({ id: 'focusPrevious', label: 'Focus follow pane' }),
  Object.freeze({ id: 'focusNext', label: 'Focus profile pane' }),
  Object.freeze({ id: 'enterPane', label: 'Enter focused napplet' }),
  Object.freeze({ id: 'toggleSettings', label: 'Open or close Settings' }),
  Object.freeze({ id: 'toggleDeveloper', label: 'Open or close diagnostics' }),
  Object.freeze({ id: 'toggleEvidence', label: 'Show or hide proof chrome' }),
]);

export const DEFAULT_KEYBINDINGS = Object.freeze({
  focusPrevious: 'ArrowLeft',
  focusNext: 'ArrowRight',
  enterPane: 'Enter',
  toggleSettings: 'Control+,',
  toggleDeveloper: 'Control+Shift+D',
  toggleEvidence: 'Control+Shift+H',
});

/** @type {ReadonlySet<string>} */
const ACTION_IDS = new Set(KEYBINDING_ACTIONS.map(({ id }) => id));

export function defaultPreferences() {
  return { version: 1, showEvidence: false, keybindings: { ...DEFAULT_KEYBINDINGS } };
}

/**
 * @param {{key: string, ctrlKey: boolean, altKey: boolean, metaKey: boolean, shiftKey: boolean}} event
 */
export function bindingFromEvent(event) {
  if (['Alt', 'Control', 'Meta', 'Shift'].includes(event.key)) return null;
  const tokens = [];
  if (event.ctrlKey) tokens.push('Control');
  if (event.altKey) tokens.push('Alt');
  if (event.metaKey) tokens.push('Meta');
  if (event.shiftKey) tokens.push('Shift');
  let key = event.key === ' ' ? 'Space' : event.key;
  if (key.length === 1 && /[a-z]/i.test(key)) key = key.toUpperCase();
  tokens.push(key);
  return tokens.join('+');
}

/**
 * @param {{key: string, ctrlKey: boolean, altKey: boolean, metaKey: boolean, shiftKey: boolean}} event
 * @param {string} binding
 */
export function bindingMatches(event, binding) {
  return bindingFromEvent(event) === binding;
}

/** @param {unknown} candidate */
export function validateKeybindings(candidate) {
  if (candidate === null || typeof candidate !== 'object' || Array.isArray(candidate)) return false;
  const entries = Object.entries(/** @type {Record<string, unknown>} */ (candidate));
  if (entries.length !== ACTION_IDS.size || entries.some(([id]) => !ACTION_IDS.has(id))) return false;
  if (entries.some(([, binding]) => typeof binding !== 'string' || binding.length === 0 || binding.length > 80)) return false;
  return new Set(entries.map(([, binding]) => binding)).size === entries.length;
}

/** @param {unknown} raw */
export function parsePreferences(raw) {
  if (typeof raw !== 'string' || raw.length > 4_096) return defaultPreferences();
  try {
    const candidate = JSON.parse(raw);
    if (
      candidate?.version !== 1
      || typeof candidate.showEvidence !== 'boolean'
      || !validateKeybindings(candidate.keybindings)
    ) return defaultPreferences();
    return {
      version: 1,
      showEvidence: candidate.showEvidence,
      keybindings: { ...candidate.keybindings },
    };
  } catch {
    return defaultPreferences();
  }
}
