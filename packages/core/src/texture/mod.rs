// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::option;
use crate::nom as n;
use crate::util::{ fs, into_err ,path_builder, ron };
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
	default: n::texture::Default,
	options: n::texture::Options
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
			Err(e) => { return Err(into_err(e)) }
		};

		let manifest_path = p.texture_manifest()
			.await
			.map_err(into_err)?;

		let meta_file = fs::read_to_string(n::global::FilePath::new(manifest_path.into_inner()))
			.await
			.map_err(into_err)?;

		let meta_file = ron::from_str(&meta_file)
			.map_err(into_err)?;

		let (name, description, default) = match meta_file {
			MetaFile::Version1 { name, description, default } => {
				(name, description, default)
			}
		};

		let options = {
			let mut options_nom = n::texture::Options::default();

			let mut read_dir = fs::read_dir(n::global::DirPath::new(texture_dir.clone().into_inner()))
				.await
				.map_err(into_err)?;
			let options = options_nom.mut_inner();

			while let Some(file) = {
				read_dir.next()
					.await
					.map_err(into_err)?
			} {
				let option_id = file.file_name();
				let option_id = option_id.to_str()
					.ok_or_else(|| Error(ErrorInner::NonUtf8Path))?;
				let option_id = n::option::ID::new(option_id.into());

				let option = option::TextureOption::new(root_dir.clone(), texture_id.clone(), option_id.clone())
					.await
					.map_err(into_err)?;

				if let Some(option) = option {
					options.insert(option_id, option);
				}
			}

			options_nom
		};

		Ok(Some(Self { name, description, texture_id, root_dir, default, options }))
	}
}
