use crate::db;
use tauri::api::dialog::FileDialogBuilder;
// use tauri::api::ipc::
use tauri::{ AppHandle, command, Manager, Runtime, Window, WindowBuilder, WindowUrl };

#[cfg(target_os = "macos")]
use {
	tauri::TitleBarStyle,
	window_vibrancy::{ apply_vibrancy, NSVisualEffectMaterial }
};

#[command]
pub async fn add_recent_project(path: String) {
	db::add_recent_project(&path).await
}

#[command]
pub async fn decode_hex_string(string: String) -> Result<String, String> {
	let bytevec = string.into_bytes().into_iter().collect::<Vec<_>>();
	let decoded = hex::decode(bytevec)
		.map_err(|e| e.to_string())?;
	String::from_utf8(decoded)
		.map_err(|e| e.to_string())
}

#[command]
pub fn get_license() -> String {
	const LICENSE_TEXT: &str = include_str!("../../LICENSE");
	LICENSE_TEXT.into()
}

#[command]
pub async fn get_platform() -> String {
	#[cfg(target_os = "macos")]
	let platform = "macos";

	#[cfg(target_os = "linux")]
	let platform = "linux";

	#[cfg(target_os = "windows")]
	let platform = "windows";

	platform.into()
}

#[command]
pub async fn get_recent_projects() {
	db::get_recent_projects().await
}

#[command]
pub async fn open_about<R: Runtime>(app: AppHandle<R>) {
	const ABOUT_WINDOW_LABEL: &str = "about";
	if let Some(window) = app.get_window(ABOUT_WINDOW_LABEL) {
		window.set_focus()
			.expect("couldn't focus the window");
	} else {
		let _window = get_window_builder(&app, ABOUT_WINDOW_LABEL, WindowUrl::App("about".into()))
			.transparent(false)
			.inner_size(550., 350.)
			.resizable(false)
			// .min_inner_size(750., 350.)
			.build()
			.unwrap();
	}
}

#[command]
pub async fn open_project<R: Runtime>(app: AppHandle<R>) {
	FileDialogBuilder::new()
		.pick_folder(move |folder| {
			if let Some(path) = folder {
				let path = path.to_str()
					.expect("only utf-8 paths are supported, could not open project")
					.to_string();

				let label = hex::encode(path);
				let existing = app.get_window(&label);
				if let Some(window) = existing {
					window.set_focus()
						.expect("couldn't focus the window");
				} else {
					let window = get_window_builder(&app, &label, WindowUrl::App("project_folder".into()))
						.inner_size(800., 500.)
						.min_inner_size(800., 500.)
						.build()
						.unwrap();
					apply_relevant_window_effects(&app, window);
				}
			}
		});
}

// internal helper functions and stuff below here

fn get_window_builder<'h, R: Runtime>(app: &'h AppHandle<R>, label: &'h str, url: WindowUrl) -> WindowBuilder<'h, R> {
	let builder = WindowBuilder::new(app, label, url)
		.accept_first_mouse(false)
		.enable_clipboard_access()
		.title("")
		.transparent(true);

	#[cfg(target_os = "macos")]
	let builder = builder.title_bar_style(TitleBarStyle::Overlay);

	// TODO send a signal back to main or something if this is Err
	// so that user gets an alert that opening it failed
	builder
}

#[allow(unused)]
fn apply_relevant_window_effects<R: Runtime>(app: &AppHandle<R>, window: Window<R>) {
	#[cfg(target_os = "macos")]
	app.run_on_main_thread(move || {
		apply_vibrancy(
			&window,
			NSVisualEffectMaterial::HudWindow,
			None,
			None
		).expect("apply_vibrancy is mac only lol");
	}).unwrap();
}
