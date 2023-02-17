use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("assets path is not a directory, path `{assets_path}`")]
	AssetsIsNotDir {
		assets_path: String
	},
	#[error("IO error: {source}")]
	IOError {
		source: std::io::Error
	},
	#[error("Parsing error")]
	ParseErrorRonSpannedError {
		path: String,
		source: ron::error::SpannedError
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self {
		Self::IOError { source: value }
	}
}
