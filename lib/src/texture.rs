// use camino::{ Utf8Path, Utf8PathBuf };
// use crate::error::{ self, Error, Result };
// use crate::ron;
// use crate::util;
// use serde::{ Deserialize, Serialize };
// use tokio::fs;
// use tokio::io::AsyncReadExt;

// pub const TEXTURES_DIR: &str = "textures";
// pub const TEXTURE_META_FILENAME: &str = "texture.wiwimeta";

// #[derive(Debug, Deserialize)]
// #[serde(tag = "meta_version")]
// enum MetaFile {
// 	#[serde(rename = "1")]
// 	Version1 {
// 		name: String,
// 		description: Option<String>,
// 		default: Option<String>
// 	}
// }

// #[derive(Debug)]
// pub struct Texture {
// 	name: String,
// 	root_dir: String,
// 	/// also the shortpath (to get absolute path, join
// 	/// [`root_dir`](Texture::root_dir) and [`texture_id`](Texture::texture_id))
// 	texture_id: String,
// 	description: Option<String>,
// 	default: Option<String>
// }

// #[derive(Debug)]
// pub struct NewTextureOptions {
// 	pub root_dir: String,
// 	pub texture_id: String
// }

// impl Texture {
// 	pub async fn new(options: NewTextureOptions) -> Result<Option<Self>> {
// 		let NewTextureOptions { root_dir, texture_id } = options;

// 		let mut texture_dir = Utf8PathBuf::from(root_dir.clone());
// 		texture_dir.push(TEXTURES_DIR);
// 		texture_dir.push(&texture_id);

// 		if !util::check_is_dir_silent_fail(texture_dir.as_str()).await {
// 			// silently ignore if its not a dir
// 			// maybe in the future we can log this as debug information, that it saw this
// 			// but skipped it
// 			return Ok(None)
// 		}

// 		let mut manifest_path = texture_dir;
// 		manifest_path.push(TEXTURE_META_FILENAME);

// 		let manifest = util::check_for_and_read_manifest(manifest_path.as_str())
// 			.await?;

// 		let manifest = match manifest {
// 			Some(manifest) => { manifest }
// 			None => { return Ok(None) }
// 		};

// 		let (name, description, default) = match manifest {
// 			MetaFile::Version1 { name, description, default } => {
// 				(name, description, default)
// 			}
// 		};

// 		Ok(Some(Texture { name, root_dir, texture_id, description, default }))
// 	}
// }
