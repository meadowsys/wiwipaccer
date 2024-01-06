use crate::nom as n;
use ::camino::Utf8PathBuf;
use super::WithOption;

const VERSION_META_FILENAME: &str = "version.wiwimeta";

pub struct WithVersion<'r, 't, 'o, 'v> {
	pub(super) with_option: WithOption<'r, 't, 'o>,
	pub(super) version_id: &'v n::version::ID
}

impl<'r, 't, 'o, 'v> WithVersion<'r, 't, 'o, 'v> {
	#[inline]
	pub(super) fn _version_dir(&self) -> Utf8PathBuf {
		let mut path = self.with_option._option_dir();
		path.push(self.version_id.ref_inner());
		path
	}

	#[inline]
	pub fn root_dir(&self) -> n::global::RootDirPath {
		self.with_option.root_dir()
	}

	#[inline]
	pub fn root_manifest(&self) -> n::global::RootManifestPath {
		self.with_option.root_manifest()
	}

	#[inline]
	pub fn texture_dir(&self) -> n::global::TextureDirPath {
		self.with_option.texture_dir()
	}

	#[inline]
	pub fn texture_manifest(&self) -> n::global::TextureManifestPath {
		self.with_option.texture_manifest()
	}

	#[inline]
	pub fn option_dir(&self) -> n::global::OptionDirPath {
		self.with_option.option_dir()
	}

	#[inline]
	pub fn option_manifest(&self) -> n::global::OptionManifestPath {
		self.with_option.option_manifest()
	}

	#[inline]
	pub fn version_dir(&self) -> n::global::VersionDirPath {
		let path = self._version_dir();
		n::global::VersionDirPath::new(path.into_string())
	}

	#[inline]
	pub fn version_manifest(&self) -> n::global::VersionManifestPath {
		let mut path = self._version_dir();
		path.push(VERSION_META_FILENAME);
		n::global::VersionManifestPath::new(path.into_string())
	}
}
