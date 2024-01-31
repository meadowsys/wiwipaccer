use crate::mc_versions::MCVersionRef;
use crate::pack2::{ self, DependencyResult };
use super::{ meta, nm, nr };
use super::error::*;
use ::async_trait::async_trait;
use ::camino::Utf8Path;
use ::hashbrown::HashMap;
use ::serde::Serialize;

pub struct WorkspaceRuntime {
	name: nr::Name,
	packs: nr::Packs,
	pack_ids: nr::PackIDs
}

impl WorkspaceRuntime {
	#[inline]
	pub fn new(name: nr::Name) -> Self {
		let packs = nr::Packs::default();
		let pack_ids = nr::PackIDs::default();
		Self { name, packs, pack_ids }
	}

	pub async fn from_config_str(config: &str) -> Result<Self> {
		let meta::WorkspaceUnversioned {
			name,
			packs
		} = meta::deserialise_workspace(config)?;

		let mut new = Self::new(nr::Name::new(name.into_inner()));

		for dir in packs.into_inner() {
			new.add_pack(dir).await?;
		}

		Ok(new)
	}

	pub async fn to_config_str(&self) -> Result<String> {
		let name = self.name.clone().transmute_nom();
		let packs = self.packs.ref_inner()
			.values()
			.map(|p| p.dir().ref_inner())
			.cloned()
			.collect();
		let packs = nm::Packs::new(packs);

		meta::serialise_workspace(meta::WorkspaceUnversioned { name, packs })
	}

	pub async fn add_pack(&mut self, dir: String) -> Result<()> {
		if !Utf8Path::new(&dir).is_absolute() {
			return Err(Error::AbsolutePathOnly(dir))
		}

		let packs = &self.packs;
		let resolver = DependencyResolver { packs };

		let pack = pack2::PackRuntime::new(&dir, resolver).await?;
		let id = pack.id().clone();

		if self.packs.ref_inner().contains_key(&id) {
			return Err(Error::DuplicateID(id.into_inner()))
		}

		self.packs.mut_inner().insert(id.clone(), pack);
		self.pack_ids.mut_inner().push(id);

		Ok(())
	}
}

pub struct DependencyResolver<'h> {
	packs: &'h nr::Packs
}

pub struct Dependency<'h> {
	pack: &'h pack2::PackRuntime
}

#[async_trait]
impl<'h> pack2::DependencyResolver for DependencyResolver<'h> {
	type Dependency = Dependency<'h>;

	async fn dependency(
		&self,
		pack_id: &pack2::nr::ID,
		version_req: &::semver::VersionReq
	) -> Result<DependencyResult<Self::Dependency>, Box<dyn std::error::Error + Send>> {
		let pack = match self.packs.ref_inner().get(pack_id) {
			Some(s) => { s }
			None => { return Ok(DependencyResult::NotFound) }
		};

		if let Some(v) = pack.version().ref_inner() {
			if !version_req.matches(v) {
				return Ok(DependencyResult::VersionNotSatisfied(v.clone()))
			}
		}

		let dependency = Dependency { pack };
		Ok(DependencyResult::Found(dependency))
	}
}

#[async_trait]
impl<'h> pack2::Dependency for Dependency<'h> {}

#[derive(Serialize)]
pub struct FrontendData<'h> {
	name: &'h nr::Name,
	packs: Vec<(&'h str, pack2::FrontendData<'h>)>
}

impl<'h> FrontendData<'h> {
	pub fn new(workspace: &'h WorkspaceRuntime, mc_version: MCVersionRef) -> Self {
		let name = &workspace.name;
		let packs = workspace.pack_ids.ref_inner()
			.iter()
			.map(|id| (id, workspace.packs.ref_inner().get(id).expect("invalid state")))
			.map(|(id, p)| (&**id.ref_inner(), pack2::FrontendData::new(p, mc_version)))
			.collect();

		Self { name, packs }
	}
}
