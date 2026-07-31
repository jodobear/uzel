import '@napplet/shim';
import { identityGetPublicKey } from '@napplet/nap/identity/sdk';
import { incOn } from '@napplet/nap/inc/sdk';
import { outboxQuery } from '@napplet/nap/outbox/sdk';
import { resourceBytes } from '@napplet/nap/resource/sdk';

import { PROFILE_OPEN_TOPIC, parseProfileOpen } from '../../../contracts/profile-open.js';
import { canonicalProfile, createLatestRequestGate, profileQueryRequest } from './model.js';

const picture = document.querySelector('#picture');
const pictureFallback = document.querySelector('#picture-fallback');
const name = document.querySelector('#name');
const pubkey = document.querySelector('#pubkey');
const about = document.querySelector('#about');
const status = document.querySelector('#status');
const evidence = document.querySelector('#evidence');
const metadata = document.querySelector('#metadata');
const profileContent = document.querySelector('#kind0');
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
  profileContent.textContent = profile.contentText;
  metadata.hidden = false;
  metadata.setAttribute('open', '');
  void renderPicture(profile.picture, generation);
}

function clearProfileDetails() {
  name.textContent = '';
  about.textContent = '';
  evidence.textContent = '';
  profileContent.textContent = '';
  metadata.hidden = true;
  clearPicture();
}

async function queryProfile(targetPubkey, requestGeneration) {
  const query = profileQueryRequest(targetPubkey);
  const result = await outboxQuery(query.filters, query.options);
  if (!profileRequests.isCurrent(requestGeneration)) return;
  const resultError = typeof result.error === 'string' ? result.error : '';
  const degraded = Boolean(result.incomplete || resultError);
  const profile = canonicalProfile(result.events, targetPubkey);
  if (profile === null) {
    name.textContent = 'Profile not found';
    status.textContent = degraded ? 'Partial evidence; no valid kind 0 found.' : 'No valid kind 0 found.';
    evidence.textContent = resultError;
    return;
  }
  renderProfile(profile, `event ${profile.eventId} · ${profile.createdAtIso}`, requestGeneration);
  if (resultError) evidence.textContent = `${evidence.textContent} · NMP: ${resultError}`;
  status.textContent = degraded ? 'Latest-known profile; evidence incomplete.' : 'Latest-known profile.';
}

async function loadActiveProfile() {
  const requestGeneration = profileRequests.begin();
  refresh.disabled = true;
  clearProfileDetails();
  status.textContent = 'Reading latest-known active profile through NMP…';
  try {
    const active = await identityGetPublicKey();
    if (!profileRequests.isCurrent(requestGeneration)) return;
    pubkey.textContent = active;
    await queryProfile(active, requestGeneration);
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
    await queryProfile(request.pubkey, requestGeneration);
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
