#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use tauri::{ TitleBarStyle, WindowBuilder, WindowUrl };

const WELCOME_WINDOW_NAME: &str = "welcome_window";

fn main() {
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.worker_threads(2)
		.build()
		.unwrap();
	tauri::async_runtime::set(rt.handle().clone());

	tauri::Builder::default()
		.setup(|app| {
			WindowBuilder::new(app, WELCOME_WINDOW_NAME, WindowUrl::App("welcome".into()))
				.accept_first_mouse(false)
				.enable_clipboard_access()
				.title_bar_style(TitleBarStyle::Overlay)
				.inner_size(700., 400.)
				.title("")
				.build()?;

			Ok(())
		})
		.invoke_handler(tauri::generate_handler![])
		.build(tauri::generate_context!())
		.expect("error while running application")
		.run(|_apphandle, _event| {
			// E
		});
}

// let system_tray_menu = {
// 	let e = tauri::CustomMenuItem::new("e", "e");
// 	tauri::SystemTrayMenu::new()
// 		.add_item(e)
// };

// tauri::SystemTray::new()
// 	.with_menu(system_tray_menu)
// 	.build(apphandle)
// 	.unwrap();
