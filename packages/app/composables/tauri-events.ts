import { listen } from "@tauri-apps/api/event";
import type { EventCallback } from "@tauri-apps/api/event";

export async function listen_refresh_locales(cb: EventCallback<Array<string>>) {
	return await listen("refresh-locales", cb);
}
