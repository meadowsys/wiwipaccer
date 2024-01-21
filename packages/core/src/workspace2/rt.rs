use crate::util::ron;
use super::nr;

pub struct WorkspaceRuntime {
	name: nr::Name,
	packs: nr::Packs,
	pack_ids: nr::PackIDs
}
