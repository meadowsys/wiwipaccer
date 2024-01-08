// https://nuxt.com/docs/api/configuration/nuxt-config

export default defineNuxtConfig({
	telemetry: false,
	devtools: { enabled: true },
	ssr: false,
	vite: {
		server: { strictPort: true }
	},
	builder: "vite"
});
