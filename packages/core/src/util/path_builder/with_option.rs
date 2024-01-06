use crate::nom as n;
use ::camino::Utf8PathBuf;
use super::super::error::*;
use super::super::fs;
use super::WithTexture;
use super::WithVersion;

const OPTION_META_FILENAME: &str = "option.wiwimeta";

pub struct WithOption<'r, 't, 'o> {
	pub(super) prev: WithTexture<'r, 't>,
	pub(super) option_id: &'o n::option::ID
}

impl<'r, 't, 'o> WithOption<'r, 't, 'o> {
	#[inline]
	pub(super) fn _option_dir(&self) -> Utf8PathBuf {
		let mut path = self.prev._texture_dir();
		path.push(self.option_id.ref_inner());
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
		self.prev.texture_dir().await
	}

	#[inline]
	pub unsafe fn texture_dir_unchecked(&self) -> n::global::TextureDirPath {
		self.prev.texture_dir_unchecked()
	}

	#[inline]
	pub async fn texture_manifest(&self) -> Result<n::global::TextureManifestPath> {
		self.prev.texture_manifest().await
	}

	#[inline]
	pub unsafe fn texture_manifest_unchecked(&self) -> n::global::TextureManifestPath {
		self.prev.texture_manifest_unchecked()
	}

	#[inline]
	pub async fn option_dir(&self) -> Result<n::global::OptionDirPath> {
		let path = unsafe { self.option_dir_unchecked() };
		let res = fs::is_dir(n::global::Path::new(path.clone().into_inner())).await?;

		if res {
			Ok(path)
		} else {
			let path = path.into_inner();
			let path_name = "option dir".into();
			Err(Error(ErrorInner::PathIsNotDir { path, path_name }))
		}
	}

	#[inline]
	pub unsafe fn option_dir_unchecked(&self) -> n::global::OptionDirPath {
		let path = self._option_dir();
		n::global::OptionDirPath::new(path.into_string())
	}

	#[inline]
	pub async fn option_manifest(&self) -> Result<n::global::OptionManifestPath> {
		let path = unsafe { self.option_manifest_unchecked() };
		let res = fs::is_file(n::global::Path::new(path.clone().into_inner())).await?;

		if res {
			Ok(path)
		} else {
			let path = path.into_inner();
			let path_name = "option manifest".into();
			Err(Error(ErrorInner::PathIsNotDir { path, path_name }))
		}
	}

	#[inline]
	pub unsafe fn option_manifest_unchecked(&self) -> n::global::OptionManifestPath {
		let mut path = self._option_dir();
		path.push(OPTION_META_FILENAME);
		n::global::OptionManifestPath::new(path.into_string())
	}

	#[inline]
	pub fn with_version(self, version_id: &n::version::ID) -> WithVersion {
		WithVersion { prev: self, version_id }
	}
}
