use ::std::future::Future;
use ::std::result::Result as StdResult;
use ::thiserror::Error;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("non UTF-8 paths are not supported")]
	NonUtf8Path,

	#[error(transparent)]
	SerdeJsonError(#[from] ::serde_json::Error),

	#[error(transparent)]
	SurrealDBError(#[from] ::surrealdb::Error),

	#[error(transparent)]
	TauriError(#[from] ::tauri::Error),

	#[error(transparent)]
	WorkspaceError(#[from] ::wiwipaccer_core::workspace::error::Error)
}

pub type ResultStringErr<T> = std::result::Result<T, String>;
#[inline]
pub async fn string_error<F, T>(future: F) -> ResultStringErr<T>
where
	F: Future<Output = Result<T>>
{
	future.await.map_err(|e| e.to_string())
}
