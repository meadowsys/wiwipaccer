// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::nom as n;
use crate::util::{ fs, path_builder, ron };
use error::*;
use ::async_trait::async_trait;
use ::camino::Utf8PathBuf;
use ::hashbrown::HashMap;
use ::serde::{ Deserialize, Serialize };
use super::texture;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: n::pack::Name,
		description: n::pack::Description,
		pack_id: n::pack::ID,
		version: n::pack_m::Version,
		dependencies: n::pack_m::Dependencies
	}
}

#[derive(Debug)]
pub struct Pack {
	name: n::pack::Name,
	description: n::pack::Description,
	pack_id: n::pack::ID,
	version: n::pack::Version,
	dependencies: n::pack::Dependencies,
	root_dir: n::global::RootDirPath,
	textures: n::pack::Textures
}

pub enum DependencyResult<D> {
	/// Found and satisfies version requirement
	Found(D),
	/// Not available at all
	NotFound,
	/// Found, but doesn't satisfy the version requirement. The version
	/// is included in this enum variant so it can be included in
	/// an error message
	VersionNotSatisfied(semver::Version)
}

#[async_trait]
pub trait DependencyResolver {
	type Dependency: Dependency;
	async fn dependency(
		&self,
		pack_id: &n::pack::ID,
		version_req: &semver::VersionReq
	) -> Result<DependencyResult<Self::Dependency>>;
}

#[async_trait]
pub trait Dependency {}

impl Pack {
	pub(crate) async fn new<R, D>(dir: n::global::DirPath, dep_resolver: R)
		-> Result<Self>
	where
		R: DependencyResolver<Dependency = D>,
		D: Dependency
	{
		let root_dir = n::global::RootDirPath::new(dir.into_inner());
		let p = path_builder(&root_dir);

		let root_dir = p.root_dir()
			.await?;

		let meta_path = p.root_manifest()
			.await?;

		let meta_file = fs::read_to_string(n::global::FilePath::new(meta_path.into_inner()))
			.await?;

		let meta_file = ron::from_str(&meta_file)?;

		let (name, pack_id, description, version, dependencies) = match meta_file {
			MetaFile::Version1 { name, pack_id, description, version, dependencies } => {
				let name = n::pack::Name::new(name.into_inner());
				let pack_id = n::pack::ID::new(pack_id.into_inner());
				let description = n::pack::Description::new(description.into_inner());
				let version = version.into_inner()
					.as_deref()
					.map(semver::Version::parse)
					.transpose()?;
				let version = n::pack::Version::new(version);
				let dependencies = dependencies.into_inner().unwrap_or_default();

				(name, pack_id, description, version, dependencies)
			}
		};

		let dependencies = {
			let mut map = HashMap::with_capacity(dependencies.len());
			let mut not_satisfied = Vec::with_capacity(dependencies.len());

			for (id, req) in dependencies {
				let id = n::pack::ID::new(id.into_inner());
				let req = semver::VersionReq::parse(req.ref_inner())?;

				let dep = match dep_resolver.dependency(&id, &req).await? {
					DependencyResult::Found(d) => { d }
					DependencyResult::VersionNotSatisfied(v) => {
						not_satisfied.push((id, req, Some(v)));
						continue
					}
					DependencyResult::NotFound => {
						not_satisfied.push((id, req, None));
						continue
					}
				};

				let id = n::pack::ID::new(id.into_inner());
				map.insert(id, (dep, req));
			}

			if !not_satisfied.is_empty() {
				return Err(Error::DepsNotSatisfied(not_satisfied))
			}

			map
		};

		let dependencies = dependencies.into_iter()
			.map(|(id, (_, req))| (id, req))
			.collect();
		let dependencies = n::pack::Dependencies::new(dependencies);

		let textures = {
			let textures_dir = p.textures_path()
				.await?;
			let mut textures_nom = n::pack::Textures::default();
			let textures = textures_nom.mut_inner();

			let mut read_dir = fs::read_dir(n::global::DirPath::new(textures_dir.clone().into_inner()))
				.await?;

			while let Some(file) = read_dir.next().await? {
				let texture_id = file.file_name();
				let texture_id = texture_id.to_str()
					.ok_or_else(|| Error::NonUtf8Path)?;
				let texture_id = n::texture::ID::new(texture_id.into());

				let texture = texture::Texture::new(root_dir.clone(), texture_id.clone())
					.await?;

				if let Some(texture) = texture {
					textures.insert(texture_id, texture);
				}
			}

			textures_nom
		};

		Ok(Pack { name, description, pack_id, version, dependencies, root_dir, textures })
	}
}

impl Pack {
	#[inline]
	pub fn name(&self) -> &n::pack::Name {
		&self.name
	}

	#[inline]
	pub fn root_dir(&self) -> &n::global::RootDirPath {
		&self.root_dir
	}

	#[inline]
	pub fn pack_id(&self) -> &n::pack::ID {
		&self.pack_id
	}

	#[inline]
	pub fn optional_description(&self) -> &n::pack::Description {
		&self.description
	}

	#[inline]
	pub fn unwrap_description(&self) -> n::pack::DescriptionUnwrapped {
		let description = self.description
			.clone()
			.into_inner()
			.unwrap_or_else(|| "no description provided".into());

		n::pack::DescriptionUnwrapped::new(description)
	}

	#[inline]
	pub fn optional_version(&self) -> &n::pack::Version {
		&self.version
	}

	#[inline]
	pub fn dependencies(&self) -> &n::pack::Dependencies {
		&self.dependencies
	}

	#[inline]
	pub fn has_dependencies(&self) -> bool {
		!self.dependencies.ref_inner().is_empty()
	}
}
