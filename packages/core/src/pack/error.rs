use crate::nom as n;
use ::std::result::Result as StdResult;
use ::thiserror::Error;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, Error)]
#[deprecated]
pub enum Error {
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
	DepsNotSatisfied(Vec<(n::pack::ID, ::semver::VersionReq, Option<::semver::Version>)>),

	#[error("non UTF-8 paths are not supported")]
	NonUtf8Path,

	#[error("error parsing semver:\n{0}")]
	SemverParseError(#[from] semver::Error),

	#[error(transparent)]
	TextureError(#[from] crate::texture::error::Error),

	#[error(transparent)]
	UtilError(#[from] crate::util::error::Error)
}
