import '@napplet/shim';
import { identityGetProfile, identityGetPublicKey } from '@napplet/nap/identity/sdk';
import { incOn } from '@napplet/nap/inc/sdk';
import { outboxQuery } from '@napplet/nap/outbox/sdk';
import { resourceBytes } from '@napplet/nap/resource/sdk';

import { PROFILE_OPEN_TOPIC, parseProfileOpen } from '../../../contracts/profile-open.js';
import {
  canonicalIdentityProfile, canonicalProfile, createLatestRequestGate, PROFILE_RESULT_LIMIT,
} from './model.js';

const picture = document.querySelector('#picture');
const pictureFallback = document.querySelector('#picture-fallback');
const name = document.querySelector('#name');
const pubkey = document.querySelector('#pubkey');
const about = document.querySelector('#about');
const status = document.querySelector('#status');
const evidence = document.querySelector('#evidence');
const refresh = document.querySelector('#refresh');
const profileRequests = createLatestRequestGate();
let pictureObjectUrl = null;

function clearPicture() {
  if (pictureObjectUrl !== null) URL.revokeObjectURL(pictureObjectUrl);
  pictureObjectUrl = null;
  picture.removeAttribute('src');
  picture.hidden = true;
  pictureFallback.hidden = false;
}

async function renderPicture(url, generation) {
  clearPicture();
  if (!url) return;
  try {
    const blob = await resourceBytes(url);
    if (!profileRequests.isCurrent(generation)) return;
    picture.src = URL.createObjectURL(blob);
    pictureObjectUrl = picture.src;
    picture.hidden = false;
    pictureFallback.hidden = true;
  } catch (error) {
    if (profileRequests.isCurrent(generation)) {
      pictureFallback.title = `Picture unavailable through NAP-RESOURCE: ${error instanceof Error ? error.message : String(error)}`;
    }
  }
}

function renderProfile(profile, source, generation) {
  pubkey.textContent = profile.pubkey;
  name.textContent = profile.name;
  about.textContent = profile.about;
  evidence.textContent = [profile.nip05, source].filter(Boolean).join(' · ');
  void renderPicture(profile.picture, generation);
}

function clearProfileDetails() {
  name.textContent = '';
  about.textContent = '';
  evidence.textContent = '';
  clearPicture();
}

async function loadActiveProfile() {
  const requestGeneration = profileRequests.begin();
  refresh.disabled = true;
  clearProfileDetails();
  status.textContent = 'Reading latest-known active profile through NMP…';
  try {
    const active = await identityGetPublicKey();
    pubkey.textContent = active;
    const projected = await identityGetProfile();
    if (!profileRequests.isCurrent(requestGeneration)) return;
    const profile = canonicalIdentityProfile(projected, active);
    if (profile === null) {
      name.textContent = 'Profile not found';
      status.textContent = 'No kind 0 in the latest-known NMP view. Reload to retry.';
      return;
    }
    renderProfile(profile, 'active identity', requestGeneration);
    status.textContent = 'Latest-known active identity profile.';
  } catch (error) {
    if (profileRequests.isCurrent(requestGeneration)) {
      status.textContent = `Profile unavailable: ${error instanceof Error ? error.message : String(error)}`;
    }
  } finally {
    if (profileRequests.isCurrent(requestGeneration)) refresh.disabled = false;
  }
}

async function openProfile(payload) {
  const request = parseProfileOpen(payload);
  if (request === null) {
    status.textContent = 'Ignored malformed profile/open payload.';
    return;
  }
  const requestGeneration = profileRequests.begin();
  refresh.disabled = true;
  pubkey.textContent = request.pubkey;
  clearProfileDetails();
  status.textContent = 'Reading latest-known kind 0…';
  try {
    const result = await outboxQuery(
      [{ kinds: [0], authors: [request.pubkey], limit: PROFILE_RESULT_LIMIT }],
      { authors: [request.pubkey], timeoutMs: 3_000 },
    );
    if (!profileRequests.isCurrent(requestGeneration)) return;
    const profile = canonicalProfile(result.events, request.pubkey);
    if (profile === null) {
      name.textContent = 'Profile not found';
      about.textContent = '';
      status.textContent = result.incomplete ? 'Partial evidence; no valid kind 0 found.' : 'No valid kind 0 found.';
      evidence.textContent = result.error ?? '';
      return;
    }
    renderProfile(profile, `event ${profile.eventId} · ${profile.observedAt}`, requestGeneration);
    status.textContent = result.incomplete ? 'Latest-known profile; evidence incomplete.' : 'Latest-known profile.';
  } catch (error) {
    if (!profileRequests.isCurrent(requestGeneration)) return;
    status.textContent = `Profile unavailable: ${error instanceof Error ? error.message : String(error)}`;
  } finally {
    if (profileRequests.isCurrent(requestGeneration)) refresh.disabled = false;
  }
}

try {
  incOn(PROFILE_OPEN_TOPIC, (payload) => void openProfile(payload));
  refresh.addEventListener('click', () => void loadActiveProfile());
  void loadActiveProfile();
} catch (error) {
  status.textContent = `INC unavailable: ${error instanceof Error ? error.message : String(error)}`;
}
