use crate::runtime_meta::Warning;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("IO error: {source}")]
	IOError {
		source: std::io::Error
	},
	#[error("Parsing error")]
	ParseErrorRonSpannedError {
		path: String,
		source: ron::error::SpannedError
	},
	#[error("Path is not a directory: {path}")]
	PathIsNotDir {
		path: String
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self {
		Self::IOError { source: value }
	}
}

impl Error {
	pub fn into_warning(self) -> Result<Warning> {
		use Error::*;

		match self {
			IOError { source } => { Err(IOError { source }) }
			ParseErrorRonSpannedError { path, source } => { Err(ParseErrorRonSpannedError { path, source }) }
			PathIsNotDir { path } => {
				Ok(Warning {
					message: format!("Path is not a directory: {path}")
				})
			}
		}
	}
}
