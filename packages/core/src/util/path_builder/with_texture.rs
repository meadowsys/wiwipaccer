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
	pub(super) texture_id: &'h n::texture::ID
}

impl<'h> WithTexture<'h> {
	#[inline]
	pub(super) fn _texture_dir(&self) -> Utf8PathBuf {
		let mut path = self._textures_path();
		path.push(self.texture_id.ref_inner());

		path
	}

	#[inline]
	pub async fn texture_dir(&self) -> Result<n::global::TextureDirPath> {
		let path = unsafe { self.texture_dir_unchecked() };
		let res = fs::is_dir(n::global::Path::new(path.clone().into_inner())).await?;

		if res {
			Ok(path)
		} else {
			let path = path.into_inner();
			let path_name = "texture dir".into();
			Err(Error::PathIsNotDir { path, path_name })
		}
	}

	#[inline]
	pub unsafe fn texture_dir_unchecked(&self) -> n::global::TextureDirPath {
		let path = self._texture_dir();
		n::global::TextureDirPath::new(path.into_string())
	}

	#[inline]
	pub async fn texture_manifest(&self) -> Result<n::global::TextureManifestPath> {
		let path = unsafe { self.texture_manifest_unchecked() };
		let res = fs::is_file(n::global::Path::new(path.clone().into_inner())).await?;

		if res {
			Ok(path)
		} else {
			let path = path.into_inner();
			let path_name = "texture manifest".into();
			Err(Error::PathIsNotFile { path, path_name })
		}
	}

	#[inline]
	pub unsafe fn texture_manifest_unchecked(&self) -> n::global::TextureManifestPath {
		let mut path = self._texture_dir();
		path.push(TEXTURE_META_FILENAME);
		n::global::TextureManifestPath::new(path.into_string())
	}

	#[inline]
	pub fn with_option(self, option_id: &'h n::option::ID) -> WithOption {
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
