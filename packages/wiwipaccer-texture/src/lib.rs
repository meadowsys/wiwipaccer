// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::error::*;
use ::camino::Utf8PathBuf;
use ::serde::{ Deserialize, Serialize };
use ::wiwipaccer_util::{ fs, ron };

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: meta_nom::Name,
		description: meta_nom::DescriptionOptional,
		// default
	}
}

#[derive(Debug)]
pub struct Texture {
	name: nom::Name,
	description: nom::DescriptionOptional,
	/// also the shortpath. To get path to texture dir, you can do
	/// `format!("{root_dir}/{TEXTURES_DIR}/{texture_id}")`
	texture_id: nom::TextureID,
	root_dir: nom::RootDir
	// default
}

pub const TEXTURES_DIR: &str = "textures";
pub const TEXTURE_META_FILENAME: &str = "texture.wiwimeta";

::nominal::nominal_mod! {
	pub mod meta_nom {
		nominal!(pub Name, inner: String);
		nominal!(pub DescriptionOptional, inner: Option<String>);
	}

	pub mod nom {
		nominal!(pub Name, inner: String);
		nominal!(pub DescriptionOptional, inner: Option<String>);
		nominal!(pub TextureID, inner: String);
		nominal!(pub RootDir, inner: String);
	}
}

impl Texture {
	pub async fn new(
		root_dir: nom::RootDir,
		texture_id: nom::TextureID
	) -> Result<Option<Self>> {
		let mut texture_dir = Utf8PathBuf::from(root_dir.ref_inner());
		texture_dir.push(TEXTURES_DIR);
		texture_dir.push(texture_id.ref_inner().as_str());

		let texture_dir_meta = fs::metadata(fs::nom::Path::new(texture_dir.as_str().into()))
			.await
			.map_err(Into::into)
			.map_err(Error)?;

		// silently ignore if its not a dir
		// maybe in the future we can log this as debug information,
		// that it saw this but skipped it
		if !texture_dir_meta.is_dir() { return Ok(None) }

		let mut manifest_path = texture_dir;
		manifest_path.push(TEXTURE_META_FILENAME);

		let meta_file = fs::read_to_string(fs::nom::Path::new(manifest_path.as_str().into()))
			.await
			.map_err(Into::into)
			.map_err(Error)?;

		let meta_file = ron::from_str(&meta_file)
			.map_err(Into::into)
			.map_err(Error)?;

		let (name, description) = match meta_file {
			MetaFile::Version1 { name, description } => {
				let name = nom::Name::new(name.into_inner());
				let description = nom::DescriptionOptional::new(description.into_inner());

				(name, description)
			}
		};

		Ok(Some(Self { name, description, texture_id, root_dir }))
	}
}
