mod workspace;
mod datasource;
mod texture;
mod option;
mod version;

use camino::{ Utf8Path, Utf8PathBuf };
use crate::pack_formats::PackVersion;
use serde::{ Deserialize, Serialize };
use serde::de::DeserializeOwned;
use std::rc::Rc;
use std::path::MAIN_SEPARATOR;
use super::{ Map, Result };
use super::error::Error;
use super::util::*;
use tokio::fs;

pub const MANIFEST_FILENAME: &str = "manifest.wiwi";

async fn read_manifest<T: DeserializeOwned>(dir: &Utf8Path) -> Result<T> {
	let manifest_path = [dir.as_ref(), MANIFEST_FILENAME]
		.into_iter()
		.collect::<Utf8PathBuf>();

	let manifest_meta = fs::metadata(&manifest_path)
		.await
		.map_err(|e| Error::ManifestReadFailed { e, path: manifest_path.clone() })?;

	if !manifest_meta.is_file() {
		return Err(Error::ManifestNotFile { path: manifest_path.clone() })
	}

	let file = fs::read_to_string(&manifest_path)
		.await
		.map_err(|e| Error::ManifestReadFailed { e, path: manifest_path.clone() })?;

	let parsed = RON.from_str::<T>(&file)
		.map_err(|e| Error::ManifestParseError { e, path: manifest_path })?;

	Ok(parsed)
}
