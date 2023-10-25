import { defineConfig } from "histoire";
import { HstVue as vue } from "@histoire/plugin-vue";
import { HstNuxt as nuxt } from "@histoire/plugin-nuxt";

export default defineConfig({
	plugins: [
		vue(),
		nuxt()
	]
});
