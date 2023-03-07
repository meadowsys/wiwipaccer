import {
	readFileSync as read_file,
	writeFileSync as write_file
} from "fs";
import {
	resolve as resolve_path
} from "path";
import { owner, repo, get_new_tag_name, get_gh } from "./lib";

export const tag_name = (async () => {
	let tauri_manifest_path = resolve_path("./src-tauri/tauri.conf.json");
	let tauri_manifest_obj = JSON.parse(read_file(tauri_manifest_path, "utf-8"));
	let version = tauri_manifest_obj.package.version as string;

	let gh = get_gh("update-tauri-version-to-next script");

	let [latest, tag_name] = await get_new_tag_name(gh, owner, repo, version);

	// substring is to get rid of the prefix "v"
	tauri_manifest_obj.package.version = tag_name.substring(1);
	write_file(tauri_manifest_path, JSON.stringify(tauri_manifest_obj, null, "\t"));

	return { latest, tag_name };
})();
