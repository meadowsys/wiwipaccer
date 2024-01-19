// https://nuxt.com/docs/api/configuration/nuxt-config

import { getIconCollections } from "@egoist/tailwindcss-icons";

export default defineNuxtConfig({
	telemetry: false,
	devtools: { enabled: true },
	ssr: false,
	vite: {
		server: { strictPort: true },
		optimizeDeps: {
			entries: "@tauri-apps/api"
		}
	},
	builder: "vite",
	modules: [
		"@pinia/nuxt"
	]
});
