use crate::db;

#[tauri::command]
pub async fn get_recent_projects() {
	db::get_recent_projects().await
}

#[tauri::command]
pub async fn add_recent_project(project_path: String) {
	db::add_recent_project(&project_path).await
}
