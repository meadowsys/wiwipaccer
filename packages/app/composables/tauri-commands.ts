import { invoke } from "@tauri-apps/api/core";

export async function invoke_open_workspace_dialog() {
	await invoke("open_workspace_dialog");
}

export async function invoke_read_locale_setting() {
	return await invoke("read_locale_setting") as Array<string>;
}

export async function invoke_write_locale_setting(locales: Array<string>) {
	await invoke("write_locale_setting", { locales });
}
