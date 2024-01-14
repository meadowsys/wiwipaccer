use crate::error::*;
use crate::window::{ self, OpenOpts };
use crate::data::DataTauriState;
use crate::data::locale::LocaleSetting;
use ::rfd::AsyncFileDialog;
use ::tauri::{ AppHandle, Runtime };

// dunno why i did this lol
#[inline]
pub fn command_handler<R: Runtime>()
	-> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static
{
	tauri::generate_handler![
		open_workspace_dialog,
		read_locale_setting,
		write_locale_setting
	]
}

#[tauri::command]
async fn open_workspace_dialog<R: Runtime>(handle: AppHandle<R>) -> ResultStringErr<()> {
	string_error(async {
		let path = AsyncFileDialog::new()
			.pick_folder()
			.await;

		let path = match path {
			Some(p) => { p }
			None => {
				// cancelled?
				return Ok(())
			}
		};

		let path = path.path().to_str().map(str::to_string);

		let path = match path {
			Some(p) => { p }
			None => { return Err(Error::NonUtf8Path) }
		};

		let _window = window::open(&handle, OpenOpts::Workspace { path }).await;

		Ok(())
	}).await
}

#[tauri::command]
async fn read_locale_setting(db: DataTauriState<'_>) -> ResultStringErr<Vec<String>> {
	string_error(LocaleSetting::read_or_default(&db))
		.await
		.map(LocaleSetting::into_inner)
}

#[tauri::command]
async fn write_locale_setting(locales: Vec<String>, db: DataTauriState<'_>) -> ResultStringErr<()> {
	string_error(LocaleSetting::new(locales).write(&db)).await?;
	Ok(())
}
