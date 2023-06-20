use camino::Utf8PathBuf;
use crate::db;
use crate::theme;
use crate::window_builder::*;
use lib::meta::pack_version_specifier::PackVersion;
use lib::runtime_meta::workspace::Workspace;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{ AppHandle, Manager, Runtime, Window, WindowBuilder, WindowUrl };

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
pub struct ProjectSupportedVersions {
	names: Vec<String>,
	versions: Vec<PackVersion>
}

#[tauri::command]
pub async fn get_project_supported_versions(path: String) -> lib::error::Result<ProjectSupportedVersions> {
	let workspace = Workspace::single_dir(&path).await?;

	Ok(ProjectSupportedVersions {
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
	lazy_static::lazy_static! {
		static ref ABOUT_WINDOW_LABEL: String = format!("about-{}", hex::encode(super::ACTUAL_APP_VERSION));
	}
	if let Some(window) = app.get_window(&ABOUT_WINDOW_LABEL) {
		window.set_focus()
			.expect("couldn't focus the window");
	} else {
		let builder = get_window_builder(&app, &ABOUT_WINDOW_LABEL, WindowUrl::App("about".into()))
			// .transparent(false)
			.inner_size(550., 350.)
			.resizable(false);
			// .min_inner_size(750., 350.)
		let _window = build_and_etc(app.clone(), builder);
	}
}

#[tauri::command]
pub async fn open_docs<R: Runtime>(app: AppHandle<R>) {
	const DOCS_WINDOW_LABEL: &str = "docs";
	if let Some(window) = app.get_window(DOCS_WINDOW_LABEL) {
		window.set_focus()
			.expect("couldn't focus the window");
	} else {
		let builder = get_window_builder(&app, DOCS_WINDOW_LABEL, WindowUrl::App("docs".into()))
			// .transparent(false)
			.inner_size(800., 500.)
			.min_inner_size(800., 500.);
		let _window = build_and_etc(app.clone(), builder);
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
			let builder = get_window_builder(app, &label, WindowUrl::App("project-folder".into()))
				// .transparent(false)
				.inner_size(800., 500.)
				.min_inner_size(800., 500.);
			let _window = build_and_etc(app.clone(), builder);
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

#[tauri::command]
pub async fn remove_recent_project<R: Runtime>(app: AppHandle<R>, path: String) {
	db::remove_recent_project(&path).await;
	emit_refresh_recents_to_welcome(&app);
}

// internal helper functions and stuff below here

fn emit_refresh_recents_to_welcome<R: Runtime>(app: &AppHandle<R>) {
	if let Some(window) = app.get_window(crate::WELCOME_WINDOW_NAME) {
		window.emit("refresh-recents", "nothin")
			.expect("failed to emit refresh-recents to welcome window");
	}
}
