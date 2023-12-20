//! Root manifest for a texture. Ex, stone texture.
//! A texture doesn't strictly have to be one texture, it can grouped in any way.
//! Possible to have entire resource pack in a texture, honestly (but that's not
//! what this is built for :p).

use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub enum Texture {
	V1 {
		/// name of texture
		name: String,
		/// description of texture
		description: Option<String>,
		/// species the option to be used as the default (there doesn't have to be
		/// a default). When a default is selected, it is included by default when
		/// the build type is "customise default" (rather than "build from scratch").
		default: Option<String>
	}
}
