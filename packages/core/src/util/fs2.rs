use crate::error::*;
use crate::nom as n;
use ::std::fs;
use ::std::io::Read as _;

// pub async fn metadata<F>(path_fn: F) -> Result<fs::Metadata, fs_err::Metadata>
// where
// 	F: Fn() -> String + Clone + Send + 'static
// {
// 	let path = path_fn();
// 	let _path_fn = path_fn.clone();
// 	spawn_blocking(
// 		|| fs::metadata(path).map_err(fs_err::metadata_fs_err(_path_fn)),
// 		fs_err::metadata_join_err(path_fn)
// 	).await
// }
pub async fn metadata<F>(path_fn: F) -> Result<fs::Metadata, fs_err::Metadata>
where
	F: Fn() -> String + Send
{
	let path = path_fn();
	spawn_blocking(
		|| fs::metadata(path).map_err(fs_err::metadata_without_path_fs),
		fs_err::metadata_without_path_join
	).await.map_err(|e| e.with_path(path_fn()))
}

#[inline]
pub async fn is_dir<F>(path_fn: F) -> Result<bool, fs_err::IsDir>
where
	F: Fn() -> String + Send + Clone
{
	metadata(path_fn.clone()).await
		.map(|m| m.is_dir())
		.map_err(|e| fs_err::is_dir(e, path_fn()))
	// Ok(metadata(path_fn).await?.is_dir())
}


#[inline]
pub async fn is_file2<F>(path_fn: F) -> Result<bool, fs_err::IsFile>
where
	F: Fn() -> String + Send + Clone
{
	metadata(path_fn.clone()).await
		.map(|m| m.is_file())
		.map_err(|e| fs_err::is_file(e, path_fn()))
	// Ok(metadata(path_fn).await?.is_file())
}

// #[inline]
// pub async fn read_to_string(path: n::global::FilePath) -> Result<String> {
// 	read_to_string2(path.into_inner()).await
// }

// // TODO: can probably be optimised (one less meta call?) if rewritten by hand?
// pub async fn read_to_string2(path: String) -> Result<String> {
// 	let f = || fs::read(path)
// 		.map_err(Error::FSError);
// 	let bytes = spawn_blocking(f).await?;

// 	match std::str::from_utf8(&bytes) {
// 		Ok(_) => { Ok(unsafe { String::from_utf8_unchecked(bytes) }) }
// 		Err(source) => { Err(Error::Utf8Error { source, bytes }) }
// 	}
// }

// #[inline]
// pub async fn read_dir(path: n::global::DirPath) -> Result<ReadDir> {
// 	read_dir2(path.into_inner()).await
// }

// #[inline]
// pub async fn read_dir2(path: String) -> Result<ReadDir> {
// 	tokio::fs::read_dir(path)
// 		.await
// 		.map(ReadDir)
// 		.map_err(Error::FSError)
// }

async fn spawn_blocking<T, E, F, EF>(f: F, e_fn: EF) -> Result<T, E>
where
	T: Send + 'static,
	E: Send + 'static,
	F: FnOnce() -> Result<T, E> + Send + 'static,
	EF: FnOnce(fs_err::SpawnBlocking) -> E
{
	match ::tokio::task::spawn_blocking(f).await {
		Ok(r) => { r }
		Err(e) => { Err(e_fn(fs_err::spawn_blocking(e))) }
	}
}

// #[repr(transparent)]
// pub struct ReadDir(tokio::fs::ReadDir);

// impl ReadDir {
// 	#[inline]
// 	pub async fn next(&mut self) -> Result<Option<tokio::fs::DirEntry>> {
// 		self.0.next_entry()
// 			.await
// 			.map_err(Error::FSError)
// 	}
// }
