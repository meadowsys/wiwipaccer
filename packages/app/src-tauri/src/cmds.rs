use crate::error::*;
use crate::window::{ self, OpenOpts };
use ::rfd::AsyncFileDialog;
use ::tauri::{ AppHandle, Runtime };

#[tauri::command]
pub async fn open_dialog<R: Runtime>(handle: AppHandle<R>) -> ResultStringErr<()> {
	string_error(|| async {
		let folder = AsyncFileDialog::new()
			.pick_folder()
			.await;
		let path = folder.unwrap();
		let path = if let Some(path) = path.path().to_str() {
			path.into()
		} else {
			return Err(Error(ErrorInner::NonUtf8Path))
		};

		let _window = window::open(&handle, OpenOpts::Workspace { path });

		Ok(())
	}).await
}
