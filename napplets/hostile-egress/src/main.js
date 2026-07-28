import '@napplet/shim';

import { nativeSurface, workerLoad } from './probes.js';

const target = 'http://127.0.0.1:9/hostile-egress';
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
  denied('fetch', () => fetch(target)),
  denied('xhr', () => new Promise((resolve, reject) => {
    const request = new XMLHttpRequest();
    request.open('GET', target);
    request.onload = resolve;
    request.onerror = reject;
    request.send();
  })),
  denied('websocket', () => new Promise((resolve, reject) => {
    const socket = new WebSocket('ws://127.0.0.1:9/hostile-egress');
    socket.onopen = resolve;
    socket.onerror = reject;
  })),
  denied('eventsource', () => new Promise((resolve, reject) => {
    const source = new EventSource(target);
    source.onopen = resolve;
    source.onerror = reject;
  })),
  denied('image', () => new Promise((resolve, reject) => {
    const image = new Image();
    image.onload = resolve;
    image.onerror = reject;
    image.src = target;
  })),
  denied('worker', () => workerLoad(target)),
  denied('serviceWorker', () => navigator.serviceWorker.register(target)),
  denied('beacon', () => navigator.sendBeacon(target, 'probe') ? Promise.resolve() : Promise.reject()),
]);

document.querySelector('#result').textContent = JSON.stringify(results, null, 2);
