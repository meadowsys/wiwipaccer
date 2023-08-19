use tauri::{ AppHandle, CustomMenuItem, Menu, MenuEvent, MenuItem, Runtime, Submenu, Manager };

pub fn welcome_menu_bar() -> Menu {
	Menu::new()
		.add_submenu(menu_wiwipaccer())
}

pub fn workspace_menu_bar() {}

pub fn menu_event_handler<R: Runtime>(event: MenuEvent, app: AppHandle<R>) {
	match event.menu_item_id() {
		"about" => {
			crate::window_manager::open_about_window(app);
		}
		"settings" => {}
		_ => {}
	}
}

/// first menu item, the one that's just the app name
fn menu_wiwipaccer() -> Submenu {
	let about = CustomMenuItem::new("about", "About");
	let settings = CustomMenuItem::new("settings", "Settings");
	let menu = Menu::new()
		.add_item(about)
		.add_native_item(MenuItem::Separator)
		.add_item(settings)
		.add_native_item(MenuItem::Separator)
		.add_native_item(MenuItem::Quit);
	let menu = Submenu::new("wiwipaccer", menu);

	menu
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
