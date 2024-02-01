use ::std::result::Result as StdResult;
use ::thiserror::Error;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("unknown minecraft version: {0}")]
	UnknownMCVersion(String)
}
