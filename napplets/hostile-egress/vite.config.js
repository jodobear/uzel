import { nip5aManifest } from '@napplet/vite-plugin';
import { defineConfig } from 'vite';

export default defineConfig({
  base: './',
  build: { modulePreload: false, target: 'es2023' },
  plugins: [
    nip5aManifest({
      nappletType: 'hostile-egress',
      title: 'Hostile egress probe',
      description: 'Test-only direct browser and native authority probe.',
      requires: { explicit: ['config'], infer: true, mode: 'error' },
      configSchema: {
        type: 'object',
        properties: {
          sentinel: {
            type: 'string',
            description: 'Unique live unprivileged http://127.0.0.1 sentinel URL.',
            default: 'unset',
          },
        },
        required: ['sentinel'],
        additionalProperties: false,
      },
      artifactMode: 'single-file',
    }),
  ],
});
