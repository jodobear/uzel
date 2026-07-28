import '@napplet/shim';

import { nativeSurface, sentinelTargets, workerLoad } from './probes.js';

const target = sentinelTargets();
const results = nativeSurface();

async function denied(name, operation) {
  try {
    await operation();
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
  denied('beacon', () => navigator.sendBeacon(target.http, 'probe') ? Promise.resolve() : Promise.reject()),
]);

document.querySelector('#result').textContent = JSON.stringify(results, null, 2);
