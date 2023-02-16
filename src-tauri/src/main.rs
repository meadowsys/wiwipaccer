#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![greet])
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
