use crate::data::AppDB;
use crate::data::workspaces::SavedWorkspace;
use crate::error::*;
use ::hashbrown::HashMap;
use ::std::sync::Arc;
use ::tauri::State;
use ::tokio::sync::RwLock;
use ::tokio::sync::Mutex;
use ::wiwipaccer_core::nom as n;
use ::wiwipaccer_core::workspace::{ self, Workspace};

/// - [`RwLock`]: most of the time will be reading (fetch a workspace etc), write
///   access for workspace creation / opening purposes (opening a new window)
/// - [`HashMap`]: name to workspace mapping
/// - [`Arc`]: own a reference to the workspace
/// - [`Mutex`]: there will usually only be one reference to each workspace
///   (commands run by the one window where its opened in)
/// - [`WorkspaceWrapper`]: wraps a workspace access methods so it can control a few
///   things, ex. enforcing saving stuff to DB or renaming in DB when certain functions
///   are called
pub struct Workspaces {
	db: AppDB,
	workspaces: RwLock<HashMap<String, Arc<Mutex<WorkspaceWrapper>>>>
}

#[repr(transparent)]
pub struct WorkspaceWrapper {
	workspace: Workspace
}

pub type WorkspacesTauriState<'h> = State<'h, Workspaces>;

impl Workspaces {
	#[inline]
	pub fn new(db: &AppDB) -> Self {
		let db = db.clone();
		let workspaces = RwLock::new(HashMap::new());
		Self { db, workspaces }
	}

	pub async fn create_or_open_or_get(&self, name: &str) -> Result<Arc<Mutex<WorkspaceWrapper>>> {
		// most accesses it'll be opened already
		let read = self.workspaces.read().await;
		if let Some(workspace) = read.get(name) {
			let workspace = Arc::clone(workspace);
			drop(read);
			return Ok(workspace)
		}

		let saved = SavedWorkspace::read_and_load(&self.db, name).await?;

		let workspace = if let Some(saved) = saved {
			let config = saved.into_inner();
			let workspace = Workspace::from_config(&config).await?;
			Arc::new(Mutex::new(WorkspaceWrapper { workspace }))
		} else {
			let workspace = Workspace::new(n::workspace::Name::new(name.into()));
			Arc::new(Mutex::new(WorkspaceWrapper { workspace }))
		};

		let cloned = Arc::clone(&workspace);
		let mut write = self.workspaces.write().await;

		debug_assert!(!write.contains_key(name));
		write.insert(name.into(), cloned);
		drop(write);

		Ok(workspace)
	}
}

impl WorkspaceWrapper {
	#[inline]
	pub fn frontend_data(&self) -> workspace::FrontendData {
		workspace::FrontendData::new(&self.workspace)
	}
}
