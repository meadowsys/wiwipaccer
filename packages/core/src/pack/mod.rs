// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::nom as n;
use crate::util::{ consts, fs, ron };
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
	pub async fn new<R, D>(dir: n::global::DirPath, dep_resolver: R)
		-> Result<Self>
	where
		R: DependencyResolver<Dependency = D>,
		D: Dependency
	{
		let path = n::global::Path::new(dir.clone().into_inner());
		let dir_metadata = fs::metadata(path)
			.await
			.map_err(Into::into)
			.map_err(Error)?;
		if !dir_metadata.is_dir() { return Err(Error(ErrorInner::PackDirIsNotDir(dir.into_inner()))) }

		let mut meta_path = Utf8PathBuf::from(dir.ref_inner());
		meta_path.push(consts::PACK_META_FILENAME);

		let meta_metadata = fs::metadata(n::global::Path::new(meta_path.as_str().into()))
			.await
			.map_err(Into::into)
			.map_err(Error)?;
		if !meta_metadata.is_file() { return Err(Error(ErrorInner::MetaFileIsNotFile(meta_path.as_str().into()))) }

		let meta_file = fs::read_to_string(n::global::FilePath::new(meta_path.as_str().into()))
			.await
			.map_err(Into::into)
			.map_err(Error)?;

		let meta_file = ron::from_str(&meta_file)
			.map_err(Into::into)
			.map_err(Error)?;

		let (name, pack_id, description, version, dependencies) = match meta_file {
			MetaFile::Version1 { name, pack_id, description, version, dependencies } => {
				let name = n::pack::Name::new(name.into_inner());
				let pack_id = n::pack::ID::new(pack_id.into_inner());
				let description = n::pack::Description::new(description.into_inner());
				let version = version.into_inner()
					.as_deref()
					.map(semver::Version::parse)
					.transpose()
					.map_err(Into::into)
					.map_err(Error)?;
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
				let req = semver::VersionReq::parse(req.ref_inner())
					.map_err(Into::into)
					.map_err(Error)?;

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
				return Err(Error(ErrorInner::DepsNotSatisfied(not_satisfied)))
			}

			map
		};

		let dependencies = dependencies.into_iter()
			.map(|(id, (_, req))| (id, req))
			.collect();
		let dependencies = n::pack::Dependencies::new(dependencies);

		let textures = {
			let mut textures_dir = Utf8PathBuf::from(dir.ref_inner());
			textures_dir.push(consts::TEXTURES_DIR);

			// TODO: check textures_dir is actually a dir first

			let mut read_dir = fs::read_dir(n::global::DirPath::new(textures_dir.as_str().into()))
				.await
				.map_err(Into::into)
				.map_err(Error)?;
			let mut t = HashMap::new();

			while let Some(file) = {
				read_dir.next()
					.await
					.map_err(Into::into)
					.map_err(Error)?
			} {
				let texture_id = file.file_name();
				let texture_id = texture_id.to_str()
					.ok_or_else(|| Error(ErrorInner::NonUtf8Path))?;

				let texture_id = n::texture::ID::new(texture_id.into());
				let root_dir = n::global::RootDirPath::new(dir.clone().into_inner());

				let texture = texture::Texture::new(root_dir, texture_id.clone())
					.await
					.map_err(Into::into)
					.map_err(Error)?;
				if let Some(texture) = texture {
					t.insert(texture_id, texture);
				}
			}

			n::pack::Textures::new(t)
		};

		let root_dir = n::global::RootDirPath::new(dir.into_inner());

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
