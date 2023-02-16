//! Root manifest for a source of data, repo of data, I guess. All textures and
//! options would be inside one of these. Ex would be all the assets for L&T,
//! would be a single datasource.

use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub enum Datasource {
	V1 {
		/// name of the data source
		name: String,
		/// version of the actual data itself (not format version of the
		/// data source, that's controlled by enum variants)
		version: Option<Version>,
		/// description of datasource
		description: Option<String>
	}
}

#[derive(Deserialize, Serialize)]
pub enum Version {
	/// explicit version
	String(String),
	/// the dir is a git repo, run `git rev-parse HEAD`
	Git
}
