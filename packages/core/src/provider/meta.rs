use crate::util::ron;
use super::error::*;
use ::serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
#[serde(tag = "meta_version")]
enum ProviderMeta {
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

pub(super) struct ProviderUnversioned {
	pub(super) versions: Vec<PackVersionSpecMeta>
}

pub(super) fn deserialise_version(s: &str) -> Result<ProviderUnversioned> {
	use ProviderMeta::*;
	Ok(match ron::from_str(s)? {
		Version1 { versions } => {
			ProviderUnversioned { versions }
		}
	})
}
