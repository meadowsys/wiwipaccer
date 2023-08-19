use crate::*;
use tauri::{ AppHandle, Runtime, Manager };
use tauri::menu::{ Menu, MenuBuilder, MenuEvent, MenuId, MenuItem, Submenu, SubmenuBuilder };

pub fn welcome_menu_bar<R: Runtime>(app: &AppHandle<R>) -> Menu<R> {
	let menu = MenuBuilder::new(app)
		.build()
		.expect("unable to build welcome menu");

	menu.append(&menu_wiwipaccer(app));
	// let menu = Menu::new(app)
	// 	.add_submenu(menu_wiwipaccer())
	menu
}

pub fn workspace_menu_bar() {}

pub fn menu_event_handler<R: Runtime>(app: &AppHandle<R>, event: MenuEvent) {
	let MenuId(id) = event.id();
	match id.as_ref() {
		"about" => {
			window_manager::open_about_window(&app);
		}
		"settings" => {}
		_ => {}
	}
}

/// first menu item, the one that's just the app name
fn menu_wiwipaccer<R: Runtime>(app: &AppHandle<R>) -> Submenu<R> {
	let about = MenuItem::new(app, "about", true, None);
	let settings = MenuItem::new(app, "settings", true, Some("CMDORCTRL+,"));

	let menu = SubmenuBuilder::new(app, "wiwipaccer")
		.item(&about)
		.separator()
		.item(&settings)
		.separator()
		.quit()
		.build()
		.expect("couldn't build `wiwipaccer` menu item");

	menu
}
// let system_tray_menu = {
// 	let e = tauri::CustomMenuItem::new("e", "e");
// 	tauri::SystemTrayMenu::new()
// 		.add_item(e)
// };

// tauri::SystemTray::new()
// 	.with_menu(system_tray_menu)
// 	.build(app)
// 	.unwrap();
