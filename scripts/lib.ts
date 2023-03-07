import { Octokit } from "@octokit/rest";

export const owner = "meadowsys";
export const repo = "wiwipaccer";

export function get_env(env: string) {
	let value = process.env[env];

	function get_optional() {
		return value;
	}

	function unwrap() {
		if (!value) throw new Error(`env ${env} does not exist!!!!`);
		return value;
	}

	return { get_optional, unwrap };
}

export function get_gh(ua: string) {
	let auth = get_env("github_pat").unwrap();
	return new Octokit({
		auth,
		userAgent: `meadowsys/wiwipaccer ${ua}`
	});
}

export async function get_new_tag_name(
	gh: Octokit,
	owner: string,
	repo: string,
	version: string
) {
	let is_release = Boolean(get_env("release").get_optional());
	if (!is_release) {
		if (version.endsWith("-dev")) return ["", `v${version}`] as const; // don't append infinitely
		else return ["", `v${version}-dev`] as const;
	}

	let releases = await gh.rest.repos.listReleases({
		owner,
		repo
	});

	let template = (n: number) => `v${version}-rolling.${n}`;
	let latest = releases.data.find(r => r.tag_name.includes("rolling.") && !r.draft)?.tag_name;

	if (!latest) return template(1);

	let i = latest.lastIndexOf(".");
	let n = Number.parseInt(latest.substring(i + 1), 10);
	return [latest, template(n + 1)] as const;
}
