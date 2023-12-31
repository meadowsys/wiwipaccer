use camino::Utf8Path;
use crate::error::{ self, Error, Result };
use crate::ron;
use serde::de::DeserializeOwned;
use tokio::fs;
use tokio::io::AsyncReadExt;

pub async fn check_is_dir(path: &str) -> Result<bool> {
	let meta = fs::metadata(path)
		.await
		.map_err(|source| Error::FileIOError { source, path: path.into() })?;
	Ok(meta.is_dir())
}

pub async fn check_is_file(path: &str) -> Result<bool> {
	let meta = fs::metadata(path)
		.await
		.map_err(|source| Error::FileIOError { source, path: path.into() })?;
	Ok(meta.is_file())
}

pub async fn check_is_dir_silent_fail(path: &str) -> bool {
	match check_is_dir(path).await {
		Ok(true) => { true }
		Ok(false) | Err(_) => { false }
	}
}
pub async fn check_is_file_silent_fail(path: &str) -> bool {
	match check_is_file(path).await {
		Ok(true) => { true }
		Ok(false) | Err(_) => { false }
	}
}

/// Ok(Some(manifest)): success (obviously)
/// Ok(None): manifest doesn't exist
/// Err(err): other error
pub async fn check_for_and_read_manifest<T>(path: &str) -> Result<Option<T>>
where
	T: DeserializeOwned
{
	if !check_is_file_silent_fail(path).await {
		return Ok(None)
	}

	let mut manifest_reader = fs::OpenOptions::new()
		.read(true)
		.open(path)
		.await
		.map_err(error::file_io_error(path))?;

	let manifest_meta = fs::metadata(path)
		.await
		.map_err(error::file_io_error(path))?;
	let mut manifest_file = Vec::with_capacity(manifest_meta.len() as usize);

	manifest_reader.read_to_end(&mut manifest_file)
		.await
		.map_err(error::file_io_error(path))?;

	let manifest_file = String::from_utf8(manifest_file)?;

	ron::from_str(&manifest_file)
		.map(|t| Some(t))
}
