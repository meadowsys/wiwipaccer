use ::thiserror::Error;
use ::std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("non UTF-8 paths are not supported")]
	NonUtf8Path,

	#[error(transparent)]
	OptionError(#[from] crate::option2::Error),

	#[error(transparent)]
	UtilError(#[from] crate::util::error::Error)
}
