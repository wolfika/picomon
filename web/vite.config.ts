import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [svelte()],
  build: { outDir: 'dist', emptyOutDir: true, assetsInlineLimit: 0 },
  // `bun run dev` proxies the API to a locally running picomon.
  server: { proxy: { '/api': process.env.PICOMON_URL ?? 'http://127.0.0.1:7340' } },
});
