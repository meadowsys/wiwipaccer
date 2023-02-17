use crate::error::Result;
use ron::extensions::Extensions;
use std::fs;

lazy_static::lazy_static! {
	pub static ref RON: ron::Options = {
		ron::Options::default()
			.with_default_extension(Extensions::IMPLICIT_SOME)
			.with_default_extension(Extensions::UNWRAP_VARIANT_NEWTYPES)
	};
}

pub fn walk_dir(path: &str) -> crate::error::Result<Vec<String>> {
	let metadata = fs::metadata(path)?;
	if !metadata.is_dir() { return Ok(vec![path.into()]) }

	let mut paths = vec![];
	let inner_files = fs::read_dir(path)?;

	for file in inner_files {
		let file = file?.path();
		for file in walk_dir(file.to_str().expect("invalid unicode paths unsupported"))? {
			paths.push(file);
		}
	}

	Ok(paths)
}
