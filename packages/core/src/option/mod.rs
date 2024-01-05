// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::util::{ consts, fs, ron };
use error::*;
use ::camino::Utf8PathBuf;
use ::serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: meta_nom::Name,
		description: meta_nom::DescriptionOptional
	}
}

pub struct TextureOption {
	name: nom::Name,
	description: nom::DescriptionOptional,
	texture_id: nom::TextureID,
	option_id: nom::OptionID,
	root_dir: nom::RootDir
}

::nominal::nominal_mod! {
	pub mod meta_nom {
		nominal!(pub Name, inner: String);
		nominal!(pub DescriptionOptional, inner: Option<String>);
	}

	pub mod nom {
		nominal!(pub Name, inner: String);
		nominal!(pub DescriptionOptional, inner: Option<String>);
		nominal!(pub TextureID, inner: String);
		nominal!(pub OptionID, inner: String);
		nominal!(pub RootDir, inner: String);
	}
}

impl TextureOption {
	pub async fn new(
		root_dir: nom::RootDir,
		texture_id: nom::TextureID,
		option_id: nom::OptionID
	) -> Result<Option<Self>> {
		let mut option_dir = Utf8PathBuf::from(root_dir.ref_inner());
		option_dir.push(consts::TEXTURES_DIR);
		option_dir.push(texture_id.ref_inner().as_str());
		option_dir.push(option_id.ref_inner().as_str());

		let option_dir_meta = fs::metadata(fs::nom::Path::new(option_dir.as_str().into()))
			.await
			.map_err(Into::into)
			.map_err(Error)?;

		// silently ignore if its not a dir
		// maybe in the future we can log this as debug information,
		// that it saw this but skipped it
		if !option_dir_meta.is_dir() { return Ok(None) }

		let mut manifest_path = option_dir;
		manifest_path.push(consts::OPTION_META_FILENAME);

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

		Ok(Some(Self { name, description, texture_id, option_id, root_dir }))
	}
}
