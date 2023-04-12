import { extend_base } from "../base.nuxt.config";

const cfg = defineNuxtConfig({
	app: {
		baseURL: "/docs/"
	}
});

extend_base(cfg);
export default cfg;
