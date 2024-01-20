use thiserror::Error;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	UtilError(#[from] crate::util::error::Error)
}
