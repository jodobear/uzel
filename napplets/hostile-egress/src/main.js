import '@napplet/shim';
import { get as getConfig } from '@napplet/nap/config/sdk';

import {
  attemptRawWebKitInvoke,
  boundedAttempt,
  nativeSurface,
  sentinelTargets,
  workerLoad,
} from './probes.js';

const config = await getConfig();
const target = sentinelTargets(config.sentinel);
const results = nativeSurface();
try {
  results.beacon = navigator.sendBeacon(target.http, 'probe') ? 'queued' : 'rejected';
} catch {
  results.beacon = 'rejected';
}

async function denied(name, operation) {
  try {
    await boundedAttempt(operation);
    results[name] = false;
  } catch {
    results[name] = true;
  }
}

await Promise.all([
  denied('fetch', () => fetch(target.http)),
  denied('xhr', () => new Promise((resolve, reject) => {
    const request = new XMLHttpRequest();
    request.open('GET', target.http);
    request.onload = resolve;
    request.onerror = reject;
    request.send();
  })),
  denied('websocket', () => new Promise((resolve, reject) => {
    const socket = new WebSocket(target.websocket);
    socket.onopen = resolve;
    socket.onerror = reject;
  })),
  denied('eventsource', () => new Promise((resolve, reject) => {
    const source = new EventSource(target.http);
    source.onopen = resolve;
    source.onerror = reject;
  })),
  denied('image', () => new Promise((resolve, reject) => {
    const image = new Image();
    image.onload = resolve;
    image.onerror = reject;
    image.src = target.http;
  })),
  denied('worker', () => workerLoad(target.http)),
  denied('serviceWorker', () => navigator.serviceWorker.register(target.http)),
  denied('media', () => new Promise((resolve, reject) => {
    const media = document.createElement('audio');
    media.oncanplay = resolve;
    media.onerror = reject;
    media.src = target.http;
    media.load();
  })),
  denied('iframe', () => new Promise((resolve, reject) => {
    const frame = document.createElement('iframe');
    frame.onload = () => {
      try {
        if (frame.contentWindow?.location.href === target.http) resolve();
        else reject(new Error('iframe did not reach the sentinel URL'));
      } catch {
        reject(new Error('iframe navigation did not expose success'));
      }
    };
    frame.onerror = reject;
    frame.src = target.http;
    document.body.append(frame);
  })),
  denied('form', () => new Promise((resolve, reject) => {
    const targetName = 'uzel-hostile-form-target';
    const frame = document.createElement('iframe');
    frame.name = targetName;
    frame.onload = () => reject(new Error('form navigation did not prove sentinel success'));
    frame.onerror = reject;
    const form = document.createElement('form');
    form.action = target.http;
    form.method = 'POST';
    form.target = targetName;
    document.body.append(frame, form);
    form.submit();
  })),
  denied('navigation', () => {
    globalThis.top.location.href = target.http;
  }),
  denied('popup', () => {
    const popup = globalThis.open(target.http, '_blank');
    if (popup === null) throw new Error('sandbox denied popup');
    popup.close();
  }),
]);

// WKScriptMessageHandler delivery is synchronous: only publish the final report
// after the invalid-key raw message has returned from the native handler.
results.rawInvokeAttempted = attemptRawWebKitInvoke();
const publishResult = () => {
  document.querySelector('#result').textContent = JSON.stringify(results, null, 2);
  globalThis.parent.postMessage({
    type: 'uzel.hostile.result',
    version: 0,
    report: results,
  }, '*');
};
publishResult();
