import { Octokit } from "@octokit/rest";
import {
	readdirSync as read_dir,
	readFileSync as read_file,
	writeFileSync as write_file
} from "fs";
import {
	resolve as resolve_path
} from "path";

(async () => {
	let auth = get_env("github_pat");
	let target_commitish = get_env("target_commitish");

	const owner = "meadowsys";
	const repo = "wiwipaccer";

	let tauri_manifest_path = resolve_path("./src-tauri/tauri.conf.json");
	let tauri_manifest_obj = JSON.parse(read_file(tauri_manifest_path, "utf-8"));
	let version = tauri_manifest_obj.package.version as string;

	let gh = new Octokit({
		auth,
		userAgent: "meadowsys/wiwipaccer release script"
	});

	let releases = await gh.rest.repos.listReleases({
		owner,
		repo
	});

	let [latest, tag_name] = get_new_tag_name();

	// substring is to get rid of the prefix "v"
	tauri_manifest_obj.package.version = tag_name.substring(1);
	write_file(tauri_manifest_path, JSON.stringify(tauri_manifest_obj, null, "\t"));

	let release = await gh.rest.repos.createRelease({
		owner,
		repo,
		tag_name,
		draft: true,
		target_commitish,
		name: `wiwipaccer ${tag_name}`,
		// body: `See the changelog for changelog: https://github.com/Meadowsys/wiwipaccer/blob/${tag_name}/CHANGELOG.md`
		body: `See [CHANGELOG.md](https://github.com/Meadowsys/wiwipaccer/blob/wiwi/CHANGELOG.md#${tag_name.replace(".", "")}) for changelog, and [compare with previous tag](https://github.com/Meadowsys/wiwipaccer/compare/${latest}...${tag_name}) to see commits.`
	});

	let artifacts = read_dir(resolve_path("./artifacts"));
	for (const artifact of artifacts) {
		await gh.request({
			url: release.data.upload_url,
			method: "POST",
			headers: {
				"content-type": "application/octet-stream",
			},
			data: read_file(resolve_path("./artifacts/" + artifact)),
			name: artifact,
		});
	}

	await gh.rest.repos.updateRelease({
		owner: "meadowsys",
		repo: "wiwipaccer",
		release_id: release.data.id,
		make_latest: "true",
		draft: false
	});

	function get_env(env: string): string {
		let value = process.env[env];
		if (!value) throw new Error(`env ${env} does not exist!!!!`);
		return value;
	}

	function get_new_tag_name() {
		let template = (n: number) => `v${version}-rolling.${n}`;
		let latest = releases.data.find(r => r.tag_name.includes("rolling.") && !r.draft)?.tag_name;

		if (!latest) return template(1);

		let i = latest.lastIndexOf(".");
		let n = Number.parseInt(latest.substring(i + 1), 10);
		return [latest, template(n + 1)] as const;
	}
})();
