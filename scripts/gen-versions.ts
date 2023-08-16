import {
	readFileSync as read_file,
	writeFileSync as write_file,
	appendFileSync as append_file,
	openSync as open
} from "fs";
import { resolve as resolve_path } from "path";
import { z } from "zod";
import { createHash } from "crypto";
import { spawnSync as spawn } from "child_process";

const src = "lib/src/meta/pack_formats_src";
const dest = "lib/src/meta/pack_formats.rs";

let is_ci = !!process.env.CI;

(async () => {
	type PackMeta = {
		lineno: number;
		mc_version: string;
		specifier_type: "Verified" | "Unverified" | "Maybe" | "Unknown" | "None";
		specifier: number;
	};

	let original_hash = createHash("sha512")
		.update(read_file(resolve_path(dest)))
		.digest()
		.toString("hex");

	let formats_meta: Record<string, PackMeta> = {};

	let duplicate_versions_specified = false;

	is_ci && console.log("::group::parsed pack formats from src file");
	read_file(resolve_path(src), "utf8")
		.trim()
		.split("\n")
		.map((line, lineno) => ({ line, lineno: lineno + 1 }))
		.filter(l => !l.line.startsWith("//") && l.line.trim() !== "")
		.flatMap<PackMeta>(({ line, lineno }) => {
			let last_space = line.lastIndexOf(" ");
			let last_value = line.substring(last_space + 1);

			if (
				last_value === "verified"
				|| last_value === "unverified"
				|| last_value === "maybe"
			) {
				let remaining = line.substring(0, last_space).trim();
				let second_last_space = remaining.lastIndexOf(" ");
				let second_last_value = remaining.substring(second_last_space + 1);
				let specifier = Number.parseInt(second_last_value, 10);
				let mc_version = remaining.substring(0, second_last_space).trim();
				let specifier_type = last_value.substring(0, 1).toUpperCase() + last_value.substring(1);
				if (!Number.isNaN(specifier)) return {
					lineno,
					mc_version,
					specifier,
					specifier_type: specifier_type as PackMeta["specifier_type"]
				};
			}

			if (last_value === "unknown" || last_value === "none") {
				let specifier = NaN; // not needed, just set to anything
				let mc_version = line.substring(0, last_space).trim();
				let specifier_type = last_value.substring(0, 1).toUpperCase() + last_value.substring(1);

				return {
					lineno,
					mc_version,
					specifier,
					specifier_type: specifier_type as PackMeta["specifier_type"]
				};
			}

			console.log(`invalid line, line ${lineno}: ${line}`);

			return [];
		})
		.forEach(packmeta => {
			if (formats_meta[packmeta.mc_version]) {
				console.log(`there is a duplicate for minecraft ${packmeta.mc_version} on line ${packmeta.lineno}!`);
				duplicate_versions_specified = true;
			}
			formats_meta[packmeta.mc_version] = packmeta;
			// console.log(JSON.stringify(packmeta));

			let should_print_specifier = packmeta.specifier_type === "Verified" || packmeta.specifier_type === "Unverified" || packmeta.specifier_type === "Maybe";
			console.log(`L${packmeta.lineno}, mc ${packmeta.mc_version}, ${packmeta.specifier_type} ${should_print_specifier ? packmeta.specifier : ""}`);
		});
	is_ci && console.log("::endgroup::");

	if (duplicate_versions_specified) {
		console.log("one or more duplicates were found, refusing to continue");
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

	let versions_from_mojang = await fetch("https://launchermeta.mojang.com/mc/game/version_manifest.json")
		.then(res => res.json())
		.then(res => mojang_versions_validator.parse(res));

	is_ci && console.log("::group::Versions from mojang");
	// sort by time released, from newest to oldest
	versions_from_mojang.versions
		.sort((a, b) => b.releaseTime.getTime() - a.releaseTime.getTime())
		.forEach(v => console.log(`version from mojang: ${v.id}`));
	is_ci && console.log("::endgroup::");

	let format_meta: PackMeta | undefined;
	// find first version with _something_ of a version, maybe, unverified, or not
	let first_specified_version = versions_from_mojang.versions.find(version => {
		format_meta = formats_meta[version.id];
		return format_meta && (
			format_meta.specifier_type === "Verified"
			|| format_meta.specifier_type === "Unverified"
			|| format_meta.specifier_type === "Maybe"
		);
	});

	// The purpose of this mechanism is to assign a better value than `Unknown`
	// for newly released versions of MC that we haven't specified yet.
	// Otherwise, `Unknown` would be used instead, which isn't great
	let new_versions_specifier = first_specified_version
		? `Maybe(${format_meta!.specifier})`
		: "Unknown"
	let still_in_new_versions = true;

	// entries here are deleted as they are used by the entry generation
	// so we can warn about unused entries at the end
	let formats_meta_for_use_tracking = { ...formats_meta };

	let final = versions_from_mojang.versions
		.map(version => {
			if (still_in_new_versions && formats_meta[version.id]) still_in_new_versions = false;
			let release_type =
				version.type === "release"  ? "Release" as const
				: version.type === "snapshot" ? "Snapshot" as const
				: version.type === "old_beta" ? "OldBeta" as const
				: "OldAlpha" as const;
			let format = formats_meta[version.id]
				// todo refactor this, i got impatient wanting to make it work lol ~vapor
				// hmmmmm, thinking about it i probably could have wrote this whole thing inside a
				// template literal (of course except imports and the async IIFE)
				? `${formats_meta[version.id].specifier_type}${formats_meta[version.id].specifier_type === "Unknown" || formats_meta[version.id].specifier_type === "None" ? "" : `(${formats_meta[version.id].specifier})`}`
				: (still_in_new_versions ? new_versions_specifier : "Unknown");
			let version_entry = `PackVersion { name: "${version.id}", release_type: MCVersionType::${release_type}, format: PackFormat::${format} }`;

			delete formats_meta_for_use_tracking[version.id];

			return version_entry;
		});

	let final_final = final.join(",\n\t");
	let final_final_2 = `pub const PACK_FORMATS: &[PackVersion] = &[\n\t${final_final}\n];`;
	let final_final_3 = `use super::pack_version_specifier::{ MCVersionType, PackFormat, PackVersion };\n\n${final_final_2}`;
	let final_final_4 = `// Autogenerated, from \`./pack_formats_src\`, by \`scripts/gen-versions.ts\`.\n// Do not edit by hand, they _will_ be overwritten; instead make changes in \`pack_formats_src\`.\n\n${final_final_3}\n`;

	write_file(resolve_path(dest), final_final_4);

	console.log();
	let unused_versions = Object.values(formats_meta_for_use_tracking);
	unused_versions.forEach(version => {
		let warn_msg = `version \`${version.mc_version}\` not returned by mojang (invalid version?)`;
		if (is_ci) {
			console.log(`::warning file=${src},line=${version.lineno}::${warn_msg}`);
		} else {
			console.log(warn_msg);
		}
	});

	console.log("\ndone!\n");

	let formats_meta_array = Object.entries(formats_meta);
	let s_verified = formats_meta_array.filter(v => v[1].specifier_type === "Verified");
	let s_unverified = formats_meta_array.filter(v => v[1].specifier_type === "Unverified");
	let s_maybe = formats_meta_array.filter(v => v[1].specifier_type === "Maybe");
	let s_unknown = formats_meta_array.filter(v => v[1].specifier_type === "Unknown");
	let s_none = formats_meta_array.filter(v => v[1].specifier_type === "None");
	let s_release = versions_from_mojang.versions.filter(v => v.type === "release");
	let s_snapshot = versions_from_mojang.versions.filter(v => v.type === "snapshot");
	let s_old_beta = versions_from_mojang.versions.filter(v => v.type === "old_beta");
	let s_old_alpha = versions_from_mojang.versions.filter(v => v.type === "old_alpha");

	is_ci ? console.log("::group::stats") : console.log("some stats:");
	console.log(`   latest: ${versions_from_mojang.latest.release}, snapshot: ${versions_from_mojang.latest.snapshot}`);
	console.log(`   ${versions_from_mojang.versions.length} releases received from Mojang`);
	console.log(`      ${s_release.length} releases`);
	console.log(`      ${s_snapshot.length} snapshots`);
	console.log(`      ${s_old_beta.length} old betas`);
	console.log(`      ${s_old_alpha.length} old alphas`);
	console.log(`   ${formats_meta_array.length} versions specified with meta in source file${unused_versions.length > 0 ? " (including unused)" : ""}`);
	console.log(`      ${s_verified.length} verified`);
	console.log(`      ${s_unverified.length} unverified`);
	console.log(`      ${s_maybe.length} maybe`);
	console.log(`      ${s_unknown.length} unknown`);
	console.log(`      ${s_none.length} none`);
	unused_versions.length > 0 && console.log(`      ${unused_versions.length} unused`);
	is_ci && console.log("::endgroup::");

	const github_output = process.env.GITHUB_OUTPUT;
	let new_hash = createHash("sha512")
		.update(read_file(resolve_path(dest)))
		.digest()
		.toString("hex");

	if (!is_ci) {
		// running outside of CI, noop
	} else if (original_hash !== new_hash) {
		console.log("::group::committing new changes");

		spawn("git", ["config", "--global", "user.name", process.env.GENVERSIONS_AUTOCOMMITTER_NAME!]);
		spawn("git", ["config", "--global", "user.email", process.env.GENVERSIONS_AUTOCOMMITTER_EMAIL!]);
		let credential_file = ".git/credentials";
		write_file(credential_file, `https://meadowsys:${process.env.GITHUB_TOKEN}@github.com\n`);
		spawn("git", ["config", "--global", "credential.helper", `store --file=${credential_file}`]);
		spawn("git", ["config", "--unset-all", "http.https://github.com/.extraheader"]); // https://stackoverflow.com/a/69979203

		spawn("git", ["add", "-A"]);
		spawn("git", ["commit", "-m", "(automated) updating mc releases"]);
		spawn("git", ["push"]);

		console.log("::endgroup::");
	} else {
		console.log("no changes, not committing");
	}
})();
