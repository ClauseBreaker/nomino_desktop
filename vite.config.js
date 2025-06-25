import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig(async () => ({
	plugins: [sveltekit()],
	clearScreen: false,
	server: {
		port: 1421,
		strictPort: true,
		watch: {
			ignored: ["**/src-tauri/**"]
		}
	}
})); 