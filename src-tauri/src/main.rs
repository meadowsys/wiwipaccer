// Windows™
#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

// TODO remove this when code is in a more complete state
#![allow(unused)]

fn main() {
	tauri::Builder::default()
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
