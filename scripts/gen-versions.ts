import * as fs from "fs";
import * as path from "path";
import { z } from "zod";
import * as crypto from "crypto";
import * as cp from "child_process";

(async () => {
	const src = "lib/src/meta/pack_formats_src";
	const dest = "lib/src/meta/pack_formats.rs";
	const mojang_manifest_url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

	const comment_marker = "//";

	const specified_formats: Map<string, PackMeta> = new Map;
	let has_duplicate_mc_versions = false;

	run_if_CI(() => console.log("::group::specified pack formats"));
	fs.readFileSync(path.resolve(src), "utf8")
		.trim()
		.split("\n")
		.map((line, i) => ({ line: line.trim(), line_number: i + 1 }))
		.filter(({ line }) => {
			return !line.startsWith(comment_marker) && line.length !== 0
		})
		.map(({ line, line_number }) => {
			const comment_start = line.indexOf(comment_marker);
			return {
				line: comment_start < 0 ? line : line.substring(0, comment_start),
				line_number
			}
		})
		.flatMap<PackMeta>(({ line, line_number }) => {
			const last_space = line.lastIndexOf(" ");

			const specifier_lowercased = line.substring(last_space).trim().toLowerCase();

			const rest = line.substring(0, last_space);

			if (specifier_lowercased === "verified") return with_specifier_version("Verified", line_number, rest) ?? [];
			if (specifier_lowercased === "unverified") return with_specifier_version("Unverified", line_number, rest) ?? [];
			if (specifier_lowercased === "maybe") return with_specifier_version("Maybe", line_number, rest) ?? [];

			if (specifier_lowercased === "unknown") return without_specifier_version("Unknown", line_number, rest);
			if (specifier_lowercased === "none") return without_specifier_version("None", line_number, rest);

			run_if_CI(
				() => console.log(`::warning file=${src},line=${line_number}::invalid line`),
				() => console.log(`L${line_number} is an invalid line`)
			);

			return [];
		})
		.forEach(pack_meta => {
			if (specified_formats.has(pack_meta.mc_version)) {
				run_if_CI(
					() => console.log(`::warning file=${src},line=${pack_meta.line_number}::duplicate line`),
					() => console.log(`L${pack_meta.line_number} is a duplicate line`)
				);
				has_duplicate_mc_versions = true;
			}

			specified_formats.set(pack_meta.mc_version, pack_meta);

			console.log(`L${
				pack_meta.line_number
			}, mc ${
				pack_meta.mc_version
			}, ${
				pack_meta.specifier
			}${
				has_specifier(pack_meta) ? `(${pack_meta.specifier})` : ""
			}`);
		});
	run_if_CI(() => console.log("::endgroup::"));

	if (has_duplicate_mc_versions) {
		run_if_CI(
			() => console.log(`::error file=${src}::one or more duplicates were found, refusing to continue`),
			() => console.log("one or more duplicates were found, refusing to continue")
		);
		process.exit(1);
	}

	let mojang_versions_validator = z.object({
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

	let versions_from_mojang = await fetch(mojang_manifest_url)
		.then(r => r.json())
		.then(r => mojang_versions_validator.parse(r))

	run_if_CI(() => console.log("::group::versions from mojang"));
	versions_from_mojang.versions
		.sort((a, b) => b.releaseTime.getTime() - a.releaseTime.getTime())
		.forEach(version => run_if_CI(
			() => console.log(`mc ${version.id}`),
			() => console.log(`version from mojang: ${version.id}`)
		));
	run_if_CI(() => console.log("::endgroup::"));

	let first_specified_pack_meta: PackMetaWithSpecifier | undefined;
	versions_from_mojang.versions.find(version => {
		let _pack_meta = specified_formats.get(version.id);
		if (_pack_meta && has_specifier(_pack_meta)) {
			first_specified_pack_meta = _pack_meta;
			return true;
		} else return false;
	});

	let new_versions_specifier = first_specified_pack_meta
		? `Maybe(${first_specified_pack_meta.specifier_version})`
		: "Unknown";
	let still_in_new_versions = true;

	let specified_formats_for_usage_tracking: typeof specified_formats = new Map;
	specified_formats.forEach((v, k) => specified_formats_for_usage_tracking.set(k, v));

	let array_of_pack_versions = versions_from_mojang.versions
		.map(version => {
			if (still_in_new_versions && specified_formats.has(version.id)) {
				still_in_new_versions = false;
			}

			let release_type =
				version.type === "release" ? "Release"
				: version.type === "snapshot" ? "Snapshot"
				: version.type === "old_beta" ? "OldBeta"
				: version.type === "old_alpha" ? "OldAlpha"
				: release_type_die(version.type);

			if (typeof release_type === "function") {
				// Hrm, weird
				release_type();
			}

			let pack_meta = specified_formats.get(version.id);

			let format = pack_meta
				? `${pack_meta.specifier}${has_specifier(pack_meta) ? `(${pack_meta.specifier_version})` : ""}`
				: (still_in_new_versions ? new_versions_specifier : "Unknown");

			let version_entry = `PackVersion { name: "${version.id}", release_type: MCVersionType::${release_type}, format: PackFormat::${format} }`;

			specified_formats_for_usage_tracking.delete(version.id);

			return version_entry;
		});

	let final_str = array_of_pack_versions.join(",\n\t");
	final_str = `pub const PACK_FORMATS: &[PackVersion] = &[\n\t${final_str}\n];`;
	final_str = `use super::pack_version_specifier::{ MCVersionType, PackFormat, PackVersion };\n\n${final_str}`;
	final_str = `// autogenerated, from \`./pack_formats_src\`, by \`scripts/gen-versions.ts\`.\n// do not edit by hand, they **will** be overwritten\n// instead, make changes in \`./pack_formats_src\`, then run \`pnpm run gen-versions\` to update this file\n\n${final_str}\n`;

	const original_hash = create_hash_of_dest();
	fs.writeFileSync(path.resolve(dest), final_str);
	const new_hash = create_hash_of_dest();

	console.log();

	let array_of_unused_pack_meta = [...specified_formats_for_usage_tracking.values()];
	array_of_unused_pack_meta.forEach(version => {
		let warn_msg = `version \`${version.mc_version}\` not returned by mojang (invalid version?)`;
		run_if_CI(
			() => console.log(`::warning file=${src},line=${version.line_number}::${warn_msg}`),
			() => console.log(`warning: ${warn_msg}`)
		);
	});

	console.log("\ndone!\n");

	let s_format_array = [...specified_formats.values()];

	let s_verified = 0;
	let s_unverified = 0;
	let s_maybe = 0;
	let s_unknown = 0;
	let s_none = 0;
	let s_pack_meta_new_available = false;

	let s_release = 0;
	let s_snapshot = 0;
	let s_old_beta = 0;
	let s_old_alpha = 0;
	let s_mojang_new_available = false;

	s_format_array.forEach(v => {
		if (v.specifier === "Verified") s_verified += 1;
		else if (v.specifier === "Unverified") s_unverified += 1;
		else if (v.specifier === "Maybe") s_maybe += 1;
		else if (v.specifier === "Unknown") s_unknown += 1;
		else if (v.specifier === "None") s_none += 1;
		else s_pack_meta_new_available = true;
	});

	versions_from_mojang.versions.forEach(v => {
		if (v.type === "release") s_release += 1;
		else if (v.type === "snapshot") s_snapshot += 1;
		else if (v.type === "old_beta") s_old_beta += 1;
		else if (v.type === "old_alpha") s_old_alpha += 1;
		else s_mojang_new_available = true;
	});

	run_if_CI(
		() => console.log("::group::stats"),
		() => console.log("some stats:\n")
	);
	console.log(`   total versions with format specified${array_of_unused_pack_meta.length > 0 ? " (including duplicates)" : ""}: ${s_format_array.length}`);
	console.log(`   verified: ${s_verified}`);
	console.log(`   unverified: ${s_unverified}`);
	console.log(`   maybe: ${s_maybe}`);
	console.log(`   unknown: ${s_unknown}`);
	console.log(`   none: ${s_none}`);
	array_of_unused_pack_meta.length > 0 && console.log(`   unused: ${array_of_unused_pack_meta.length}`);
	console.log();
	console.log(`   total releases received from mojang: ${versions_from_mojang.versions.length}`);
	console.log(`   latest release: ${versions_from_mojang.latest.release}`);
	console.log(`   latest snapshot: ${versions_from_mojang.latest.snapshot}`);
	console.log(`   releases: ${s_release}`);
	console.log(`   snapshots: ${s_snapshot}`);
	console.log(`   old betas: ${s_old_beta}`);
	console.log(`   old alphas: ${s_old_alpha}`);
	run_if_CI(() => console.log("::endgroup::"));

	run_if_CI(() => {
		if (original_hash !== new_hash) {
			console.log("::group::committing new changes");

			cp.spawnSync("git", ["config", "--global", "user.name", process.env.GENVERSIONS_AUTOCOMMITTER_NAME!]);
			cp.spawnSync("git", ["config", "--global", "user.email", process.env.GENVERSIONS_AUTOCOMMITTER_EMAIL!]);

			let credential_file = ".git/credentials";
			fs.writeFileSync(credential_file, `https://meadowsys:${process.env.GITHUB_TOKEN}@github.com\n`);
			cp.spawnSync("git", ["config", "--global", "credential.helper", `store --file=${credential_file}`]);
			cp.spawnSync("git", ["config", "--unset-all", "http.https://github.com/.extraheader"]); // https://stackoverflow.com/a/69979203

			cp.spawnSync("git", ["add", "-A"]);
			cp.spawnSync("git", ["commit", "-m", "(automated) updating mc releases"]);
			cp.spawnSync("git", ["push"])

			console.log("::endgroup::");
		} else {
			console.log("no new changes, not committing");
		}
	});

	type PackMetaWithSpecifier = {
		mc_version: string;
		line_number: number;
		specifier: "Verified" | "Unverified" | "Maybe";
		specifier_version: number;
	};

	type PackMetaWithoutSpecifier = {
		mc_version: string;
		line_number: number;
		specifier: "Unknown" | "None";
	};

	type PackMeta = PackMetaWithSpecifier | PackMetaWithoutSpecifier;

	function specifier_has_version(specifier: PackMeta["specifier"]): specifier is PackMetaWithSpecifier["specifier"] {
		const _specifier = specifier as PackMetaWithSpecifier["specifier"];

		if (_specifier === "Verified") return true;
		if (_specifier === "Unverified") return true;
		if (_specifier === "Maybe") return true;

		// this will error is something is added to PackMetaWithSpecifier
		// cause then _specifier will not have type `never` and not assign properly
		const _typecheck: never = _specifier;

		return false;
	}

	function has_specifier(pack_meta: PackMeta): pack_meta is PackMetaWithSpecifier {
		return specifier_has_version(pack_meta.specifier);
	}

	function with_specifier_version<Specifier extends PackMetaWithSpecifier["specifier"]>(
		specifier: Specifier,
		line_number: number,
		rest: string
	): PackMetaWithSpecifier | undefined {
		const last_space = rest.lastIndexOf(" ");

		const mc_version = rest.substring(0, last_space).trim();

		const specifier_version_str = rest.substring(last_space).trim();
		const specifier_version = Number.parseInt(specifier_version_str, 10);

		if (Number.isNaN(specifier_version)) return undefined;

		return { mc_version, line_number, specifier, specifier_version };
	}

	function without_specifier_version<Specifier extends PackMetaWithoutSpecifier["specifier"]>(
		specifier: Specifier,
		line_number: number,
		rest: string
	): PackMetaWithoutSpecifier {
		const mc_version = rest.trim();

		return { mc_version, line_number, specifier };
	}

	function run_if_CI(f: () => void, not_ci?: () => void) {
		if (!!process.env.CI) f();
		else not_ci?.();
	}

	function create_hash_of_dest(): string {
		return crypto.createHash("sha512")
			.update(fs.readFileSync(path.resolve(dest)))
			.digest()
			.toString("hex");
	}

	function release_type_die(release_type: string) {
		return (): never => {
			const err = `release type ${release_type} is invalid, but passed zod validation somehow`;
			run_if_CI(
				() => console.log(`::error::${err}`),
				() => console.log(err)
			);
			process.exit(1);
		}
	}
})();
