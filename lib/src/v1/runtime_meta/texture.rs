use ahash::{ RandomState, HashMapExt };
use crate::error::{ Error, Result };
use crate::external_meta::pack_formats::PackVersion;
use crate::meta::texture::Texture;
use crate::runtime_meta::pack_version_specifier::PackVersionSpecifierRuntimeMeta;
use crate::runtime_meta::option;
use crate::runtime_meta::{ Message, MessageSeverity, read_meta_file };
use super::super::util::RON;
use std::collections::HashMap;
use super::META_NAME;
use tokio::fs;

#[derive(Debug, serde::Serialize)]
pub struct WithoutMCVersion(InnerWithoutMCVersion);

#[derive(Debug, serde::Serialize)]
pub struct InnerWithoutMCVersion {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub default: Option<String>,
	pub options: HashMap<String, option::WithoutMCVersion, RandomState>,
	pub messages: Vec<Message>
}

#[derive(Debug)]
pub enum WithMCVersion {
	Available(Available),
	Unavailable(Unavailable)
}

#[derive(Debug, serde::Serialize)]
pub struct Available(InnerAvailable);
#[derive(Debug, serde::Serialize)]
pub struct Unavailable(InnerUnavailable);

#[derive(Debug, serde::Serialize)]
pub struct InnerAvailable {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub default: Option<String>,
	pub available_options: HashMap<String, option::Available, RandomState>,
	pub unavailable_options: HashMap<String, option::Unavailable, RandomState>,
	pub messages: Vec<Message>
}

#[derive(Debug, serde::Serialize)]
pub struct InnerUnavailable {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub default: Option<String>,
	pub options: HashMap<String, option::WithMCVersion, RandomState>,
	pub messages: Vec<Message>
}

crate::impl_deref!(WithoutMCVersion, target InnerWithoutMCVersion);
crate::impl_deref!(Available, target InnerAvailable);
crate::impl_deref!(Unavailable, target InnerUnavailable);

impl WithoutMCVersion {
	pub async fn new(path: &str) -> Result<Self> {
		let mut messages = vec![];
		let texture = read_meta_file::<Texture>(path).await?;

		struct Destructure {
			name: String,
			description: String,
			default: Option<String>
		}

		let Destructure { name, description, default } = match texture {
			Texture::V1 { name, description, default } => {
				Destructure {
					name,
					description: description.unwrap_or_else(|| "description not provided".into()),
					default
				}
			}
		};

		// let mut available_options: HashMap<String, AvailableOptionRuntimeMeta, RandomState> = HashMapExt::new();
		// let mut unavailable_options: HashMap<String, UnavailableOptionRuntimeMeta, RandomState> = HashMapExt::new();
		let mut options: HashMap<String, option::WithoutMCVersion, RandomState> = HashMapExt::new();

		let mut dir_contents = fs::read_dir(&path).await
			.map_err(|e| Error::IOError { source: e })?;

		let mut default_found = false;

		while let Some(dir_entry) = dir_contents.next_entry().await.map_err(|e| Error::IOError { source: e })? {
			if let Some(ref default) = default {
				if !default_found && default == dir_entry.file_name().to_str().unwrap() {
					default_found = true;
				}
			}

			let dir_entry_path = dir_entry.path();
			let dir_entry_path = dir_entry_path.to_str()
				.expect("invalid unicode paths unsupported");

			if dir_entry_path.ends_with(META_NAME) { continue }

			let dir_entry_metadata = fs::metadata(&dir_entry_path).await
				.map_err(|e| Error::IOError { source: e })?;
			if !dir_entry_metadata.is_dir() {
				messages.push(Message {
					message: format!("item in a texture dir is not a dir (potential option) or the manifest file: {dir_entry_path}"),
					severity: MessageSeverity::Info
				});
				continue
			}

			match option::WithoutMCVersion::new(dir_entry_path).await {
				Ok(option) => {
					// TODO handle defaults
					options.insert(option.shortpath.clone(), option);
				}
				Err(e) => { messages.push(e.to_message()) }
			}
		}

		if let Some(ref default) = default {
			if !default_found {
				return Err(Error::DefaultDoesNotExist { default: default.clone() })
			}
		}

		let shortpath = std::path::Path::new(path)
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.into();

		Ok(WithoutMCVersion(InnerWithoutMCVersion {
			path: path.into(),
			shortpath,
			name,
			description,
			default,
			options,
			messages
		}))
	}

	pub fn get_supported_mc_versions(&self) -> Result<Vec<PackVersion>> {
		let mut versions = vec![];

		for option in self.options.values() {
			versions.append(&mut option.get_supported_mc_versions()?);
		}

		versions.dedup_by_key(|v| v.name);
		Ok(versions)
	}
}

impl WithMCVersion {
	pub async fn from(
		texture_without_mc_version: &WithoutMCVersion,
		mc_version: String
	) -> Result<Self> {
		if texture_without_mc_version.options.is_empty() {
			return Ok(WithMCVersion::Unavailable(Unavailable(InnerUnavailable {
				path: texture_without_mc_version.path.clone(),
				shortpath: texture_without_mc_version.shortpath.clone(),
				name: texture_without_mc_version.name.clone(),
				description: texture_without_mc_version.description.clone(),
				default: texture_without_mc_version.default.clone(),
				options: HashMapExt::new(),
				messages: texture_without_mc_version.messages.clone()
			})))
		}

		let mut messages = texture_without_mc_version.messages.clone();
		let mut available = HashMap::<String, option::Available, RandomState>::new();
		let mut unavailable = HashMap::<String, option::Unavailable, RandomState>::new();

		for (shortpath, option) in texture_without_mc_version.options.iter() {
			use option::WithMCVersion::*;
			match option::WithMCVersion::from(option, mc_version.clone()).await {
				Ok(option) => match option {
					Available(option) => {
						available.insert(shortpath.clone(), option);
					}
					Unavailable(option) => {
						unavailable.insert(shortpath.clone(), option);
					}
				}
				Err(err) => {
					messages.push(err.to_message());
				}
			}
		}

		if available.is_empty() {
			messages.push(Error::UnavailableInfo {
				thing: format!("Texture {}", texture_without_mc_version.shortpath),
				reason: "no options are available".into()
			}.to_message());

			return Ok(Self::Unavailable(Unavailable(InnerUnavailable {
				path: texture_without_mc_version.path.clone(),
				shortpath: texture_without_mc_version.shortpath.clone(),
				name: texture_without_mc_version.name.clone(),
				description: texture_without_mc_version.description.clone(),
				default: texture_without_mc_version.default.clone(),
				options: unavailable.into_iter().map(|(s, o)| (s, option::WithMCVersion::Unavailable(o))).collect(),
				messages
			})))
		}

		Ok(WithMCVersion::Available(Available(InnerAvailable {
			path: texture_without_mc_version.path.clone(),
			shortpath: texture_without_mc_version.shortpath.clone(),
			name: texture_without_mc_version.name.clone(),
			description: texture_without_mc_version.description.clone(),
			default: texture_without_mc_version.default.clone(),
			available_options: available,
			unavailable_options: unavailable,
			messages
		})))
	}
}
