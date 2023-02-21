use crate::runtime_meta::{ Message, MessageSeverity };
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Assets path is not a directory: {path}")]
	AssetsPathIsNotDir {
		path: String
	},
	#[error("Unable to fetch MC Versions from Mojang: {source}")]
	FailedToFetchMCVersions {
		source: reqwest::Error
	},
	#[error("Failed to parse MC versions response from Mojang: {source}")]
	FailedToFetchMCVersionsInvalidUTF8 {
		source: std::string::FromUtf8Error
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
	#[error("Manifest that is supposed to be at path {path} does not exist. source: {source}")]
	ManifestDoesNotExist {
		path: String,
		source: std::io::Error
	},
	#[error("Item at manifest path is not a file: {path}")]
	ManifestIsNotFile {
		path: String
	},
	#[error("Ron parsing error for path {path}: {source}")]
	ParseErrorRonSpannedError {
		path: String,
		source: ron::error::SpannedError
	},
	#[error("Unable to initialise HTTP client: {source}")]
	UnableToInitialiseHttpClient {
		source: reqwest::Error
	}
}

impl Error {
	pub fn to_warning(&self) -> Message {
		use Error::*;

		let severity = match self {
			AssetsPathIsNotDir { .. } => { MessageSeverity::Error }
			FailedToFetchMCVersions { .. } => { MessageSeverity::Fatal }
			FailedToFetchMCVersionsInvalidUTF8 { .. } => { MessageSeverity::Fatal }
			FileDoesNotExist { .. } => { MessageSeverity::Warning }
			IOError { .. } => { MessageSeverity::Warning }
			InvalidBlockID { .. } => { MessageSeverity::Fatal }
			ManifestDoesNotExist { .. } => { MessageSeverity::Warning }
			ManifestIsNotFile { .. } => { MessageSeverity::Warning }
			ParseErrorRonSpannedError { .. } => { MessageSeverity::Fatal }
			UnableToInitialiseHttpClient { .. } => { MessageSeverity::Fatal }
		};

		Message { message: self.to_string(), severity }
	}
}
