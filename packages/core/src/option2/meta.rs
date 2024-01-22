use crate::util::ron;
use super::error::*;
use super::nm;
use ::serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
#[serde(tag = "meta_version")]
enum OptionMeta {
	#[serde(rename = "1")]
	Version1 {
		name: nm::Name,
		description: nm::Description
	}
}

pub(super) struct OptionUnversioned {
	pub(super) name: nm::Name,
	pub(super) description: nm::Description
}

pub(super) fn deserialise_option(s: &str) -> Result<OptionUnversioned> {
	use OptionMeta::*;
	Ok(match ron::from_str(s)? {
		Version1 { name, description } => {
			OptionUnversioned { name, description }
		}
	})
}
