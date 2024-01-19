import { FluentBundle, FluentResource, type FluentVariable } from "@fluent/bundle";

const en = "en";

export type LocaleMap = {
	locale: string;
	map: Record<string, FluentBundle>;
};

export const use_i18n = defineStore("i18n-strings", () => {
	const loaded_locales = ref<Array<LocaleMap>>([]);

	async function _set_locales(locales: Array<string>) {
		if (!locales.includes(en)) locales = [...locales, en];

		const all_files = (await $fetch("/i18n/files.txt") as string)
			.trim()
			.split("\n")
			.map(l => l.trim());

		const locale_bundles = await Promise.all(locales.map(async locale => {
			const loaded_files = (await Promise.all(all_files.map(async file => {
				try {
					const resource_blob = await $fetch(`/i18n/${locale}/${file}.ftl`) as Blob;
					const resource_str = await resource_blob.text();
					return [{ file, resource_str }];
				} catch (_) {
					// probably doesn't exist
					return []
				}
			}))).flat();

			const bundles = {
				locale,
				map: {}
			} as LocaleMap;

			for (const { file, resource_str } of loaded_files) {
				const bundle = new FluentBundle(locale);
				const resource = new FluentResource(resource_str);
				// TODO: do something with errors
				const errors = bundle.addResource(resource);
				bundles.map[file] = bundle;
			}

			return bundles;
		}));

		loaded_locales.value = locale_bundles;
	}

	function t(key: string, opts?: Record<string, FluentVariable>): ComputedRef<string> {
		const splitkey = key.split(".");
		if (splitkey.length < 2) return computed(() => key);

		const k = splitkey.pop()!;
		const p = splitkey.join("/");

		return computed(() => {
			for (const locale of loaded_locales.value) {
				const bundle = locale.map[p];
				const message = bundle?.getMessage(k);

				if (message && message.value) {
					return bundle.formatPattern(message.value, opts);
				}
			}

			return key;
		});
	}

	return {
		loaded_locales: computed(() => loaded_locales.value.map(l => l.locale)),
		_set_locales,
		t
	};
});

export const use_t = () => use_i18n().t;
