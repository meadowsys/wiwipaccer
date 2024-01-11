import { FluentBundle, FluentResource } from "@fluent/bundle";
import type { FluentVariable } from "@fluent/bundle";

export type Locale = string;
export type Path = string;
export type I18nStore = {
	[k: Locale]: {
		[k: Path]: FluentBundle
	}
};

export const use_i18n = defineStore("i18n-strings", () => {
	const current_locale = ref("en");

	const locales = ref<I18nStore>({});
	const loaded_locales = ref<Array<string>>([]);

	async function fetch_all(locale: string) {
		let all_files = (await $fetch("/i18n/files.txt") as string)
			.trim()
			.split("\n")
			.map(l => l.trim());

		let fetched_optional = await Promise.all(all_files.map(async (path) => {
			try {
				return [path, await (await $fetch(`/i18n/${locale}/${path}.ftl`) as Blob).text()] as const
			} catch (e) {
				// probably doesn't exist
			}
		}));

		let bundles = fetched_optional
			.filter((l): l is [string, string] => Boolean(l))
			.map(([path, res_str]) => {
				let resource = new FluentResource(res_str);
				let bundle = new FluentBundle(locale);
				bundle.addResource(resource);
				return [path, bundle] as const;
			});

		if (!locales.value[locale]) locales.value[locale] = {};
		for (const [path, bundle] of bundles) {
			locales.value[locale][path] = bundle;
		}
	}

	async function set_locale(locale: string) {
		if (!loaded_locales.value.includes(locale)) {
			await fetch_all(locale);
		}

		current_locale.value = locale;
	}

	function t<Opts extends Record<string, FluentVariable>>(key: string, opts?: Opts) {
		let splitkey = key.split(".");
		if (splitkey.length < 2) return key;

		let k = splitkey.pop()!;
		let p = splitkey.join("/");

		return computed(() => {
			let bundle = locales.value[current_locale.value]?.[p];
			let message = bundle?.getMessage(k);

			if (message && message.value) return bundle.formatPattern(message.value, opts);
			else return key;
		});
	}

	return {
		current_locale: readonly(current_locale),
		loaded_locales: readonly(loaded_locales),
		force_refresh: fetch_all,
		force_refresh_current: () => fetch_all(current_locale.value),
		set_locale,
		t
	};
});

export const useT = () => use_i18n().t;
