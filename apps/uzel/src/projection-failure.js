/**
 * @param {unknown} value
 * @returns {value is Record<string, unknown>}
 */
function plainObject(value) {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

export const OUTBOX_PROJECTION_ERROR = 'trusted-shell-response-rejected';

/**
 * @param {unknown} request
 * @param {unknown} response
 */
export function projectedOutboxQueryFailure(request, response) {
  if (!plainObject(request) || !plainObject(response)) return null;
  if (request.type !== 'outbox.query' || response.type !== 'outbox.query.result') return null;
  if (
    typeof request.id !== 'string'
    || request.id.length === 0
    || request.id.length > 128
    || response.id !== request.id
  ) return null;

  return Object.freeze({
    type: 'outbox.query.result',
    id: request.id,
    events: Object.freeze([]),
    incomplete: true,
    error: OUTBOX_PROJECTION_ERROR,
  });
}
