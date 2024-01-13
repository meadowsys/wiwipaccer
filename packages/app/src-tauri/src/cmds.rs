use crate::error::*;
use crate::window::{ self, OpenOpts };
use ::rfd::AsyncFileDialog;
use ::tauri::{ AppHandle, Runtime };

#[tauri::command]
pub async fn open_workspace_dialog<R: Runtime>(handle: AppHandle<R>) -> ResultStringErr<()> {
	string_error(|| async {
		let path = AsyncFileDialog::new()
			.pick_folder()
			.await
			.expect("failed to pick workspace folder");
		let path = if let Some(path) = path.path().to_str() {
			path.into()
		} else {
			return Err(Error::NonUtf8Path)
		};

		let _window = window::open(&handle, OpenOpts::Workspace { path }).await;

		Ok(())
	}).await
}
