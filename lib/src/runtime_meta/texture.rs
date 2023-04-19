use ahash::{ RandomState, HashMapExt };
use crate::error::{ Error, Result };
use crate::meta::texture::Texture;
use crate::runtime_meta::pack_version_specifier::PackVersionSpecifierRuntimeMeta;
use crate::runtime_meta::option;
use crate::runtime_meta::{ Message, MessageSeverity };
use crate::util::RON;
use std::collections::HashMap;
use super::META_NAME;
use tokio::fs;

#[derive(Debug)]
pub struct WithoutMCVersion(InnerWithoutMCVersion);

#[derive(Debug)]
pub struct InnerWithoutMCVersion {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub options: HashMap<String, option::WithoutMCVersion, RandomState>,
	pub messages: Vec<Message>
}

#[derive(Debug)]
pub enum WithMCVersion {
	Available(Available),
	Unavailable(Unavailable)
}

#[derive(Debug)]
pub struct Available(InnerAvailable);
#[derive(Debug)]
pub struct Unavailable(InnerUnavailable);

#[derive(Debug)]
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

#[derive(Debug)]
pub struct InnerUnavailable {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub options: HashMap<String, option::WithMCVersion, RandomState>,
	pub messages: Vec<Message>
}

crate::impl_deref!(WithoutMCVersion, target InnerWithoutMCVersion);
crate::impl_deref!(Available, target InnerAvailable);
crate::impl_deref!(Unavailable, target InnerUnavailable);

impl WithoutMCVersion {
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
		let texture = RON.from_str::<Texture>(&file)
			.map_err(|e| Error::ParseErrorRonSpannedError {
				path: manifest_path,
				source: e
			})?;

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

		// let mut available_options: HashMap<String, AvailableOptionRuntimeMeta, RandomState> = HashMapExt::new();
		// let mut unavailable_options: HashMap<String, UnavailableOptionRuntimeMeta, RandomState> = HashMapExt::new();
		let mut options: HashMap<String, option::WithoutMCVersion, RandomState> = HashMapExt::new();

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
			options,
			messages
		}))
	}
}

impl WithMCVersion {
	pub async fn from(
		texture_without_mc_version: &WithoutMCVersion,
		mc_version: PackVersionSpecifierRuntimeMeta
	) -> Result<Self> {
		if texture_without_mc_version.options.is_empty() {
			return Ok(WithMCVersion::Unavailable(Unavailable(InnerUnavailable {
				path: texture_without_mc_version.path.clone(),
				shortpath: texture_without_mc_version.shortpath.clone(),
				name: texture_without_mc_version.name.clone(),
				description: texture_without_mc_version.description.clone(),
				options: HashMapExt::new(),
				messages: texture_without_mc_version.messages.clone()
			})))
		}

		let mut messages = texture_without_mc_version.messages.clone();
		let mut default = vec![]; // TODO change this
		let mut available = HashMap::<String, option::Available, RandomState>::new();
		let mut unavailable = HashMap::<String, option::Unavailable, RandomState>::new();

		for (shortpath, option) in texture_without_mc_version.options.iter() {
			use option::WithMCVersion::*;
			match option::WithMCVersion::from(option, mc_version.clone()).await {
				Ok(option) => match option {
					Available(option) => {
						if option.default { default.push(option.shortpath.clone()) }
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
				options: unavailable.into_iter().map(|(s, o)| (s, option::WithMCVersion::Unavailable(o))).collect(),
				messages
			})))
		}

		if default.len() > 1 {
			messages.push(Error::UnavailableError {
				thing: format!("Texture {}", texture_without_mc_version.shortpath),
				reason: "more than 1 default was specified".into()
			}.to_message());

			return Ok(Self::Unavailable(Unavailable(InnerUnavailable {
				path: texture_without_mc_version.path.clone(),
				shortpath: texture_without_mc_version.shortpath.clone(),
				name: texture_without_mc_version.name.clone(),
				description: texture_without_mc_version.description.clone(),
				options: {
					let unavailable_iter = unavailable.into_iter()
						.map(|(shortpath, o)| (shortpath, option::WithMCVersion::Unavailable(o)));
					available.into_iter()
						.map(|(shortpath, o)| (shortpath, option::WithMCVersion::Available(o)))
						.chain(unavailable_iter)
						.collect()
				},
				messages
			})))
		}

		// TODO this needs to be moved to be declared in self instead of in the options
		let default = if default.is_empty() {
			None
		} else {
			Some(default.into_iter().next().unwrap())
		};

		Ok(WithMCVersion::Available(Available(InnerAvailable {
			path: texture_without_mc_version.path.clone(),
			shortpath: texture_without_mc_version.shortpath.clone(),
			name: texture_without_mc_version.name.clone(),
			description: texture_without_mc_version.description.clone(),
			default,
			available_options: available,
			unavailable_options: unavailable,
			messages
		})))
	}
}
