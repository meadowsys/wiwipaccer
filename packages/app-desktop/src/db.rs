use surrealdb::Surreal;
use surrealdb::engine::any::Any as EngineAny;
use tauri::State;

pub type DbState<'h> = State<'h, Surreal<EngineAny>>;
