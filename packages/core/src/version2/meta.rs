use crate::util::ron;
use super::error::*;
use ::serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
#[serde(tag = "meta_version")]
pub(super) enum VersionMeta {
	#[serde(rename = "1")]
	Version1 {
		versions: Vec<PackVersionSpecMeta>
	}
}

#[derive(Deserialize, Serialize)]
pub(super) enum PackVersionSpecMeta {
	PackVersion(u8),
	MCVersion(String),
	MCVersionRange(String, String)
}

#[inline]
pub(super) fn deserialise_version(s: &str) -> Result<VersionMeta> {
	Ok(ron::from_str(s)?)
}
