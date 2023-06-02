import { colord } from "colord";

export type Theme = Partial<{
	/** Primary colour */
	"--p": string;
	/** Primary colour when focused */
	"--pf": string;
	/** Foreground content colour to use on primary colour */
	"--pc": string;

	/** Secondary colour */
	"--s": string;
	/** Secondary colour when focused */
	"--sf": string;
	/** Foreground content colour to use on secondary colour */
	"--sc": string;

	/** Accent colour */
	"--a": string;
	/** Accent colour when focused */
	"--af": string;
	/** Foreground content colour to use on accent colour */
	"--ac": string;

	/** Neutral colour */
	"--n": string;
	/** Neutral colour when focused */
	"--nf": string;
	/** Foreground content colour to use on neutral colour */
	"--nc": string;

	/** Base colour of page, used on blank backgrounds */
	"--b1": string;
	/** Base colour of page, a bit darker */
	"--b2": string;
	/** Base colour of page, a bit even more darker */
	"--b3": string;
	/** Foreground content colour to use on base colour */
	"--bc": string;

	/** Info colour */
	"--in": string;
	/** Foreground content colour to use on info colour */
	"--inc": string;
	/** Success colour */
	"--su": string;
	/** Foreground content colour to use on success colour */
	"--suc": string;
	/** Warning colour */
	"--wa": string;
	/** Foreground content colour to use on warning colour */
	"--wac": string;
	/** Error colour */
	"--er": string;
	/** Foreground content colour to use on error colour */
	"--erc": string;
}>;

const default_theme: Required<Theme> = {
	"--p": "#9e6bff",
	"--pf": "#aa82ff",
	"--pc": "#ffffff",

	"--s": "#ff794c",
	"--sf": "#fc9778",
	"--sc": "#ffffff",

	"--a": "#8cffdb",
	"--af": "#c6fee9",
	"--ac": "#ffffff",

	"--n": "#555555",
	"--nf": "#666666",
	"--nc": "#000000",

	"--b1": "#ffffff",
	"--b2": "#dddddd",
	"--b3": "#bbbbbb",
	"--bc": "#000000",

	"--in": "#3abff8",
	"--inc": "#002b3d",
	"--su": "#36d399",
	"--suc": "#003320",
	"--wa": "#fbbd23",
	"--wac": "#382800",
	"--er": "#f87272",
	"--erc": "#470000"
};

const theme_config = reactive<Theme>({ ...default_theme });
const theme = computed(() => {
	let theme: Theme = {};

	Object.entries(theme_config).forEach(([_k, v]) => {
		let k = _k as keyof Theme;
		let { h, s, l } = colord(v).toHsl();
		theme[k] = `${h} ${s}% ${l}%`;
	});

	Object.entries(default_theme).forEach(([_k, v]) => {
		let k = _k as keyof Theme;
		if (!theme[k]) {
			let { h, s, l } = colord(v).toHsl();
			theme[k] = `${h} ${s}% ${l}`;
		}
	});

	return theme;
});

export function use_theme() {
	return theme;
}
