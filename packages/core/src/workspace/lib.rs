// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::error::*;
use ::async_trait::async_trait;
use ::hashbrown::HashMap;
use ::serde::{ Deserialize, Serialize };
use ::wiwipaccer_pack::{ self as pack, nom as pack_nom };

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
pub enum WorkspaceConfig {
	#[serde(rename = "1")]
	Version1 {
		name: meta_nom::Name,
		packs: meta_nom::Packs
	}
}

#[derive(Debug)]
pub struct Workspace {
	name: nom::Name,
	packs: nom::Packs,
	pack_ids: nom::PackIDs
}

::nominal::nominal_mod! {
	pub mod meta_nom {
		nominal!(pub Name, inner: String);
		nominal!(pub Pack, inner: String);
		nominal!(pub Packs, inner: Vec<Pack>);
	}

	pub mod nom {
		nominal!(pub Name, inner: String);
		nominal!(pub Packs, inner: HashMap<pack_nom::PackID, pack::Pack>);
		nominal!(pub PackIDs, inner: Vec<pack_nom::PackID>);
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
			WorkspaceConfig::Version1 { name, packs } => {
				let mut new = Self::new(nom::Name::new(name.into_inner()));

				for dir in packs.into_inner() {
					let dir = pack_nom::Dir::new(dir.into_inner());
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

		let packs = pack_ids.ref_inner()
			.iter()
			.map(|id| packs.ref_inner().get(id).expect("invalid state"))
			.map(|pack| pack.pack_id().clone().into_inner())
			.map(meta_nom::Pack::new)
			.collect();
		let packs = meta_nom::Packs::new(packs);

		WorkspaceConfig::Version1 { name, packs }
	}

	pub async fn add_pack(&mut self, dir: pack_nom::Dir) -> Result<()> {
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
		pack_id: &pack_nom::PackID,
		version_req: &pack_nom::VersionReq
	) -> pack::error::Result<pack::DependencyResult<Self::Dependency>> {
		let pack = match self.packs.ref_inner().get(pack_id) {
			Some(s) => { s }
			None => { return Ok(pack::DependencyResult::NotFound) }
		};

		if let Some(version) = pack.optional_version().ref_inner() {
			if !version_req.ref_inner().matches(version) {
				return Ok(pack::DependencyResult::VersionNotSatisfied(version.clone()))
			}
		}

		let dependency = Dependency { pack };
		Ok(pack::DependencyResult::Found(dependency))
	}
}

#[async_trait]
impl<'h> pack::Dependency for Dependency<'h> {}
