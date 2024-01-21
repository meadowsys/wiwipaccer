use crate::util::ron;
use super::nr;

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
