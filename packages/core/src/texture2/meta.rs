use crate::util::ron;
use super::error::*;
use super::nm;
use ::serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
#[serde(tag = "meta_version")]
pub(super) enum TextureMeta {
	#[serde(rename = "1")]
	Version1 {
		name: nm::Name,
		description: nm::Description,
		default: nm::Default
	}
}

#[inline]
pub(super) fn deserialise_texture(s: &str) -> Result<TextureMeta> {
	Ok(ron::from_str(s)?)
}
