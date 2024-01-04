pub mod error;

use crate::error::*;
use ::async_trait::async_trait;
use ::hashbrown::HashMap;
use ::serde::{ Deserialize, Serialize };
use ::wiwipaccer_pack as pack;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
pub enum WorkspaceConfig {
	#[serde(rename = "1")]
	Version1 {
		name: meta_nom::Name,
		projects: meta_nom::Projects
	}
}

pub struct Workspace {
	name: nom::Name,
	packs: nom::Packs,
	pack_ids: nom::PackIDs
}

::nominal::nominal_mod! {
	pub mod meta_nom {
		nominal!(pub Name, inner: String);
		nominal!(pub Project, inner: String);
		nominal!(pub Projects, inner: Vec<Project>);
	}

	pub mod nom {
		nominal!(pub Name, inner: String);
		nominal!(pub Packs, inner: HashMap<pack::nom::PackID, pack::Pack>);
		nominal!(pub PackIDs, inner: Vec<pack::nom::PackID>);
	}
}

impl Workspace {
	#[inline]
	pub fn new(name: nom::Name) -> Self {
		let packs = nom::Packs::new(HashMap::new());
		let pack_ids = nom::PackIDs::new(Vec::new());

		Self { name, packs, pack_ids }
	}

	pub async fn from_config(config: WorkspaceConfig) -> Result<Self> {
		let new = match config {
			WorkspaceConfig::Version1 { name, projects } => {
				let mut new = Self::new(nom::Name::new(name.into_inner()));

				for dir in projects.into_inner() {
					let dir = pack::nom::Dir::new(dir.into_inner());
					new.add_pack(dir).await?;
				}

				new
			}
		};

		Ok(new)
	}

	pub fn into_config(self) -> WorkspaceConfig {
		let Self { name, packs, pack_ids } = self;

		let name = meta_nom::Name::new(name.into_inner());

		let projects = pack_ids.ref_inner()
			.iter()
			.map(|id| packs.ref_inner().get(id).expect("invalid state"))
			.map(|source| source.pack_id().clone().into_inner())
			.map(meta_nom::Project::new)
			.collect();
		let projects = meta_nom::Projects::new(projects);

		WorkspaceConfig::Version1 { name, projects }
	}

	pub async fn add_pack(&mut self, dir: pack::nom::Dir) -> Result<()> {
		let packs = &self.packs;
		let resolver = DependencyResolver { packs };

		let pack = pack::Pack::new(dir, resolver)
			.await
			.map_err(Into::into)
			.map_err(Error)?;
		let id = pack.pack_id().clone();

		self.packs.mut_inner().insert(id.clone(), pack);
		self.pack_ids.mut_inner().push(id);

		Ok(())
	}
}

struct DependencyResolver<'h> {
	packs: &'h nom::Packs
}

struct Dependency<'h> {
	pack: &'h pack::Pack
}

#[async_trait]
impl<'h> pack::DependencyResolver for DependencyResolver<'h> {
	type Dependency = Dependency<'h>;

	async fn dependency(
		&self,
		pack_id: &pack::nom::PackID,
		version_req: &pack::nom::VersionReq
	) -> pack::error::Result<Option<Self::Dependency>> {
		let pack = match self.packs.ref_inner().get(pack_id) {
			Some(s) => { s }
			None => { return Ok(None) }
		};

		if let Some(version) = pack.optional_version().ref_inner() {
			if !version_req.ref_inner().matches(version) { return Ok(None) }
		}

		let dependency = Dependency { pack };
		Ok(Some(dependency))
	}
}

#[async_trait]
impl<'h> pack::Dependency for Dependency<'h> {}
