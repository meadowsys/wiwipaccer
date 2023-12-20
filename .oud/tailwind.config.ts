import type { Config } from "tailwindcss";
import plugin from "tailwindcss/plugin";

export default {
	content: [
		"./app.vue",
		"./{assets,components,composables,layouts,pages}/**/*.{vue,ts,js,tsx,jsx}"
	],
	darkMode: "class",
	plugins: [
		plugin(({ addBase }) => {
			addBase({
				html: { "tab-size": "3" },
				body: { "tab-size": "3" }
			});
		})
	]
} satisfies Config;
