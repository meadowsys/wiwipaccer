use ahash::{ RandomState, HashMapExt };
use crate::error::{ Error, Result };
use crate::meta::option::TextureOption;
use crate::runtime_meta::pack_version_specifier::PackVersionSpecifierRuntimeMeta;
use crate::runtime_meta::{ Message, MessageSeverity };
use crate::runtime_meta::version::{ VersionRuntimeMeta, AvailableVersionRuntimeMeta, UnavailableVersionRuntimeMeta };
use crate::util::RON;
use std::collections::HashMap;
use super::META_NAME;
use tokio::fs;

#[derive(Debug)]
pub enum OptionRuntimeMeta {
	Available(AvailableOptionRuntimeMeta),
	Unavailable(UnavailableOptionRuntimeMeta)
}

#[derive(Debug)]
pub struct AvailableOptionRuntimeMeta {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub available_version: AvailableVersionRuntimeMeta,
	pub unavailable_versions: HashMap<String, UnavailableVersionRuntimeMeta, RandomState>,
	pub messages: Vec<Message>
}

#[derive(Debug)]
pub struct UnavailableOptionRuntimeMeta {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub versions: HashMap<String, UnavailableVersionRuntimeMeta, RandomState>,
	pub messages: Vec<Message>
}

impl OptionRuntimeMeta {
	pub async fn new(path: &str, mc_version: PackVersionSpecifierRuntimeMeta) -> Result<Self> {
		let mut messages = vec![];
		let manifest_path = format!("{path}/{META_NAME}");

		let manifest_file_meta = fs::metadata(&manifest_path).await
			.map_err(|e| Error::ManifestDoesNotExist { path: manifest_path.clone(), source: e })?;
		if !manifest_file_meta.is_file() {
			return Err(Error::ManifestIsNotFile { path: manifest_path })
		}

		let file = fs::read_to_string(&manifest_path).await
			.map_err(|e| Error::IOError { source: e })?;
		let option = RON.from_str::<TextureOption>(&file)
			.map_err(|e| Error::ParseErrorRonSpannedError {
				path: manifest_path,
				source: e
			})?;

		struct Destructure {
			name: String,
			description: String
		}

		let Destructure { name, description } = match option {
			TextureOption::V1 { name, description } => {
				Destructure {
					name,
					description: description.unwrap_or_else(|| "description not provided".into())
				}
			}
		};

		let mut available_versions = HashMap::<String, AvailableVersionRuntimeMeta, RandomState>::new();
		let mut unavailable_versions = HashMap::<String, UnavailableVersionRuntimeMeta, RandomState>::new();

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
				messages.push(Message {
					message: format!("item in an option dir is not a dir (potential version) or the manifest file: {dir_entry_path}"),
					severity: MessageSeverity::Info
				});
				continue
			}

			match VersionRuntimeMeta::new(dir_entry_path, mc_version.clone()).await {
				Ok(version) => match version {
					VersionRuntimeMeta::Available(version) => {
						available_versions.insert(version.shortpath.clone(), version);
					}
					VersionRuntimeMeta::Unavailable(version) => {
						unavailable_versions.insert(version.shortpath.clone(), version);
					}
				}
				Err(err) => {
					messages.push(err.to_warning());
				}
			}
		}

		let shortpath = std::path::Path::new(path)
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.into();

		if available_versions.is_empty() {
			return Ok(OptionRuntimeMeta::Unavailable(UnavailableOptionRuntimeMeta {
				path: path.into(),
				shortpath,
				name,
				description,
				versions: unavailable_versions,
				messages
			}))
		}

		if available_versions.len() > 1 {
			return Err(Error::MultipleAvailableVersions {
				available_versions_shortnames_formatted: {
					available_versions.iter()
						.map::<&str, _>(|v| &v.1.shortpath)
						.collect::<Vec<_>>()
						.join(", ")
				}
			})
		}

		Ok(OptionRuntimeMeta::Available(AvailableOptionRuntimeMeta {
			path: path.into(),
			shortpath,
			name,
			description,
			available_version: available_versions.into_iter().next().unwrap().1,
			unavailable_versions,
			messages
		}))
	}
}
