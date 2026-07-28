import '@napplet/shim';
import { incOn } from '@napplet/nap/inc/sdk';
import { outboxQuery } from '@napplet/nap/outbox/sdk';

import { PROFILE_OPEN_TOPIC, parseProfileOpen } from '../../../contracts/profile-open.js';
import { createLatestRequestGate, latestProfile } from './model.js';

const name = document.querySelector('#name');
const pubkey = document.querySelector('#pubkey');
const about = document.querySelector('#about');
const status = document.querySelector('#status');
const evidence = document.querySelector('#evidence');
const profileRequests = createLatestRequestGate();

function clearProfileDetails() {
  name.textContent = '';
  about.textContent = '';
  evidence.textContent = '';
}

async function openProfile(payload) {
  const request = parseProfileOpen(payload);
  if (request === null) {
    status.textContent = 'Ignored malformed profile/open payload.';
    return;
  }
  const requestGeneration = profileRequests.begin();
  pubkey.textContent = request.pubkey;
  clearProfileDetails();
  status.textContent = 'Reading latest-known kind 0…';
  try {
    const result = await outboxQuery(
      [{ kinds: [0], authors: [request.pubkey], limit: 1 }],
      { authors: [request.pubkey], limit: 1, timeoutMs: 3_000 },
    );
    if (!profileRequests.isCurrent(requestGeneration)) return;
    const profile = latestProfile(result.events, request.pubkey);
    if (profile === null) {
      name.textContent = 'Profile not found';
      about.textContent = '';
      status.textContent = result.incomplete ? 'Partial evidence; no valid kind 0 found.' : 'No valid kind 0 found.';
      evidence.textContent = result.error ?? '';
      return;
    }
    name.textContent = profile.name;
    about.textContent = profile.about;
    status.textContent = result.incomplete ? 'Latest-known profile; evidence incomplete.' : 'Latest-known profile.';
    evidence.textContent = `event ${profile.eventId} · ${new Date(profile.createdAt * 1_000).toISOString()}`;
  } catch (error) {
    if (!profileRequests.isCurrent(requestGeneration)) return;
    status.textContent = `Profile unavailable: ${error instanceof Error ? error.message : String(error)}`;
  }
}

let subscription = null;
try {
  subscription = incOn(PROFILE_OPEN_TOPIC, (payload) => void openProfile(payload));
  status.textContent = 'Waiting for profile/open…';
} catch (error) {
  status.textContent = `INC unavailable: ${error instanceof Error ? error.message : String(error)}`;
}

window.addEventListener('pagehide', () => subscription?.close(), { once: true });
