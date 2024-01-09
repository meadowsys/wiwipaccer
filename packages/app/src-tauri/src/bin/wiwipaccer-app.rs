#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use wiwipaccer_app::*;
use wiwipaccer_app::window::OpenOpts;

fn main() {
	let rt = rt::get_rt();
	::tauri::async_runtime::set(rt.handle().clone());

	tauri::Builder::default()
		.setup(|app| {
			let handle = app.handle();

			let _window = window::open(handle, OpenOpts::Start);

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
