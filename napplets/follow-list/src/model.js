import { isCanonicalPubkey } from '../../../contracts/profile-open.js';

export const MAXIMUM_RENDERED_FOLLOWS = 1_024;
export const MAXIMUM_AVATAR_REQUESTS = 4;
export const MAXIMUM_PROFILE_REQUESTS = 2;

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
    run(task) {
      if (typeof task !== 'function') return Promise.reject(new TypeError('task must be a function'));
      const result = new Promise((resolve, reject) => pending.push({ task, resolve, reject }));
      drain();
      return result;
    },
    clear(reason = new Error('task queue cleared')) {
      for (const item of pending.splice(0)) item.reject(reason);
    },
  };
}
