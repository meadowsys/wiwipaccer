use crate::nom as n;
use super::super::error::*;
use super::super::fs;
use super::WithTexture;
use super::WithVersion;
use ::camino::Utf8PathBuf;
use ::std::ops::Deref;

const OPTION_META_FILENAME: &str = "option.wiwimeta";

pub struct WithOption<'h> {
	pub(super) prev: WithTexture<'h>,
	pub(super) option_id: &'h str
}

impl<'h> WithOption<'h> {
	#[inline]
	pub(super) fn _option_dir(&self) -> Utf8PathBuf {
		let mut path = self._texture_dir();
		path.push(self.option_id);
		path
	}

	#[inline]
	pub async fn option_dir(&self) -> Result<n::global::OptionDirPath> {
		self.option_dir2().await.map(n::global::OptionDirPath::new)
	}

	#[inline]
	pub async fn option_dir2(&self) -> Result<String> {
		let path = unsafe { self.option_dir_unchecked2() };
		let res = fs::is_dir2(path.clone()).await?;

		if res {
			Ok(path)
		} else {
			let path_name = "option dir".into();
			Err(Error::PathIsNotDir { path, path_name })
		}
	}

	#[inline]
	pub unsafe fn option_dir_unchecked(&self) -> n::global::OptionDirPath {
		n::global::OptionDirPath::new(self.option_dir_unchecked2())
	}

	#[inline]
	pub unsafe fn option_dir_unchecked2(&self) -> String {
		let path = self._option_dir();
		path.into_string()
	}

	#[inline]
	pub async fn option_manifest(&self) -> Result<n::global::OptionManifestPath> {
		self.option_manifest2().await.map(n::global::OptionManifestPath::new)
	}

	#[inline]
	pub async fn option_manifest2(&self) -> Result<String> {
		let path = unsafe { self.option_manifest_unchecked2() };
		let res = fs::is_file2(path.clone()).await?;

		if res {
			Ok(path)
		} else {
			let path_name = "option manifest".into();
			Err(Error::PathIsNotDir { path, path_name })
		}
	}

	#[inline]
	pub unsafe fn option_manifest_unchecked(&self) -> n::global::OptionManifestPath {
		n::global::OptionManifestPath::new(self.option_manifest_unchecked2())
	}

	#[inline]
	pub unsafe fn option_manifest_unchecked2(&self) -> String {
		let mut path = self._option_dir();
		path.push(OPTION_META_FILENAME);
		path.into_string()
	}

	#[inline]
	pub fn with_version(self, version_id: &'h n::version::ID) -> WithVersion {
		self.with_version2(version_id.ref_inner())
	}

	#[inline]
	pub fn with_version2(self, version_id: &'h str) -> WithVersion {
		WithVersion { prev: self, version_id }
	}
}

impl<'h> Deref for WithOption<'h> {
	type Target = WithTexture<'h>;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.prev
	}
}
