import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = fileURLToPath(new URL('../../../../', import.meta.url));
const runId = `${new Date().toISOString().replaceAll(/[:.]/g, '-')}.${process.pid}`;

export const ARTIFACT_ROOT = process.env.UZEL_UI_ARTIFACT_ROOT
  ? resolve(process.env.UZEL_UI_ARTIFACT_ROOT)
  : join(repositoryRoot, '.artifacts', 'ui-acceptance', runId);

export const FIXTURE_PATH = '/tests/ui/fixtures/index.html';
export const EXTERNAL_SENTINEL_URL = 'https://ui-acceptance.invalid/network-sentinel';
export const HARNESS_MODE = 'mocked-native-real-artifact-renderer';

export const SCENARIOS = Object.freeze([
  'ready',
  'projection-overflow',
  'avatar-active-cancel',
  'follow-reload-failure',
  'profile-delay',
  'initialization-failure',
  'initialization-empty-identity',
  'initialization-identity-failure',
  'naddr-denied',
  'review-ambiguous',
  'confirmation-ambiguous',
  'cleanup-failure',
  'restart-reconciliation',
  'fault-proof',
]);

export const VIEWPORTS = Object.freeze([
  Object.freeze({ name: 'desktop-1366x768', width: 1366, height: 768 }),
  Object.freeze({ name: 'desktop-1920x1080', width: 1920, height: 1080 }),
]);

export const TEST_NADDR =
  'naddr1qqxxwmm0vskk6mmjde5kuecpzemhxue69uhhyetvv9ujuurjd9kkzmpwdejhgq3qye5ptcxfyyxl5vjvdjar2ua3f0hynkjzpx552mu5snj3qmx5pzjsxpqqqzynjsul3vr';
