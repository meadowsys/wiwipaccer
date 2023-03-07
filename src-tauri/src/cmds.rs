use crate::db;
use tauri::api::dialog::FileDialogBuilder;
// use tauri::api::ipc::
use tauri::{ Manager, Runtime, WindowBuilder, WindowUrl };

#[cfg(target_os = "macos")]
use {
	tauri::TitleBarStyle,
	window_vibrancy::{ apply_vibrancy, NSVisualEffectMaterial }
};

#[tauri::command]
pub async fn add_recent_project(path: String) {
	db::add_recent_project(&path).await
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
pub async fn get_recent_projects() {
	db::get_recent_projects().await
}

#[tauri::command]
pub async fn open_project<R: Runtime>(app: tauri::AppHandle<R>) {
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
					return
				}
				let builder = WindowBuilder::new(&app, &label, WindowUrl::App("project_folder".into()))
					.accept_first_mouse(false)
					.enable_clipboard_access()
					.min_inner_size(800., 500.)
					.title("")
					.transparent(true);

				#[cfg(target_os = "macos")]
				let builder = builder.title_bar_style(TitleBarStyle::Overlay);

				// TODO send a signal back to main or something if this is Err
				// so that user gets an alert that opening it failed
				#[allow(unused)]
				let window = builder.build().unwrap();

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
		});
}

#[tauri::command]
pub async fn platform() -> String {
	#[cfg(target_os = "macos")]
	compile_error!("macos");

	#[cfg(target_os = "linux")]
	compile_error!("linux");

	#[cfg(target_os = "windows")]
	compile_error!("windows");

	"e".into()
}
