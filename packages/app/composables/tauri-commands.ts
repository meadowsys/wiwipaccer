import { invoke } from "@tauri-apps/api/core";

export async function invoke_read_locale_setting() {
	return await invoke("read_locale_setting") as Array<string>;
}

export async function invoke_write_locale_setting(locales: Array<string>) {
	await invoke("write_locale_setting", { locales });
}

export async function invoke_list_existing_workspaces() {
	return await invoke("list_existing_workspaces") as Array<string>;
}

export async function invoke_check_workspace_name_is_available(name: string) {
	return await invoke("check_workspace_name_is_available", { name }) as boolean;
}

export async function invoke_create_new_workspace(name: string) {
	await invoke("create_new_workspace", { name });
}

export async function invoke_open_workspace(name: string) {
	await invoke("open_workspace", { name });
}
