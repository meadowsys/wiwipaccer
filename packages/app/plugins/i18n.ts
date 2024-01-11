import type { App, InjectionKey } from "vue";
import { FluentBundle, FluentResource } from "@fluent/bundle";
import type { FluentVariable } from "@fluent/bundle";

type Locale = string;
type Path = string;
type TFn = (k: string, opts: Record<string, FluentVariable>) => Promise<string>;

export const t_key = Symbol("t_key") as InjectionKey<TFn>;

export default defineNuxtPlugin(nuxt => {
	create_i18n(nuxt.vueApp);
});


function create_i18n(app: App) {
	let locale = "en";

	let strings = new Map<Locale, Map<Path, FluentBundle | "none">>();

	const t = async (_k: string, opts: Record<string, FluentVariable>) => {
		let splitkey = _k.split(".");
		if (splitkey.length < 2) return _k;

		let key = splitkey.pop()!;
		let path = splitkey.join("/");

		let locale_map = strings.get(locale);
		let bundle = locale_map?.get(path);
		if (bundle === "none") return _k;
		let message = bundle?.getMessage(key);

		if (!message?.value) return _k;
		if (!bundle) {
			bundle = new FluentBundle(locale);

			let fetched: string;
			try {
				fetched = await $fetch(`/i18n/${locale}/${path}`) as string;
			} catch (e) {
				return _k
			}
			let resource = new FluentResource(fetched);

			let errors = bundle.addResource(resource);
			console.error(`fluent errors: ${
				errors.map(e => e.message).join(", ")
			}`);

			if (!locale_map) {
				locale_map = new Map();
				strings.set(locale, locale_map);
			}

			locale_map.set(path, bundle);
		}

		// if the bundle exists, there's gotta be a locale to pull from
		// and if there wasn't bundle, its been taken care of now
		locale_map = locale_map!;

		return bundle.formatPattern(message.value, opts);
	};

	app.provide(t_key, t);
}
