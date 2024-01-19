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
		"@nuxt/ui",
		"@pinia/nuxt",
		"@unocss/nuxt",
		"@vueuse/nuxt"
	],
	ui: {
		global: true,
		icons: ["ph"]
	}
});
