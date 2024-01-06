use crate::nom as n;
use ::camino::Utf8PathBuf;
use super::super::error::*;
use super::super::fs;
use super::WithOption;

const VERSION_META_FILENAME: &str = "version.wiwimeta";

pub struct WithVersion<'h> {
	pub(super) prev: WithOption<'h>,
	pub(super) version_id: &'h n::version::ID
}

impl<'h> WithVersion<'h> {
	#[inline]
	pub(super) fn _version_dir(&self) -> Utf8PathBuf {
		let mut path = self.prev._option_dir();
		path.push(self.version_id.ref_inner());
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
	pub async fn textures_path(&self) -> Result<n::global::TexturesPath> {
		self.prev.textures_path().await
	}

	#[inline]
	pub unsafe fn textures_path_unchecked(&self) -> n::global::TexturesPath {
		self.prev.textures_path_unchecked()
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
		self.prev.option_dir().await
	}

	#[inline]
	pub unsafe fn option_dir_unchecked(&self) -> n::global::OptionDirPath {
		self.prev.option_dir_unchecked()
	}

	#[inline]
	pub async fn option_manifest(&self) -> Result<n::global::OptionManifestPath> {
		self.prev.option_manifest().await
	}

	#[inline]
	pub unsafe fn option_manifest_unchecked(&self) -> n::global::OptionManifestPath {
		self.prev.option_manifest_unchecked()
	}

	#[inline]
	pub async fn version_dir(&self) -> Result<n::global::VersionDirPath> {
		let path = unsafe { self.version_dir_unchecked() };
		let res = fs::is_dir(n::global::Path::new(path.clone().into_inner())).await?;

		if res {
			Ok(path)
		} else {
			let path = path.into_inner();
			let path_name = "version dir".into();
			Err(Error(ErrorInner::PathIsNotDir { path, path_name }))
		}
	}

	#[inline]
	pub unsafe fn version_dir_unchecked(&self) -> n::global::VersionDirPath {
		let path = self._version_dir();
		n::global::VersionDirPath::new(path.into_string())
	}

	#[inline]
	pub async fn version_manifest(&self) -> Result<n::global::VersionManifestPath> {
		let path = unsafe { self.version_manifest_unchecked() };
		let res = fs::is_file(n::global::Path::new(path.clone().into_inner())).await?;

		if res {
			Ok(path)
		} else {
			let path = path.into_inner();
			let path_name = "version manifest".into();
			Err(Error(ErrorInner::PathIsNotDir { path, path_name }))
		}
	}

	#[inline]
	pub unsafe fn version_manifest_unchecked(&self) -> n::global::VersionManifestPath {
		let mut path = self._version_dir();
		path.push(VERSION_META_FILENAME);
		n::global::VersionManifestPath::new(path.into_string())
	}
}
