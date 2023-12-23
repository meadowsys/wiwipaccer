//! root manifest for pack sources (equivalent-ish to pack.mcmeta of a resource pack,
//! i suppose)

use camino::{ Utf8Path, Utf8PathBuf };
use crate::error::{ self, Error, Result };
use crate::ron;
use crate::texture::{ NewTextureOptions, Texture, TEXTURES_DIR };
use crate::util;
use serde::{ Deserialize, Serialize };
use std::process::Stdio;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

pub const SOURCE_META_FILENAME: &str = "pack.wiwimeta";

#[derive(Debug, Deserialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: String,
		pack_id: String,
		description: Option<String>,
		version: Option<String>
	}
}

#[derive(Debug)]
pub struct Source {
	name: String,
	dir: Utf8PathBuf,
	pack_id: String,
	description: Option<String>,
	version: String,
	textures: Vec<Texture>
}

impl Source {
	pub async fn new(dir: Utf8PathBuf) -> Result<Self> {
		if !util::check_is_dir(&dir).await? {
			return Err(Error::PackSourcePathIsNotDir)
		}

		let mut manifest_path = dir.clone();
		manifest_path.push(SOURCE_META_FILENAME);

		let manifest = util::check_for_and_read_manifest(&manifest_path)
			.await?
			.ok_or_else(|| Error::PackSourceDirContainsNoManifest)?;
		let (name, pack_id, description, version) = match manifest {
			MetaFile::Version1 { name, pack_id, description, version } => {
				(name, pack_id, description, version)
			}
		};

		let version = version.unwrap_or_else(|| "unknown".into());

		let textures = read_textures(&dir)
			.await?;

		Ok(Source { name, dir, pack_id, description, version, textures })
	}
}

async fn read_textures(dir: &Utf8Path) -> Result<Vec<Texture>> {
	let mut textures_dir = dir.to_owned();
	textures_dir.push(TEXTURES_DIR);
	let mut dir_contents = fs::read_dir(&textures_dir)
		.await
		.map_err(|source| Error::FileIOError { source, path: textures_dir.clone() })?;
	let mut textures = vec![];

	while let Some(entry) = {
		dir_contents
			.next_entry()
			.await
			.map_err(|source| Error::FileIOError { source, path: textures_dir.clone() })?
	} {
		let texture_id = entry.file_name()
			.to_str()
			.ok_or_else(|| Error::NonUTF8PathsUnsupported)?
			.into();

		let options = NewTextureOptions {
			root_dir: dir.into(),
			texture_id
		};

		if let Some(texture) = Texture::new(options).await? {
			textures.push(texture);
		}
	}

	Ok(textures)
}
