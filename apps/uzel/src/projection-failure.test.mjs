import assert from 'node:assert/strict';
import test from 'node:test';

import {
  OUTBOX_PROJECTION_ERROR, projectedOutboxQueryFailure,
} from './projection-failure.js';

test('turns a correlated rejected outbox result into a bounded terminal result', () => {
  assert.deepEqual(
    projectedOutboxQueryFailure(
      { type: 'outbox.query', id: 'request-1' },
      { type: 'outbox.query.result', id: 'request-1', events: ['x'.repeat(70_000)] },
    ),
    {
      type: 'outbox.query.result',
      id: 'request-1',
      events: [],
      incomplete: true,
      error: OUTBOX_PROJECTION_ERROR,
    },
  );
});

test('does not fabricate a terminal result without exact request correlation', () => {
  assert.equal(
    projectedOutboxQueryFailure(
      { type: 'outbox.query', id: 'request-1' },
      { type: 'outbox.query.result', id: 'request-2', events: [] },
    ),
    null,
  );
  assert.equal(
    projectedOutboxQueryFailure(
      { type: 'outbox.getEvent', id: 'request-1' },
      { type: 'outbox.getEvent.result', id: 'request-1' },
    ),
    null,
  );
});
