#![warn(unused)]

use camino::Utf8PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("error doing IO on file at path \"{path}\":\n{source}")]
	FileIOError {
		path: Utf8PathBuf,
		source: std::io::Error
	},

	#[error("invalid utf-8:\n{0}")]
	InvalidUTF8(#[from] std::string::FromUtf8Error),

	#[error("error reading datasource manifest file at path \"{path}\":\n{source}")]
	SourceManifestReadError {
		path: Utf8PathBuf,
		source: std::io::Error
	},

	#[error("error reading source directory at path \"{path}\":\n{source}")]
	SourceDirReadError{
		path: Utf8PathBuf,
		source: std::io::Error
	},

	#[error("error parsing ron:\n{source}")]
	RonSpannedError {
		#[from]
		source: ron::error::SpannedError
	}
}
