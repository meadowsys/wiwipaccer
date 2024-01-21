use super::error::*;
use super::nr;
use ::async_trait::async_trait;

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
		pack_id: &nr::ID,
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
	VersionNotSatisfied(semver::Version)
}

impl PackRuntime {
	pub(crate) async fn new<R, D>(dir: nr::Dir, dep_resolver: R)
		-> Result<Self>
	where
		R: DependencyResolver<Dependency = D>
	{
		todo!()
	}
}
