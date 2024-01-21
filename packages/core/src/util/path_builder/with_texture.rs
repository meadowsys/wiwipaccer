use crate::nom as n;
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
	pub async fn texture_dir(&self) -> Result<n::global::TextureDirPath> {
		self.texture_dir2().await.map(n::global::TextureDirPath::new)
	}

	#[inline]
	pub async fn texture_dir2(&self) -> Result<String> {
		let path = unsafe { self.texture_dir_unchecked2() };
		let res = fs::is_dir2(path.clone()).await?;

		if res {
			Ok(path)
		} else {
			let path_name = "texture dir".into();
			Err(Error::PathIsNotDir { path, path_name })
		}
	}

	#[inline]
	pub unsafe fn texture_dir_unchecked(&self) -> n::global::TextureDirPath {
		n::global::TextureDirPath::new(self.texture_dir_unchecked2())
	}

	#[inline]
	pub unsafe fn texture_dir_unchecked2(&self) -> String {
		self._texture_dir().into_string()
	}

	#[inline]
	pub async fn texture_manifest(&self) -> Result<n::global::TextureManifestPath> {
		self.texture_manifest2().await.map(n::global::TextureManifestPath::new)
	}

	#[inline]
	pub async fn texture_manifest2(&self) -> Result<String> {
		let path = unsafe { self.texture_manifest_unchecked2() };
		let res = fs::is_file2(path.clone()).await?;

		if res {
			Ok(path)
		} else {
			let path_name = "texture manifest".into();
			Err(Error::PathIsNotFile { path, path_name })
		}
	}

	#[inline]
	pub unsafe fn texture_manifest_unchecked(&self) -> n::global::TextureManifestPath {
		n::global::TextureManifestPath::new(self.texture_manifest_unchecked2())
	}

	#[inline]
	pub unsafe fn texture_manifest_unchecked2(&self) -> String {
		let mut path = self._texture_dir();
		path.push(TEXTURE_META_FILENAME);
		path.into_string()
	}

	#[inline]
	pub fn with_option(self, option_id: &'h n::option::ID) -> WithOption {
		self.with_option2(option_id.ref_inner())
	}

	#[inline]
	pub fn with_option2(self, option_id: &'h str) -> WithOption {
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
