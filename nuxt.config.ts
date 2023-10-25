// https://nuxt.com/docs/api/configuration/nuxt-config

export default defineNuxtConfig({
	telemetry: false,
	devtools: { enabled: true },
	ssr: false,
	postcss: {
		plugins: {
			tailwindcss: {},
			autoprefixer: {}
		}
	},
	typescript: {
		strict: true,
		shim: false,
		typeCheck: "build"
	},
	vite: {
		build: {
			minify: "terser",
			terserOptions: {
				compress: {
					passes: 3
				}
			}
		}
	}
});
