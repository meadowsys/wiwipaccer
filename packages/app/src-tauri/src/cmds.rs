use crate::error::*;
use crate::window::{ self, OpenOpts };
use crate::data::DataTauriState;
use crate::data::locale::LocaleSetting;
use crate::workspaces::WorkspacesTauriState;
use ::rfd::AsyncFileDialog;
use ::tauri::{ AppHandle, Manager as _, Runtime };

#[macro_export]
macro_rules! command_handler {
	() => {{
		use $crate::cmds::*;

		::tauri::generate_handler![
			get_frontend_data_for,
			read_locale_setting,
			write_locale_setting
		]
	}}
}
pub use command_handler;

#[tauri::command]
pub async fn get_frontend_data_for(name: String, workspaces: WorkspacesTauriState<'_>) -> ResultStringErr<::serde_json::Value> {
	string_error(async {
		let workspace = workspaces.create_or_open_or_get(&name).await?;

		let lock = workspace.lock().await;
		let frontend_data = lock.frontend_data();
		let frontend_data = ::serde_json::to_value(frontend_data)?;
		drop(lock);

		Ok(frontend_data)
	}).await
}

#[tauri::command]
pub async fn read_locale_setting(db: DataTauriState<'_>) -> ResultStringErr<Vec<String>> {
	string_error(async {
		LocaleSetting::read_or_default(&db)
			.await
			.map(LocaleSetting::into_inner)
	}).await
}

#[tauri::command]
pub async fn write_locale_setting<R: Runtime>(
	handle: AppHandle<R>,
	locales: Vec<String>,
	db: DataTauriState<'_>
) -> ResultStringErr<()> {
	string_error(async {
		let locales = LocaleSetting::new(locales);
		locales.write(&db).await?;
		handle.emit("refresh-locales", locales.into_inner())?;
		Ok(())
	}).await
}
