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
      requires: [],
      artifactMode: 'single-file',
    }),
  ],
});
