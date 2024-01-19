import { defineConfig } from "unocss";
import preset_mini from "@unocss/preset-mini";

export default defineConfig({
	presets: [
		preset_mini()
	]
	// theme: {
	// 	colors: {
	// 		primary: "var(--wiwi-primary)",
	// 		secondary: "var(--wiwi-secondary)",
	// 		background: "var(--wiwi-background)",
	// 		text: "var(--wiwi-text)"
	// 	}
	// }
});
