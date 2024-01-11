import { invoke } from "@tauri-apps/api/core";

export async function invoke_open_dialog() {
	await invoke("open_dialog");
}
