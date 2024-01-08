#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![])
		.build(tauri::generate_context!())
		.expect("error running app")
		.run(|_app, _event| {
			// stuff happens
		})
}
