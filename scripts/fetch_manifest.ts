#!/usr/bin/env bun

import { z } from "zod";

// a bit unnecessary, since the proc macro will parse/check this too
// but strict mode will fail if someday, something new suddenly shows up
let validator = z.object({
	latest: z.object({
		release: z.string(),
		snapshot: z.string()
	}).strict(),
	versions: z.object({
		id: z.string(),
		type: z.union([
			z.literal("snapshot"),
			z.literal("release"),
			z.literal("old_beta"),
			z.literal("old_alpha")
		]),
		url: z.string().url(),
		// these dates will be parsed by the proc macro
		time: z.string(),
		releaseTime: z.string(),
		sha1: z.string().length(40),
		complianceLevel: z.union([
			z.literal(1),
			z.literal(0)
		])
	}).strict().array()
}).strict();

let manifest_url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
let manifest = await fetch(manifest_url).then(r => r.text());
let json = validator.parse(JSON.parse(manifest));
let formatted = JSON.stringify(json, undefined, "\t") + "\n";

await Bun.write("./packages/mc-versions-macro/src/version_manifest_v2.json", formatted);
