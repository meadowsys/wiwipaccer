use ::std::result::Result as StdResult;
use ::thiserror::Error;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, Error)]
#[deprecated]
pub enum Error {
	#[error("only absolute paths are allowed, provided path is relative:\n{0}")]
	AbsolutePathOnly(String),

	#[error(transparent)]
	PackError(#[from] crate::pack::error::Error),

	#[error(transparent)]
	UtilError(#[from] crate::util::error::Error)
}
