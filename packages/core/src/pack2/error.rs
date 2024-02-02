use ::thiserror::Error;
use ::std::result::Result as StdResult;
use super::nr;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, Error)]
#[deprecated]
pub enum Error {
	#[error("error resolving dependency: {0}")]
	DependencyResolverError(#[from] Box<dyn std::error::Error + Send>),

	#[error(
		"dependencies not satisfied: {}",
		.0.iter()
			.map(|(id, req, v)| match v {
				Some(v) => { format!("{id} {req} ({v} available)") }
				None => { format!("{id} {req}") }
			})
			.collect::<Vec<_>>()
			.join(", ")
	)]
	DepsNotSatisfied(Vec<(nr::ID, ::semver::VersionReq, Option<::semver::Version>)>),

	#[error("non UTF-8 paths are not supported")]
	NonUtf8Path,

	#[error("error parsing semver:\n{0}")]
	SemverParseError(#[from] ::semver::Error),

	#[error(transparent)]
	TextureError(#[from] crate::texture2::Error),

	#[error(transparent)]
	UtilError(#[from] crate::util::error::Error)
}
