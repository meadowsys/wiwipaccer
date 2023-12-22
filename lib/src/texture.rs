use camino::{ Utf8Path, Utf8PathBuf };
use crate::error::*;
use crate::ron;
use serde::{ Deserialize, Serialize };
use tokio::fs;

#[derive(Debug, Deserialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: String,
		description: Option<String>,
		default: Option<String>
	}
}

#[derive(Debug)]
pub struct Texture {
	name: String,
	texture_id: String,
	description: Option<String>,
	default: Option<String>
}

#[derive(Debug)]
pub struct NewTextureOptions {
	pub root_dir: Utf8PathBuf,
	pub dir_name: Utf8PathBuf
}

impl Texture {
	pub async fn new(options: NewTextureOptions) -> Result<Option<Self>> {
		println!("{}, {} pag", options.root_dir, options.dir_name);
		Ok(None)
	}
}
