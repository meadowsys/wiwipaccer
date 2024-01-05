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
		let mut texture_dir = Utf8PathBuf::from(root_dir.ref_inner());
		texture_dir.push(consts::TEXTURES_DIR);
		texture_dir.push(texture_id.ref_inner().as_str());

		let texture_dir_meta = fs::metadata(n::global::Path::new(texture_dir.as_str().into()))
			.await
			.map_err(Into::into)
			.map_err(Error)?;

		// silently ignore if its not a dir
		// maybe in the future we can log this as debug information,
		// that it saw this but skipped it
		if !texture_dir_meta.is_dir() { return Ok(None) }

		let mut manifest_path = texture_dir;
		manifest_path.push(consts::TEXTURE_META_FILENAME);

		let meta_file = fs::read_to_string(n::global::FilePath::new(manifest_path.as_str().into()))
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
