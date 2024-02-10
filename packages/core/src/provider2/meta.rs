use crate::gen::Generator;
use crate::util::ron;
use super::error::*;
use ::serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
#[serde(tag = "meta_version")]
enum ProviderMeta {
	#[serde(rename = "1")]
	Version1 {
		#[serde(flatten)]
		gen: Generator
	}
}

pub(super) struct ProviderUnversioned {
	pub(super) gen: Generator
}

pub(super) fn deserialise_version(s: &str) -> Result<ProviderUnversioned> {
	use ProviderMeta::*;
	Ok(match ron::from_str(s)? {
		Version1 { gen } => {
			ProviderUnversioned { gen }
		}
	})
}
