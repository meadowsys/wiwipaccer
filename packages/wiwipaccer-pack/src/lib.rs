// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::error::*;
use ::async_trait::async_trait;
use ::camino::Utf8PathBuf;
use ::hashbrown::HashMap;
use ::serde::{ Deserialize, Serialize };
use ::wiwipaccer_texture as textures;
use ::wiwipaccer_util::{ fs, ron };

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "meta_version")]
enum MetaFile {
	#[serde(rename = "1")]
	Version1 {
		name: meta_nom::Name,
		pack_id: meta_nom::PackID,
		description: meta_nom::DescriptionOptional,
		version: meta_nom::VersionOptional,
		dependencies: meta_nom::DependenciesOptional
	}
}

#[derive(Debug)]
pub struct Pack {
	name: nom::Name,
	dir: nom::Dir,
	pack_id: nom::PackID,
	description: nom::DescriptionOptional,
	version: nom::VersionOptional,
	dependencies: nom::Dependencies,
	textures: nom::Textures
}

pub const PACK_META_FILENAME: &str = "pack.wiwimeta";

::nominal::nominal_mod! {
	pub mod meta_nom {
		nominal!(pub Name, inner: String);
		nominal!(pub PackID, inner: String);
		nominal!(pub Description, inner: String);
		nominal!(pub DescriptionOptional, inner: Option<Description>);
		nominal!(pub VersionOptional, inner: Option<String>);
		nominal!(pub VersionReq, inner: String);
		nominal!(pub DependenciesOptional, inner: Option<HashMap<PackID, VersionReq>>);
	}

	pub mod nom {
		nominal!(pub Name, inner: String);
		nominal!(pub Dir, inner: String);
		nominal!(pub PackID, inner: String);
		nominal!(pub Description, inner: String);
		nominal!(pub DescriptionOptional, inner: Option<Description>);
		nominal!(pub VersionOptional, inner: Option<semver::Version>);
		nominal!(pub VersionReq, inner: semver::VersionReq);
		nominal!(pub Dependencies, inner: HashMap<PackID, VersionReq>);
		nominal!(pub Textures, inner: HashMap<textures::nom::TextureID, textures::Texture>);
	}
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
		pack_id: &nom::PackID,
		version_req: &nom::VersionReq
	) -> Result<DependencyResult<Self::Dependency>>;
}

#[async_trait]
pub trait Dependency {}

impl Pack {
	pub async fn new<R, D>(dir: nom::Dir, dep_resolver: R)
		-> Result<Self>
	where
		R: DependencyResolver<Dependency = D>,
		D: Dependency
	{
		let path = fs::nom::Path::new(dir.clone().into_inner());
		let dir_metadata = fs::metadata(path)
			.await
			.map_err(Into::into)
			.map_err(Error)?;
		if !dir_metadata.is_dir() { return Err(Error(ErrorInner::PackDirIsNotDir(dir.into_inner()))) }

		let mut meta_path = Utf8PathBuf::from(dir.ref_inner());
		meta_path.push(PACK_META_FILENAME);

		let meta_metadata = fs::metadata(fs::nom::Path::new(meta_path.as_str().into()))
			.await
			.map_err(Into::into)
			.map_err(Error)?;
		if !meta_metadata.is_file() { return Err(Error(ErrorInner::MetaFileIsNotFile(meta_path.as_str().into()))) }

		let meta_file = fs::read_to_string(fs::nom::Path::new(meta_path.as_str().into()))
			.await
			.map_err(Into::into)
			.map_err(Error)?;

		let meta_file = ron::from_str(&meta_file)
			.map_err(Into::into)
			.map_err(Error)?;

		let (name, pack_id, description, version, dependencies) = match meta_file {
			MetaFile::Version1 { name, pack_id, description, version, dependencies } => {
				let name = nom::Name::new(name.into_inner());
				let pack_id = nom::PackID::new(pack_id.into_inner());
				let description = nom::DescriptionOptional::new(
					description.into_inner()
						.map(|d| nom::Description::new(d.into_inner()))
				);
				let version = version.into_inner()
					.as_deref()
					.map(semver::Version::parse)
					.transpose()
					.map_err(Into::into)
					.map_err(Error)?;
				let version = nom::VersionOptional::new(version);
				let dependencies = dependencies.into_inner().unwrap_or_default();

				(name, pack_id, description, version, dependencies)
			}
		};

		let dependencies = {
			let mut map = HashMap::with_capacity(dependencies.len());
			let mut not_satisfied = Vec::with_capacity(dependencies.len());

			for (id, req) in dependencies {
				let id = nom::PackID::new(id.into_inner());
				let req = semver::VersionReq::parse(req.ref_inner())
					.map_err(Into::into)
					.map_err(Error)?;
				let req = nom::VersionReq::new(req);

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

				let id = nom::PackID::new(id.into_inner());
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
		let dependencies = nom::Dependencies::new(dependencies);

		let textures = {
			let mut textures_dir = Utf8PathBuf::from(dir.ref_inner());
			textures_dir.push(textures::TEXTURES_DIR);

			let mut read_dir = fs::read_dir(fs::nom::Path::new(textures_dir.as_str().into()))
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

				let texture_id = textures::nom::TextureID::new(texture_id.into());
				let root_dir = textures::nom::RootDir::new(dir.clone().into_inner());

				let texture = textures::Texture::new(root_dir, texture_id.clone())
					.await
					.map_err(Into::into)
					.map_err(Error)?;
				if let Some(texture) = texture {
					t.insert(texture_id, texture);
				}
			}

			nom::Textures::new(t)
		};

		Ok(Pack { name, dir, pack_id, description, version, dependencies, textures })
	}
}

impl Pack {
	#[inline]
	pub fn name(&self) -> &nom::Name {
		&self.name
	}

	#[inline]
	pub fn dir(&self) -> &nom::Dir {
		&self.dir
	}

	#[inline]
	pub fn pack_id(&self) -> &nom::PackID {
		&self.pack_id
	}

	#[inline]
	pub fn optional_description(&self) -> &nom::DescriptionOptional {
		&self.description
	}

	#[inline]
	pub fn unwrap_description(&self) -> nom::Description {
		self.description
			.clone()
			.into_inner()
			.unwrap_or_else(|| nom::Description::new("no description provided".into()))
	}

	#[inline]
	pub fn optional_version(&self) -> &nom::VersionOptional {
		&self.version
	}

	#[inline]
	pub fn dependencies(&self) -> &nom::Dependencies {
		&self.dependencies
	}

	#[inline]
	pub fn has_dependencies(&self) -> bool {
		!self.dependencies.ref_inner().is_empty()
	}
}
