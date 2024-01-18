pub mod locale;
pub mod workspace;

use crate::error::*;
use crate::workspaces::WorkspacesTauriState;

#[macro_export]
macro_rules! command_handler {
	() => {{
		use $crate::cmds::*;

		::tauri::generate_handler![
			locale::read_locale_setting,
			locale::write_locale_setting,

			workspace::list_existing_workspaces,
			workspace::check_workspace_name_is_available,
			workspace::create_new_workspace,
			workspace::open_workspace,

			get_frontend_data_for
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
