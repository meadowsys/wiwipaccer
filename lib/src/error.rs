use crate::runtime_meta::Warning;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Assets path is not a directory: {path}")]
	AssetsPathIsNotDir {
		path: String
	},
	#[error("IO Error (does the file/directory at this path exist, and does wiwipaccer have access to it?): path {path}, filesystem error: {source}")]
	FileDoesNotExist {
		path: String,
		source: std::io::Error
	},
	#[error("IO error: {source}")]
	IOError {
		source: std::io::Error
	},
	#[error("Invalid block ID: {id}")]
	InvalidBlockID {
		id: String
	},
	#[error("Ron parsing error for path {path}: {source}")]
	ParseErrorRonSpannedError {
		path: String,
		source: ron::error::SpannedError
	}
	// #[error("Path is not a directory: {path}")]
	// PathIsNotDir {
	// 	path: String
	// }
}

impl Error {
	pub fn into_warning(self) -> Warning {
		Warning { message: self.to_string() }
	}
}
