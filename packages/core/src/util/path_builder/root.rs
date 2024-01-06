use crate::nom as n;
use ::camino::Utf8PathBuf;
use super::super::error::*;
use super::super::fs;
use super::WithTexture;

const PACK_META_FILENAME: &str = "pack.wiwimeta";

pub struct Root<'r> {
	pub(super) root_dir: &'r n::global::RootDirPath
}

impl<'r> Root<'r> {
	#[inline]
	pub(super) fn _root_dir(&self) -> Utf8PathBuf {
		Utf8PathBuf::from(self.root_dir.clone().into_inner())
	}

	#[inline]
	pub async fn root_dir(&self) -> Result<n::global::RootDirPath> {
		let path = unsafe { self.root_dir_unchecked() };
		let res = fs::is_dir(n::global::Path::new(path.clone().into_inner())).await?;

		if res {
			Ok(path)
		} else {
			let path = path.into_inner();
			let path_name = "root dir".into();
			Err(Error(ErrorInner::PathIsNotDir { path, path_name }))
		}
	}

	#[inline]
	pub unsafe fn root_dir_unchecked(&self) -> n::global::RootDirPath {
		self.root_dir.clone()
	}

	#[inline]
	pub async fn root_manifest(&self) -> Result<n::global::RootManifestPath> {
		let path = unsafe { self.root_manifest_unchecked() };
		let res = fs::is_file(n::global::Path::new(path.clone().into_inner())).await?;

		if res {
			Ok(path)
		} else {
			let path = path.into_inner();
			let path_name = "root manifest".into();
			Err(Error(ErrorInner::PathIsNotFile { path, path_name }))
		}
	}

	#[inline]
	pub unsafe fn root_manifest_unchecked(&self) -> n::global::RootManifestPath {
		let mut path = self._root_dir();
		path.push(PACK_META_FILENAME);
		n::global::RootManifestPath::new(path.into_string())
	}

	#[inline]
	pub fn with_texture(self, texture_id: &n::texture::ID) -> WithTexture {
		WithTexture { prev: self, texture_id }
	}
}
