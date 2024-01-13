import { invoke } from "@tauri-apps/api/core";

export async function invoke_open_workspace_dialog() {
	await invoke("open_workspace_dialog");
}
