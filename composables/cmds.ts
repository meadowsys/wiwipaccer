import { invoke } from "@tauri-apps/api";

export async function invoke_add_recent_project(path: string) {
	await invoke("add_recent_project", { path }) as void;
}

export async function invoke_get_recent_projects() {
	// TODO figure out the db and its proper return value
	return await invoke("get_recent_projects") as void;
}

export async function invoke_open_project() {
	await invoke("open_project");
}

export async function invoke_decode_hex_string(string: string) {
	return await invoke("decode_hex_string", { string }) as string;
}
