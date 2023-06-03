// https://nuxt.com/docs/api/configuration/nuxt-config

import { NuxtConfig } from "@nuxt/schema";

export function extend_base(cfg: NuxtConfig) {
	cfg.telemetry = false;

	cfg.devServer ??= {};
	cfg.devServer.port ??= 3000;

	cfg.postcss ??= {};
	cfg.postcss.plugins ??= {};
	cfg.postcss.plugins.tailwindcss ??= {};
	cfg.postcss.plugins.autoprefixer ??= {};

	// cfg.modules ??= [];
	// cfg.modules.push("i18n");

	cfg.typescript ??= {};
	cfg.typescript.shim ??= false;
	cfg.typescript.strict ??= true;
	cfg.typescript.typeCheck ??= "build";

	cfg.vite ??= {};
	cfg.vite.build ??= {};
	cfg.vite.build.minify = "terser";
	cfg.vite.build.terserOptions ??= {};
	// assuming this will not be false
	if (typeof cfg.vite.build.terserOptions.compress !== "undefined") {
		if (typeof cfg.vite.build.terserOptions.compress === "object") {
			cfg.vite.build.terserOptions.compress.passes ??= 3;
		}
	} else {
		cfg.vite.build.terserOptions.compress = { passes: 3 };
	}

	return cfg;
}
