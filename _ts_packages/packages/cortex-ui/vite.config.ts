import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte({ hot: false })],
  base: './',
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    sourcemap: true,
  },
  // vitest picks the browser-side Svelte entry, matching how @testing-library/svelte
  // mounts components in jsdom. Without this Svelte 5 resolves to the SSR build
  // and `mount()` throws `lifecycle_function_unavailable`.
  resolve: {
    conditions: ['browser'],
  },
  test: {
    environment: 'jsdom',
    environmentOptions: {
      jsdom: {
        url: 'http://localhost/',
      },
    },
    globals: false,
    include: ['tests/**/*.test.ts'],
    server: {
      deps: {
        inline: ['@testing-library/svelte'],
      },
    },
  },
});
