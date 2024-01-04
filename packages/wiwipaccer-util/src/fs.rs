use crate::error::*;
use ::std::fs;
use ::std::io::Read as _;

::nominal::nominal_mod! {
	pub mod nom {
		nominal!(pub Path, inner: String);
		nominal!(pub FileSize, inner: usize);
	}
}

#[inline]
pub async fn metadata(path: nom::Path) -> Result<fs::Metadata> {
	let f = || fs::metadata(path.into_inner())
		.map_err(|e| Error(ErrorInner::FSError(e)));
	spawn_blocking(f).await
}

// TODO: can probably be optimised (one less meta call?) if rewritten by hand?
pub async fn read_to_string(path: nom::Path) -> Result<String> {
	let f = || fs::read(path.into_inner())
		.map_err(|e| Error(ErrorInner::FSError(e)));
	let bytes = spawn_blocking(f).await?;

	match std::str::from_utf8(&bytes) {
		Ok(_) => { Ok(unsafe { String::from_utf8_unchecked(bytes) }) }
		Err(source) => { Err(Error(ErrorInner::Utf8Error { source, bytes })) }
	}
}

#[inline]
async fn spawn_blocking<F, T>(f: F) -> Result<T>
where
	F: FnOnce() -> Result<T> + Send + 'static,
	T: Send + 'static
{
	match ::tokio::task::spawn_blocking(f).await {
		Ok(r) => { r }
		Err(e) => { Err(Error(ErrorInner::BackgroundTaskFailed(e))) }
	}
}
