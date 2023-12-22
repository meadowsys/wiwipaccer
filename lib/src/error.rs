#![warn(unused)]

use camino::{ Utf8PathBuf, Utf8Path };

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("failed to spawn child process using command \"{command}\":\n{source}")]
	ChildProcessFailedToSpawn {
		source: std::io::Error,
		command: String
	},

	#[error("failed to spawn child process getting version information using command \"{command}\":\n{source}")]
	ChildProcessFailedToSpawnForGitVersioning {
		source: std::io::Error,
		command: String
	},

	#[error("error doing IO on file at path \"{path}\":\n{source}")]
	FileIOError {
		source: std::io::Error,
		path: Utf8PathBuf
	},

	#[error("invalid utf-8:\n{0}")]
	InvalidUTF8(#[from] std::string::FromUtf8Error),

	/// TODO: this should have a better error message
	#[error("non UTF-8 paths are not supported")]
	NonUTF8PathsUnsupported,

	#[error("error reading pack sources manifest file at path \"{path}\":\n{source}")]
	PackSourcesManifestReadError {
		source: std::io::Error,
		path: Utf8PathBuf
	},

	#[error("error reading pack sources directory at path \"{path}\":\n{source}")]
	PackSourcesDirReadError {
		source: std::io::Error,
		path: Utf8PathBuf
	},

	#[error("provided pack source path is invalid (not a directory or doesn't exist)")]
	PackSourcePathIsNotDir,

	#[error(
		"provided pack source path is invalid (manifest file (\"{}\") is not a file or doesn't exist?)",
		crate::pack_sources::SOURCE_META_FILENAME
	)]
	PackSourceDirContainsNoManifest,

	#[error("error parsing ron:\n{source}")]
	RonSpannedError {
		#[from]
		source: ron::error::SpannedError
	}
}

pub fn file_io_error(path: &Utf8Path)
	-> impl FnOnce(std::io::Error) -> Error + '_
{
	|source| Error::FileIOError { source, path: path.into() }
}
