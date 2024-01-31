use crate::util::ron;
use super::error::*;
use super::nm;
use ::serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
enum WorkspaceMeta {
	#[serde(rename = "1")]
	Version1 {
		name: nm::Name,
		packs: nm::Packs
	}
}

pub(super) struct WorkspaceUnversioned {
	pub(super) name: nm::Name,
	pub(super) packs: nm::Packs
}

pub(super) fn deserialise_workspace(s: &str) -> Result<WorkspaceUnversioned> {
	use WorkspaceMeta::*;
	Ok(match ron::from_str(s)? {
		Version1 { name, packs } => {
			WorkspaceUnversioned { name, packs }
		}
	})
}

#[inline]
pub(super) fn serialise_workspace(workspace: WorkspaceUnversioned) -> Result<String> {
	let WorkspaceUnversioned { name, packs } = workspace;
	let workspace = WorkspaceMeta::Version1 { name, packs };
	ron::to_string_minified(&workspace).map_err(Into::into)
}
