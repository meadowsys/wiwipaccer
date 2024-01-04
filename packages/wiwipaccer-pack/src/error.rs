use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[repr(transparent)]
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error(pub(crate) ErrorInner);

#[derive(Debug, Error)]
pub(crate) enum ErrorInner {
	#[error("expected source dir to be a dir:\n{0}")]
	SourceDirIsNotDir(String),

	#[error("expected meta file to be a file:\n{0}")]
	MetaFileIsNotFile(String),

	#[error(transparent)]
	UtilError(#[from] wiwipaccer_util::error::Error)
}
