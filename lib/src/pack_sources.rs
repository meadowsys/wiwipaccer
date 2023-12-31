//! root manifest for pack sources (equivalent-ish to pack.mcmeta of a resource pack,
//! i suppose)

use async_trait::async_trait;
use camino::{ Utf8Path, Utf8PathBuf };
use crate::error::{ self, Error, Result };
use crate::ron;
use crate::texture::{ NewTextureOptions, Texture, TEXTURES_DIR };
use crate::util;
use hashbrown::{ HashMap, HashSet };
use semver::{ Version, VersionReq };
use serde::{ Deserialize, Serialize };
use std::process::Stdio;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

pub const SOURCE_META_FILENAME: &str = "pack.wiwimeta";

#[derive(Debug, Deserialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: String,
		pack_id: String,
		description: Option<String>,
		version: Option<String>,
		dependencies: Option<HashMap<String, String>>
	}
}

#[derive(Debug)]
pub struct Source {
	name: String,
	dir: String,
	pack_id: String,
	description: Option<String>,
	version: Version,
	dependencies: HashSet<String>,
	textures: Vec<Texture>
}

#[async_trait]
pub trait DependencyResolver {
	type Dependency: Dependency;
	async fn depedency(&self, name: &str, req: &VersionReq) -> Result<Option<Self::Dependency>>;
}

#[async_trait]
pub trait Dependency {}

impl Source {
	pub async fn new<R, D>(dir: String, dependency_resolver: R) -> Result<Self>
	where
		R: DependencyResolver<Dependency = D>,
		D: Dependency
	{
		if !util::check_is_dir(&dir).await? {
			return Err(Error::PackSourcePathIsNotDir)
		}

		let mut manifest_path = Utf8PathBuf::from(dir.clone());
		manifest_path.push(SOURCE_META_FILENAME);

		let manifest = util::check_for_and_read_manifest(manifest_path.as_str())
			.await?
			.ok_or_else(|| Error::PackSourceDirContainsNoManifest)?;
		let (name, pack_id, description, version, dependencies) = match manifest {
			MetaFile::Version1 { name, pack_id, description, version, dependencies } => {
				let dependencies = dependencies.unwrap_or_default();
				(name, pack_id, description, version, dependencies)
			}
		};

		let dependencies = {
			let mut map = HashMap::with_capacity(dependencies.len());

			for (name, req) in dependencies {
				let req = VersionReq::parse(&req)?;
				let dep = dependency_resolver.depedency(&name, &req).await?;
				map.insert(name, dep);
			}

			map
		};

		let version = version.unwrap_or_else(|| "unknown".into());
		let version = Version::parse(&version)?;

		// TODO: this will take the dependencies and use it to resolve things
		let textures = read_textures(&dir)
			.await?;

		let dependencies = dependencies.into_keys().collect();

		Ok(Source { name, dir, pack_id, description, version, dependencies, textures })
	}

	#[inline]
	pub fn name(&self) -> &str {
		&self.name
	}

	#[inline]
	pub fn version(&self) -> &Version {
		&self.version
	}
}

async fn read_textures(dir: &str) -> Result<Vec<Texture>> {
	let mut textures_dir = Utf8PathBuf::from(dir.to_owned());
	textures_dir.push(TEXTURES_DIR);

	let mut dir_contents = fs::read_dir(&textures_dir)
		.await
		.map_err(|source| Error::FileIOError { source, path: textures_dir.to_string() })?;
	let mut textures = vec![];

	while let Some(entry) = {
		dir_contents
			.next_entry()
			.await
			.map_err(|source| Error::FileIOError { source, path: textures_dir.to_string() })?
	} {
		let texture_id = entry.file_name()
			.to_str()
			.ok_or_else(|| Error::NonUTF8PathsUnsupported)?
			.into();

		let options = NewTextureOptions {
			root_dir: dir.into(),
			texture_id
		};

		if let Some(texture) = Texture::new(options).await? {
			textures.push(texture);
		}
	}

	Ok(textures)
}
