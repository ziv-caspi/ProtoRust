import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	optimizeDeps: {
        exclude: ["codemirror", "@codemirror/lang-json",  "@codemirror/language-json", "@codemirror/state", 'totalist', 'sirv', 'local-access', 'totalist', 'sirv', 'local-access' /* ... */],
    },
});
