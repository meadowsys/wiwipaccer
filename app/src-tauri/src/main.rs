#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
// TODO: remove this when more complete
#![allow(unused)]
#![deny(unused_must_use)]

fn main() {
	tauri::Builder::default()
		.setup(|app| {
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![])
		.build(tauri::generate_context!())
		.expect("failed to run application")
		.run(|app, event| {});
}
