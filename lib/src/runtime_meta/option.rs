use ahash::{ RandomState, HashMapExt };
use crate::error::{ Error, Result };
use crate::meta::option::TextureOption;
use crate::runtime_meta::pack_version_specifier::PackVersionSpecifierRuntimeMeta;
use crate::runtime_meta::{ Message, MessageSeverity };
use crate::runtime_meta::version::{ self, VersionWithoutMCVersion, VersionWithMCVersion, AvailableVersionRuntimeMeta, UnavailableVersionRuntimeMeta };
use crate::util::RON;
use std::collections::HashMap;
use super::META_NAME;
use tokio::fs;

#[derive(Debug)]
pub struct OptionWithoutMCVersion(InnerOptionWithoutMCVersion);

#[derive(Debug)]
pub struct InnerOptionWithoutMCVersion {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub default: bool,
	pub versions: HashMap<String, VersionWithoutMCVersion, RandomState>,
	pub messages: Vec<Message>
}

#[derive(Debug)]
pub enum OptionWithMCVersion {
	Available(AvailableOptionRuntimeMeta),
	Unavailable(UnavailableOptionRuntimeMeta)
}

#[derive(Debug)]
pub struct AvailableOptionRuntimeMeta(InnerAvailable);
#[derive(Debug)]
pub struct UnavailableOptionRuntimeMeta(InnerUnavailable);


#[derive(Debug)]
pub struct InnerAvailable {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub default: bool,
	pub available_version: AvailableVersionRuntimeMeta,
	pub unavailable_versions: HashMap<String, UnavailableVersionRuntimeMeta, RandomState>,
	pub messages: Vec<Message>
}

#[derive(Debug)]
pub struct InnerUnavailable {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub default: bool,
	pub versions: HashMap<String, UnavailableVersionRuntimeMeta, RandomState>,
	pub messages: Vec<Message>
}

crate::impl_deref!(OptionWithoutMCVersion, target InnerOptionWithoutMCVersion);
crate::impl_deref!(AvailableOptionRuntimeMeta, target InnerAvailable);
crate::impl_deref!(UnavailableOptionRuntimeMeta, target InnerUnavailable);

impl OptionWithoutMCVersion {
	pub async fn new(path: &str) -> Result<Self> {
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
			description: String,
			default: bool
		}

		let Destructure { name, description, default } = match option {
			TextureOption::V1 { name, description, default } => {
				Destructure {
					name,
					description: description.unwrap_or_else(|| "description not provided".into()),
					default: default.unwrap_or(false)
				}
			}
		};

		let mut versions = HashMap::<String, VersionWithoutMCVersion, RandomState>::new();

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

			match VersionWithoutMCVersion::new(dir_entry_path).await {
				Ok(version) => { versions.insert(version.shortpath.clone(), version); }
				Err(e) => { messages.push(e.to_message()) }
			}
		}

		let shortpath = std::path::Path::new(path)
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.into();

		if versions.is_empty() {
			messages.push(Error::UnavailableInfo {
				thing: format!("Option {shortpath}"),
				reason: "no versions are available".into()
			}.to_message());
		}

		Ok(Self(InnerOptionWithoutMCVersion {
			path: path.into(),
			shortpath,
			name,
			description,
			default,
			versions,
			messages
		}))
	}
}

impl OptionWithMCVersion {
	pub async fn from(
		option_without_mc_version: &OptionWithoutMCVersion,
		mc_version: PackVersionSpecifierRuntimeMeta
	) -> Result<Self> {
		if option_without_mc_version.versions.is_empty() {
			return Ok(OptionWithMCVersion::Unavailable(UnavailableOptionRuntimeMeta(InnerUnavailable {
				path: option_without_mc_version.path.clone(),
				shortpath: option_without_mc_version.shortpath.clone(),
				name: option_without_mc_version.name.clone(),
				description: option_without_mc_version.description.clone(),
				default: option_without_mc_version.default,
				versions:  HashMap::<String, UnavailableVersionRuntimeMeta, RandomState>::new(),
				messages: option_without_mc_version.messages.clone()
			})))
		}

		let mut messages = option_without_mc_version.messages.clone();
		let mut available_versions = HashMap::<String, AvailableVersionRuntimeMeta, RandomState>::new();
		let mut unavailable_versions = HashMap::<String, UnavailableVersionRuntimeMeta, RandomState>::new();

		for (shortpath, version) in option_without_mc_version.versions.iter() {
			use VersionWithMCVersion::*;
			match VersionWithMCVersion::from(version, mc_version.clone()).await {
				Ok(version) => match version {
					Available(version) => { available_versions.insert(shortpath.clone(), version); }
					Unavailable(version) => { unavailable_versions.insert(shortpath.clone(), version); }
				}
				Err(e) => { messages.push(e.to_message()) }
			}
		}

		if available_versions.is_empty() {
			messages.push(Error::UnavailableInfo {
				thing: format!("Option {}", option_without_mc_version.shortpath),
				reason: "no compatible versions are available".into()
			}.to_message());

			return Ok(OptionWithMCVersion::Unavailable(UnavailableOptionRuntimeMeta(InnerUnavailable {
				path: option_without_mc_version.path.clone(),
				shortpath: option_without_mc_version.shortpath.clone(),
				name: option_without_mc_version.name.clone(),
				description: option_without_mc_version.description.clone(),
				default: option_without_mc_version.default,
				versions: unavailable_versions,
				messages
			})))
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

		Ok(OptionWithMCVersion::Available(AvailableOptionRuntimeMeta(InnerAvailable {
			path: option_without_mc_version.path.clone(),
			shortpath: option_without_mc_version.shortpath.clone(),
			name: option_without_mc_version.name.clone(),
			description: option_without_mc_version.description.clone(),
			default: option_without_mc_version.default,
			available_version: available_versions.into_iter().next().unwrap().1,
			unavailable_versions,
			messages
		})))
	}
}
