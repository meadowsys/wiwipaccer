// TODO: remove this when more done
#![allow(dead_code, unused_imports, unused_variables)]

pub mod error;

use crate::error::*;
use ::async_trait::async_trait;
use ::camino::Utf8PathBuf;
use ::hashbrown::HashMap;
use ::serde::{ Deserialize, Serialize };
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
	dependencies: nom::Dependencies
}

pub const PACK_META_FILENAME: &str = "pack.wiwimeta";

::nominal::nominal_mod! {
	pub mod meta_nom {
		nominal!(pub Name, inner: String);
		nominal!(pub PackID, inner: String);
		nominal!(pub DescriptionOptional, inner: Option<String>);
		nominal!(pub VersionOptional, inner: Option<String>);
		nominal!(pub VersionReq, inner: String);
		nominal!(pub DependenciesOptional, inner: Option<HashMap<PackID, VersionReq>>);
	}

	pub mod nom {
		nominal!(pub Name, inner: String);
		nominal!(pub NameBorrowed, inner: ref <'h> &'h str);

		nominal!(pub Dir, inner: String);
		nominal!(pub DirBorrowed, inner: ref<'h> &'h str);

		nominal!(pub PackID, inner: String);
		nominal!(pub PackIDBorrowed, inner: ref <'h> &'h str);

		nominal!(pub DescriptionOptional, inner: Option<String>);
		nominal!(pub DescriptionOptionalBorrowed, inner: ref <'h> Option<&'h str>);
		nominal!(pub DescriptionUnwrapped, inner: String);

		nominal!(pub VersionOptional, inner: Option<semver::Version>);
		nominal!(pub VersionOptionalBorrowed, inner: ref <'h> Option<&'h semver::Version>);

		nominal!(pub VersionReq, inner: semver::VersionReq);
		nominal!(pub VersionReqBorrowed, inner: ref <'h> &'h semver::VersionReq);

		nominal!(pub Dependencies, inner: HashMap<PackID, VersionReq>);
		nominal!(pub DependenciesBorrowed, inner: ref <'h> &'h HashMap<PackID, VersionReq>);
	}
}

#[async_trait]
pub trait DependencyResolver {
	type Dependency: Dependency;
	async fn dependency(
		&self,
		pack_id: nom::PackIDBorrowed<'_>,
		version_req: nom::VersionReqBorrowed<'_>
	) -> Result<Option<Self::Dependency>>;
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
				let description = nom::DescriptionOptional::new(description.into_inner());
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

			for (id, req) in dependencies {
				let req = semver::VersionReq::parse(req.ref_inner())
					.map_err(Into::into)
					.map_err(Error)?;
				let borrowed_id = nom::PackIDBorrowed::new(id.ref_inner());
				let borrowed_req = nom::VersionReqBorrowed::new(&req);

				let dep = dep_resolver.dependency(borrowed_id, borrowed_req).await?;

				let id = nom::PackID::new(id.into_inner());
				map.insert(id, (dep, req));
			}

			map
		};

		let dependencies = dependencies.into_iter()
			.map(|(id, (_, req))| (
				id,
				nom::VersionReq::new(req)
			))
			.collect();
		let dependencies = nom::Dependencies::new(dependencies);

		Ok(Pack { name, dir, pack_id, description, version, dependencies })
	}
}

impl Pack {
	#[inline]
	pub fn name(&self) -> nom::NameBorrowed {
		nom::NameBorrowed::new(self.name.ref_inner())
	}

	#[inline]
	pub fn dir(&self) -> nom::DirBorrowed {
		nom::DirBorrowed::new(self.dir.ref_inner())
	}

	#[inline]
	pub fn pack_id(&self) -> nom::PackIDBorrowed {
		nom::PackIDBorrowed::new(self.pack_id.ref_inner())
	}

	#[inline]
	pub fn optional_description(&self) -> nom::DescriptionOptionalBorrowed {
		nom::DescriptionOptionalBorrowed::new(self.description.ref_inner().as_deref())
	}

	#[inline]
	pub fn unwrap_description(&self) -> nom::DescriptionUnwrapped {
		let description = self.description
			.clone()
			.into_inner()
			.unwrap_or_else(|| "no description provided".into());
		nom::DescriptionUnwrapped::new(description)
	}

	#[inline]
	pub fn optional_version(&self) -> nom::VersionOptionalBorrowed {
		nom::VersionOptionalBorrowed::new(self.version.ref_inner().as_ref())
	}

	#[inline]
	pub fn dependencies(&self) -> nom::DependenciesBorrowed {
		nom::DependenciesBorrowed::new(self.dependencies.ref_inner())
	}

	#[inline]
	pub fn has_dependencies(&self) -> bool {
		!self.dependencies.ref_inner().is_empty()
	}
}
