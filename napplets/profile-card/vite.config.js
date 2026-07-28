import { nip5aManifest } from '@napplet/vite-plugin';
import { defineConfig } from 'vite';

export default defineConfig({
  base: './',
  build: { modulePreload: false, target: 'es2023' },
  plugins: [
    nip5aManifest({
      nappletType: 'profile-card',
      title: 'Profile card',
      description: 'Latest-known profile using runtime-mediated outbox data.',
      requires: { explicit: ['inc', 'outbox'], infer: true, mode: 'error' },
      artifactMode: 'single-file',
    }),
  ],
});
