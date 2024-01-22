use crate::util::ron;
use super::error::*;
use ::serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
#[serde(tag = "meta_version")]
enum VersionMeta {
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

pub(super) struct VersionUnversioned {
	pub(super) versions: Vec<PackVersionSpecMeta>
}

pub(super) fn deserialise_version(s: &str) -> Result<VersionUnversioned> {
	use VersionMeta::*;
	Ok(match ron::from_str(s)? {
		Version1 { versions } => {
			VersionUnversioned { versions }
		}
	})
}
