use ::std::result::Result as StdResult;
use ::thiserror::Error;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, Error)]
#[deprecated]
pub enum Error {
	#[error("unknown minecraft version: {0}")]
	UnknownMCVersions(String),

	#[error(transparent)]
	UtilError(#[from] crate::util::error::Error)
}
