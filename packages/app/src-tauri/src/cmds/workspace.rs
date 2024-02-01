use crate::data::DataTauriState;
use crate::data::workspaces::SavedWorkspace;
use crate::error::*;
use crate::window::{ self, WindowType };
use crate::core::WorkspacesTauriState;
use ::tauri::{ AppHandle, Runtime, Window };
use ::wiwipaccer_core::mc_versions::MCVersion;

/// checks existing workspaces in db
#[tauri::command]
pub async fn list_existing_workspaces(
	db: DataTauriState<'_>
) -> ResultStringErr<Vec<String>> {
	string_error(SavedWorkspace::list(&db)).await
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
	let _window = window::open(&handle, WindowType::Workspace { name }).await;
}

// TODO fetch state out of window automatically here
#[tauri::command]
pub async fn get_frontend_data_for(
	name: String,
	mc_version: String,
	workspaces: WorkspacesTauriState<'_>
) -> ResultStringErr<::serde_json::Value> {
	string_error(async {
		let mc_version = MCVersion::get(&mc_version)?;
		let workspace = workspaces.create_or_open_or_get(&name).await?;

		let lock = workspace.lock().await;
		let frontend_data = lock.frontend_data(mc_version);
		let frontend_data = ::serde_json::to_value(frontend_data)?;
		drop(lock);

		Ok(frontend_data)
	}).await
}

// TODO fetch state out of window automatically here
#[tauri::command]
pub async fn prompt_add_pack/* <R: Runtime> */(
	name: String,
	// window: Window<R>,
	workspaces: WorkspacesTauriState<'_>,
) -> WrappedTSResult<()> {
	wrapped_ts_result(async {
		let workspace = workspaces.create_or_open_or_get(&name).await?;
		let mut lock = workspace.lock().await;

		let file_handle = ::rfd::AsyncFileDialog::new()
			.pick_folder()
			.await;
		let file_handle = match file_handle {
			Some(f) => { f }
			None => { return Ok(()) }
		};

		let dir = file_handle.path();
		lock.add_pack_osstr(dir.as_os_str()).await?;
		drop(lock);

		Ok(())
	}).await
}
