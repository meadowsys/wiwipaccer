use crate::error::*;
use crate::nom as n;
use ::std::fs;
use ::std::io::Read as _;

pub async fn metadata<F>(path_fn: F) -> Result<fs::Metadata, fs_err::MetadataWithPath>
where
	F: Fn() -> String
{
	let path = path_fn();
	spawn_blocking(
		|| fs::metadata(path).map_err(fs_err::metadata_fs),
		fs_err::metadata_join
	).await.map_err(|e| e.with_path(path_fn()))
}

pub async fn is_dir<F>(path_fn: F) -> Result<bool, fs_err::IsDir>
where
	F: Fn() -> String
{
	metadata(path_fn).await
		.map(|m| m.is_dir())
		.map_err(fs_err::is_dir)
}


pub async fn is_file<F>(path_fn: F) -> Result<bool, fs_err::IsFile>
where
	F: Fn() -> String
{
	metadata(path_fn).await
		.map(|m| m.is_file())
		.map_err(fs_err::is_file)
}

// TODO: can probably be optimised (one less meta call?) if rewritten by hand?
pub async fn read_to_string<F>(path_fn: F) -> Result<String, fs_err::ReadToString>
where
	F: Fn() -> String
{
	let path = path_fn();
	let bytes = spawn_blocking(
		|| fs::read(path).map_err(fs_err::read_to_string_fs),
		fs_err::read_to_string_join
	).await?;

	match std::str::from_utf8(&bytes) {
		Ok(_) => { Ok(unsafe { String::from_utf8_unchecked(bytes) }) }
		Err(error) => { Err(fs_err::read_to_string_utf8(error, bytes)) }
	}
}

pub async fn read_dir<F>(path_fn: F) -> Result<ReadDir<F>, fs_err::WithPath<fs_err::ReadDir>>
where
	F: Fn() -> String
{
	match tokio::fs::read_dir(path_fn()).await {
		Ok(read_dir) => { Ok(ReadDir { read_dir, path_fn }) }
		Err(e) => { Err(fs_err::read_dir(e, path_fn())) }
	}
}

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

pub struct ReadDir<F> {
	read_dir: tokio::fs::ReadDir,
	path_fn: F
}


impl<F> ReadDir<F>
where
	F: Fn() -> String
{
	pub async fn next(&mut self) -> Result<Option<tokio::fs::DirEntry>, fs_err::WithPath<fs_err::ReadDirEntry>> {
		self.read_dir.next_entry()
			.await
			.map_err(|e| fs_err::read_dir_entry(e, (self.path_fn)()))
	}
}
