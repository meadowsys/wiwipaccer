import {
	writeFileSync as write_file,
	copyFileSync as copy_file,
} from "fs";
import { resolve as resolve_path } from "path";
import { spawnSync as spawn, execSync as spawn_shell } from "child_process";

if (!process.env.CI) {
	console.log("deploy-site script intended for use in CI only");
	process.exit(1);
}

(async () => {
	spawn_shell("rm -rf gh-pages/*");
	spawn_shell("mv gh-pages/.git gh-pages/git");
	spawn_shell("rm -rf gh-pages/.*");
	spawn_shell("mv gh-pages/git gh-pages/.git");

	spawn_shell("cp -R site/.output/public/* gh-pages");
	spawn_shell("cp -R site/.output/public/.* gh-pages");
	copy_file(resolve_path(".gitignore"), resolve_path("gh-pages/.gitignore"));

	const commit_message = `automated deploy from commit ${
		spawn("git", ["rev-parse", "HEAD"]).stdout.toString()
	}`;

	let site_path = resolve_path("./gh-pages");
	let site_spawn = (cmd: string, args: Array<string>) => spawn(cmd, args, {
		cwd: site_path
	});

	site_spawn("git", ["config", "--global", "user.name", process.env.GENVERSIONS_AUTOCOMMITTER_NAME!]);
	site_spawn("git", ["config", "--global", "user.email", process.env.GENVERSIONS_AUTOCOMMITTER_EMAIL!]);
	let credential_file = resolve_path(site_path, "./.git/credentials");
	write_file(credential_file, `https://meadowsys:${process.env.GITHUB_TOKEN}@github.com\n`);
	site_spawn("git", ["config", "--global", "credential.helper", `store --file=${credential_file}`]);
	site_spawn("git", ["config", "--unset-all", "http.https://github.com/.extraheader"]); // https://stackoverflow.com/a/69979203

	site_spawn("git", ["add", "-A"]);
	site_spawn("git", ["commit", "-m", commit_message]);
	site_spawn("git", ["push"]);
})();
