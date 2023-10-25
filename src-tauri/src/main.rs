// Windows™
#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

// TODO remove this when code is in a more complete state
#![allow(unused)]

mod window;

use tauri::async_runtime;

fn main() {
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.worker_threads(2)
		.build()
		.expect("failed to build tokio runtime");
	async_runtime::set(rt.handle().clone());

	tauri::Builder::default()
		.setup(|app| {
			let app = app.handle();

			window::welcome(app);
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![])
		.build(tauri::generate_context!())
		.expect("failed to run application")
		.run(|app, event| {});
}
