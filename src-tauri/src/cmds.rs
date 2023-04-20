use camino::Utf8PathBuf;
use crate::db;
use lib::meta::pack_version_specifier::PackVersion;
use lib::runtime_meta::datasource::Datasource;
use tauri::api::dialog::FileDialogBuilder;
// use tauri::api::ipc::
use tauri::{ AppHandle, Manager, Runtime, Window, WindowBuilder, WindowUrl };

#[cfg(target_os = "macos")]
use {
	tauri::TitleBarStyle,
	window_vibrancy::{ apply_vibrancy, NSVisualEffectMaterial }
};

#[tauri::command]
pub async fn add_recent_project<R: Runtime>(app: AppHandle<R>, path: String) {
	db::add_recent_project(&path).await;

	if let Some(window) = app.get_window(crate::WELCOME_WINDOW_NAME) {
		window.emit("refresh-recents", "nothin")
			.expect("failed to emit refresh-recents to welcome window");
	}
}

#[tauri::command]
pub async fn decode_hex_string(string: String) -> Result<String, String> {
	let bytevec = string.into_bytes().into_iter().collect::<Vec<_>>();
	let decoded = hex::decode(bytevec)
		.map_err(|e| e.to_string())?;
	String::from_utf8(decoded)
		.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_license() -> String {
	const LICENSE_TEXT: &str = include_str!("../../LICENSE");
	LICENSE_TEXT.into()
}

#[tauri::command]
pub async fn get_platform() -> String {
	#[cfg(target_os = "macos")]
	let platform = "macos";

	#[cfg(target_os = "linux")]
	let platform = "linux";

	#[cfg(target_os = "windows")]
	let platform = "windows";

	platform.into()
}

#[tauri::command]
pub async fn get_project_meta(path: String) -> lib::error::Result<Datasource> {
	Datasource::new(&path).await
}

#[tauri::command]
pub async fn get_project_supported_versions(path: String) -> lib::error::Result<Vec<PackVersion>> {
	Datasource::new(&path).await?.get_supported_mc_versions()
}

#[tauri::command]
pub async fn get_recent_projects() -> Vec<(String, String)> {
	// TODO this is just temporary implementation of the recent projects
	// later, store the name of the project in the db
	// probably even more later, we can check and read it from dirs, marking them as unavailable if
	// erroring, and reading the name from there

	db::get_recent_projects().await
		.into_iter()
		.map(|p| (
			Utf8PathBuf::from(&p).file_name().unwrap().to_string(),
			p
		))
		.collect()
}

#[tauri::command]
pub async fn open_about<R: Runtime>(app: AppHandle<R>) {
	lazy_static::lazy_static! {
		static ref ABOUT_WINDOW_LABEL: String = format!("about-{}", hex::encode(super::ACTUAL_APP_VERSION));
	}
	if let Some(window) = app.get_window(&ABOUT_WINDOW_LABEL) {
		window.set_focus()
			.expect("couldn't focus the window");
	} else {
		let _window = get_window_builder(&app, &ABOUT_WINDOW_LABEL, WindowUrl::App("about".into()))
			.transparent(false)
			.inner_size(550., 350.)
			.resizable(false)
			// .min_inner_size(750., 350.)
			.build()
			.unwrap();
	}
}

#[tauri::command]
pub async fn open_docs<R: Runtime>(app: AppHandle<R>) {
	const DOCS_WINDOW_LABEL: &str = "docs";
	if let Some(window) = app.get_window(DOCS_WINDOW_LABEL) {
		window.set_focus()
			.expect("couldn't focus the window");
	} else {
		let _window = get_window_builder(&app, DOCS_WINDOW_LABEL, WindowUrl::App("docs".into()))
			.transparent(false)
			.inner_size(800., 500.)
			.min_inner_size(800., 500.)
			.build()
			.unwrap();
	}
}

#[tauri::command]
pub async fn open_project<R: Runtime>(app: AppHandle<R>, path: Option<String>) {
	fn open_project_window<R: Runtime>(app: &AppHandle<R>, path: &str) {
		let label = hex::encode(path);
		let existing = app.get_window(&label);
		if let Some(window) = existing {
			window.set_focus()
				.expect("couldn't focus window");
		} else {
			// TODO maybe send a signal back to main or something if this is Err
			// so that user gets an alert that opening it failed
			let window = get_window_builder(app, &label, WindowUrl::App("project_folder".into()))
				.transparent(false)
				.inner_size(800., 500.)
				.min_inner_size(800., 500.)
				.build()
				.unwrap();
		}
	}

	if let Some(path) = path {
		open_project_window(&app, &path);
	} else {
		FileDialogBuilder::new().pick_folder(move |folder| {
			if let Some(path) = folder {
				let path: Utf8PathBuf = path.try_into()
					.expect("only utf-8 paths are supported, could not open project");

				open_project_window(&app, path.as_str());
			}
		});
	}
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
