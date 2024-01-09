import type { Config } from "tailwindcss";

export default <Config>{
	content: [
		"./app.vue",
		"./{assets,components,composables,layouts,pages}/**/*.{vue,ts,js,tsx,jsx}"
	]
};
