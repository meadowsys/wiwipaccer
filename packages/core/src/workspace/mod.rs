// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::nom as n;
use crate::util::into_err;
use error::*;
use super::pack;
use ::async_trait::async_trait;
use ::hashbrown::HashMap;
use ::serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
pub enum WorkspaceConfig {
	#[serde(rename = "1")]
	Version1 {
		name: n::workspace::Name,
		packs: n::workspace_m::Packs
	}
}

#[derive(Debug)]
pub struct Workspace {
	name: n::workspace::Name,
	packs: n::workspace::Packs,
	pack_ids: n::workspace::PackIDs
}

impl Workspace {
	#[inline]
	pub fn new(name: n::workspace::Name) -> Self {
		let packs = n::workspace::Packs::new(HashMap::new());
		let pack_ids = n::workspace::PackIDs::new(Vec::new());

		Self { name, packs, pack_ids }
	}

	pub async fn from_config(config: WorkspaceConfig) -> Result<Self> {
		let new = match config {
			WorkspaceConfig::Version1 { name, packs } => {
				let mut new = Self::new(n::workspace::Name::new(name.into_inner()));

				for dir in packs.into_inner() {
					let dir = n::global::DirPath::new(dir.into_inner());
					new.add_pack(dir).await?;
				}

				new
			}
		};

		Ok(new)
	}

	pub fn into_config(self) -> WorkspaceConfig {
		let Self { name, packs, pack_ids } = self;

		let name = n::workspace::Name::new(name.into_inner());

		let packs = pack_ids.ref_inner()
			.iter()
			.map(|id| packs.ref_inner().get(id).expect("invalid state"))
			.map(|pack| pack.pack_id().clone().into_inner())
			.map(n::global::Path::new)
			.collect();
		let packs = n::workspace_m::Packs::new(packs);

		WorkspaceConfig::Version1 { name, packs }
	}

	pub async fn add_pack(&mut self, dir: n::global::DirPath) -> Result<()> {
		let packs = &self.packs;
		let resolver = DependencyResolver { packs };

		let pack = pack::Pack::new(dir, resolver)
			.await
			.map_err(into_err)?;
		let id = pack.pack_id().clone();

		self.packs.mut_inner().insert(id.clone(), pack);
		self.pack_ids.mut_inner().push(id);

		Ok(())
	}
}

struct DependencyResolver<'h> {
	packs: &'h n::workspace::Packs
}

struct Dependency<'h> {
	pack: &'h crate::pack::Pack
}

#[async_trait]
impl<'h> pack::DependencyResolver for DependencyResolver<'h> {
	type Dependency = Dependency<'h>;

	async fn dependency(
		&self,
		pack_id: &n::pack::ID,
		version_req: &semver::VersionReq
	) -> pack::error::Result<pack::DependencyResult<Self::Dependency>> {
		let pack = match self.packs.ref_inner().get(pack_id) {
			Some(s) => { s }
			None => { return Ok(pack::DependencyResult::NotFound) }
		};

		if let Some(version) = pack.optional_version().ref_inner() {
			if !version_req.matches(version) {
				return Ok(pack::DependencyResult::VersionNotSatisfied(version.clone()))
			}
		}

		let dependency = Dependency { pack };
		Ok(pack::DependencyResult::Found(dependency))
	}
}

#[async_trait]
impl<'h> pack::Dependency for Dependency<'h> {}
