// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::option;
use crate::nom as n;
use crate::util::{ fs, path_builder, ron };
use error::*;
use ::camino::Utf8PathBuf;
use ::hashbrown::HashMap;
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
	pub(crate) async fn new(
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
			Err(e) if e.is_wrong_type_error() => { return Ok(None) }
			Err(e) => { return Err(e.into()) }
		};

		let manifest_path = match p.texture_manifest().await {
			Ok(p) => { p }
			Err(e) if e.is_wrong_type_error() => { return Ok(None) }
			Err(e) => { return Err(e.into()) }
		};

		let meta_file = fs::read_to_string(n::global::FilePath::new(manifest_path.into_inner()))
			.await?;

		let meta_file = ron::from_str(&meta_file)?;

		let (name, description, default) = match meta_file {
			MetaFile::Version1 { name, description, default } => {
				(name, description, default)
			}
		};

		let options = {
			let mut options_nom = n::texture::Options::default();

			let mut read_dir = fs::read_dir(n::global::DirPath::new(texture_dir.clone().into_inner()))
				.await?;
			let options = options_nom.mut_inner();

			while let Some(file) = read_dir.next().await? {
				let option_id = file.file_name();
				let option_id = option_id.to_str()
					.ok_or_else(|| Error::NonUtf8Path)?;
				let option_id = n::option::ID::new(option_id.into());

				let option = option::TextureOption::new(root_dir.clone(), texture_id.clone(), option_id.clone())
					.await?;

				if let Some(option) = option {
					options.insert(option_id, option);
				}
			}

			options_nom
		};

		Ok(Some(Self { name, description, texture_id, root_dir, default, options }))
	}
}

pub struct FrontendData<'h> {
	name: &'h n::texture::Name,
	description: &'h n::texture::Description,
	texture_id: &'h n::texture::ID,
	default: &'h n::texture::Default,
	options: HashMap<&'h n::option::ID, option::FrontendData<'h>>
}

impl<'h> FrontendData<'h> {
	pub fn new(texture: &'h Texture) -> Self {
		let Texture { name, description, texture_id, default, options, .. } = texture;

		let options = options
			.ref_inner()
			.iter()
			.map(|(k, v)| (k, option::FrontendData::new(v)))
			.collect();

		Self { name, description, texture_id, default, options }
	}
}
