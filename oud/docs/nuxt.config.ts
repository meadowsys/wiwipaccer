import { extend_base } from "../nuxt.config.base";

const cfg = defineNuxtConfig({
	app: {
		baseURL: "/docs/"
	},
	devServer: {
		port: 3001
	},
	modules: [
		"@nuxt/content"
	]
});

extend_base(cfg);
export default cfg;
