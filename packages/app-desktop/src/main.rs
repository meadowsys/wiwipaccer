// because windows
#![cfg_attr(
	not(debug_assertions),
	windows_subsystem = "windows"
)]

use tauri::{ LogicalPosition, TitleBarStyle, WebviewWindowBuilder, WebviewUrl };

mod db;

// try to save on linking time by only doing this in release
#[cfg(not(debug_assertions))]
#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
	tauri::Builder::<tauri::Wry>::new()
		.invoke_handler(tauri::generate_handler![])
		.setup(|app| {
			WebviewWindowBuilder::new(app.handle(), "main", WebviewUrl::App("".into()))
				.accept_first_mouse(false)
				.enable_clipboard_access()
				.hidden_title(true)
				.traffic_light_position(LogicalPosition { x: 20.0, y: 25.0 })
				.min_inner_size(800.0, 500.0)
				.title("")
				.disable_drag_drop_handler()
				.title_bar_style(TitleBarStyle::Overlay)
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
