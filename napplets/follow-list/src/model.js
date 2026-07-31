import { isCanonicalPubkey } from '../../../contracts/profile-open.js';

export const MAXIMUM_RENDERED_FOLLOWS = 1_024;
export const MAXIMUM_AVATAR_REQUESTS = 4;
export const MAXIMUM_AVATAR_OBJECT_URLS = 32;
export const MAXIMUM_PROFILE_REQUESTS = 2;
export const MAXIMUM_PROFILE_RETRY_REQUESTS = 32;

export function directFollows(values, limit = MAXIMUM_RENDERED_FOLLOWS) {
  if (!Array.isArray(values)) return [];
  const unique = new Set();
  for (const value of values) {
    if (isCanonicalPubkey(value)) unique.add(value);
    if (unique.size === limit) break;
  }
  return [...unique];
}

export function shortPubkey(pubkey) {
  return `${pubkey.slice(0, 12)}…${pubkey.slice(-8)}`;
}

export function createAvatarObjectUrlStore(limit = MAXIMUM_AVATAR_OBJECT_URLS) {
  if (!Number.isSafeInteger(limit) || limit < 1) {
    throw new RangeError('avatar object URL limit must be a positive safe integer');
  }
  const entries = new Map();
  return Object.freeze({
    remember(objectUrl, row) {
      entries.delete(objectUrl);
      entries.set(objectUrl, row);
      const evicted = [];
      while (entries.size > limit) {
        const oldest = entries.entries().next().value;
        entries.delete(oldest[0]);
        evicted.push(oldest);
      }
      return evicted;
    },
    remove(objectUrl) {
      entries.delete(objectUrl);
    },
    drain() {
      const retained = [...entries];
      entries.clear();
      return retained;
    },
    get size() {
      return entries.size;
    },
  });
}

export function createProfileRetryBudget(limit = MAXIMUM_PROFILE_RETRY_REQUESTS) {
  if (!Number.isSafeInteger(limit) || limit < 0) {
    throw new RangeError('profile retry limit must be a non-negative safe integer');
  }
  let remaining = limit;
  return Object.freeze({
    take() {
      if (remaining === 0) return false;
      remaining -= 1;
      return true;
    },
    get remaining() {
      return remaining;
    },
  });
}

export function createBoundedTaskQueue(limit = MAXIMUM_AVATAR_REQUESTS) {
  if (!Number.isSafeInteger(limit) || limit < 1) {
    throw new RangeError('task queue limit must be a positive safe integer');
  }

  const pending = [];
  let active = 0;

  function drain() {
    while (active < limit && pending.length > 0) {
      const item = pending.shift();
      active += 1;
      Promise.resolve()
        .then(item.task)
        .then(item.resolve, item.reject)
        .finally(() => {
          active -= 1;
          drain();
        });
    }
  }

  return {
    run(task, key = null) {
      if (typeof task !== 'function') return Promise.reject(new TypeError('task must be a function'));
      const result = new Promise((resolve, reject) => pending.push({ key, task, resolve, reject }));
      drain();
      return result;
    },
    cancel(key, reason = new Error('queued task cancelled')) {
      const index = pending.findIndex((item) => item.key === key);
      if (index < 0) return false;
      const [item] = pending.splice(index, 1);
      item.reject(reason);
      return true;
    },
    clear(reason = new Error('task queue cleared')) {
      for (const item of pending.splice(0)) item.reject(reason);
    },
  };
}
