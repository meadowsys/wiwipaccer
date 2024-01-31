use crate::data::AppDB;
use crate::data::workspaces::SavedWorkspace;
use crate::error::*;
use ::hashbrown::HashMap;
use ::std::ffi::OsStr;
use ::std::sync::Arc;
use ::tauri::State;
use ::tokio::sync::RwLock;
use ::tokio::sync::Mutex;
use ::wiwipaccer_core::mc_versions::MCVersionRef;
use ::wiwipaccer_core::workspace2::{ self, WorkspaceRuntime };
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
	workspace: WorkspaceRuntime
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

		// necessary to prevent deadlock
		drop(read);

		let saved = SavedWorkspace::read(&self.db, name).await?;

		let workspace = if let Some(saved) = saved {
			let config = saved.into_inner();
			let workspace = WorkspaceRuntime::from_config_str(&config).await?;
			Arc::new(Mutex::new(WorkspaceWrapper { workspace }))
		} else {
			let workspace = WorkspaceRuntime::new(workspace2::nr::Name::new(name.into()));
			SavedWorkspace::new(workspace.to_config_str()?).write(name, &self.db).await?;
			Arc::new(Mutex::new(WorkspaceWrapper { workspace }))
		};

		let cloned = Arc::clone(&workspace);
		let mut write = self.workspaces.write().await;

		let result = write.insert(name.into(), cloned);
		debug_assert!(result.is_none());
		drop(write);

		Ok(workspace)
	}
}

impl WorkspaceWrapper {
	#[inline]
	pub fn frontend_data(&self, mc_version: MCVersionRef) -> workspace2::FrontendData {
		workspace2::FrontendData::new(&self.workspace, mc_version)
	}

	#[inline]
	pub async fn add_pack_osstr(&mut self, dir: &OsStr) -> Result<()> {
		// TODO: save to db, somehow
		self.workspace.add_pack_with_dir_osstr(dir).await.map_err(Into::into)
	}
}
