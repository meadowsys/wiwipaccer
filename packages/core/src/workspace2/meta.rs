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
