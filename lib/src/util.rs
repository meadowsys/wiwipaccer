use camino::Utf8Path;
use crate::error::*;
use tokio::fs;

pub async fn check_is_dir(path: &Utf8Path) -> Result<bool> {
	let meta = fs::metadata(path)
		.await
		.map_err(|source| Error::FileIOError { source, path: path.into() })?;
	Ok(meta.is_dir())
}

pub async fn check_is_file(path: &Utf8Path) -> Result<bool> {
	let meta = fs::metadata(path)
		.await
		.map_err(|source| Error::FileIOError { source, path: path.into() })?;
	Ok(meta.is_file())
}
