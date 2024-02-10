use ::thiserror::Error;
use ::std::result::Result as StdResult;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	UtilError(#[from] crate::util::error::Error),

	#[error(transparent)]
	ProviderError(#[from] crate::provider2::Error)
}
