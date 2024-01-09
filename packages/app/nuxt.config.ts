// https://nuxt.com/docs/api/configuration/nuxt-config

import { getIconCollections } from "@egoist/tailwindcss-icons";

export default defineNuxtConfig({
	telemetry: false,
	devtools: { enabled: true },
	ssr: false,
	vite: {
		server: { strictPort: true }
	},
	builder: "vite",
	modules: [
		"@nuxt/ui"
	],
	ui: {
		global: true,
		icons: ["ph"]
	}
});
