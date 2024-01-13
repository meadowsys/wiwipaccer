use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("background task failed:\n{0}")]
	BackgroundTaskFailed(#[source] tokio::task::JoinError),

	#[error("FS error:\n{0}")]
	FSError(#[source] std::io::Error),

	#[error("provided path for {path_name} is not a dir: {path}")]
	PathIsNotDir {
		path: String,
		path_name: String
	},
	#[error("provided path for {path_name} is not a file: {path}")]
	PathIsNotFile {
		path: String,
		path_name: String
	},

	#[error("error parsing ron:\n{0}")]
	RonError(#[from] ron::error::Error),

	#[error("error parsing ron:\n{0}")]
	RonSpannedError(#[from] ron::error::SpannedError),

	#[error("error parsing utf-8 text: {source}")]
	Utf8Error {
		source: std::str::Utf8Error,
		bytes: Vec<u8>
	}
}

impl Error {
	#[inline]
	pub fn is_wrong_type_error(&self) -> bool {
		use Error::*;
		matches!(self, PathIsNotDir { .. } | PathIsNotFile { .. })
	}
}
