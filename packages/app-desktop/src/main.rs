// because windows
#![cfg_attr(
	not(debug_assertions),
	windows_subsystem = "windows"
)]

fn main() {
	tauri::Builder::<tauri::Wry>::new()
		.invoke_handler(tauri::generate_handler![])
		.build(tauri::generate_context!())
		.expect("error while running app")
		.run(|_app_handle, _event| {
			// uwu
		});
}
