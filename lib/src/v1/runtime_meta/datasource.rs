use ahash::{ RandomState, HashMapExt };
use crate::error::{ Error, Result };
use crate::meta::datasource::{ Datasource as DatasourceMeta, Version };
use crate::external_meta::PACK_FORMATS;
use crate::external_meta::pack_formats::PackVersion;
use crate::runtime_meta::pack_version_specifier::PackVersionSpecifierRuntimeMeta;
use crate::runtime_meta::texture;
use crate::runtime_meta::{ Message, MessageSeverity, read_meta_file };
use crate::runtime_meta::action::Action;
use super::super::util::RON;
use super::super::util::sort_versions_inefficient;
use std::collections::HashMap;
use std::path;
use std::sync::Arc;
use super::{ META_NAME, TEXTURES_DIR };
use tokio::io::AsyncWriteExt;
use tokio::fs;
use tokio::process::Command;

#[derive(Debug, serde::Serialize)]
pub enum Datasource {
	WithoutMCVersion(WithoutMCVersion),
	WithMCVersion {
		without_mc_version: WithoutMCVersion,
		with_mc_version: WithMCVersion
	}
}

#[derive(Debug, serde::Serialize)]
pub struct WithoutMCVersion(InnerWithoutMCVersion);

#[derive(Debug, serde::Serialize)]
pub struct InnerWithoutMCVersion {
	pub path: String,
	pub name: String,
	pub description: String,
	pub version: String,
	pub textures: HashMap<String, texture::WithoutMCVersion, RandomState>,
	pub messages: Vec<Message>
}

#[derive(Debug, serde::Serialize)]
pub struct WithMCVersion(InnerWithMCVersion);

#[derive(Debug, serde::Serialize)]
pub struct InnerWithMCVersion {
	pub mc_version: String,
	pub available_textures: HashMap<String, texture::Available, RandomState>,
	pub unavailable_textures: HashMap<String, texture::Unavailable, RandomState>,
	pub messages: Vec<Message>
}

crate::impl_deref!(WithoutMCVersion, target InnerWithoutMCVersion);
crate::impl_deref!(WithMCVersion, target InnerWithMCVersion);

#[derive(Debug, Clone, Copy)]
pub enum BuildType {
	CustomiseDefault,
	FromScratch
}

impl Datasource {
	pub async fn new(path: &str) -> Result<Self> {
		let mut messages = vec![];
		let datasource = match read_meta_file::<DatasourceMeta>(path).await {
			Err(e) => match e {
				Error::ManifestDoesNotExist { path, .. } | Error::ManifestIsNotFile { path } => {
					Err(Error::ManifestDoesNotExistLikelyNotProject { path })
				}
				e => { Err(e) }
			}
			res => { res }
		}?;

		struct Destructure {
			name: String,
			version: Version,
			description: String
		}

		let Destructure { name, version, description } = match datasource {
			DatasourceMeta::V1 { name, version, description } => {
				Destructure {
					name,
					version: version.unwrap_or_else(|| Version::String("unknown".into())),
					description: description.unwrap_or_else(|| "description not provided".into())
				}
			}
		};

		// let mut available_textures: HashMap<String, texture::Available, RandomState> = HashMapExt::new();
		// let mut unavailable_textures: HashMap<String, texture::Unavailable, RandomState> = HashMapExt::new();
		let mut textures = HashMap::<String, texture::WithoutMCVersion, RandomState>::new();

		let textures_dir = format!("{path}/{TEXTURES_DIR}");
		let mut dir_contents = fs::read_dir(&textures_dir).await
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
					message: format!("item in datasource isn't a dir (potential texture) or manifest file: {dir_entry_path}"),
					severity: MessageSeverity::Info
				});
				continue
			}

			match texture::WithoutMCVersion::new(dir_entry_path).await {
				Ok(texture) => { textures.insert(texture.shortpath.clone(), texture); }
				Err(e) => { messages.push(e.to_message()) }
			}
		}

		let version = match version {
			Version::Git => {
				Command::new("git")
					.arg("rev-parse")
					.arg("HEAD")
					.current_dir(path)
					.output()
					.await
					.map(|g| String::from_utf8(g.stdout).unwrap().trim().into())
					.unwrap_or_else(|_| "unknown (git failed to run)".into())
			}
			Version::String(v) => { v }
		};

		Ok(Datasource::WithoutMCVersion(WithoutMCVersion(InnerWithoutMCVersion {
			path: path.into(),
			name,
			description,
			version,
			textures,
			messages
		})))
	}

	pub fn get_name(&self) -> &str {
		use Datasource::*;
		match self {
			WithMCVersion { without_mc_version, .. } | WithoutMCVersion(without_mc_version) => {
				&without_mc_version.name
			}
		}
	}

	pub fn get_supported_mc_versions(&self) -> Result<Vec<PackVersion>> {
		let mut versions = self.get_supported_mc_versions_no_sort()?;
		sort_versions_inefficient(&mut versions);
		Ok(versions)
	}

	/// pub for use with workspace
	pub(crate) fn get_supported_mc_versions_no_sort(&self) -> Result<Vec<PackVersion>> {
		let without_mc_version = match self {
			Datasource::WithMCVersion { ref without_mc_version, .. } => { without_mc_version }
			Datasource::WithoutMCVersion(ref without_mc_version) => { without_mc_version }
		};

		let mut versions = Vec::with_capacity(without_mc_version.textures.len());
		for version in without_mc_version.textures.values() {
			versions.push(version.get_supported_mc_versions()?);
		}

		let versions = versions.into_iter()
			.flatten()
			.collect();

		Ok(versions)
	}

	pub async fn with_mc_version(self, mc_version: String) -> Self {
		let without_mc_version = match self {
			Datasource::WithMCVersion { without_mc_version, .. } => { without_mc_version }
			Datasource::WithoutMCVersion(without_mc_version) => { without_mc_version }
		};

		let mut messages = vec![];
		let mut available = HashMap::<String, texture::Available, RandomState>::new();
		let mut unavailable = HashMap::<String, texture::Unavailable, RandomState>::new();

		for (shortpath, texture) in &without_mc_version.textures {
			match texture::WithMCVersion::from(texture, mc_version.clone()).await {
				Ok(texture) => match texture {
					texture::WithMCVersion::Available(texture) => {
						available.insert(shortpath.clone(), texture);
					}
					texture::WithMCVersion::Unavailable(texture) => {
						unavailable.insert(shortpath.clone(), texture);
					}
				}
				Err(err) => {
					messages.push(err.to_message());
				}
			}
		}

		Self::WithMCVersion {
			without_mc_version,
			with_mc_version: WithMCVersion(InnerWithMCVersion {
				mc_version,
				available_textures: available,
				unavailable_textures: unavailable,
				messages
			})
		}
	}

	pub async fn build(
		&self,
		dir: &str,
		choices: &HashMap<String, String, RandomState>,
		buildtype: BuildType,
	) -> Result<()> {
		let versioned = match self {
			Datasource::WithMCVersion { with_mc_version, .. } => { Ok(with_mc_version) }
			Datasource::WithoutMCVersion(_) => { Err(Error::MCVersionUnspecified) }
		}?;

		let mut messages = vec![];
		let include_default = matches!(buildtype, BuildType::CustomiseDefault);

		fs::create_dir_all(dir).await
			.map_err(|e| {
				let error = Box::new(Error::IOError { source: e });
				Error::ActionFailedToExecute { error }
			})?;

		let mut executed = vec![];

		for (texture, option) in choices.iter() {
			let texture = match versioned.available_textures.get(texture) {
				Some(texture) => { texture }
				None => {
					if versioned.unavailable_textures.contains_key(texture) {
						messages.push(Error::TextureUnavailable {
							texture: texture.into()
						}.to_message());
					} else {
						messages.push(Error::TextureNotFound {
							texture: texture.into()
						}.to_message());
					}

					continue
				}
			};

			let option = match texture.available_options.get(option) {
				Some(option) => { option }
				None => {
					if texture.unavailable_options.contains_key(option) {
						messages.push(Error::OptionUnavailable {
							option: option.into()
						}.to_message());
					} else {
						messages.push(Error::OptionNotFound {
							option: option.into()
						}.to_message());
					}

					continue
				}
			};

			for action in &option.available_version.actions {
				action.execute_in(dir).await
					.map_err(|e| Error::ActionFailedToExecute { error: Box::new(e) })?;
			}

			executed.push(&texture.name);
		}

		if include_default {
			for (texture_name, texture) in &versioned.available_textures {
				if executed.contains(&texture_name) { continue }

				if let Some(option) = &texture.default {
					let option = texture.available_options.get(option).unwrap();
					for action in &option.available_version.actions {
						action.execute_in(dir).await
							.map_err(|e| Error::ActionFailedToExecute { error: Box::new(e) })?;
					}
				}
			}
		}

		Ok(())
	}
}
