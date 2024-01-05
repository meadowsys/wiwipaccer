// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::nom as n;
use crate::util::{ consts, fs, ron };
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
		let mut option_dir = Utf8PathBuf::from(root_dir.ref_inner());
		option_dir.push(consts::TEXTURES_DIR);
		option_dir.push(texture_id.ref_inner().as_str());
		option_dir.push(option_id.ref_inner().as_str());

		let option_dir_meta = fs::metadata(n::global::Path::new(option_dir.as_str().into()))
			.await
			.map_err(Into::into)
			.map_err(Error)?;

		// silently ignore if its not a dir
		// maybe in the future we can log this as debug information,
		// that it saw this but skipped it
		if !option_dir_meta.is_dir() { return Ok(None) }

		let mut manifest_path = option_dir;
		manifest_path.push(consts::OPTION_META_FILENAME);

		let meta_file = fs::read_to_string(n::global::FilePath::new(manifest_path.as_str().into()))
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
