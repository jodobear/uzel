const EVIDENCE_REFRESH_ATTEMPTS = 32;
const EVIDENCE_REFRESH_INTERVAL_MS = 250;

function delay(milliseconds) {
  return new Promise((resolve) => setTimeout(() => resolve(), milliseconds));
}

export async function waitForEvidence(
  load,
  accepts,
  {
    attempts = EVIDENCE_REFRESH_ATTEMPTS,
    intervalMs = EVIDENCE_REFRESH_INTERVAL_MS,
    onAttempt = () => {},
  } = {},
) {
  if (!Number.isSafeInteger(attempts) || attempts < 1) throw new TypeError('attempts must be positive');
  if (!Number.isSafeInteger(intervalMs) || intervalMs < 0) throw new TypeError('intervalMs must be non-negative');
  let latest;
  for (let attempt = 1; attempt <= attempts; attempt += 1) {
    onAttempt(attempt, attempts);
    latest = await load();
    if (accepts(latest) || attempt === attempts) return latest;
    await delay(intervalMs);
  }
  return latest;
}
