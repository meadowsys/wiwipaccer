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
		name: n::option::Name,
		description: n::option::Description
	}
}

pub struct TextureOption {
	name: n::option::Name,
	description: n::option::Description,
	texture_id: n::texture::ID,
	option_id: n::option::ID,
	root_dir: n::global::RootDirPath
}

impl TextureOption {
	pub async fn new(
		root_dir: n::global::RootDirPath,
		texture_id: n::texture::ID,
		option_id: n::option::ID
	) -> Result<Option<Self>> {
		let p = path_builder(&root_dir)
			.with_texture(&texture_id)
			.with_option(&option_id);

		// silently ignore if its not a dir
		// maybe in the future we can log this as debug information,
		// that it saw this but skipped it
		let option_dir = match p.option_dir().await {
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

		let (name, description) = match meta_file {
			MetaFile::Version1 { name, description } => {
				(name, description)
			}
		};

		Ok(Some(Self { name, description, texture_id, option_id, root_dir }))
	}
}
