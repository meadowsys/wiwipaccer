use camino::{ Utf8Path, Utf8PathBuf };
use crate::error::{ self, Error, Result };
use crate::ron;
use crate::util;
use serde::{ Deserialize, Serialize };
use tokio::fs;
use tokio::io::AsyncReadExt;

pub const TEXTURES_DIR: &str = "textures";
pub const TEXTURE_META_FILENAME: &str = "texture.wiwimeta";

#[derive(Debug, Deserialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: String,
		description: Option<String>,
		default: Option<String>
	}
}

#[derive(Debug)]
pub struct Texture {
	name: String,
	root_dir: Utf8PathBuf,
	/// also the shortpath (to get absolute path, join
	/// [`root_dir`](Texture::root_dir) and [`texture_id`](Texture::texture_id))
	texture_id: String,
	description: Option<String>,
	default: Option<String>
}

#[derive(Debug)]
pub struct NewTextureOptions {
	pub root_dir: Utf8PathBuf,
	pub texture_id: Utf8PathBuf
}

impl Texture {
	pub async fn new(options: NewTextureOptions) -> Result<Option<Self>> {
		let NewTextureOptions { root_dir, texture_id } = options;

		let mut texture_dir = root_dir.clone();
		texture_dir.push(TEXTURES_DIR);
		texture_dir.push(&texture_id);

		match util::check_is_dir(&texture_dir).await {
			Ok(true) => { /* noop */ }
			Ok(false) | Err(_) => {
				// silently ignore if its not a dir
				// maybe in the future we can log this as debug information, that it saw this
				// but skipped it
				return Ok(None)
			}
		}

		let mut manifest_path = texture_dir;
		manifest_path.push(TEXTURE_META_FILENAME);

		match util::check_is_file(&manifest_path).await {
			Ok(true) => { /* noop */ }
			Ok(false) | Err(_) => {
				// the directory doesn't have a manifest so probaby isn't a texture
				// silently ignore
				// see above if statement for note on future logging
				return Ok(None)
			}
		}

		let mut manifest_reader = fs::OpenOptions::new()
			.read(true)
			.open(&manifest_path)
			.await
			.map_err(error::file_io_error(&manifest_path))?;

		let manifest_meta = fs::metadata(&manifest_path)
			.await
			.map_err(error::file_io_error(&manifest_path))?;
		let mut manifest_file = Vec::with_capacity(manifest_meta.len() as usize);

		manifest_reader.read_to_end(&mut manifest_file)
			.await
			.map_err(error::file_io_error(&manifest_path))?;

		let manifest_file = String::from_utf8(manifest_file)?;

		let (name, description, default) = match ron::from_str(&manifest_file)? {
			MetaFile::Version1 { name, description, default } => {
				(name, description, default)
			}
		};

		let texture_id = texture_id.to_string();

		Ok(Some(Texture { name, root_dir, texture_id, description, default }))
	}
}
