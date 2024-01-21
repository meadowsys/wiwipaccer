// use crate::nom as n;
use super::super::error::*;
use super::super::fs;
use super::Root;
use super::WithOption;
use ::camino::Utf8PathBuf;
use ::std::ops::Deref;

const TEXTURE_META_FILENAME: &str = "texture.wiwimeta";

pub struct WithTexture<'h> {
	pub(super) prev: Root<'h>,
	pub(super) texture_id: &'h str
}

impl<'h> WithTexture<'h> {
	#[inline]
	pub(super) fn _texture_dir(&self) -> Utf8PathBuf {
		let mut path = self._textures_path();
		path.push(self.texture_id);

		path
	}

	#[inline]
	pub async fn texture_dir(&self) -> Result<String> {
		let path = unsafe { self.texture_dir_unchecked() };
		let res = fs::is_dir2(path.clone()).await?;

		if res {
			Ok(path)
		} else {
			let path_name = "texture dir".into();
			Err(Error::PathIsNotDir { path, path_name })
		}
	}

	#[inline]
	pub unsafe fn texture_dir_unchecked(&self) -> String {
		self._texture_dir().into_string()
	}

	#[inline]
	pub async fn texture_manifest(&self) -> Result<String> {
		let path = unsafe { self.texture_manifest_unchecked() };
		let res = fs::is_file2(path.clone()).await?;

		if res {
			Ok(path)
		} else {
			let path_name = "texture manifest".into();
			Err(Error::PathIsNotFile { path, path_name })
		}
	}

	#[inline]
	pub unsafe fn texture_manifest_unchecked(&self) -> String {
		let mut path = self._texture_dir();
		path.push(TEXTURE_META_FILENAME);
		path.into_string()
	}

	#[inline]
	pub fn with_option(self, option_id: &'h str) -> WithOption {
		WithOption { prev: self, option_id }
	}
}

impl<'h> Deref for WithTexture<'h> {
	type Target = Root<'h>;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.prev
	}
}
