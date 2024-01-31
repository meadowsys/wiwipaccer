use crate::window::WindowType;
use ::tauri::{ Runtime, Window };

#[tauri::command]
pub fn get_workspace_name<R: Runtime>(window: Window<R>) -> Option<String> {
	let label = window.label();

	if let WindowType::Workspace { name } = WindowType::decode_window_label(label)? {
		Some(name)
	} else {
		None
	}
}
