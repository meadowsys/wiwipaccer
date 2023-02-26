use ahash::{ RandomState, HashMapExt };
use crate::error::{ Error, Result };
use crate::meta::datasource::{ Datasource, Version };
use crate::runtime_meta::pack_version_specifier::PackVersionSpecifierRuntimeMeta;
use crate::runtime_meta::texture::{ TextureRuntimeMeta, AvailableTextureRuntimeMeta, UnavailableTextureRuntimeMeta };
use crate::runtime_meta::{ Message, MessageSeverity };
use crate::util::RON;
use std::collections::HashMap;
use super::{ META_NAME, TEXTURES_DIR };
use tokio::fs;
use tokio::process::Command;

#[derive(Debug)]
pub struct DatasourceRuntimeMeta {
	pub path: String,
	pub name: String,
	pub version: String,
	pub description: String,
	pub available_textures: HashMap<String, AvailableTextureRuntimeMeta, RandomState>,
	pub unavailable_textures: HashMap<String, UnavailableTextureRuntimeMeta, RandomState>,
	pub messages: Vec<Message>
}

impl DatasourceRuntimeMeta {
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
		let datasource = RON.from_str::<Datasource>(&file)
			.map_err(|e| Error::ParseErrorRonSpannedError {
				path: manifest_path,
				source: e
			})?;

		struct Destructure {
			name: String,
			version: Version,
			description: String
		}

		let Destructure { name, version, description } = match datasource {
			Datasource::V1 { name, version, description } => {
				Destructure {
					name,
					version: version.unwrap_or_else(|| Version::String("unknown".into())),
					description: description.unwrap_or_else(|| "description not provided".into())
				}
			}
		};

		let mut available_textures: HashMap<String, AvailableTextureRuntimeMeta, RandomState> = HashMapExt::new();
		let mut unavailable_textures: HashMap<String, UnavailableTextureRuntimeMeta, RandomState> = HashMapExt::new();

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

			match TextureRuntimeMeta::new(dir_entry_path, mc_version.clone()).await {
				Ok(texture) => match texture {
					TextureRuntimeMeta::Available(texture) => {
						available_textures.insert(texture.shortpath.clone(), texture);
					}
					TextureRuntimeMeta::Unavailable(texture) => {
						unavailable_textures.insert(texture.shortpath.clone(), texture);
					}
				}
				Err(err) => {
					messages.push(err.to_warning());
				}
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

		Ok(DatasourceRuntimeMeta {
			path: path.into(),
			name,
			description,
			version,
			available_textures,
			unavailable_textures,
			messages
		})
	}
}
