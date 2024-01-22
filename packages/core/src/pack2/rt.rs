use crate::texture2;
use crate::util::{ fs, path_builder2 };
use crate::util::path_builder::Root;
use super::error::*;
use super::{ meta, nm, nr };
use ::async_trait::async_trait;
use ::hashbrown::HashMap;
use ::nominal::Dummy;

pub struct PackRuntime {
	name: nr::Name,
	description: nr::Description,
	id: nr::ID,
	dir: nr::Dir,
	version: nr::Version,
	dependencies: nr::Dependencies,
	textures: nr::Textures
}

#[async_trait]
pub trait DependencyResolver {
	type Dependency: Dependency;
	async fn dependency(
		&self,
		id: &nr::ID,
		version_req: &::semver::VersionReq
	) -> Result<DependencyResult<Self::Dependency>>;
}

#[async_trait]
pub trait Dependency {}

pub enum DependencyResult<D> {
	/// Found and satisfies version requirement
	Found(D),

	/// Not available
	NotFound,

	/// Found, but doesn't satisfy the version requirement.
	///
	/// The version is included in this enum variant so
	/// it can be included in an error message
	VersionNotSatisfied(::semver::Version)
}

impl PackRuntime {
	pub(crate) async fn new<R, D>(dir: &str, dep_resolver: R)
		-> Result<Self>
	where
		R: DependencyResolver<Dependency = D>,
		D: Dependency
	{
		let p = path_builder2(dir);
		let dir = p.root_dir2().await?;
		let meta_path = p.root_manifest2().await?;

		let meta_file = fs::read_to_string2(meta_path).await?;
		let meta::PackUnversioned {
			name,
			description,
			id,
			version,
			dependencies
		} = meta::deserialise_pack(&meta_file)?;

		let name = name.transmute_nom();
		let description = description.transmute_nom();
		let id = id.transmute_nom();
		let dir = nr::Dir::new(dir);
		let version = process_version(version)?;
		let dependencies = process_deps(dependencies, &dep_resolver).await?;

		// TODO: do something with dependencies in hashmap when actual logic is in
		// I think we'll need to pass to read_textures to process it
		let dependencies = dependencies.into_iter()
			.map(|(id, (_, req))| (id, req))
			.collect();
		let dependencies = nr::Dependencies::new(dependencies);

		let textures = read_textures(&p).await?;

		Ok(Self { name, description, id, dir, version, dependencies, textures })
	}
}
