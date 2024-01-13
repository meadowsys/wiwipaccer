use ::std::future::Future;
use ::thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
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
