use crate::nom as n;
use ::camino::Utf8PathBuf;
use super::super::error::*;
use super::super::fs;
use super::WithTexture;

const PACK_META_FILENAME: &str = "pack.wiwimeta";
const TEXTURES_DIR: &str = "textures";

pub struct Root<'h> {
	pub(super) root_dir: &'h str
}

impl<'h> Root<'h> {
	#[inline]
	pub(super) fn _root_dir(&self) -> Utf8PathBuf {
		Utf8PathBuf::from(self.root_dir)
	}

	#[inline]
	pub(super) fn _textures_path(&self) -> Utf8PathBuf {
		let mut dir = self._root_dir();
		dir.push(TEXTURES_DIR);
		dir
	}

	#[inline]
	pub async fn root_dir(&self) -> Result<String> {
		let path = unsafe { self.root_dir_unchecked() };
		let res = fs::is_dir(n::global::Path::new(path.clone())).await?;

		if res {
			Ok(path)
		} else {
			let path_name = "root dir".into();
			Err(Error::PathIsNotDir { path, path_name })
		}
	}

	#[inline]
	pub unsafe fn root_dir_unchecked(&self) -> String {
		self.root_dir.into()
	}

	#[inline]
	pub async fn root_manifest(&self) -> Result<String> {
		let path = unsafe { self.root_manifest_unchecked() };
		let res = fs::is_file(n::global::Path::new(path.clone())).await?;

		if res {
			Ok(path)
		} else {
			let path_name = "root manifest".into();
			Err(Error::PathIsNotFile { path, path_name })
		}
	}

	#[inline]
	pub unsafe fn root_manifest_unchecked(&self) -> String {
		let mut path = self._root_dir();
		path.push(PACK_META_FILENAME);
		path.into_string()
	}

	#[inline]
	pub async fn textures_path(&self) -> Result<String> {
		let path = unsafe { self.textures_path_unchecked() };
		let res = fs::is_dir(n::global::Path::new(path.clone())).await?;

		if res {
			Ok(path)
		} else {
			let path_name = "textures path".into();
			Err(Error::PathIsNotFile { path, path_name })
		}
	}

	#[inline]
	pub unsafe fn textures_path_unchecked(&self) -> String {
		self._textures_path().into_string()
	}

	#[inline]
	pub fn with_texture(self, texture_id: &'h str) -> WithTexture {
		WithTexture { prev: self, texture_id }
	}
}
