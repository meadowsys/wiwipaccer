use crate::util::ron;
use super::error::*;
use super::nm;
use ::serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
pub(super) enum PackMeta {
	#[serde(rename = "1")]
	Version1 {
		name: nm::Name,
		description: nm::Description,
		id: nm::ID,
		version: nm::Version,
		dependencies: nm::Dependencies
	}
}

#[inline]
pub(super) fn deserialise_pack(s: &str) -> Result<PackMeta> {
	Ok(ron::from_str(s)?)
}
