import { extend_base } from "./base.nuxt.config";

const cfg = defineNuxtConfig({
	ssr: false,
	nitro: {
		devProxy: {
			"/docs": "http://localhost:3001/docs"
		}
	}
});

extend_base(cfg);
export default cfg;
