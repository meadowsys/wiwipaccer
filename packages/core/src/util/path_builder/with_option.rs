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
	pub(super) option_id: &'h n::option::ID
}

impl<'h> WithOption<'h> {
	#[inline]
	pub(super) fn _option_dir(&self) -> Utf8PathBuf {
		let mut path = self._texture_dir();
		path.push(self.option_id.ref_inner());
		path
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
			Err(Error::PathIsNotDir { path, path_name })
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
			Err(Error::PathIsNotDir { path, path_name })
		}
	}

	#[inline]
	pub unsafe fn option_manifest_unchecked(&self) -> n::global::OptionManifestPath {
		let mut path = self._option_dir();
		path.push(OPTION_META_FILENAME);
		n::global::OptionManifestPath::new(path.into_string())
	}

	#[inline]
	pub fn with_version(self, version_id: &'h n::version::ID) -> WithVersion {
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
