use crate::error::{ Error, Result };
use crate::external_meta::PACK_FORMATS;
use crate::external_meta::pack_formats::PackVersion;
use ron::extensions::Extensions;
use tokio::fs;

lazy_static::lazy_static! {
	pub static ref RON: ron::Options = {
		ron::Options::default()
			.with_default_extension(Extensions::IMPLICIT_SOME)
			.with_default_extension(Extensions::UNWRAP_VARIANT_NEWTYPES)
	};
}

#[async_recursion::async_recursion]
pub async fn walk_dir(path: &str) -> Result<Vec<String>> {
	let metadata = fs::metadata(path).await
		.map_err(|e| Error::FileDoesNotExist { path: path.into(), source: e })?;
	if !metadata.is_dir() { return Ok(vec![path.into()]) }

	let mut paths = vec![];
	let mut inner_files = fs::read_dir(path).await
		.map_err(|e| Error::IOError { source: e })?;

	while let Some(file) = inner_files.next_entry().await.map_err(|e| Error::IOError { source: e })? {
		let file = file.path();
		for file in walk_dir(file.to_str().expect("invalid unicode paths unsupported")).await? {
			paths.push(file);
		}
	}

	Ok(paths)
}

#[inline]
pub fn hash(thing: &str) -> String {
	let hex = blake3::hash(thing.as_bytes()).to_hex();

	#[cfg(target_os = "windows")]
	let rv = hex[0..10].to_string();
	#[cfg(not(target_os = "windows"))]
	let rv = hex.to_string();

	rv
}

pub fn sort_versions_inefficient(versions: &mut Vec<PackVersion>) {
	versions.dedup_by_key(|v| v.name);

	let mut sorted = Vec::with_capacity(versions.len());
	for version in PACK_FORMATS {
		if let Some(version) = versions.iter().find(|v| v.name == version.name) {
			sorted.push(version.clone());
		}
	}

	*versions = sorted;
}
