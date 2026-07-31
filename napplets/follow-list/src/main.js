import '@napplet/shim';
import { identityGetFollows } from '@napplet/nap/identity/sdk';
import { incEmit } from '@napplet/nap/inc/sdk';
import { outboxQuery } from '@napplet/nap/outbox/sdk';
import { resourceBytes } from '@napplet/nap/resource/sdk';

import {
  canonicalProfiles, profileQueryBatches, splitProfileQueryRequest,
} from '../../../contracts/kind0-profile.js';
import { PROFILE_OPEN_TOPIC, profileOpen } from '../../../contracts/profile-open.js';
import {
  createAvatarObjectUrlStore, createBoundedTaskQueue, directFollows, MAXIMUM_AVATAR_REQUESTS,
  MAXIMUM_PROFILE_REQUESTS, shortPubkey,
} from './model.js';

const status = document.querySelector('#status');
const list = document.querySelector('#follows');
const refresh = document.querySelector('#refresh');
const rowTemplate = document.querySelector('#follow-row');
const avatarQueue = createBoundedTaskQueue(MAXIMUM_AVATAR_REQUESTS);
const profileQueue = createBoundedTaskQueue(MAXIMUM_PROFILE_REQUESTS);
const rows = new Map();
const objectUrls = createAvatarObjectUrlStore();
let refreshGeneration = 0;
let refreshController = null;
let avatarObserver = null;

function current(generation) {
  return generation === refreshGeneration;
}

function resetEnrichment() {
  refreshController?.abort();
  refreshController = new AbortController();
  const reason = new DOMException('A newer follow refresh started.', 'AbortError');
  avatarQueue.clear(reason);
  profileQueue.clear(reason);
  avatarObserver?.disconnect();
  avatarObserver = null;
  clearObjectUrls();
  rows.clear();
  return refreshController;
}

function fallbackText(name) {
  const value = name.trim();
  return value.length > 0 ? value.slice(0, 1).toUpperCase() : '?';
}

function setAvatarFallback(row, name = '') {
  row.fallback.textContent = fallbackText(name);
  row.fallback.hidden = false;
  row.image.hidden = true;
}

function releaseObjectUrl(objectUrl, row, clearImage) {
  objectUrls.remove(objectUrl);
  URL.revokeObjectURL(objectUrl);
  if (row.objectUrl !== objectUrl) return;
  row.objectUrl = null;
  row.image.onload = null;
  row.image.onerror = null;
  if (clearImage) {
    row.image.removeAttribute('src');
    setAvatarFallback(row, row.name.textContent);
  }
}

function rememberObjectUrl(objectUrl, row) {
  row.objectUrl = objectUrl;
  for (const [oldestUrl, oldestRow] of objectUrls.remember(objectUrl, row)) {
    releaseObjectUrl(oldestUrl, oldestRow, true);
  }
}

function clearObjectUrls() {
  for (const [objectUrl, row] of objectUrls.drain()) {
    releaseObjectUrl(objectUrl, row, false);
  }
}

async function loadAvatar(row, picture, generation, signal) {
  try {
    const blob = await resourceBytes(picture, { signal });
    if (!current(generation) || signal.aborted || !row.image.isConnected) return;

    let objectUrl = '';
    row.image.onload = () => {
      if (current(generation) && !signal.aborted) {
        row.image.hidden = false;
        row.fallback.hidden = true;
      }
      releaseObjectUrl(objectUrl, row, false);
    };
    row.image.onerror = () => {
      releaseObjectUrl(objectUrl, row, true);
    };
    row.image.src = URL.createObjectURL(blob);
    objectUrl = row.image.src;
    rememberObjectUrl(objectUrl, row);
  } catch (error) {
    if (current(generation) && !signal.aborted) {
      row.fallback.title = `Picture unavailable through NAP-RESOURCE: ${error instanceof Error ? error.message : String(error)}`;
    }
  }
}

function observeAvatar(row, picture, generation, signal) {
  row.avatarRequest = { generation, picture, signal };
  avatarObserver?.observe(row.avatar);
}

function createAvatarObserver() {
  return new IntersectionObserver((entries, observer) => {
    for (const entry of entries) {
      const { isIntersecting, target } = entry;
      if (!isIntersecting) continue;
      const row = rows.get(target.dataset.pubkey);
      observer.unobserve(target);
      if (!row?.avatarRequest) continue;
      const { generation, picture, signal } = row.avatarRequest;
      row.avatarRequest = null;
      void avatarQueue.run(() => loadAvatar(row, picture, generation, signal)).catch(() => {});
    }
  });
}

function render(values, generation, signal) {
  const pubkeys = directFollows(values);
  avatarObserver = createAvatarObserver();
  list.replaceChildren();
  for (const pubkey of pubkeys) {
    const item = rowTemplate.content.firstElementChild.cloneNode(true);
    const button = item.querySelector('button');
    const avatar = item.querySelector('.avatar');
    const image = item.querySelector('img');
    const fallback = item.querySelector('.avatar-fallback');
    const name = item.querySelector('.follow-name');
    const key = item.querySelector('.follow-pubkey');

    button.dataset.pubkey = pubkey;
    button.setAttribute('aria-label', `Open profile ${pubkey}`);
    button.addEventListener('click', () => incEmit(PROFILE_OPEN_TOPIC, profileOpen(pubkey)));
    avatar.dataset.pubkey = pubkey;
    fallback.textContent = fallbackText('');
    name.textContent = shortPubkey(pubkey);
    list.append(item);
    rows.set(pubkey, {
      avatar,
      avatarRequest: null,
      fallback,
      image,
      key,
      name,
      objectUrl: null,
    });
  }
  status.textContent = pubkeys.length === 0
    ? 'No direct follows in the latest-known NMP view.'
    : `${pubkeys.length} latest-known direct follows`;

  void enrichRows(pubkeys, generation, signal);
}

async function enrichQuery(query, generation, signal) {
  let result;
  try {
    result = await profileQueue.run(() => outboxQuery(query.filters, query.options));
    if (result.error) throw new Error(result.error);
  } catch (error) {
    if (!current(generation) || signal.aborted) return;
    const retries = splitProfileQueryRequest(query);
    if (retries.length > 0) {
      await Promise.allSettled(retries.map((retry) => enrichQuery(retry, generation, signal)));
      return;
    }
    const row = rows.get(query.options.authors[0]);
    if (row) {
      row.name.title = `Profile unavailable through NAP-OUTBOX: ${error instanceof Error ? error.message : String(error)}`;
    }
    return;
  }
  if (!current(generation) || signal.aborted) return;

  const profiles = canonicalProfiles(result.events, query.options.authors);
  for (const [pubkey, profile] of profiles) {
    const row = rows.get(pubkey);
    if (!row) continue;
    row.name.textContent = profile.name;
    row.key.textContent = shortPubkey(pubkey);
    setAvatarFallback(row, profile.name);
    if (profile.picture) observeAvatar(row, profile.picture, generation, signal);
  }
}

async function enrichRows(pubkeys, generation, signal) {
  const requests = profileQueryBatches(pubkeys)
    .map((query) => enrichQuery(query, generation, signal));
  await Promise.allSettled(requests);
}

async function loadFollows() {
  const generation = ++refreshGeneration;
  const controller = resetEnrichment();
  refresh.disabled = true;
  status.textContent = 'Reading latest-known follows through NMP…';
  try {
    const values = await identityGetFollows();
    if (current(generation)) {
      render(values, generation, controller.signal);
    }
  } catch (error) {
    if (current(generation)) {
      status.textContent = `Identity unavailable: ${error instanceof Error ? error.message : String(error)}`;
    }
  } finally {
    if (current(generation)) refresh.disabled = false;
  }
}

refresh.addEventListener('click', () => void loadFollows());
addEventListener('pagehide', () => {
  refreshController?.abort();
  avatarObserver?.disconnect();
  const reason = new DOMException('Follow list closed.', 'AbortError');
  avatarQueue.clear(reason);
  profileQueue.clear(reason);
  clearObjectUrls();
});
void loadFollows();
