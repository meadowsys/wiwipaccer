use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("non UTF-8 paths are not supported")]
	NonUtf8Path,

	#[error(transparent)]
	OptionError(#[from] crate::option::error::Error),

	#[error(transparent)]
	UtilError(#[from] crate::util::error::Error)
}
