import '@napplet/shim';
import { identityGetFollows } from '@napplet/nap/identity/sdk';
import { incEmit } from '@napplet/nap/inc/sdk';

import { PROFILE_OPEN_TOPIC, profileOpen } from '../../../contracts/profile-open.js';
import { directFollows, shortPubkey } from './model.js';

const status = document.querySelector('#status');
const list = document.querySelector('#follows');

function render(pubkeys) {
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
  status.textContent = pubkeys.length === 0 ? 'No direct follows in current evidence.' : `${pubkeys.length} direct follows`;
}

async function start() {
  try {
    render(directFollows(await identityGetFollows()));
  } catch (error) {
    status.textContent = `Identity unavailable: ${error instanceof Error ? error.message : String(error)}`;
  }
}

void start();
