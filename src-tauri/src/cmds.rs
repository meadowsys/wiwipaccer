use camino::Utf8PathBuf;
use crate::*;
use lib::meta::pack_version_specifier::PackVersion;
use lib::runtime_meta::workspace::Workspace;
use tauri::{ AppHandle, Manager, Runtime, Window, WindowBuilder, WindowUrl };
use tauri_plugin_dialog::{ DialogExt as _, FileDialogBuilder };

#[cfg(target_os = "macos")]
use {
	tauri::TitleBarStyle,
	window_vibrancy::{ apply_vibrancy, NSVisualEffectMaterial }
};

#[tauri::command]
pub async fn add_recent_project<R: Runtime>(app: AppHandle<R>, path: String) {
	db::add_recent_project(&path).await;
	emit_refresh_recents_to_welcome(&app);
}

#[tauri::command]
pub async fn clear_recent_projects<R: Runtime>(app: AppHandle<R>) {
	db::clear_recent_projects().await;
	emit_refresh_recents_to_welcome(&app);
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
pub async fn get_platform() -> &'static str {
	#[cfg(target_os = "macos")]
	let platform = "macos";

	#[cfg(target_os = "linux")]
	let platform = "linux";

	#[cfg(target_os = "windows")]
	let platform = "windows";

	platform
}

#[tauri::command]
pub async fn get_project_meta(path: String) -> lib::error::Result<Workspace> {
	Workspace::single_dir(&path).await
}

#[derive(serde::Serialize)]
pub struct ProjectBasicMeta {
	names: Vec<String>,
	versions: Vec<PackVersion>
}

#[tauri::command]
pub async fn get_project_basic_meta(path: String) -> lib::error::Result<ProjectBasicMeta> {
	let workspace = Workspace::single_dir(&path).await?;

	Ok(ProjectBasicMeta {
		names: workspace.get_names().iter().map(|s| (*s).into()).collect(),
		versions: workspace.get_supported_mc_versions()?
	})
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
pub async fn get_theme() -> theme::Theme {
	let setting = db::get_theme_setting().await;
	let theme = theme::get_system_theme().await;
	theme::get_theme_from_setting(setting, theme)
}

#[tauri::command]
pub async fn open_about<R: Runtime>(app: AppHandle<R>) {
	window_manager::open_about_window(&app);
}

#[tauri::command]
pub async fn open_docs<R: Runtime>(app: AppHandle<R>) {
	window_manager::open_docs_window(&app);
}

#[tauri::command]
pub async fn open_project<R: Runtime>(app: AppHandle<R>, path: Option<String>) {
	if let Some(path) = path {
		window_manager::open_project_window(&app, &path);
	} else {
		app.dialog().file().pick_folder(move |folder| {
			if let Some(path) = folder {
				let path: Utf8PathBuf = path.try_into()
					.expect("only utf-8 paths are supported, could not open project");

				window_manager::open_project_window(&app, path.as_str());
			}
		});
	}
}

#[tauri::command]
pub async fn open_settings<R: Runtime>(app: AppHandle<R>) {
	window_manager::open_settings_window(&app);
}

#[tauri::command]
pub async fn remove_recent_project<R: Runtime>(app: AppHandle<R>, path: String) {
	db::remove_recent_project(&path).await;
	emit_refresh_recents_to_welcome(&app);
}

// internal helper functions and stuff below here

fn emit_refresh_recents_to_welcome<R: Runtime>(app: &AppHandle<R>) {
	if let Some(window) = window_manager::get_welcome_window(app) {
		window.emit("refresh-recents", "nothin")
			.expect("failed to emit refresh-recents to welcome window");
	}
}
