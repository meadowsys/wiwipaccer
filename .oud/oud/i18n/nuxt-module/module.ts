import { defineNuxtModule } from "@nuxt/kit";

type Config = {};

export default defineNuxtModule<Config>({
	meta: {
		name: "i18n",
		configKey: "i18n"
	},
	defaults: {},
	setup(cfg, nuxt) {
	}
});
