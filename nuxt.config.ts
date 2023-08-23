import { extend_base } from "./nuxt.config.base";

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
