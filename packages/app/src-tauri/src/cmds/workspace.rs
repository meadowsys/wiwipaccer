use crate::data::DataTauriState;
use crate::data::workspaces::SavedWorkspace;
use crate::error::*;
use crate::window::{ self, OpenOpts };
use crate::workspaces::WorkspacesTauriState;
use ::tauri::{ AppHandle, Runtime };

/// checks existing workspaces in db
#[tauri::command]
pub async fn list_existing_workspaces(
	db: DataTauriState<'_>
) -> ResultStringErr<Vec<String>> {
	string_error(async {
		let saved = SavedWorkspace::list(&db).await?;
		let list = saved.into_iter()
			.map(|w| w.into_inner().name().ref_inner().into())
			.collect();
		Ok(list)
	}).await
}

/// checks existing workspaces in db
#[tauri::command]
pub async fn check_workspace_name_is_available(
	name: String,
	db: DataTauriState<'_>
) -> ResultStringErr<bool> {
	string_error(async {
		// could this be optimised?
		// this unnecessarily parses and returns all contents of workspace
		Ok(SavedWorkspace::read(&db, &name).await?.is_none())
	}).await
}

/// creates new workspace in db and loads it
#[tauri::command]
pub async fn create_new_workspace(
	name: String,
	workspaces: WorkspacesTauriState<'_>
) -> ResultStringErr<()> {
	string_error(async {
		workspaces.create_or_open_or_get(&name).await?;
		Ok(())
	}).await
}

/// opens a window with label to workspace (window will read its label)
/// and load appropriately
#[tauri::command]
pub async fn open_workspace<R: Runtime>(handle: AppHandle<R>, name: String) {
	let _window = window::open(&handle, OpenOpts::Workspace { name }).await;
}
