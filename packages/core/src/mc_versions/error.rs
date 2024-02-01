use ::std::result::Result as StdResult;
use ::thiserror::Error;
use ::ts_result::{ NiceErrorMessage, impl_display };

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
	UnknownMCVersion(String)
}

impl NiceErrorMessage for Error {
	fn to_error_message(&self) -> String {
		use Error::*;
		match self {
			UnknownMCVersion(version) => {
				format!("unknown minecraft version: {version}")
			}
		}
	}
}

impl_display!(Error);
