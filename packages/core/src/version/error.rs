use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("unknown minecraft version: {0}")]
	UnknownMCVersions(String),

	#[error(transparent)]
	UtilError(#[from] crate::util::error::Error)
}
