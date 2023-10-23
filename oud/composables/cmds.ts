import { invoke } from "@tauri-apps/api";

export async function invoke_decode_hex_string(string: string) {
	return await invoke("decode_hex_string", { string }) as string;
}

export async function invoke_get_license() {
	return await invoke("get_license") as string;
}

export async function invoke_get_platform() {
	return await invoke("get_platform") as "macos" | "linux" | "windows";
}

export async function invoke_open_about() {
	await invoke("open_about");
}

export async function invoke_open_docs() {
	await invoke("open_docs");
}

export async function invoke_open_settings() {
	await invoke("open_settings");
}
