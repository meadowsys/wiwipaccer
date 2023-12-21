//! root manifest for a datasource (equivalent-ish to pack.mcmeta of a resource pack,
//! i suppose)

use camino::Utf8PathBuf;
use crate::error::*;
use serde::{ Deserialize, Serialize };
use tokio::fs;
use tokio::io::AsyncReadExt;

const SOURCE_META_FILENAME: &str = "pack.wiwimeta";

#[derive(Debug, Deserialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: String,
		description: Option<String>,
		version: Option<String>
	}
}

#[derive(Debug)]
pub struct Source {
	name: String,
	description: Option<String>,
	version: String
}

impl Source {
	pub async fn new(dir: Utf8PathBuf) -> Result<Self> {
		let _ = fs::read_dir(&dir).await
			.map_err(|source| Error::SourceDirReadError { path: dir.clone(), source })?;

		let mut manifest_path = dir;
		manifest_path.push(SOURCE_META_FILENAME);

		let mut manifest_reader = fs::OpenOptions::new()
			.read(true)
			.open(&manifest_path)
			.await
			.map_err(|source| Error::SourceManifestReadError { path: manifest_path.clone(), source })?;
		let manifest_meta = fs::metadata(&manifest_path)
			.await
			.map_err(|source| Error::FileIOError { path: manifest_path.clone(), source })?;

		let mut manifest_file = Vec::with_capacity(manifest_meta.len() as usize);
		manifest_reader.read_to_end(&mut manifest_file)
			.await
			.map_err(|source| Error::FileIOError { path: manifest_path, source })?;

		let manifest_file = String::from_utf8(manifest_file)?;

		let (name, description, version) = match ron::from_str(&manifest_file)? {
			MetaFile::Version1 { name, description, version } => {
				let version = version.unwrap_or_else(|| "unknown".into());
				(name, description, version)
			}
		};

		Ok(Source { name, description, version })
	}
}
