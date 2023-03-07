import { Octokit } from "@octokit/rest";
import {
	readdirSync as read_dir,
	readFileSync as read_file,
	writeFileSync as write_file
} from "fs";
import {
	resolve as resolve_path
} from "path";
import { get_env } from "./lib";
import { tag_name as _tag_name } from "./update-tauri-version-to-next";

(async () => {
	let auth = get_env("github_pat").unwrap();
	let target_commitish = get_env("target_commitish").unwrap();

	const owner = "meadowsys";
	const repo = "wiwipaccer";

	let gh = new Octokit({
		auth,
		userAgent: "meadowsys/wiwipaccer release script"
	});

	let { latest, tag_name } = await _tag_name;

	let release = await gh.rest.repos.createRelease({
		owner,
		repo,
		tag_name,
		draft: true,
		target_commitish,
		name: `wiwipaccer ${tag_name}`,
		// body: `See the changelog for changelog: https://github.com/Meadowsys/wiwipaccer/blob/${tag_name}/CHANGELOG.md`
		body: `See [CHANGELOG.md](https://github.com/Meadowsys/wiwipaccer/blob/wiwi/CHANGELOG.md#${tag_name.replace(/\./g, "")}) for changelog, and [compare with previous tag](https://github.com/Meadowsys/wiwipaccer/compare/${latest}...${tag_name}) to see commits.`
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
})();
