use camino::Utf8PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("reading manifest file (`{path}`) failed: {e}")]
	ManifestReadFailed {
		e: std::io::Error,
		path: Utf8PathBuf
	},
	#[error("item at manifest path is not a file: {path}")]
	ManifestNotFile {
		path: Utf8PathBuf
	},
	#[error("parsing manifest file (`{path}`) failed: {e}")]
	ManifestParseError {
		e: ron::error::SpannedError,
		path: Utf8PathBuf
	}
}
