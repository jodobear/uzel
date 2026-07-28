import { nip5aManifest } from '@napplet/vite-plugin';
import { defineConfig } from 'vite';

export default defineConfig({
  base: './',
  build: { modulePreload: false, target: 'es2023' },
  plugins: [
    nip5aManifest({
      nappletType: 'follow-list',
      title: 'Direct follows',
      description: 'Small direct-follow selector using runtime-mediated identity data.',
      requires: { explicit: ['identity', 'inc'], infer: true, mode: 'error' },
      artifactMode: 'single-file',
    }),
  ],
});
