use crate::runtime_meta::{ Message, MessageSeverity };
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Assets path is not a directory: {path}")]
	AssetsPathIsNotDir {
		path: String
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
	#[error("There are multiple available versions for this: {available_versions_shortnames_formatted}. Please check that they don't overlap in which versions they provide for")]
	MultipleAvailableVersions {
		available_versions_shortnames_formatted: String
	},
	#[error("Ron parsing error for path {path}: {source}")]
	ParseErrorRonSpannedError {
		path: String,
		source: ron::error::SpannedError
	},
	#[error("{thing} is unavailable because {reason}")]
	UnavailableError {
		thing: String,
		reason: String
	},
	#[error("{thing} is unavailable because {reason}")]
	UnavailableInfo {
		thing: String,
		reason: String
	}
}

impl Error {
	pub fn to_warning(&self) -> Message {
		use Error::*;

		let severity = match self {
			AssetsPathIsNotDir { .. } => { MessageSeverity::Error }
			FailedToFetchMCVersionsInvalidUTF8 { .. } => { MessageSeverity::Fatal }
			FileDoesNotExist { .. } => { MessageSeverity::Warning }
			IOError { .. } => { MessageSeverity::Warning }
			InvalidBlockID { .. } => { MessageSeverity::Fatal }
			ManifestDoesNotExist { .. } => { MessageSeverity::Warning }
			ManifestIsNotFile { .. } => { MessageSeverity::Warning }
			MultipleAvailableVersions { .. } => { MessageSeverity::Error }
			ParseErrorRonSpannedError { .. } => { MessageSeverity::Fatal }
			UnavailableError { .. } => { MessageSeverity::Error }
			UnavailableInfo { .. } => { MessageSeverity::Info }
		};

		Message { message: self.to_string(), severity }
	}
}
