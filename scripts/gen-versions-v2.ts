import * as fs from "fs";
import * as path from "path";
import { z } from "zod";
import * as crypto from "crypto";
import * as cp from "child_process";
import { run_if_ci } from "./lib";

const src = "lib/src/internal/pack_formats_src";
const dest = "lib/src/internal/pack_formats.rs";

(async () => {
	const versions = parse_pack_formats(src);
	const versions_mojang = await fetch_mojang_versions();
	const combined = combine_versions(versions, versions_mojang);

	const mapped_versions = gen_mapped_versions(combined);
	const list_of_pack_versions = gen_list_of_pack_versions(mapped_versions);
	// const consts = gen_consts(list_of_pack_versions);

	const pack_formats_const = gen_pack_formats_const(list_of_pack_versions);
	// const enum_and_etc = gen_enum_and_etc_from_enum(list_of_pack_versions);

	const lines = get_src_lines(src);
	const [pre, post] = get_pre_post_generated(lines);

	const file: Array<string> = [
		...pre,

		// ...consts,
		// "",
		pack_formats_const,
		// "",
		// ...enum_and_etc,

		...post
	];

	fs.writeFileSync(path.resolve(dest), file.join("\n"));
})();

type SpecifiersWithVersion = "Verified" | "Unverified";
type SpecifiersWithoutVersion = "Unknown" | "None";
type Specifiers = SpecifiersWithVersion | SpecifiersWithoutVersion;

function parse_pack_formats(src: string) {
	const comment_marker = "//";

	let parsed = fs.readFileSync(path.resolve(src), "utf8")
		.trim()
		.split("\n")
		.map((line, i) => ({ line: line.trim(), line_number: i + 1 }))
		.filter(({ line }) => line.length > 0 && !line.startsWith(comment_marker))
		.map(l => {
			const comment_start = l.line.indexOf(comment_marker);
			const line = comment_start < 0 ? l.line : l.line.substring(0, comment_start);
			return { ...l, line };
		})
		.map(l => ({ l, parsed: parse_line(l.line) }))
		.filter(l => l.parsed !== undefined)
		.map(({ l, parsed }) => ({ ...l, ...(parsed as NonNullable<typeof parsed>) }));

	return parsed;
}

function parse_line(line: string) {
	const last_space = line.lastIndexOf(" ");

	const specifier_lowercased = line.substring(last_space)
		.trim()
		.toLowerCase();

	const rest = line.substring(0, last_space);

	if (specifier_lowercased === "verified") return specifier_with_version("Verified", rest);
	if (specifier_lowercased === "unverified") return specifier_with_version("Unverified", rest);

	if (specifier_lowercased === "unknown") return specifier_without_version("Unknown", rest);
	if (specifier_lowercased === "none") return specifier_without_version("None", rest);

	return undefined;
}

function specifier_with_version(specifier: SpecifiersWithVersion, rest: string) {
	const last_space = rest.lastIndexOf(" ");

	const mc_version = rest.substring(0, last_space).trim();

	const specifier_version_str = rest.substring(last_space).trim();
	const specifier_version = Number.parseInt(specifier_version_str, 10);

	if (Number.isNaN(specifier_version)) return undefined;

	return { mc_version, specifier, specifier_version };
}

function specifier_without_version(specifier: SpecifiersWithoutVersion, rest: string) {
	const mc_version = rest.trim();

	return { mc_version, specifier };
}

async function fetch_mojang_versions() {
	const mojang_manifest_url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

	const mojang_versions_validator = z.object({
		latest: z.object({
			release: z.string(),
			snapshot: z.string()
		}).strict(),
		versions: z.object({
			id: z.string(),
			type: z.union([
				z.literal("release"),
				z.literal("snapshot"),
				z.literal("old_beta"),
				z.literal("old_alpha")
			]),
			url: z.string().url(),
			time: z.coerce.date(),
			releaseTime: z.coerce.date()
		}).strict().array()
	}).strict();

	let mojang_versions = await fetch(mojang_manifest_url)
		.then(r => r.json())
		.then(r => mojang_versions_validator.parse(r));

	// sorts in place
	mojang_versions.versions
		.sort((a, b) => b.releaseTime.getTime() - a.releaseTime.getTime());

	return mojang_versions;
}

function combine_versions(
	versions: ReturnType<typeof parse_pack_formats>,
	versions_mojang: Awaited<ReturnType<typeof fetch_mojang_versions>>
) {
	let combined = versions_mojang.versions.map(version => {
		const release_type =
			version.type === "release" ? "Release" as const
			: version.type === "snapshot" ? "Snapshot" as const
			: version.type === "old_beta" ? "OldBeta" as const
			: version.type === "old_alpha" ? "OldAlpha" as const
			: undefined;

		if (!release_type) throw new Error(`version.type not correct, got ${version.type}`);

		let local_version = versions.find(v => v.mc_version === version.id);

		if (local_version?.specifier === "Verified" || local_version?.specifier === "Unverified") {
			return {
				name: version.id,
				release_type,
				specifier: local_version.specifier,
				specifier_version: local_version.specifier_version
			};
		}

		if (local_version?.specifier === "Unknown" || local_version?.specifier === "None") {
			return {
				name: version.id,
				release_type,
				specifier: local_version.specifier
			};
		}

		return {
			name: version.id,
			release_type,
			specifier: "Unknown" as SpecifiersWithoutVersion satisfies SpecifiersWithoutVersion
		};

	});

	return combined;
}

function get_src_lines(src: string) {
	return fs.readFileSync(path.resolve(dest), "utf8").split("\n");
}

function get_pre_post_generated(lines: Array<string>) {
	const generated_start_marker = "// begin autogenerated section";
	const generated_end_marker = "// end autogenerated section";

	const start_i = lines.findIndex(l => l.endsWith(generated_start_marker));
	const end_i = lines.findIndex(l => l.endsWith(generated_end_marker));

	return [
		lines.slice(0, start_i + 1),
		lines.slice(end_i)
	] as const;
}

function gen_mapped_versions(versions: ReturnType<typeof combine_versions>) {
	return versions.map(v => {
		let name = [
			"V_",
			v.name.replaceAll(/[^a-zA-Z0-9_]/g, "_")
		].join("");

		return [name, v] as const;
	});
}

function gen_list_of_pack_versions(mapped_versions: ReturnType<typeof gen_mapped_versions>) {
	return mapped_versions.map(([name, v]) => {
		return [name, v, `${gen_one_pack_format(v)}`] as const;
	});
}

function gen_consts(list: ReturnType<typeof gen_list_of_pack_versions>) {
	return list.map(([name, v, pack_version]) => {
		return `const ${name}: PackVersion = ${pack_version};`
	});
}

function gen_pack_formats_const(mapped_versions: ReturnType<typeof gen_list_of_pack_versions>) {
	// let lines = mapped_versions.map(([n]) => n);
	let lines = mapped_versions.map(v => v[2]);

	let content = lines.join(",\n\t");
	let constant = `pub const PACK_FORMATS: &[PackVersion] = &[\n\t${content}\n];`;
	return constant;
}

function gen_one_pack_format(v: ReturnType<typeof combine_versions>[number]) {
	let name = `name: ${JSON.stringify(v.name)}`;
	let release_type = `release_type: MCVersionType::${v.release_type}`;
	let specifier = v.specifier_version
		? `${v.specifier}(${v.specifier_version})`
		: `${v.specifier}`;
	let format = `format: PackFormat::${specifier}`;

	return `PackVersion { ${name}, ${release_type}, ${format} }`;
}

function gen_enum_and_etc_from_enum(mapped_versions: ReturnType<typeof gen_list_of_pack_versions>) {

	let enumstr = `pub enum PackFormats {\n\t${
		mapped_versions.map(([n]) => n).join(",\n\t")
	}\n}`;

	const fn_get_pack_format = [
		"\tpub fn get_pack_format(&self) -> PackVersion {",
		"\t\tmatch *self {",
		...mapped_versions.map(([name]) =>
			`\t\t\tPackFormats::${name} => { ${name} }`
		),
		"\t\t}",
		"\t}"
	];

	// let impl_enum = `impl PackFormats {\n${fn_get_pack_format}\n}`;
	let impl_enum = [
		"impl PackFormats {",
		...fn_get_pack_format,
		"}"
	];

	return [
		enumstr,
		...impl_enum
	];
}
