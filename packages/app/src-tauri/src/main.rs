#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod cmds;
mod error;

use ::tauri::{ async_runtime, WindowBuilder, WindowUrl };

fn main() {
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.worker_threads(2)
		.build()
		.expect("error building async runtime");
	async_runtime::set(rt.handle().clone());

	tauri::Builder::default()
		.setup(|app| {
			let handle = app.handle();

			let start_url = WindowUrl::App("start".into());
			let _window = WindowBuilder::new(handle, "start", start_url)
				.accept_first_mouse(false)
				.enable_clipboard_access()
				.title("")
				.build()
				.expect("failed to create window");

			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			cmds::open_dialog
		])
		.build(tauri::generate_context!())
		.expect("error running app")
		.run(|_app, _event| {
			// stuff happens
		})
}
