use ::std::result::Result as StdResult;
use ::thiserror::Error;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, Error)]
#[deprecated]
pub enum Error {
	#[error("non UTF-8 paths are not supported")]
	NonUtf8Path,

	#[error(transparent)]
	UtilError(#[from] crate::util::error::Error),

	#[error(transparent)]
	VersionError(#[from] crate::version::error::Error)
}
