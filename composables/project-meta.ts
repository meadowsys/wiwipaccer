import { invoke } from "@tauri-apps/api";
import { z } from "zod";

const project_meta_validator = z.object({
	WithoutMCVersion: z.object({
		name: z.string(),
		description: z.string(),
		path: z.string(),
	})
});

export async function invoke_get_project_meta(path: string) {
	return await invoke("get_project_meta", { path });
}

export type ProjectSupportedVersions = z.infer<typeof project_basic_meta_validator>;
const project_basic_meta_validator = z.object({
	names: z.string().array(),
	versions: z.object({
		name: z.string(),
		release_type: z.union([
			z.literal("release"),
			z.literal("snapshot"),
			z.literal("old_beta"),
			z.literal("old_alpha")
		]),
		format: z.union([
			z.object({
				Verified: z.number()
			}),
			z.object({
				Unverified: z.number()
			}),
			z.object({
				Maybe: z.number()
			}),
			z.literal("Unknown"),
			z.literal("None"),
		])
	}).strict().array()
});

export async function invoke_get_project_basic_meta(path: string) {
	let versions = await invoke("get_project_basic_meta", { path });
	return project_basic_meta_validator.parse(versions);
}
