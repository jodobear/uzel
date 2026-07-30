import '@napplet/shim';
import { identityGetFollows } from '@napplet/nap/identity/sdk';
import { incEmit } from '@napplet/nap/inc/sdk';

import { waitForEvidence } from '../../../contracts/evidence-refresh.js';
import { PROFILE_OPEN_TOPIC, profileOpen } from '../../../contracts/profile-open.js';
import { directFollows, shortPubkey } from './model.js';

const status = document.querySelector('#status');
const list = document.querySelector('#follows');
const refresh = document.querySelector('#refresh');
let refreshGeneration = 0;

function render(values) {
  const pubkeys = directFollows(values);
  list.replaceChildren();
  for (const pubkey of pubkeys) {
    const item = document.createElement('li');
    const button = document.createElement('button');
    button.type = 'button';
    button.textContent = shortPubkey(pubkey);
    button.dataset.pubkey = pubkey;
    button.setAttribute('aria-label', `Open profile ${pubkey}`);
    button.addEventListener('click', () => incEmit(PROFILE_OPEN_TOPIC, profileOpen(pubkey)));
    item.append(button);
    list.append(item);
  }
  status.textContent = pubkeys.length === 0
    ? 'No direct follows in current evidence.'
    : `${pubkeys.length} direct follows`;
}

async function loadFollows() {
  const generation = ++refreshGeneration;
  refresh.disabled = true;
  try {
    const values = await waitForEvidence(
      identityGetFollows,
      (candidate) => Array.isArray(candidate) && candidate.length > 0,
      {
        onAttempt: (attempt, attempts) => {
          if (generation === refreshGeneration) {
            status.textContent = `Refreshing NMP evidence… ${attempt}/${attempts}`;
          }
        },
      },
    );
    if (generation === refreshGeneration) render(values);
  } catch (error) {
    if (generation === refreshGeneration) {
      status.textContent = `Identity unavailable: ${error instanceof Error ? error.message : String(error)}`;
    }
  } finally {
    if (generation === refreshGeneration) refresh.disabled = false;
  }
}

refresh.addEventListener('click', () => void loadFollows());
void loadFollows();
