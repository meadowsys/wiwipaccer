use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	UtilError(#[from] crate::util::error::Error)
}
