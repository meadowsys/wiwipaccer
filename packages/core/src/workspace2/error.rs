use ::thiserror::Error;
use ::std::result::Result as StdResult;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("only absolute paths are allowed, provided path is relative:\n{0}")]
	AbsolutePathOnly(String),

	#[error("pack with duplicate id of \"{0}\"")]
	DuplicateID(String),

	#[error(transparent)]
	PackError(#[from] crate::pack2::Error),

	#[error(transparent)]
	UtilError(#[from] crate::util::error::Error)
}
