use crate::mc_versions::MCVersionRef;
use crate::pack2;
use crate::util::ron;
use super::nr;
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
}

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
