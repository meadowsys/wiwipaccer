use crate::util::ron;
use super::error::*;
use super::nm;
use ::serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
#[serde(tag = "meta_version")]
pub(super) enum OptionMeta {
	#[serde(rename = "1")]
	Version1 {
		name: nm::Name,
		description: nm::Description
	}
}

#[inline]
pub(super) fn deserialise_option(s: &str) -> Result<OptionMeta> {
	Ok(ron::from_str(s)?)
}
