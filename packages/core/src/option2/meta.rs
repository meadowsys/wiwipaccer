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
