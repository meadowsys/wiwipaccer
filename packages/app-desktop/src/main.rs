// because windows
#![cfg_attr(
	not(debug_assertions),
	windows_subsystem = "windows"
)]

use tauri::{ WebviewWindowBuilder, WebviewUrl };

fn main() {
	tauri::Builder::<tauri::Wry>::new()
		.invoke_handler(tauri::generate_handler![])
		.setup(|app| {
			WebviewWindowBuilder::new(app.handle(), "main", WebviewUrl::App("".into()))
				.accept_first_mouse(!cfg!(target_os = "macos"))
				.build()
				.unwrap();
			Ok(())
		})
		.build(tauri::generate_context!())
		.expect("error while running app")
		.run(|_app_handle, _event| {
			// uwu
		});
}
