use ahash::{ RandomState, HashMapExt };
use crate::error::{ Error, Result };
use crate::meta::texture::Texture;
use crate::runtime_meta::pack_version_specifier::PackVersionSpecifierRuntimeMeta;
use crate::runtime_meta::option::{ OptionRuntimeMeta, AvailableOptionRuntimeMeta, UnavailableOptionRuntimeMeta };
use crate::runtime_meta::{ Message, MessageSeverity };
use crate::util::RON;
use std::collections::HashMap;
use super::META_NAME;
use tokio::fs;

#[derive(Debug)]
pub enum TextureRuntimeMeta {
	Available(AvailableTextureRuntimeMeta),
	Unavailable(UnavailableTextureRuntimeMeta)
}

#[derive(Debug)]
pub struct AvailableTextureRuntimeMeta(InnerAvailable);
#[derive(Debug)]
pub struct UnavailableTextureRuntimeMeta(InnerUnavailable);

#[derive(Debug)]
pub struct InnerAvailable {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub default: Option<String>,
	pub available_options: HashMap<String, AvailableOptionRuntimeMeta, RandomState>,
	pub unavailable_options: HashMap<String, UnavailableOptionRuntimeMeta, RandomState>,
	pub messages: Vec<Message>
}

#[derive(Debug)]
pub struct InnerUnavailable {
	pub path: String,
	pub shortpath: String,
	pub name: String,
	pub description: String,
	pub options: HashMap<String, OptionRuntimeMeta, RandomState>,
	pub messages: Vec<Message>
}

crate::impl_deref!(AvailableTextureRuntimeMeta, target InnerAvailable);
crate::impl_deref!(UnavailableTextureRuntimeMeta, target InnerUnavailable);

impl TextureRuntimeMeta {
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

		let mut default = vec![];
		let mut available_options: HashMap<String, AvailableOptionRuntimeMeta, RandomState> = HashMapExt::new();
		let mut unavailable_options: HashMap<String, UnavailableOptionRuntimeMeta, RandomState> = HashMapExt::new();

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

			match OptionRuntimeMeta::new(dir_entry_path, mc_version.clone()).await {
				Ok(option) => match option {
					OptionRuntimeMeta::Available(option) => {
						if option.default { default.push(option.shortpath.clone()) }
						available_options.insert(option.shortpath.clone(), option);
					}
					OptionRuntimeMeta::Unavailable(option) => {
						unavailable_options.insert(option.shortpath.clone(), option);
					}
				}
				Err(err) => {
					messages.push(err.to_message());
				}
			}
		}

		let shortpath = std::path::Path::new(path)
			.file_name()
			.unwrap()
			.to_str()
			.unwrap()
			.into();

		if available_options.is_empty() {
			messages.push(Error::UnavailableInfo {
				thing: format!("Texture {shortpath}"),
				reason: "no options are available".into()
			}.to_message());

			return Ok(TextureRuntimeMeta::Unavailable(UnavailableTextureRuntimeMeta(InnerUnavailable {
				path: path.into(),
				shortpath,
				name,
				description,
				options: {
					unavailable_options.into_iter()
						.map(|(shortpath, o)| (shortpath, OptionRuntimeMeta::Unavailable(o)))
						.collect()
				},
				messages
			})))
		}

		if default.len() > 1 {
			messages.push(Error::UnavailableError {
				thing: format!("Texture {shortpath}"),
				reason: "more than 1 default was specified".into()
			}.to_message());

			return Ok(TextureRuntimeMeta::Unavailable(UnavailableTextureRuntimeMeta(InnerUnavailable {
				path: path.into(),
				shortpath,
				name,
				description,
				options: {
					let unavailable_iter = unavailable_options.into_iter()
						.map(|(shortpath, o)| (shortpath, OptionRuntimeMeta::Unavailable(o)));

					available_options.into_iter()
						.map(|(shortpath, o)| (shortpath, OptionRuntimeMeta::Available(o)))
						.chain(unavailable_iter)
						.collect()
				},
				messages
			})))
		}

		let default = if default.is_empty() {
			None
		} else {
			Some(default.into_iter().next().unwrap())
		};

		Ok(TextureRuntimeMeta::Available(AvailableTextureRuntimeMeta(InnerAvailable {
			path: path.into(),
			shortpath,
			name,
			description,
			default,
			available_options,
			unavailable_options,
			messages
		})))
	}
}
