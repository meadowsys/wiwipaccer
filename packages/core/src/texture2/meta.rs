use crate::util::ron;
use super::error::*;
use super::nm;
use ::serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
#[serde(tag = "meta_version")]
enum TextureMeta {
	#[serde(rename = "1")]
	Version1 {
		name: nm::Name,
		description: nm::Description,
		default: nm::Default
	}
}

pub(super) struct TextureUnversioned {
	pub(super) name: nm::Name,
	pub(super) description: nm::Description,
	pub(super) default: nm::Default
}

pub(super) fn deserialise_texture(s: &str) -> Result<TextureUnversioned> {
	use TextureMeta::*;
	Ok(match ron::from_str(s)? {
		Version1 { name, description, default } => {
			TextureUnversioned { name, description, default }
		}
	})
}
