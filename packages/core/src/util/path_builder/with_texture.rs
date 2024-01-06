use crate::nom as n;
use ::camino::Utf8PathBuf;
use super::super::error::*;
use super::super::fs;
use super::Root;
use super::WithOption;

const TEXTURES_DIR: &str = "textures";
const TEXTURE_META_FILENAME: &str = "texture.wiwimeta";

pub struct WithTexture<'r, 't> {
	pub(super) prev: Root<'r>,
	pub(super) texture_id: &'t n::texture::ID
}

impl<'r, 't> WithTexture<'r, 't> {
	#[inline]
	pub(super) fn _texture_dir(&self) -> Utf8PathBuf {
		let mut path = self.prev._root_dir();
		path.push(TEXTURES_DIR);
		path.push(self.texture_id.ref_inner());

		path
	}

	#[inline]
	pub async fn root_dir(&self) -> Result<n::global::RootDirPath> {
		self.prev.root_dir().await
	}

	#[inline]
	pub unsafe fn root_dir_unchecked(&self) -> n::global::RootDirPath {
		self.prev.root_dir_unchecked()
	}

	#[inline]
	pub async fn root_manifest(&self) -> Result<n::global::RootManifestPath> {
		self.prev.root_manifest().await
	}

	#[inline]
	pub unsafe fn root_manifest_unchecked(&self) -> n::global::RootManifestPath {
		self.prev.root_manifest_unchecked()
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
			Err(Error(ErrorInner::PathIsNotDir { path, path_name }))
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
			Err(Error(ErrorInner::PathIsNotFile { path, path_name }))
		}
	}

	#[inline]
	pub unsafe fn texture_manifest_unchecked(&self) -> n::global::TextureManifestPath {
		let mut path = self._texture_dir();
		path.push(TEXTURE_META_FILENAME);
		n::global::TextureManifestPath::new(path.into_string())
	}

	#[inline]
	pub fn with_option(self, option_id: &n::option::ID) -> WithOption {
		WithOption { prev: self, option_id }
	}
}
