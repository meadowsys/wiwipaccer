// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::nom as n;
use crate::util::{ fs, path_builder, ron };
use error::*;
use ::camino::Utf8PathBuf;
use ::serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: n::texture::Name,
		description: n::texture::Description,
		default: n::texture::Default
	}
}

#[derive(Debug)]
pub struct Texture {
	name: n::texture::Name,
	description: n::texture::Description,
	/// also the shortpath. To get path to texture dir, you can do
	/// `format!("{root_dir}/{TEXTURES_DIR}/{texture_id}")`
	texture_id: n::texture::ID,
	root_dir: n::global::RootDirPath,
	default: n::texture::Default
}

impl Texture {
	pub async fn new(
		root_dir: n::global::RootDirPath,
		texture_id: n::texture::ID
	) -> Result<Option<Self>> {
		let p = path_builder(&root_dir)
			.with_texture(&texture_id);


		// silently ignore if its not a dir
		// maybe in the future we can log this as debug information,
		// that it saw this but skipped it
		let texture_dir = match p.texture_dir().await {
			Ok(p) => { p }
			Err(e) if e.is_not_dir_error() => { return Ok(None) }
			Err(e) => { return Err(e).map_err(Into::into).map_err(Error) }
		};

		let manifest_path = p.texture_manifest()
			.await
			.map_err(Into::into)
			.map_err(Error)?;

		let meta_file = fs::read_to_string(n::global::FilePath::new(manifest_path.into_inner()))
			.await
			.map_err(Into::into)
			.map_err(Error)?;

		let meta_file = ron::from_str(&meta_file)
			.map_err(Into::into)
			.map_err(Error)?;

		let (name, description, default) = match meta_file {
			MetaFile::Version1 { name, description, default } => {
				(name, description, default)
			}
		};

		Ok(Some(Self { name, description, texture_id, root_dir, default }))
	}
}
