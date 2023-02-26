//! Option for a texture.

use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub enum TextureOption {
	V1 {
		/// name of option
		name: String,
		/// description of option
		description: Option<String>,
		/// whether or not this option is the default option.
		/// When a default is selected, it is included by default when
		/// the build type is "customise default" (rather than "build from scratch").
		///
		/// Only one default can be selected at a time; it is an
		/// error if there is more than one (but there can be zero).
		default: Option<bool>
	}
}
