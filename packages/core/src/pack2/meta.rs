use crate::util::ron;
use super::error::*;
use super::nm;
use ::serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
enum PackMeta {
	#[serde(rename = "1")]
	Version1 {
		name: nm::Name,
		description: nm::Description,
		id: nm::ID,
		version: nm::Version,
		dependencies: nm::Dependencies
	}
}

pub(super) struct PackUnversioned {
	pub(super) name: nm::Name,
	pub(super) description: nm::Description,
	pub(super) id: nm::ID,
	pub(super) version: nm::Version,
	pub(super) dependencies: nm::Dependencies
}

pub(super) fn deserialise_pack(s: &str) -> Result<PackUnversioned> {
	use PackMeta::*;
	Ok(match ron::from_str(s)? {
		Version1 { name, description, id, version, dependencies } => {
			PackUnversioned { name, description, id, version, dependencies }
		}
	})
}
