import { defu } from "defu";

export type ResolvedTheme = {
	[k in string]?: string;
};

export type Theme = {
	extends?: Array<Theme>;
} & ResolvedTheme

export const use_theme = defineStore("theme", () => {
	const loaded_theme = ref<Theme>({});
	const resolved_theme = computed<ResolvedTheme>(() => resolve_theme(loaded_theme.value));

	function _set_theme(theme: Theme) {
		loaded_theme.value = theme;
	}

	return {
		theme: resolved_theme,
		_set_theme
	};
});

function resolve_theme(theme: Theme) {
	const resolved: ResolvedTheme = {};

	Object.entries(theme)
		.filter(k => k[0] !== "extends")
		.forEach(([k, v]) => {
			if (!(k in resolved)) {
				resolved[k] = v as any;
			}
		});

	if ("extends" in theme) {
		defu(resolved, ...theme.extends!.map(resolve_theme));
	}

	return resolved;
}
