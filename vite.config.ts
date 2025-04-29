import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	server: {
		port: 1420,
		watch: {
		  // 3. tell vite to ignore watching `src-tauri`
		  ignored: ["**/src-tauri/**"],
		},
		hmr: {
			overlay: false, // 禁用错误覆盖
		  }
	  }
});
