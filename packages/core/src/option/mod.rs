// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::nom as n;
use crate::util::{ fs, path_builder, ron };
use crate::version;
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

#[derive(Debug)]
pub struct TextureOption {
	name: n::option::Name,
	description: n::option::Description,
	texture_id: n::texture::ID,
	option_id: n::option::ID,
	root_dir: n::global::RootDirPath,
	versions: n::option::Versions
}

impl TextureOption {
	pub(crate) async fn new(
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
			Err(e) if e.is_wrong_type_error() => { return Ok(None) }
			Err(e) => { return Err(e.into()) }
		};

		let meta_path = match p.texture_manifest().await {
			Ok(p) => { p }
			Err(e) if e.is_wrong_type_error() => { return Ok(None) }
			Err(e) => { return Err(e.into()) }
		};

		let meta_file = fs::read_to_string(n::global::FilePath::new(meta_path.into_inner()))
			.await?;

		let meta_file = ron::from_str(&meta_file)?;

		let (name, description) = match meta_file {
			MetaFile::Version1 { name, description } => {
				(name, description)
			}
		};

		let versions = {
			let mut versions_nom = n::option::Versions::default();

			let mut read_dir = fs::read_dir(n::global::DirPath::new(option_dir.clone().into_inner()))
				.await?;
			let versions = versions_nom.mut_inner();

			while let Some(file) = read_dir.next().await? {
				let version_id = file.file_name();
				let version_id = version_id.to_str()
					.ok_or_else(|| Error::NonUtf8Path)?;
				let version_id = n::version::ID::new(version_id.into());

				let version = version::Version::new(
					root_dir.clone(),
					texture_id.clone(),
					option_id.clone(),
					version_id.clone()
				).await?;

				if let Some(version) = version {
					versions.insert(version_id, version);
				}
			}

			versions_nom
		};

		Ok(Some(Self { name, description, texture_id, option_id, root_dir, versions }))
	}
}
