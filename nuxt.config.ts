import { extend_base } from "./base.nuxt.config";

const cfg = defineNuxtConfig({
	ssr: false
});

extend_base(cfg);
export default cfg;
