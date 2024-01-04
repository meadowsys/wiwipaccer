use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[repr(transparent)]
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error(pub(crate) ErrorInner);

#[derive(Debug, Error)]
pub(crate) enum ErrorInner {
	#[error("background task failed: {0}")]
	BackgroundTaskFailed(#[source] tokio::task::JoinError),

	#[error("FS error: {0}")]
	FSError(#[source] std::io::Error),

	#[error("error parsing utf-8 text: {source}")]
	Utf8Error {
		source: std::str::Utf8Error,
		bytes: Vec<u8>
	}
}
