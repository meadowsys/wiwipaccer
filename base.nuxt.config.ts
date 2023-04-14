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

	cfg.typescript ??= {}
	cfg.typescript.shim ??= false;
	cfg.typescript.strict ??= true;
	cfg.typescript.typeCheck ??= "build";

	return cfg;
}
