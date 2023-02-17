//! A supported version for an option.

use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub enum Version {
	V1 {
		/// Versions that this option supports
		versions: Vec<PackVersionSpecifier>,
		/// Type of option. Default is [`CopyPaste`][`OptionType::CopyPaste`]
		r#type: Option<OptionType>
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PackVersionSpecifier {
	PackVersion(u8)
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OptionType {
	/// Option should be copy pasted into the output, with no modifications
	CopyPaste,
	/// Allows to provide some textures, and blockstate/models will be generated
	/// for random textures.
	Random {
		block_id: String,
		mirror: Option<bool>,
		y: Option<Vec<u16>>
	}
}
