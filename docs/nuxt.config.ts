import { extend_base } from "../base.nuxt.config";

const cfg = defineNuxtConfig({
	app: {
		baseURL: "/docs/"
	},
	devServer: {
		port: 3001
	}
});

extend_base(cfg);
export default cfg;
