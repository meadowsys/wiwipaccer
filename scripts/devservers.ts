import * as cp from "child_process";
import * as path from "path";
import * as rl from "readline";

(async () => {
	const docs = cp.spawn("./node_modules/.bin/nuxt", ["dev", "--no-clear"], {
		cwd: path.resolve("./docs")
	});
	const docs_stdout = proxy_lines(docs.stdout, "[docs]", console.log);
	const docs_stderr = proxy_lines(docs.stderr, "[docs]", console.error);

	await new Promise(res => setTimeout(res, 5000));

	const main = cp.spawn("./node_modules/.bin/nuxt", ["dev", "--no-clear"], {});
	const main_stdout = proxy_lines(main.stdout, "[main]", console.log);
	const main_stderr = proxy_lines(main.stderr, "[main]", console.error);

	process.once("exit", () => {
		docs.kill("SIGINT");
		main.kill("SIGINT");
	});
})();

function proxy_lines(stream: NodeJS.ReadableStream, prefix: string, fn: (s: string) => void) {
	let lines = rl.createInterface(stream);
	lines.on("line", line => {
		fn(`${prefix} ${line}`);
	});
	return lines
}
