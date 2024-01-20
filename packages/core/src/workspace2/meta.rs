use crate::util::ron;
use super::error::*;
use super::nm;
use ::serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
pub(super) enum WorkspaceMeta {
	#[serde(rename = "1")]
	Version1 {
		name: nm::Name,
		packs: nm::Packs
	}
}

#[inline]
pub(super) fn deserialise_workspace(s: &str) -> Result<WorkspaceMeta> {
	Ok(ron::from_str(s)?)
}
