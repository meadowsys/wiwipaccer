use crate::data::DataTauriState;
use crate::data::locale::LocaleSetting;
use crate::error::*;
use ::tauri::{ AppHandle, Manager as _, Runtime };

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
