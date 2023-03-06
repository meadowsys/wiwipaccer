import { Octokit } from "@octokit/rest";
import {
	readFileSync as read_file,
	readdirSync as read_dir
} from "fs";
import {
	resolve as resolve_path
} from "path";

(async () => {
	let auth = get_env("github-pat");
	let target_commitish = get_env("target-commitish");

	const owner = "meadowsys";
	const repo = "wiwipaccer";

	let version = JSON.parse(read_file(resolve_path("./src-tauri/tauri.conf.json"), "utf-8"))
		.package
		.version as string;

	let gh = new Octokit({
		auth,
		userAgent: "meadowsys/wiwipaccer release script"
	});

	let releases = await gh.rest.repos.listReleases({
		owner,
		repo
	});

	let tag_name = get_new_tag_name();

	let release = await gh.rest.repos.createRelease({
		owner,
		repo,
		tag_name,
		draft: true,
		generate_release_notes: true,
		target_commitish,
		name: `wiwipaccer ${tag_name}`
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

	function get_new_tag_name(): string {
		let template = (n: number) => `v${version}-rolling.${n}`;
		let latest = releases.data.find(r => r.tag_name.includes("rolling."))?.tag_name;

		if (!latest) return template(1);

		let i = latest.lastIndexOf(".");
		let n = Number.parseInt(latest.substring(i + 1), 10);
		return template(n);
	}
})();
