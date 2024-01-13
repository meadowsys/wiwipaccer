use ::std::future::Future;
use ::thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[repr(transparent)]
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error(pub(crate) ErrorInner);

#[derive(Debug, Error)]
pub(crate) enum ErrorInner {
	#[error(transparent)]
	SurrealDBError(#[from] surrealdb::Error),

	#[error("non UTF-8 paths are not supported")]
	NonUtf8Path
}

pub type ResultStringErr<T> = std::result::Result<T, String>;
#[inline]
pub async fn string_error<F, T, R>(f: F) -> ResultStringErr<T>
where
	F: FnOnce() -> R,
	R: Future<Output = Result<T>>
{
	f().await.map_err(|e| e.to_string())
}

pub(crate) fn into_err<E>(error: E) -> Error
where
	E: Into<ErrorInner>
{
	Error(error.into())
}
