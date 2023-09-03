import * as fs from "fs";
import * as path from "path";
import { z } from "zod";
import * as crypto from "crypto";
import * as cp from "child_process";
const dest = "lib/src/internal/pack_formats.rs";
import { run_if_ci } from "./lib";

(async () => {
	const original_hash = create_hash_of_dest();
	await import("./gen-versions-v2");
	const new_hash = create_hash_of_dest();

	console.log();

	console.log("\ndone!\n");

	run_if_ci(() => {
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

	function create_hash_of_dest(): string {
		return crypto.createHash("sha512")
			.update(fs.readFileSync(path.resolve(dest)))
			.digest()
			.toString("hex");
	}
})();
