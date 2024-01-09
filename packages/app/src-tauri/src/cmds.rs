use crate::error::*;
use rfd::AsyncFileDialog;

use tauri::{ AppHandle, Runtime, WindowBuilder, WindowUrl };

#[tauri::command]
pub async fn open_dialog<R: Runtime>(handle: AppHandle<R>) -> ResultStringErr<()> {
	string_error(|| async {
		let folder = AsyncFileDialog::new()
			.pick_folder()
			.await;
		let path = folder.unwrap();
		let path = if let Some(path) = path.path().to_str() {
			path
		} else {
			return Err(Error(ErrorInner::NonUtf8Path))
		};

		let label = hex::encode(path);

		let _window = WindowBuilder::new(&handle, label, WindowUrl::App("opened-workspace".into()))
			.accept_first_mouse(false)
			.enable_clipboard_access()
			.title("")
			.build()
			.expect("failed to create window");

		Ok(())
	}).await
}
