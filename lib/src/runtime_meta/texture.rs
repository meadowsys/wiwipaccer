use ahash::{ RandomState, HashMapExt };
use crate::error::{ Error, Result };
use crate::meta::texture::Texture;
use crate::runtime_meta::option::OptionRuntimeMeta;
use crate::runtime_meta::Warning;
use crate::util::RON;
use std::collections::HashMap;
use super::META_NAME;
use tokio::fs;

#[derive(Debug)]
pub struct TextureRuntimeMeta {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub options: HashMap<String, OptionRuntimeMeta, RandomState>,
	pub warnings: Vec<Warning>
}

impl TextureRuntimeMeta {
	pub async fn new(path: &str) -> Result<Self> {
		let mut warnings = vec![];
		let manifest_path = format!("{path}/{META_NAME}");

		let manifest_file_meta = fs::metadata(&manifest_path).await
			.map_err(|e| Error::ManifestDoesNotExist { path: manifest_path.clone(), source: e })?;
		if !manifest_file_meta.is_file() {
			return Err(Error::ManifestIsNotFile { path: manifest_path })
		}

		let file = fs::read_to_string(&manifest_path).await
			.map_err(|e| Error::IOError { source: e })?;
		let texture = RON.from_str::<Texture>(&file)
			.map_err(|e| Error::ParseErrorRonSpannedError {
				path: manifest_path,
				source: e
			})?;

		dbg!(&texture);

		struct Destructure {
			name: String,
			description: String
		}

		let Destructure { name, description } = match texture {
			Texture::V1 { name, description } => {
				Destructure {
					name,
					description: description.unwrap_or_else(|| "description not provided".into())
				}
			}
		};

		let mut options: HashMap<String, OptionRuntimeMeta, RandomState> = HashMapExt::new();

		let mut dir_contents = fs::read_dir(&path).await
			.map_err(|e| Error::IOError { source: e })?;

		while let Some(dir_entry) = dir_contents.next_entry().await.map_err(|e| Error::IOError { source: e })? {
			let dir_entry_path = dir_entry.path();
			let dir_entry_path = dir_entry_path.to_str()
				.expect("invalid unicode paths unsupported");

			if dir_entry_path.ends_with(META_NAME) { continue }

			let dir_entry_metadata = fs::metadata(&dir_entry_path).await
				.map_err(|e| Error::IOError { source: e })?;
			if !dir_entry_metadata.is_dir() {
				warnings.push(Warning {
					message: format!("item in a texture dir is not an option or the manifest file: {dir_entry_path}")
				});
			}

			match OptionRuntimeMeta::new(dir_entry_path).await {
				Ok(option) => {
					options.insert(option.shortpath.clone(), option);
				}
				Err(err) => {
					warnings.push(err.to_warning())
				}
			}
		}

		let shortpath = std::path::Path::new(path)
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.into();

		Ok(TextureRuntimeMeta {
			path: path.into(),
			shortpath,
			name,
			description,
			options,
			warnings
		})
	}
}
