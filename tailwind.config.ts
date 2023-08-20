import type { Config } from "tailwindcss";
import plugin from "tailwindcss/plugin";
import daisyui from "daisyui";
import tailwind_typography from "@tailwindcss/typography";
import { getIconCollections, iconsPlugin } from "@egoist/tailwindcss-icons";

const var_prefix = "kiwin-";
const weight_var = `--${var_prefix}font-wght`;
// const width_var = `--${var_prefix}font-wdth`;
// const mono_var = `--${var_prefix}font-mono`;
// const casual_var = `--${var_prefix}font-casl`;
// const slant_var = `--${var_prefix}font-slnt`;
// const cursive_var = `--${var_prefix}font-crsv`;

export const gen_config = (rel: string) => ({
	content: [
		"./app.vue",
		"./{assets,components,composables,layouts,pages}/**/*.{vue,ts,js,tsx,jsx}"
	],
	darkMode: "class",
	theme: {
		extend: {}
	},
	daisyui: {
		base: false,
		themes: [
			// {
			// 	light: {
			// 		primary: "#9e6bff",
			// 		secondary: "#ff794c",
			// 		accent: "#8cffdb",
			// 		neutral: "#bababa",
			// 		"base-100": "#ffffff",
			// 		info: "#3abff8",
			// 		success: "#36d399",
			// 		warning: "#fbbd23",
			// 		error: "#f87272"
			// 	}
			// }
		]
	},
	plugins: [
		tailwind_typography,
		daisyui,
		iconsPlugin({
			// https://icones.js.org
			collections: getIconCollections(["octicon"])
		}),

		plugin(({ addBase }) => {
			// this deserves its own plugin
			addBase({
				html: { "tab-size": "3" },
				body: { "tab-size": "3" } // ensure
			});
		}),

		// variable font switches
		plugin(({ addBase, addUtilities, matchUtilities, theme }) => {
			addBase({
				"*, ::before, ::after": {
					[weight_var]: "400"
					// [width_var]: "100",
					// [mono_var]: "0",
					// [casual_var]: "0",
					// [slant_var]: "0",
					// [cursive_var]: "0.5"
				}
			});

			matchUtilities(
				{ "font-wght": weight => ({ [weight_var]: weight } as any) },
				{
					values: {
						.../** @type {any} */(theme("fontWeight")),
						"100": "100",
						"200": "200",
						"300": "300",
						"400": "400",
						"500": "500",
						"600": "600",
						"700": "700",
						"800": "800",
						"900": "900",
						"1000": "1000"
					}
				}
			);

			// matchUtilities(
			// 	{ "font-wdth": width => ({ [width_var]: width }) },
			// 	{
			// 		values: {
			// 			"normal": "100",
			// 			"75": "75",
			// 			"80": "80",
			// 			"90": "90",
			// 			"100": "100",
			// 			"110": "110",
			// 			"120": "120",
			// 			"125": "125",
			// 		}
			// 	}
			// );

			addUtilities({
				".font-ital": {
					"font-style": "italic"
				}
			});

			// matchUtilities(
			// 	{ "font-mono": mono => ({ [mono_var]: mono }) },
			// 	{
			// 		values: {
			// 			DEFAULT: "1",
			// 			"0": "0",
			// 			"0.25": "0.25",
			// 			"0.5": "0.5",
			// 			"0.75": "0.75",
			// 			"1": "1"
			// 		}
			// 	}
			// );

			// matchUtilities(
			// 	{ "font-casl": casual => ({ [casual_var]: casual }) },
			// 	{
			// 		values: {
			// 			DEFAULT: "1",
			// 			"0": "0",
			// 			"0.25": "0.25",
			// 			"0.5": "0.5",
			// 			"0.75": "0.75",
			// 			"1": "1"
			// 		}
			// 	}
			// );

			// matchUtilities(
			// 	{ "font-slnt": slant => ({ [slant_var]: slant }) },
			// 	{
			// 		values: {
			// 			DEFAULT: "-15",
			// 			"-15": "-15",
			// 			"-14": "-14",
			// 			"-13": "-13",
			// 			"-12": "-12",
			// 			"-11": "-11",
			// 			"-10": "-10",
			// 			"-9": "-9",
			// 			"-8": "-8",
			// 			"-7": "-7",
			// 			"-6": "-6",
			// 			"-5": "-5",
			// 			"-4": "-4",
			// 			"-3": "-3",
			// 			"-2": "-2",
			// 			"-1": "-1",
			// 			"0": "0"
			// 		}
			// 	}
			// );

			// matchUtilities(
			// 	{ "font-crsv": cursive => ({ [cursive_var]: cursive }) },
			// 	{
			// 		values: {
			// 			DEFAULT: "1",
			// 			"0": "0",
			// 			"0.5": "0.5",
			// 			"1": "1"
			// 		}
			// 	}
			// );
		}),

		// font Andika
		plugin(({ addBase, addUtilities }) => {
			addBase({
				"@font-face": {
					"font-family": "Andika",
					"src": `url("~/${rel}/fonts/Andika-6.200/Andika-Regular.ttf"), url("~/${rel}/fonts/Andika-6.200/Andika-Bold.ttf"), url("~/${rel}/fonts/Andika-6.200/Andika-Italic.ttf"), url("~/${rel}/fonts/Andika-6.200/Andika-BoldItalic.ttf")`
				}
			});
			addUtilities({
				".font-andika": {
					"font-family": "Andika"
				}
			});
		}),

		// font Fira Code (variable font)
		plugin(({ addBase, addUtilities }) => {
			addBase({
				"@font-face": {
					"font-family": "Fira Code",
					"src": `url("~/${rel}/fonts/Fira_Code_v6.2/variable_ttf/FiraCode-VF.ttf")`
				}
			});
			addUtilities({
				".font-fira-code": {
					"font-family": "Fira Code",
					"font-variation-settings": `"wght" var(${weight_var})`
				}
			})
		}),

		// font Permanent Marker
		plugin(({ addBase, addUtilities }) => {
			addBase({
				"@font-face": {
					"font-family": "Permanent Marker",
					"src": `url("~/${rel}/fonts/Permanent_Marker/PermanentMarker-Regular.ttf")`
				}
			});
			addUtilities({
				".font-permanent-marker": {
					"font-family": "Permanent Marker"
				}
			});
		})
	]
}) satisfies Config;

export default gen_config(".");
